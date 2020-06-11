#!/bin/sh

set -e
set -x

PATH=~/.cargo/bin/:${PATH}

mkdir -p ~/.config/crev/proofs/

# This also ensures that the rest of the script won't execute outside a CI environment
ln --force --no-target-directory --symbolic "${PWD}/" ~/.config/crev/proofs/gitlab_com_chrysn_auto-crev-proofs-SQMK-9lvFGG0TNopVnQ0uQ

cargo-crev crev id import < ids/W-RXYmWCrsXJWinxMMdjCjR9ywGlH9srvMi0cmYL2rI.yaml

git config --global user.name "auto-crev-proofs automaton"
git config --global user.email "5935030-auto-crev-bot@users.noreply.gitlab.com"
