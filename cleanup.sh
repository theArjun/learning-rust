#!/bin/bash

# Find and delete all target directories in Rust projects
find . -type d -name "target" -exec rm -rf {} \;

# Find and delete any executable files that might be outside target directories
find . -type f -executable -not -path "./.git/*" -not -name "*.sh" -exec rm -f {} \;

echo "Rust project cleanup complete."