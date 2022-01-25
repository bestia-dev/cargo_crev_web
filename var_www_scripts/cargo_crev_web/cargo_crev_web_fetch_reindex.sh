#!/bin/sh
cargo crev repo fetch trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1
curl --silent https://bestia.dev/rust-reviews/reserved_folder/reindex_after_fetch_new_reviews/ | grep -o "Reindex finished"

# for manual searching inside crev files:
#
# cd ~/.cache/crev/remotes
#
# cd ~/.config/crev/proofs/github_com_cargo-crev-web_crev-proofs-NfdERRQ6ONoBLjIp0YbFVw/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/trust
#
# grep -r -i --include "*.crev" huitseeker .
# grep -r -i --include "*.crev" cargo_crev_web .
# grep -r -i --include "*.crev" cargo-crev-web .
# grep -r -i --include "*.crev" auto-crev-proofs .

