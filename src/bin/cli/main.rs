extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};
use flic::events::{self, Event};
use flic::{commands, FlicError, Result};
use rand::Rng;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, SystemTime};

fn main() -> Result<()> {
    // Flic CLI
    let app_m = App::new("Flic CLI")
        .version("0.1")
        .author("bcspragu")
        .about("Interfaces with flicd")
        .arg(
            Arg::with_name("flicd-address")
                .long("flicd_addr")
                .value_name("ADDR")
                .default_value("localhost:5551")
                .help("host:port address of the flicd service")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("lists buttons known to flicd or to this service")
                .arg(
                    Arg::with_name("scan-timeout")
                        .long("timeout")
                        .help("the number of seconds to scan for new buttons")
                        .default_value("5"),
                ),
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
    let addr = app_m.value_of("flicd-address").unwrap();

    let client = Client::new_client(addr)?;

    match app_m.subcommand() {
        ("list", Some(m)) => handle_list(client, m)?,
        ("connect", Some(m)) => handle_connect(client, m)?,
        _ => {}
    }

    Ok(())
}

fn handle_list(client: Client, m: &ArgMatches) -> Result<()> {
    // Scan timeout has a default.
    let timeout = match m.value_of("scan-timeout").unwrap().parse() {
        Ok(v) => v,
        Err(err) => return Err(FlicError::from("failed to parse int", err)),
    };

    let timeout = Duration::from_secs(timeout);

    println!("Scanning for buttons...");
    let buttons = client.list(timeout)?;

    if buttons.is_empty() {
        println!("No buttons found.");
    }

    for button in buttons {
        println!("{}: {}", button.name, button.bd_addr);
    }

    Ok(())
}

fn handle_connect(client: Client, m: &ArgMatches) -> Result<()> {
    // Button ID is required.
    client.connect(m.value_of("button-id").unwrap())?;

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

    fn list(self, timeout: Duration) -> Result<Vec<events::AdvertisementPacket>> {
        let client = Arc::new(self.client);

        client.send_command(commands::CreateScanner {
            scan_id: self.scan_id,
        })?;

        let c = Arc::clone(&client);
        let (tx, rx) = mpsc::sync_channel::<Option<Result<events::AdvertisementPacket>>>(0);
        let handle = thread::spawn(move || {
            let start = SystemTime::now();
            loop {
                // See how much time has elapsed. If we've been here longer than the requested scan
                // timeout, return.
                match start.elapsed() {
                    Ok(elapsed) if elapsed > timeout => break,
                    Err(e) => {
                        tx.send(Some(Err(FlicError::from("failed to get elapsed time", e))))
                            .unwrap();
                        return;
                    }
                    _ => {}
                }

                let event = c.next_event_with_timeout(Some(timeout));

                let event = match event {
                    Ok(Some((Event::AdvertisementPacket(pkt), _))) => pkt,
                    Ok(None) => {
                        // Means we've timed out and we're done.
                        tx.send(None).unwrap();
                        return;
                    }
                    Err(e) => {
                        tx.send(Some(Err(e))).unwrap();
                        return;
                    }
                    _ => continue, // Means we got some other event.
                };

                tx.send(Some(Ok(event))).unwrap();
            }
        });

        let mut buttons = Vec::new();
        loop {
            let evt = match rx.recv().unwrap() {
                Some(Ok(v)) => v,
                Some(Err(e)) => return Err(e),
                None => break,
            };

            // Filter out events that weren't for our scanner.
            if evt.scan_id != self.scan_id {
                continue;
            }

            buttons.push(evt);
        }

        handle.join().unwrap();

        client.send_command(commands::RemoveScanner {
            scan_id: self.scan_id,
        })?;

        Ok(buttons)
    }

    fn connect(&self, button_id: &str) -> Result<()> {
        println!("Connect invoked for button {:?}", button_id);

        Ok(())
    }
}
