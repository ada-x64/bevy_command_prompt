use crate::prelude::*;

pub trait ConsoleCommand: clap::Parser + Message {}
impl<T> ConsoleCommand for T where T: clap::Parser + Message {}

pub trait CommandExt {
    /// Registers a console command to the application.
    fn add_console_command<T: ConsoleCommand>(&mut self) -> &mut Self;
}
impl CommandExt for App {
    // TODO: If a command is not initialized, it will panic.
    // Make sure to add a nice error message if possible.
    fn add_console_command<T: ConsoleCommand>(&mut self) -> &mut Self {
        self.add_message::<T>();
        let cmd = T::command().no_binary_name(true);
        let name = cmd.get_name();
        let dispatch = self.world_mut().register_system(dispatch_cmd::<T>);
        let mut cmds = self.world_mut().resource_mut::<ConsoleCommands>();
        cmds.insert(name.to_string(), ConcreteConsoleCommand { cmd, dispatch });
        self
    }
}

fn dispatch_cmd<T: ConsoleCommand>(
    raw_cmd: In<String>,
    mut writer: MessageWriter<T>,
    mut commands: Commands,
) {
    let res: Result<(), String> = (|| {
        let split = shlex::split(raw_cmd.as_str()).ok_or("Invalid quoting".to_string())?;
        let res = T::try_parse_from(split.iter()).map_err(|e| format!("{e}"))?;
        writer.write(res);
        Ok(())
    })();
    if let Err(e) = res {
        commands.trigger(ConsolePrint(e))
    }
}
