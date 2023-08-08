[comment]: # (lmake_md_to_doc_comments segment start A)

# Rust-reviews (working title cargo_crev_web)

[comment]: # (lmake_cargo_toml_to_md start)

**web server to query reviews from cargo-crev**  
***version: 2020.822.1757 date: 2020-08-22 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/cargo_crev_web)***  

![status](https://img.shields.io/badge/maintained-green) 
![status](https://img.shields.io/badge/ready_for_use-green) 

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-4270-green.svg)](https://github.com/bestia-dev/cargo_crev_web/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-508-blue.svg)](https://github.com/bestia-dev/cargo_crev_web/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-588-purple.svg)](https://github.com/bestia-dev/cargo_crev_web/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/cargo_crev_web/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-57-orange.svg)](https://github.com/bestia-dev/cargo_crev_web/)

[comment]: # (lmake_lines_of_code end)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_crev_web/raw/main/LICENSE)
![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/489082694.svg)

Hashtags: #rustlang #buildtool #developmenttool #server #crev #review #web #html  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

<!-- markdownlint-disable MD033 -->
<img src="https://github.com/bestia-dev/cargo_crev_web/raw/main/web_server_folder/web_content_folder/images/snip_01.png" width="400"/><img src="https://github.com/bestia-dev/cargo_crev_web/raw/main/web_server_folder/web_content_folder/images/snip_02.png" width="400"/><img src="https://github.com/bestia-dev/cargo_crev_web/raw/main/web_server_folder/web_content_folder/images/snip_03.png" width="400"/><img src="https://github.com/bestia-dev/cargo_crev_web/raw/main/web_server_folder/web_content_folder/images/snip_04.png" width="400"/><img src="https://github.com/bestia-dev/cargo_crev_web/raw/main/web_server_folder/web_content_folder/images/snip_05.png" width="400"/>
<!-- markdownlint-enable MD033 -->

## Try it out

<https://web.crev.dev/rust-reviews/crates>  

## Motivation

Cargo-crev is a system of review for Rust crates in crates.io.  
<https://github.com/crev-dev/cargo-crev>  
Originally it is a CLI that programmers use on their local machines while developing. The installation of cargo-crev is complicated and involving.  
I would like to make a public cargo-crev web app to query reviews globally. Having a web app will be very good for promoting the crev system.  
This web app will try to show all publicly available reviews, so the community will have a broader understanding of the reviews and crates.  
The web server will use cargo-crev internally to fetch the reviews.  

## CREV - Rust code reviews - Raise awareness

Please, spread this info !\
Open source code needs a community effort to express trustworthiness.\
Start with reading the reviews of the crates on [web.crev.dev](https://web.crev.dev/rust-reviews/crates). \
Then install the GUI [cargo_crev_reviews](https://crates.io/crates/cargo_crev_reviews) or the CLI [cargo-crev](https://github.com/crev-dev/cargo-crev)\.  
Your personal reviews are most important. If you have a boss, he will sooner or later ask you if you reviewed all the dependencies. With [cargo_crev_reviews](https://crates.io/crates/cargo_crev_reviews) you have a basic tool to do that. \
Write your reviews! Describe the crates you trust. Or warn about the crate versions you think are dangerous. Publish and share your opinion with other developers.\

## crates.io and lib.rs

A similar web page is also created by @Kornelski at <https://lib.rs/crates/num-traits/crev>.  
lib.rs is an alternative index to crates.io.  
Crates.io is the official Rust-lang storage of crates source code that is trusted and immutable.  
Lib.rs is focused on searching for crates in a categories with handy additional information.  

## warp

Warp is a web server written in Rust.  
<https://github.com/seanmonstar/warp>  
It will listen on port 8051 listens to http.  

## Google vm

A working web server is installed on my google vm.  
There is a nginx server reverse proxy that accepts https http2 on 443 and relay to internal 8051. Nginx also redirects all http 80 to https 443.  
In sites-available/default I added this lines:

```nginx
#region rust-reviews
    #without the trailing / it is not a directory (for the server and for the browser)
    #do the browser redirect 301
    location = /rust-reviews {
      return 301 /rust-reviews/;
    }
    #folder name only, pass to index.html on port 8051
    location = /rust-reviews/ {
      proxy_pass http://127.0.0.1:8051/index.html;
      proxy_buffering off;
    }
    # the trailing / after both of these lines means this route is not appended to the forwarding
    location /rust-reviews/ {
      proxy_pass http://127.0.0.1:8051/;
      proxy_buffering off;
    }
  #endregion
```

## screen (Linux program)

My application will run in background with the command "screen" with a session_name.  
So I can see all the stdout of the application easily.  
create a new session  
`screen -S cargo_crev_web_8051`,  
connect to an existing session  
`screen -r cargo_crev_web_8051`,  
Set the credentials for cargo-crev CLI in the env variable:  
`(space) export CREV_PASSPHRASE=your_passphrase`  
Warning: never write secrets in code or files that are published on Github.  
Add a space before the command to avoid it to be saved in bash history.
start the web server  
`cd /var/www/webapps/cargo_crev_web; ./cargo_crev_web`  
If you want to scroll the screen session:  
`ctrl+a ESC`  
scroll with arrows or page-up, page-down. End scroll mode with:  
`ESC`  
detach the session  
`ctrl+a d`  

## install cargo-crev to fetch reviews

On my web server I want to fetch the cargo-crev reviews from GitHub in regular intervals.  
I need to install cargo-crev.  
My first attempt was to install Rust and cargo with rustup with minimal profile.
`curl https://sh.rustup.rs -sSf | sh -s -- --profile minimal`  
Then I tried to install cargo-crev with cargo:  
`cargo install cargo-crev`  
It was a disaster. I have the smallest, tiniest possible VM and it looks that compiling the source code of cargo-crev is too much for it. I tried 3 times, waited for a few hours and it didn't succeed. I then deleted the big folder `~/.cargo/registry/src` to free some disk space.  
Fortunately there is a binary release already compiled here:  
`https://github.com/crev-dev/cargo-crev/releases/download/v0.16.1/cargo-crev-v0.16.1-x86_64-unknown-linux-musl.tar.gz`  
I unzip it twice and saved the binary file `cargo-crev` in:  
`~/.cargo/bin`  

## GitHub crev-proofs

I followed the instructions [getting_started](https://github.com/crev-dev/cargo-crev/raw/main/cargo-crev/src/doc/getting_started.md).  
I created a new GitHub user: `cargo-crev-web`. I wanted to name it `cargo_crev_web`, but underscore is not allowed :-( So now I have inconsistent names :-(  
I used my second email, because my first email is used for my personal GitHub bestia-dev.  
On the google vm web server I created an SSH key and copied the key to GitHub to have SSH access.  
I forked the template <https://github.com/crev-dev/crev-proofs>.  
For git fetch and reindex I will open a new screen session:  
`screen -S cargo_crev_web_git`  
to reconnect later:  
`screen -r cargo_crev_web_git`  
I will need the credentials for ssh for GitHub:  
`eval $(ssh-agent -s)`  
`ssh-add ~/.ssh/bestia2_for_github`  
create new crev id with my new github repo:  
`cargo crev id new --url https://github.com/cargo-crev-web/crev-proofs`  
add the trusted user `dpc`, the author of cargo-crev:  
`cargo crev trust --level medium https://github.com/dpc/crev-proofs`  
Push my crev data to github:  
`cargo crev repo publish`  

## trusted reviewers

For the purpose of showing all public reviews, `cargo_rev_web` will "trust --level low" everybody.  
I will personally, manually maintain this list.  
I have a function that searches Github for all crev-proofs repositories. Beside this, the command `cargo crev id query all` returns a list of all repos found in locally cached crev files (trusted people from trusted people).
Then I manually check every repository if it's correct: it has to contain a crev-id and some reviews in the correct format.  
If the repo is correct then "cargo crev trust --level low repo". If not I add it to "blocklisted repos" with a description what is wrong.  

## Linux scheduler

I need to call every hour:  
`cargo crev repo fetch trusted`
to have fresh reviews available locally in `~/.cache/crev/`.  
The Linux scheduler `crontab` is ok, but I miss something more visual.  
I wrote <https://github.com/bestia-dev/foreground_scheduler> to do this.  
It is a normal CLI and it is easy to read the output on the screen.  
To make this run indefinitely in another terminal session I use `screen`.
Open a new screen session:  
`screen -S cargo_crev_web_git`  
to reconnect later:  
`screen -r cargo_crev_web_git`  
The script is stored in `/var/www/scripts/cargo_crev_web_fetch_reindex.sh`
I run it (every 5th minute of every hour):  
`foreground_scheduler 05 /bin/bash "/var/www/scripts/cargo_crev_web/cargo_crev_web_fetch_reindex.sh"`  
To stop it:  
`ctrl+c`  

## testing .cache/crev

Not all data is required in every review, so I need to test examples that contains different data.  
<https://web.crev.dev/rust-reviews/crate/btoi>  alternatives  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  issues  
<https://web.crev.dev/rust-reviews/crate/protobuf>  advisory old  
<https://web.crev.dev/rust-reviews/crate/inventory>   advisories

Locally in development is the same, just the server is 127.0.0.1:8051/.  

## html templating

Like many developers I also suffer from "graphical designitis".  
It is very hard for me to find the exact color variations and shape proportions and subtle font differences to make a web page beautiful. It is not lack of knowledge of html and css. It is lack of style and taste.  
Simply unrepairable!  
So I created a simple html templating system to separate the graphical designer work from the developer work. As much as possible.  
First the graphical designer prepares a nice html+css with static data, that looks awesome.  
The 2 files are on his local disk and don't need any server or configuration. The static data must be as realistic as possible.  
Then I add comments that are commands where to insert the dynamic data. This comments don't destroy the original html. That html can still be visualized statically from the disk. It is easy to add to or modify the design. Just avoid to touch the comments.  
On the web server the HtmlTemplating trait takes the template and inserts the dynamic data.  
The result is normal html and is sent to the browser.

## markdown md to html

I use the Rust comrak lib to convert the review comment from md to html.  

## Badges

A service for SVG badges for `crev count of reviews for one crate` is on url:\
<https://web.crev.dev/rust-reviews/badge/crev_count/reader_for_microxml.svg>\
Example how it looks like on GitHub:\
<https://github.com/bestia-dev/reader_for_microxml/>  

```markdown
[![crev reviews](
https://web.crev.dev/rust-reviews/badge/crev_count/reader_for_microxml.svg
)](https://web.crev.dev/rust-reviews/crate/reader_for_microxml/)
```

### xhtml validator

Manually coded xhtml can be full of "bugs".\
The validator will quickly find them out.\
I would like to have this tool inside VsCode.\
<https://www.liquid-technologies.com/online-xml-validator>

## CodeTour

I like very much the VSCode extension CodeTour.  
It makes a special kind of documentation that shows the code flow.  
No other type of documentation is so useful as this.  
It works only in VSCode. I made an export to md utility because is so much easier to distribute the md file around.  

## javascript Click to Copy

I used <https://clipboardjs.com/> for Click to Copy.  
For tooltips I used <https://chrisbracco.com/a-simple-css-tooltip/>  

## colors

Every person on this planet would like to have different colors. That is human nature. Or just nature's statistical dispersion to increase chance of survival when the environment changes dramatically...  
To build a website that satisfies everybody's taste for color is impossible.  
Even hoping to satisfy a few close friends is mission impossible.  
So there is this Chrome extension named User Css:  
<https://chrome.google.com/webstore/detail/user-css/okpjlejfhacmgjkmknjhadmkdbcldfcb>  
You write a css in your browser and while you are typing it immediately changes the look of the webpage.  
In the webpage <https://web.crev.dev/rust-reviews/> you can find a basic css for the color palette. Just Click-to-Copy and try it in User Css. Then change colors to your liking.  
At the end save the css for the next time you visit the website.  
If somebody is very happy with his/her user css, please send it to me. I will make it available to other users. They will be happy. Humans love colors - different colors.  

## ideas

The same web server can easily run on the local machine of the developer.  
It is just one single binary executable file.  
It will read only the trusted reviews specific to that developer.  
So now we have a local web server and a browser. It means we have now the possibility to make a beautiful GUI for cargo-crev that works on any OS and remotely also. Good.  
This local server will ue crev-lib to access the crev functionality.  

## Stats

<https://github.com/bestia-dev/cargo_crev_web/issues/4>

## References

<https://github.com/rustomax/rust-iterators>  

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[comment]: # (lmake_md_to_doc_comments segment end A)
