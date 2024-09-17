# Simple rust curl

A simple CLI tool that acts like a curl

# Disclaimer

I'm doing this project for educational pourpose only, so i can learn rust and understand better how web requests are made.<br/>

I'm still working on it on my free time, and will be updating it.

~~I started using rustls and it works fine, but since it doesn't support some legacy protocols, I'll probably change it to opnessl or maybe use both.~~
Added openssl and left the tls function and crates.
# Usage
You'll need cargo to build and compile<br />
```
cargo run -- "http://www.yoururl.com/fictional-url"
```
You can use -h to get all the options:
```
cargo run -- -h
```

# NOTE:
As this is my first time doing my own requets without a library on any language, I used as a starting point this [tutorial](https://dev.to/chaudharypraveen98/build-your-own-curl-rust-5cj6) on how to build your own curl in rust.<br/>
Since I'm basing my CLI on this tutorial, I think it's only fair to give the due credit.
