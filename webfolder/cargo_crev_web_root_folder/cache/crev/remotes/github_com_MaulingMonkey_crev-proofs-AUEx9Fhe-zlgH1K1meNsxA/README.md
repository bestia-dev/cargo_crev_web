<!-- CREV_README_MARKER_V0 - Please don't remove this first line, or `crev` might overwrite this file.  -->

# Proof Repository

This git repository is used a [Crev Proof
Repository](https://github.com/dpc/crev/wiki/Proof-Repository).

<!-- Feel free to customize this file below this line -->

# My use

I mostly care about malicious actors gaining access to data, or causing data loss.  So, any kind of RCE is a
concern (buffer overflows, SQL or shell injection attacks, network exposed commands, etc.) as is unsound code,
as it may be leveraged to create RCEs.  In code, this means paying special attention to:

* Test Coverage
* Fuzzing Coverage
* `unsafe` code
* FFI types and definitions
* `std::process::Command` use
* `std::fs::*`, `std::path::*`, or other filesystem use (possible data exfil, user controlled data)
* `std::net::*` or other network use (possible data exfil, user controlled data)
* `std::io::*` use (although these are usually mostly fine)
* `std::os::*` equivalents to the above.

I care less about "mere" Denial of Service attacks, even if those are still exploitable to ransom businesses,
although intentional vulnerabilities will still be grounds for completely distrusting their author.

# Personal Trust Criteria

Subject to change.

| trust     | criteria  |
| --------- | --------- |
| high      | Myself
| medium    | People I know offline
| low       | People I know online
| none      | People I don't know
| distrust  | People I believe have written malicious code, or have simply written far too much unsound code without good reason

# Personal Review Criteria

Subject to change.

| rating    | criteria |
| --------- | -------- |
| strong    | 100% safe code, good docs, good tests
| positive  | 100% sound code, good docs, good tests (FFI crates excepted on the docs/tests front.)
| neutral   | History of soundness issues, or possibly just rife enough with unsafe without having sufficient test coverage.
| negative  | Current soundness issues, or a history of poor responses to soundness issues
| ~~dangerous~~ | ~~Current soundness issues, or a history of poor responses to soundness issues~~ This level does not currently exist.

# Official Trust Criteria

| trust     | criteria |
| --------- | -------- |
| high      | "for most practically purposes, I trust this ID as much or more than myself" eg. "my dayjob ID", "known and reputatable expert", "employee within my team"
| medium    | typical, normal level of trust
| low       | "I have some reservations about trusting this entity"
| none      | "I don't actually trust this entity"; use to revoke trust (or distrust) from a previously issued Trust Proof
| distrust  | "I distrust this person and so should you"

# Official Review Criteria

| rating    | criteria |
| --------- | -------- |
| strong    | secure and good in all respects, for all applications
| positive  | secure and ok to use; possibly minor issues
| neutral   | secure but with flaws
| negative  | severe flaws and not ok for production usage
| ~~dangerous~~ | ~~unsafe to use; severe flaws and/or possibly malicious~~ This level does not currently exist.

| thoroughness | criteria |
| ------------ | -------- |
| high      | long, deep, focused review - possibly as a part of a formal security review; "hour or more per file"
| medium    | a standard, focused code review of a decent depth; "~15 minutes per file"
| low       | low intensity review: "~2 minutes per file"
| none      | no review, incomplete review, or just skimming; "seconds per file"

| understanding | criteria  |
| ------------- | --------- |
| high          | complete understanding
| medium        | good understanding
| low           | some parts are unclear
| none          | lack of understanding

# Review Concerns

## Unsafe Code

Common mistakes include:

* Using transmute or pointer casts to...
    * ...alias `&mut T` and `&T` to the same memory, [which is undefined behavior](https://doc.rust-lang.org/nomicon/aliasing.html).  Examples: [rdrand#13](https://github.com/nagisa/rust_rdrand/issues/13)
    * ...alias structs not marked [#\[repr(transparent)\]](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent), [which is undefined behavior](https://doc.rust-lang.org/nomicon/transmutes.html).  Examples: [ascii#65](https://github.com/tomprogrammer/rust-ascii/issues/65)
    * ...convert mutable references to Rust enums, into references to their underlying Rust reprs, when the enum doesn't exhaustively list every possible value.  Examples:  [ascii#64](https://github.com/tomprogrammer/rust-ascii/issues/64)
* Using std::mem::uninitialized.  It's deprecated in favor of `MaybeUninit`, as it turns out [it was always undefined behavior](https://doc.rust-lang.org/std/mem/fn.uninitialized.html) to use uninitialized.
* Using `static mut`s.  It's not yet deprecated, but it's [very hard to use correctly](https://github.com/rust-lang/rust/issues/53639) and may be removed/deprecated in the future - prefer `static UnsafeCell`s instead.  Examples: [lazy_static#117](https://github.com/rust-lang-nursery/lazy-static.rs/issues/117)?
* Double frees.  Examples: [smallvec#148](https://github.com/servo/rust-smallvec/issues/148)

## FFI Code

Common mistakes include:

* Dereferencing unvalidated pointers.  Examples:  [jni#197](https://github.com/jni-rs/jni-rs/issues/197)
* Using FFI on structs that are not [#\[repr(C)\]](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc) (or [#\[repr(transparent)\]](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent) for newtypes)
* Using FFI on structs that don't match the equivalent C layout/alignment/???
* Using FFI on fns without using the correct [ABI](https://doc.rust-lang.org/reference/items/external-blocks.html#abi)
* Using FFI to create rust enums from C enums.  Creating a Rust enum with an unlisted value is undefined behavior, no matter which repr you use.  Use structs instead.  Examples: [bindgen#667](https://github.com/rust-lang/rust-bindgen/issues/667)

## "Safe" APIs

Common concerns:

* Using `std::process::Command` to create shell commands or run shell scripts without the proper shell escaping.
* Using `std::fs::*`, `std::path::*`, or `std::os::*` equivalents to access filesystem data without proper path sanitization, or to get untrusted user input.
* Using `std::net::*` to get untrusted user input or send unexpected telemetry.
* Third party crates that extend network, filesystem, scripting, or execution access.
