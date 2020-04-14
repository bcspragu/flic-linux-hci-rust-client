[![Build Status](https://travis-ci.com/bcspragu/flic-linux-hci-rust-client.svg?branch=master)](https://travis-ci.com/bcspragu/flic-linux-hci-rust-client)

# Rust Client for Flic Linux SDK

This repo contains a Rust library crate for interacting with Shortcut Labs
Linux service, which in turn interacts with Flic buttons. The main repo for
that service is https://github.com/50ButtonsEach/fliclib-linux-hci

This is my first attempt at writing Rust code, all critiques, suggestions, and
comments are welcome!

## TODO

- [ ] Fix the current model of TcpStream communication. Basically, commands
  never get sent because we're locked on waiting for the data to come back from
  the stream. We need a way to stop waiting for a request so we can issue
  commands.
- [ ] Actually try it out (use it in a binary)
- [ ] Update comments to make decent-looking rustdoc output
- [ ] Add tests for events and stuff
- [x] Handle unmarshalling for the remaining few events
- [x] Set up automated testing and stuff with Travis
- [x] Document the commands and events and stuff with the documentation from the protocol
  - [x] Commands
  - [x] Enums
  - [x] Events
- [x] Figure out how to actually send messages.
- [x] Marshal the full command (including opcode and length header) and send it over the wire
- [x] Implement [the rest of the API](https://github.com/50ButtonsEach/fliclib-linux-hci/blob/master/ProtocolDocumentation.md)
