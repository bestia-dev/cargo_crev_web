# cargo_crev_web

[comment]: # (lmake_readme cargo.toml data start)
version: 2020.530.1604  date: 2020-05-30 authors: Luciano Bestia  
**web server to query reviews from cargo-crev**

[comment]: # (lmake_readme cargo.toml data end)  

## cargo-crev

Cargo-crev is a system of review for rust crates in crates.io.  
<https://github.com/crev-dev/cargo-crev>  
Originally it is a CLI that programmers use on their local machines while developing.  
I would like to make a public cargo-crev web app to query reviews globally.  
The installation of cargo-crev is complicated and involving.  
Having a web app will be very good for promoting the system.  
The basis of cargo-crev is a list of trusted individuals.  
For the web it would be a broader number of people to achieve more understanding in the community.  
The same machine will have the web server and the git repository for cargo-crev.  

## crates.io and lib.rs

A similar web page is also created by @Kornelski at <https://lib.rs/crates/num-traits/crev>.  
lib.rs is an alternative index to crates.io.  
Crates.io is official rust-lang server, focused more on the trusted storage of crates. It does near to nothing for searching a crate.  
Lib.rs is more focused on making easier to find a crate in a category. The code is still stored on crates.io. So the trust of authenticity of the code is high.  

## warp

Warp is a web server written in rust.  
<https://github.com/seanmonstar/warp>  
It will listen on port 8051 listens to http.  

## Google vm

One beta working server is installed on my google vm.  
There is a nginx server reverse proxy that accepts https http2 on 443 and relay to internal 8051.
Nginx also redirects all http 80 to https 443.  
In sites-available/default I added this lines:

```nginx
#region cargo_crev_web
    #without the trailing / it is not a directory (for the server and for the browser)
    #do the browser redirect 301
    location = /cargo_crev_web {
      return 301 /cargo_crev_web/;
    }
    #folder name only, pass to index.html on port 8051
    location = /cargo_crev_web/ {
      proxy_pass http://127.0.0.1:8051/index.html;
      proxy_buffering off;
    }
    # the trailing / after both of these lines means this route is not appended to the forwarding
    location /cargo_crev_web/ {
      proxy_pass http://127.0.0.1:8051/;
      proxy_buffering off;
    }
  #endregion
```

The application will be in background with the command "screen" with a session_name.  
So I can see all the stdout of the application easily.  
create a new session  
`screen -S cargo_crev_web_8051`,  
connect to an existing session  
`screen -r cargo_crev_web_8051`,  
start the web server  
`cd /var/www/webapps/cargo_crev_web; ./cargo_crev_web`
detach the session
`ctrl+a d`

## install cargo-crev to fetch reviews

On my web server I want to fetch the cargo-crev reviews from GitHub in regular intervals.  
I need to install cargo-crev.  
My first attempt was to install rust and cargo with rustup with minimal profile.
`curl https://sh.rustup.rs -sSf | sh -s -- --profile minimal`  
Then I tried to install cargo-crev with cargo:  
`cargo install cargo-crev`  
It was a disaster. I have the smallest, tiniest possible VM and it looks that compiling the source code of cargo-crev is too much for it. I tried 3 times, waited for a few hours and it didn't succeed.  
Fortunately there is a binary release already compiled here:  
`https://github.com/crev-dev/cargo-crev/releases/download/v0.16.1/cargo-crev-v0.16.1-x86_64-unknown-linux-musl.tar.gz`  
I unzip it twice and saved the binary file `cargo-crev` in:  
`~/.cargo/bin`  
I could use it already and fetch all the repos, but that is not super safe. Better is to fetch only the trusted repos.  
For this I need to create a crev Id and for that I need to have a GitHub repo.  
Size of .cache/crev
On my local machine is 7 MB
On web server 2 MB
It looks that it is not extremely big.

## GitHub crev-proofs

