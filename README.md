# Simple rust curl

A simple CLI tool that acts like a curl

# Disclaimer

I'm doing this project for educational pourpose only, so i can learn rust and understand better how web requests are made.<br/>
I want to make this CLI a little closer to the real curl command, as it is now, it doesn't even has a location option to follow redirects and it doesn`t support https requests.<br/>
I'm still working on it on my free time, and will be updating it.
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
Of course i didn't just copied the code, because than i would learn nothing, and I made some changes and added some features so i could learn more than just the tutorial provides.<br/>
But, because I used it as a starting point, I think it's only fair to give the due credit.
