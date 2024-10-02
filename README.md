# Simple rust curl

A simple CLI tool that acts like a curl

# Disclaimer

I'm doing this project for educational pourpose only, so i can learn rust and understand better how web requests are made, withou using a library like reqwest.<br/>

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
As this is my first time doing http requets without a library like reqwest, I used as a starting point this [tutorial](https://dev.to/chaudharypraveen98/build-your-own-curl-rust-5cj6) on how to build your own curl in rust.<br />
Since I'm basing my CLI on this tutorial, I think it's only fair to give it the due credit.

# NOTE 2:
It is actually painful to work with the openssl on windows.

# NOTE 3:
I think this is in a relatively stable state, of course it doesn't have nearly as many options as the real curl has and it ain't pretty (not at all), but for the pourpose that i wanted to build, it's fine.<br />
I could improve on the header request, add a few options. Right now, either you provide the entire thing or none at all.<br />
Also, the response is basicly raw. It would be a good idea to parse the response. I only used the httparser on the header and it was to easily verify the response code so i could implement the "-L" follow location option.<br />
I'm not sure if I'll be updating this CLI, as there are other things I want to work on.