[![Build Status](https://travis-ci.org/tpltnt/fudi-rs.svg?branch=master)](https://travis-ci.org/tpltnt/fudi-rs)
[![dependency status](https://deps.rs/repo/github/tpltnt/fudi-rs/status.svg)](https://deps.rs/repo/github/tpltnt/fudi-rs)

An implementation of the Fast Universal Digital Interface networking protocol. This can be used to communicate with Pure Data via the netsend / netreceive objects.
*note*: This implementation does not handle escaped whitepace in message atoms.

The specification implies ASCII encoding for the messages.
A message needs a trailing newline (i.e. '\n') according to the Java example in the [old wiki page](https://web.archive.org/web/20120304071510/http://wiki.puredata.info/en/FUDI). This is not explicitly mentioned in the FUDI specification.

Use [libpd-rs](https://github.com/alisomay/libpd-rs) if you want to interface with libpd.

# examples:
* send random floats
  * run patch "send_random_floats.pd"
  * run: `cargo run --examples send_random_floats`
* receive random floats
  * run patch "receive_random_floats.pd"
  * run: `cargo run --examples receive_random_floats`
* receive bangs
  * run patch "recevie_bang.pd"
  * run: `cargo run --example receive_bang"

# TODO
* handle non-alphanumeric characters in message
* handling escaped whitespace in atoms
* handle TCP

# references #
* [specification](https://web.archive.org/web/20120304071510/http://wiki.puredata.info/en/FUDI) (via archive.org)
* [wikipedia: FUDI](https://en.wikipedia.org/wiki/FUDI)
* [Pure Data](http://puredata.info/)
* [Pure Data messages](https://puredata.info/dev/PdMessages)
* [undocumented internal messages](https://puredata.info/docs/tutorials/TipsAndTricks#undocumented-pd-internal-messages)
* [nom](https://github.com/Geal/nom) - parser combinator framework
* [ASCII](https://en.wikipedia.org/wiki/ASCII)
* [graphviz](https://graphviz.org/) for drawing the parsing finite state machine
