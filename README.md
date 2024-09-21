# Simple rust curl

A simple CLI tool that acts like a curl

# Disclaimer

I'm doing this project for educational pourpose only, so i can learn rust and understand better how web requests are made.<br/>

I'm still working on it on my free time, and will be updating it.

~~I started using rustls and it works fine, but since it doesn't support some legacy protocols, I'll probably change it to opnessl or maybe use both.~~<br/>
Added openssl and left the rustls function and crates.
# Usage
Using cargo:<br />
```
cargo run -- "http://www.yoururl.com/fictional-url"
```
You can use -h to get all the options:
```
cargo run -- -h
```

# NOTE:
As this is my first time doing my own requets without a library (not couting the tls connection and http parsing) on any language, I used as a starting point this [tutorial](https://dev.to/chaudharypraveen98/build-your-own-curl-rust-5cj6) on how to build your own curl in rust.<br />
Since I'm basing my CLI on this tutorial, I think it's only fair to give it the due credit.

# NOTE 2:
It is actually painful to work with the openssl on windows.

# NOTE 3:
Yes, the code it's very messy and I need to refactor and optimize a lot of it. <br />
Since I added the follow location option, it became a lot worst.<br />
So I'll do what every programmer does:<br /> 
```
//TODO: Refactor and optimize the code :)
```