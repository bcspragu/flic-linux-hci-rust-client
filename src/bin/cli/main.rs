extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    // Flic CLI
    let app_m = App::new("Flic CLI")
        .version("0.1")
        .author("bcspragu")
        .about("Interfaces with flicd")
        .arg(
            Arg::with_name("flicd address")
                .long("flicd_addr")
                .value_name("ADDR")
                .help("host:port address of the flicd service")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("list").about("lists buttons known to flicd or to this service"),
        )
        .subcommand(
            SubCommand::with_name("connect").arg(
                Arg::with_name("button-id")
                    .required(true)
                    .help("the button ID to connect to"),
            ),
        )
        .get_matches();

    match app_m.subcommand() {
        ("list", Some(m)) => handle_list(m),
        ("connect", Some(m)) => handle_connect(m),
        _ => {}
    }
}

fn handle_list(_m: &ArgMatches) {
    println!("List invoked");
}

fn handle_connect(m: &ArgMatches) {
    println!("Connect invoked for button {:?}", m.value_of("button-id"));
}
