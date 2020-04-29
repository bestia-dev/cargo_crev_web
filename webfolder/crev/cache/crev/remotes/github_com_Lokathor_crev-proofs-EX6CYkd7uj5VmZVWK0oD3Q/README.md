<!-- CREV_README_MARKER_V0 - Please don't remove this first line, or `crev` might overwrite this file.  -->

# crev-proofs

* Lokathor's public-key: ZCBwWlOeJyU79adJqX9-irGH5wrmuYxUPXeSrFKuayg

## Quick Guide

See Also: [Icefox - ActuallyUsingCrev.wikirant](https://wiki.alopex.li/ActuallyUsingCrev)

### Install The tool

`cargo install cargo-crev`

### Create An ID

The ID system uses git URLs, so make a git repo named `crev-proofs` wherever you keep your git stuff. Then create your ID by pointing `crev` at that repository's URL.

`cargo crev new id --url YOUR_GIT_URL`

(**WARNING:** this doesn't actually clone the URL you give it, it just sets up a new git that will want to be uploaded to that URL, and then you'll have to actually fix it to work right later on when you want to publish your work. It's pure stupid.)

You'll be asked for a password to associate with the ID, this is the cryptographic-style "forever, you can never recover this" sort of password. Keep it secret, keep it safe.

It'll print your profile to the terminal and also save it in to a yaml file named after your public key. The file will be located in an OS dependent directory:

* Windows: `%APPDATA%\crev\config\ids`
* Unix: `~/.config/crev/ids/`

The profile info is encrypted so you can totally share it around if you want, but that's not necessary for people to be able to add you.

### Trust Some Folks

The `crev` process doesn't expect you to personally review every crate. Instead you trust some people and they trust some people and everyone reviews a bit and it all gets around.

The general way to mark an ID as trusted is

`cargo crev trust ID_STRING`

This will first prompt you for your own ID's password (since you're editing what your ID trusts), and then it will try to open an editor according to [a best effort process](https://github.com/crev-dev/cargo-crev/blob/44133383da6e7e06d7fcbbb328493763dae75299/crev-lib/src/util/mod.rs#L22-L37). The file opened will have some little spots for you to fill in about your trust of the new ID. Once you're done you just save the file and close the editor and the process is done.

You probably want to trust the creator of `cargo-crev`, or why would you be using their tool:

`cargo crev trust FYlr8YoYGVvDwHQxqEIs89reKKDy-oWisoO0qXXEfHE`

And if you want you can trust me, my public key is up at the top of the file.

### Find More Folks To Trust

Add a repo to your pile of repo URLs really easily:

`cargo crev fetch url REPO_URL_HERE`

And then you can see what ID's are in those URLs with

`cargo crev query id all`

Note that one URL might have more than one ID! This isn't necessarily a problem, unless you for some reason trust one person commiting to that repo but _not_ someone else commiting to that repo. Which sounds a little unlikely to me.

### A Web Of Trust

Once you trust one person, you also trust anyone that they trust. The trust level between you and any particular ID is the _minimum_ trust level in the chain between you and someone else. You can check exactly who is given what trust level at any time.

`cargo crev query id trusted`

### Verify Your Project

From inside your project's root you can check out the trust in all the dependencies it has.

`cargo crev verify`

### Review A Crate

You can review crates, but you must review a _specific version_ of a crate. Down to the exact patch level, and all that.

The easiest way to do this is to go to the directory of your project and then use `crev` to open a new shell into a particular dependency.

`cargo crev goto DEPENDENCY`

This will open a new shell into the root of the version of that dependency that got downloaded from crates.io for your current project.

Well, assuming you have the `SHELL` envrionment set. Otherwise it'll fail and you'll have to go set the `SHELL` value and try again.

In this sub-shell we can do anything we can do in a normal shell.

The message will say that When we're ready you can type `review` without any arguments to start a review, but on Windows at least that's a lie. Instead you have to start a review though the `crev` command

`cargo crev review`

As it says in the ActuallyUsingCrev article at the top there's a number of things that can go wrong at this point because the project is a little young and a little pedantic, but assuming that you get that sorted out you'll be able to start a review.

This will require your ID's password, since you're going to start claiming to trust a thing in a moment.

Then you get a file to fill out like before. There's notes in the comments on how to fill things out.

Save it and exit the editor and you're done with the review.

You can use `exit` to close the sub-shell once you're satisfied.

### Publish Your Work

All of your reviews go into a local folder that's also a git clone of that git url you gave when you made your ID. Just prefix any normal git command with the crev command and it'll execute the command in that git repo without you having to change around your directory.

`cargo crev git GIT_COMMAND`

The actual files on disk are in a folder named after your ID that's near the yaml file location:

* Windows: `%APPDATA%\crev\config\proofs`
* Unix: `~/.config/crev/proofs/`

## Conclusion

As an idea, it's very nice.

As a tool, it's very WIP and has some ergonomics pitfalls to be improved over time.
