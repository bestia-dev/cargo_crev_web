#!/bin/sh

set -e

# It appears that `sed -i` is racy against cargo-crev's "did you really write"
# protection -- or it's just a matter of file system timestamp granularity.
sleep 2
sed -i 's/^trust: .*/trust: none/' "$1"
sleep 1
grep -q 'trust: none' "$1"
