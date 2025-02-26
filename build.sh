#!/bin/bash

# List all subfolders
PROJECTS=$(ls -d */)

# Iterate over each one
for dir in $PROJECTS; do

  # Only process if there is a Cargo.toml
  if [ -f "$dir/Cargo.toml" ]; then

    echo "Building $dir"

    # Change to folder and build
    (cd "$dir" && cargo build)

  fi

done