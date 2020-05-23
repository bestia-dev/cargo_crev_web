
# TODO

crate review repeat the scan of authors

- i could find the current version of every crate on crates.io
- with github api search for all crev-proof.
- there find the author_id.
- than add this id to trusted reviewers

the fetch review could open an url, that re-creates the index.
it is a vector of dos attack, but for now don't know what to do. Maybe authentication.
an admin page, and then links to different actions

how to find all crev-proofs repo on github
github api: search repos
<https://api.github.com/search/repositories?q=crev-proofs>

<https://api.github.com/repos/niklasf/crev-proofs/contents/>

"url": "https://api.github.com/repos/LaurenceGA/crev-proofs",
<https://api.github.com/repos/LaurenceGA/crev-proofs/contents>
"git_url": "https://api.github.com/repos/LaurenceGA/crev-proofs/git/blobs/b89bbe6824219875b5bb3fd90d814956b69c8ada",
"content": "IyBjcmV2LXByb29mcwpQdWJsaWMgY3JldiBwcm9vZnMK\n",

 464 on Mac, 501 on google VM

this command does not exist. I must find the id.
cargo crev trust <https://github.com/otavio/crev-proofs>


vec_of_urls 30: [
    "crev-dev/crev-proofs",
    "BurntSushi/crev-proofs",
    "MaulingMonkey/crev-proofs",
    "huitseeker/crev-proofs",
    "MaulingMonkey/rust-reviews",
    "Alxandr/crev-proofs",
    "lo48576/crev-proofs",
    "alaric/crev-proofs",
    "Flakebi/crev-proofs",
    "dmerejkowsky/crev-proofs",
    "roblabla/crev-proofs",
    "LaurenceGA/crev-proofs",
    "thorhs/crev-proofs",
    "Canop/crev-proofs",
    "sphinxc0re/crev-proofs",
    "oherrala/crev-proofs",
    "flodiebold/crev-proofs",
    "scott-wilson/crev-proofs",
    "JamesHinshelwood/crev-proofs",
    "maccam912/crev-proofs",
    "Thinkofname/crev-proofs",
    "pimotte/crev-proofs",
    "cyplo/crev-proofs",
    "jonas-schievink/crev-proofs",
    "confio/crev-proofs",
    "ivanceras/crev-proofs",
    "Gaelan/crev-proofs",
    "HeroicKatora/crev-proofs",
    "hgzimmerman/crev-proofs",
    "niklasf/crev-proofs",
]