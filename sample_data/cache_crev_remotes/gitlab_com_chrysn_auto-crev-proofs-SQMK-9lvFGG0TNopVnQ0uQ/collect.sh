#!/bin/sh

set -e
set -x

curl https://api.github.com/repos/crev-dev/crev-proofs/forks | jq '.[]["html_url"]' > want.1
curl https://gitlab.com/api/v4/projects/16421315/forks | jq '.[]["web_url"]' > want.2
cat ./others.jsonl > want.3

sort -u want.* > want

grep --no-filename 'url:' */trust/*  | sed 's/.*url:.*\(".*"\)/\1/' | sort -u > have

comm want have -23 | EDITOR="./editor.sh" xargs -n1 ./sign.sh
