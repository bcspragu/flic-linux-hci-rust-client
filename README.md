# Rust Client for Flic Linux SDK

This repo contains a Rust library crate for interacting with Shortcut Labs
Linux service, which in turn interacts with Flic buttons. The main repo for
that service is https://github.com/50ButtonsEach/fliclib-linux-hci

This is my first attempt at writing Rust code, all critiques, suggestions, and
comments are welcome!

## TODO

There's still lots left to do:

x Figure out how to actually send messages.
x Marshal the full command (including opcode and length header) and send it over the wire
x Implement [the rest of the
  API](https://github.com/50ButtonsEach/fliclib-linux-hci/blob/master/ProtocolDocumentation.md)
- Actually try it out (use it in a binary)
- Handle unmarshalling for the remaining few events
x Document the commands and events and stuff with the documentation from the
  protocol, so that the rust docs look good
  - Still need to do the commands
- [Maybe] Set up automated testing and stuff with Travis
