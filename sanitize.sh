#!/bin/bash

# This script allows you to test the sanitization step of the grading pipeline on your files. Ensure
# that you have your changes commited before running this script so that you can reset the files to
# the state before sanitization. with `git reset --hard HEAD`

# Workflow when using this file:
# 1. Make sure everything is commited and there are no untracked files with `git status`
# 2. Run this script with `./sanitize.sh`
# 3. Check if the code compiles with your IDE or with `cargo test --workspace`
# 4. Identify issues to fix (e.g., print, assert or dbg not on a single line)
# 5. Undo sanitization with `git reset --hard HEAD`
# 6. Fix issues and commit fixes
# 7. Repeat until everything compiles.

# Since the private test files are overridden by the pipeline, we exclude them from sanitization.

find ./src -type f ! -name '*tests*.rs' -exec sed -i '/#\s*\[.*test.*\]/s/^/\/\/ /g' {} \;
find ./src -type f ! -name '*tests*.rs' -exec sed -i '/print\|assert\|dbg!/s/^/\/\/ /g' {} \;