I followed the instructions <https://github.com/crev-dev/cargo-crev/blob/master/cargo-crev/src/doc/getting_started.md>  
I created a new GitHub user: `cargo-crev-web`. I wanted cargo_crev_web, but I couldn't. So I have inconsistent name here.  
I used my second email, because my first email is used for my personal GitHub LucianoBestia.  
On the google vm web server I created an SSH key and copied the key to GitHub to have SSH access.  
I forked the template <https://github.com/crev-dev/crev-proofs>.  
For fetch I will open a new screen session:  
`screen -S cargo_crev_web_git`  
to reconnect later: `screen -r cargo_crev_web_git`  
I will need the credentials for ssh for GitHub:  
`eval $(ssh-agent -s)`  
`ssh-add ~/.ssh/bestia2_for_github`  
create new crev id with my new github repo:  
`cargo crev id new --url https://github.com/cargo-crev-web/crev-proofs`  
add a trusted user:  
`crev id trust <hash>`  
example for dpc - Dawid Ciężarkiewicz, the author of cargo-crev. I trust him:  
`cargo crev id trust FYlr8YoYGVvDwHQxqEIs89reKKDy-oWisoO0qXXEfHE`  
it is possible also to trust a repo:  
`cargo crev trust <url of someone's crev-proofs repo>`  
At the end of editing the local data push:  
`cargo crev repo publish`  

## trusted authors

For the purpose of showing most existing reviews, the cargo_rev_web will "trust" anybody.  
It is not really trusting, it is just showing their reviews.  
The repo <https://gitlab.com/crev-dev/auto-crev-proofs> contains all of the proof repos.  
It is automated and maintained by @chrysn.  
<https://github.com/crev-dev/cargo-crev/issues/336>  
For incomplete, obsolete or otherwise unwanted repos I will have an editable blacklist.  

## Linux scheduler

I need to call every hour:  
`cargo crev repo fetch trusted`
to have fresh reviews available locally in `~/.cache/crev/`.  
The Linux scheduler `crontab` is ok, but I miss something more visual.  
I wrote <https://github.com/LucianoBestia/foreground_scheduler> to do this.  
It is a normal CLI and it is easy to see the output on the screen.  
To make this run indefinitely in another terminal session I use `screen`.

## testing .cache/crev

Not all data is required in every review, so I need to test examples that contains different data.  
<https://bestia.dev/cargo_crev_web/query/btoi>  alternatives  
<https://bestia.dev/cargo_crev_web/crate/num-traits/>  issues  
<https://bestia.dev/cargo_crev_web/query/protobuf>  advisory old  
<https://bestia.dev/cargo_crev_web/query/inventory>   advisories

Locally in development is the same, just the server is 127.0.0.1:8051/.  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://bestia.dev/cargo_crev_web/crate/num-traits/>  

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

## Badges

A service for SVG badges for `crev count of reviews for one crate` is on url:\
<https://bestia.dev/cargo_crev_web/badge/crev_count/reader_for_microxml.svg>\
Example how it looks like on GitHub:\
<https://github.com/LucianoBestia/reader_for_microxml/>  

## CodeTour

I like very much the VSCode extension CodeTour.  
It makes a special kind of documentation that shows the code flow.  
No other type of documentation is so useful as this.  
It works only in VSCode. I made an export to md utility because is so much easier to distribute the md file around.  

## ideas

The same web server can easily run on the local machine of the developer.  
It is just one single binary executable file.  
It will read only the trusted reviews specific to that developer.  
So now we have a local web server and a browser. It means we have now the possibility to make a beautiful GUI for cargo-crev that works on any OS and remotely also. Good.  
Maybe it will be possible to add the functionality to write new reviews in the browser.  
I am terrible in VIM, I confess.  

## TODO
web form for entering a new review

Stats:

2020-05-27 authors:55 crates: 514, reviews: 886

## References

<https://github.com/rustomax/rust-iterators>  

