export CARGO_TERM_COLOR="always"

if [[ -n "$SSH_CLIENT" ]]; then
   export FEATURES=""
else
    export FEATURES="dylib"
fi

if [[ -n "$CI" ]]; then
    export MISE_ENV="ci"
elif [[ -n "$SSH_CLIENT" ]]; then
    export MISE_ENV="ssh"
fi
