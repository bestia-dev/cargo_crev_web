#!/bin/sh

set -x
set -e

# for diagnostics
git log --decorate

git remote add bot-origin https://gitlab-ci-token:"${PUSHER_TOKEN}"@gitlab.com/crev-dev/auto-crev-proofs.git

git push bot-origin HEAD:"${CI_COMMIT_BRANCH}"
