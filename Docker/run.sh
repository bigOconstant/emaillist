#!/bin/bash

export PATH="$HOME/.cargo/bin:$PATH";

echo "Running migration";

diesel migration run;


exec "$@"

