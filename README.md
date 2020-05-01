# cargo_crev_web

[comment]: # (lmake_readme cargo.toml data start)
version: 2020.501.1211  date: 2020-05-01 authors: Luciano Bestia  
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

## Linux scheduler cron tab

I will repeatedly call  
`cargo crev repo fetch trusted`
to have fresh reviews available locally in `~/.cache/crev/`.  
Open the Linux scheduler config file:  
`crontab -e`  
add this line to start every hour at x:04 minutes  
`4 * * * * cargo crev repo fetch trusted`  

## testing my .cache/crev

in development:  
<http://127.0.0.1:8051/query/btoi>    alternatives  
<http://127.0.0.1:8051/query/num-traits>   issues  
<http://127.0.0.1:8051/query/protobuf>   advisory  

on the web:  
<https://bestia.dev/cargo_crev_web/query/btoi>  alternatives  
<https://bestia.dev/cargo_crev_web/query/num-traits>  issues  
<https://bestia.dev/cargo_crev_web/query/protobuf>  advisory  
