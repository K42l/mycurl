use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn get_arguments() -> ArgMatches{
    Command::new("mcurl - My curl")
        .about("Simple Curl to make HTTP methods")
        .version("1.0")
        .author("K")
        .arg(Arg::new("url").index(1).required(true))
        .arg(
            Arg::new("method")
                .help("The HTTP method")
                .long("method")
                .short('m')
                .default_value("GET")
        )
        .arg(
            Arg::new("data")
                .help("The data to be sent with the request")
                .long("data")
                .short('d')
        )
        .arg(
            Arg::new("header")
                .help("The request header")
                .long("header")
                .short('H')
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("file")
                .help("Write to file instead of stdout")
                .long("output")
                .short('o')
                .value_parser(clap::value_parser!(std::path::PathBuf))
        )
        .arg(
            Arg::new("include")
                .help("Include response headers in output file")
                .long("include")
                .short('i')
                .action(clap::ArgAction::SetTrue)
                .requires("file")
        )
        .arg(
            Arg::new("verbose")
                .help("Verbose Mode")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("location")
                .help("Follow the location on the header of the response")
                .long("location")
                .short('L')
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches()
}