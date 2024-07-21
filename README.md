# Simple rust curl

A simple CLI tool that acts like a curl

# Disclaimer

I'm doing this project as educational pourpose only, so i can learn rust and understand better how web requests are made.<br/>
I want to make this CLI a little closer to the real curl command, as it is now, it doesn't even has a location option to follow redirects when you get a 301 status.
I'm still working on it on my free time, and will be updating my progress.
# Usage

```
cargo run -- "https://www.yoururl.com/fictional-url"
```
As I'm using clap, wonderful crate by the way, you can use -h to get all the options:
Of course, you can see the options with:
```
cargo run -- -h
```

# NOTE:
As this is my first time doing my own requets without a library on any language, I used as a starting point this [tutorial](https://dev.to/chaudharypraveen98/build-your-own-curl-rust-5cj6) on how to build your own curl in rust.<br/>
Of course i didn't just copied the code, because than i would learn nothing, and I made some changes and added some features so i could learn more than just the tutorial provides.<br/>
But, because a I used it as a starting point, I think it's only fair to give the due credit.