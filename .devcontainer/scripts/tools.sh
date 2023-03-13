#!/bin/bash
# install tools for container standup
echo "cwd: $(pwd)"
echo "---getting tools---"
. .devcontainer/scripts/rust.sh
. .devcontainer/scripts/websocat.sh
. .devcontainer/scripts/preCommit.sh
echo "---tools done---"
