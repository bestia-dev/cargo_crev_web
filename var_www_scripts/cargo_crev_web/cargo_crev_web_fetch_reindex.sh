#!/bin/sh
cargo crev repo fetch trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1
curl https://bestia.dev/rust-reviews/reserved_folder/reindex_after_fetch_new_reviews/
