extern crate clap;

use clap::{App, Arg, SubCommand};
use flic::commands;
use flic::events::Event;
use flic::Result;
use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    // Flic CLI
    let app_m = App::new("Flic CLI")
        .version("0.1")
        .author("bcspragu")
        .about("Interfaces with flicd")
        .arg(
            Arg::with_name("flicd address")
                .long("flicd_addr")
                .value_name("ADDR")
                .default_value("localhost:5551")
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

    // Unwrap is fine here because we've set a default.
    let addr = app_m.value_of("flicd_addr").unwrap();

    let client = Client::new_client(addr)?;

    match app_m.subcommand() {
        ("list", Some(_m)) => {
            client.list()?;
        }
        ("connect", Some(m)) => {
            // Button ID is required.
            client.connect(m.value_of("button-id").unwrap())?;
        }
        _ => {}
    }

    Ok(())
}

struct Client {
    client: flic::Client,
    scan_id: u32,
}

impl Client {
    fn new_client(addr: &str) -> Result<Client> {
        let client = flic::Client::new(addr)?;
        let scan_id = rand::thread_rng().gen::<u32>();
        Ok(Client { client, scan_id })
    }

    fn list(self) -> Result<Vec<()>> {
        println!("List invoked");

        self.client.send_command(commands::CreateScanner {
            scan_id: self.scan_id,
        })?;

        let (tx, rx) = mpsc::sync_channel(0);
        let handle = thread::spawn(move || loop {
            let event = self
                .client
                .next_event_with_timeout(Some(Duration::from_secs(10)));

            let (event, _) = match event {
                Ok(Some(v)) => v,
                Ok(None) => {
                    tx.send(Ok(None)).unwrap();
                    return;
                }
                Err(err) => {
                    tx.send(Err(err)).unwrap();
                    return;
                }
            };

            if let Event::AdvertisementPacket(pkt) = event {
                tx.send(Ok(Some(pkt))).unwrap();
            }
        });

        loop {
            let evt = match rx.recv().unwrap() {
                Ok(Some(v)) => v,
                Ok(None) => break,
                Err(err) => return Err(err),
            };

            println!("{:?}", evt);
        }

        handle.join().unwrap();

        Ok(vec![])
    }

    fn connect(&self, button_id: &str) -> Result<()> {
        println!("Connect invoked for button {:?}", button_id);

        Ok(())
    }
}
