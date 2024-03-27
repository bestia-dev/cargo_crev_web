#!/bin/sh

set -x

PATH=~/.cargo/bin/:${PATH}

printf "${CREV_SECRET_KEY}\n" | cargo-crev crev trust "$1" --level none

RESULT=$?

# "No identities found" is an OK result; many people fork and then just don't put IDs in
test ${RESULT} -eq 254 && exit 0

exit ${RESULT}
