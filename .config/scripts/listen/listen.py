#!/usr/bin/env -S uv run --script

from http.server import BaseHTTPRequestHandler, HTTPServer
import subprocess
import os
from pathlib import Path
import json
from dataclasses import dataclass
import shlex
from enum import Enum
from argparse import ArgumentParser
from typing import cast, TypeVar
from threading import Thread
import urllib.request
import signal
import re

# data

# todo: probably get this from an env variable or argument
# echo pepperwepper | sha256sum
pw = "08c505587bc1244785fb5311e8b0498adebe05117a145fe53215c2971db33d14"
env_expr = re.compile(r"(\w+)=(\"[^\"]*\"|'[^']*'|[^\\w]+)")


class Mode(Enum):
    Host = "host"
    Client = "client"


class ReqKind(Enum):
    Open = "open"
    Kill = "kill"
    Log = "log"


@dataclass
class RequestData:
    auth: str
    kind: ReqKind


@dataclass
class OpenReq(RequestData):
    kind = ReqKind.Open
    path: str
    args: str
    env: str


@dataclass
class KillReq(RequestData):
    kind = ReqKind.Kill


@dataclass
class LogReq(RequestData):
    kind = ReqKind.Log
    msg: str


Req = TypeVar("Req", OpenReq, KillReq, LogReq)


# parse args
@dataclass
class Args:
    mode: Mode


parser = ArgumentParser(add_help=False)
_ = parser.add_argument(
    "--mode", choices=[m.value for m in Mode], default=Mode.Client.value
)

args = parser.parse_args()
args = cast(Args, args)


process: subprocess.Popen | None = None
log_thread: Thread | None = None
partner_url: str | None = None


# handle http reqs
class RequestHandler(BaseHTTPRequestHandler):
    disable_log = False

    def send(self, code: int, message: str):
        self.send_response_only(code, message)
        self.send_header("Server", self.version_string())
        self.send_header("Date", self.date_time_string())
        self.send_header("Content-Type", "text/plain")
        self.end_headers()
        self.wfile.write(message.encode())

    def parse_data(self):
        global partner_url
        partner_url = f"http://{self.client_address[0]}:9995"
        body = self.rfile.read(int(self.headers["Content-Length"]))
        data = json.loads(body.decode())
        if data is None or "kind" not in data:
            self.log_error("data is none or no kind found")
            self.send(400, "Bad request\n")
            return None
        if data["kind"] not in [k.value for k in ReqKind]:
            self.log_error("data is none or no kind found")
            self.send(400, f"Unknown kind '{data['kind']}'")
            return None
        if "auth" not in data or data["auth"] != pw:
            self.log_error("Bad auth")
            self.send(401, "Unauthorized\n")
            return None
        return data

    def open_req(self, data: OpenReq):
        path = Path(data.path).expanduser()
        if os.path.exists(path):
            args = shlex.split(data.args)
            if data.env != "":
                env = data.env.split(" ")
                env_dict = dict()
                for s in env:
                    match = env_expr.match(s)
                    if match is not None:
                        env_dict[match.group(1)] = match.group(2)
                    else:
                        self.log_error(f"Invalid env variable! {s}")
                        self.send(400, f"Invalid env variable! {s}")
                        return
                env = {**os.environ, **env_dict} if data.env else None
            else:
                env = None
            self.log_message(f"Running {path} {args}")
            global process, log_thread
            process = subprocess.Popen(
                [path, *args],
                env=env,
                stderr=subprocess.STDOUT,
                stdout=subprocess.PIPE,
                bufsize=1,
                text=True,
            )
            log_thread = Thread(target=self.mk_send_client_logs())
            log_thread.start()
            self.send(200, "OK\n")
        else:
            self.send(404, "Not found\n")

    def kill_req(self, data: KillReq):
        global process
        if process is None:
            self.log_message("Got kill request when no process was running")
            self.send(500, "Nothing to kill\n")
            process = None
            return
        self.log_message("Got kill request, killing ")
        process.kill()
        process.wait()
        process = None
        self.send(200, "Process closed\n")

    def client_post(self):
        if process is not None:
            self.log_message("Killing current process...")
            process.kill()
            process.wait()
        data = self.parse_data()
        if data is None:
            self.send(400, "Bad request\n")
            return

        match data["kind"]:
            case ReqKind.Kill.value:
                self.kill_req(KillReq(**data))
            case ReqKind.Open.value:
                self.open_req(OpenReq(**data))
            case _:
                self.send(400, "Bad request\n")

    def host_post(self):
        self.disable_log = True
        data = self.parse_data()
        if data is None:
            return
        match data["kind"]:
            case ReqKind.Log.value:
                data = LogReq(**data)
                print(data.msg, end="")
                self.send(200, "recv'd")
            case ReqKind.Kill.value:
                data = KillReq(**data)
                self.send(200, "recv'd")
                exit(0)

            case _:
                self.send(400, "Bad request\n")

    def do_POST(self):
        if args.mode == Mode.Client.value:
            self.client_post()
        else:
            self.host_post()

    def mk_send_client_logs(self):
        host = f"http://{self.client_address[0]}:9995"
        return lambda: send_client_logs(host)

    def log_message(self, format: str, *args):
        if not self.disable_log:
            super().log_message(format, *args)


def send_req(data_in: dict[str, str], host: str):
    try:
        data = json.dumps(data_in).encode("utf-8")
        req = urllib.request.Request(
            url=host,
            data=data,
            headers={"content-type": "application/json"},
        )
        # discard response for now
        _resp = urllib.request.urlopen(req)
    except Exception as _:
        pass


# collect new stdout on timeout, then pipe to host
def send_client_logs(host: str):
    print("(syncing logs...)")
    global process
    if process is not None:
        while process.poll() is None:
            if process.stdout is not None:
                for line in process.stdout:
                    print(line)
                    data = {"auth": pw, "msg": line, "kind": ReqKind.Log.value}
                    send_req(data, host)
        process = None
        data = {"auth": pw, "kind": ReqKind.Kill.value}
        send_req(data, host)
        print("(closing process thread)")


def catch_signal(sig, frame):
    if partner_url is not None:
        if log_thread is not None:
            log_thread.join(0)
        send_req({"auth": pw, "kind": ReqKind.Kill.value}, partner_url)
    exit(0)


def run(server_class=HTTPServer, port=9995):
    server_address = ("", port)
    httpd = server_class(server_address, RequestHandler)
    print(f"Serving on port {port} (mode={args.mode})")
    httpd.serve_forever()


if __name__ == "__main__":
    signal.signal(signal.SIGINT, catch_signal)
    run()
