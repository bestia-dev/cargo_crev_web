# cargo_crev_web

[comment]: # (lmake_readme cargo.toml data start)

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

One working server is installed on my google vm.  
There is a nginx server reverse proxy that accepts https http2 on 443 and relay to internal 8051.
Nginx also redirects all http 80 to https 443.  
-- to start the application in background with the command "screen" with a session_name  
`screen -S cargo_crev_web_8051`,  
`screen -r cargo_crev_web_8051`,  

## testing my .cache/crev

in development:  
<http://127.0.0.1:8051/query/btoi>    alternatives  
<http://127.0.0.1:8051/query/num-traits>   issues  
<http://127.0.0.1:8051/query/protobuf>   advisory  

on the web:  
<https://bestia.dev/cargo_crev_web/query/btoi>  alternatives  
<https://bestia.dev/cargo_crev_web/query/num-traits>  issues  
<https://bestia.dev/cargo_crev_web/query/protobuf>  advisory  
