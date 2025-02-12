<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

<!-- CREV_README_MARKER_V0 - Please don't remove this first line, or `crev` might overwrite this file.  -->

# Proof Repository

This git repository is a [Crev Proof Repository](https://github.com/crev-dev/crev/wiki/Proof-Repository).

<!-- Feel free to customize this file below this line -->

Self updating non-trusting crev proofs for everyone
===================================================

This is a [crev proof repository](https://github.com/crev-dev/crev/wiki/Proof-Repository)
that is operated automatically
and issues `level: none` proofs for each and every crev proof repository that is easily found online.

Its purpose is to serve as one way for `cargo crev fetch all` to reach all currently published repositories
without any (or too much) manual maintenance.

Do not trust the identity contained herein with a higher level than `none`:
While this promises not to issue any none-"none" trust proofs,
this is run on "public infrastructure" and might be taken over.

Request inclusion in the network
--------------------------------

If your repository is hosted at GitHub or GitLab and was forked from one of the template repositories there
([GitHub](https://github.com/crev-dev/crev-proofs/), [GitLab](https://gitlab.com/crev-dev/crev-proofs/)),
it will be included automatically in the next scheduled run.

Otherwise, you can submit a pull request here
to add a line containing your ID URL
(in a single line, surrounded by double quotes)
in the `others.jsonl` file.

Inner workings
--------------

This repository contains actually four things:

* A crev proof repository
* The encrypted secret key backing the proof repository (the passphrase is entered through the gitlab CI mechanisms)
* A list of points where to look for crev proof repositories
* The mechanism that scrapes those starting points and issues proofs for them.

[//]: # (auto_md_to_doc_comments segment end A)
