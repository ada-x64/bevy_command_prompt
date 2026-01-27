export CARGO_TERM_COLOR="always"
export PYTHONUNBUFFERED=1
export RUSTC_WRAPPER="sccache"

if [[ -n "$SSH_CLIENT" ]]; then
   export FEATURES=""
else
    export FEATURES="dylib"
fi

if [[ -n "$CI" ]]; then
    export MISE_ENV="ci"
    # export RUSTFLAGS="-Clink-arg=-fuse-ld=$(which mold)"
elif [[ -n "$SSH_CLIENT" ]]; then
    export MISE_ENV="ssh"
fi
