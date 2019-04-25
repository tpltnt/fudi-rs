An implementation of the Fast Universal Digital Interface networking protocol. This can be used to communicate with Pure Data via the netsend / netreceive objects.
*note*: This implementation does not handle escaped whitepace in message atoms.

The specification implies ASCII encoding for the messages.
A message needs a trailing newline (i.e. '\n') according to the Java example in the [old wiki page](https://web.archive.org/web/20120304071510/http://wiki.puredata.info/en/FUDI). This is not explicitly mentioned in the FUDI specification.

[![Build Status](https://travis-ci.org/tpltnt/fudi-rs.svg?branch=master)](https://travis-ci.org/tpltnt/fudi-rs)

# TODO
* handling escaped whitespace in atoms
* examples (also pure data patches)
* handle TCP

# references #
* [specification](https://web.archive.org/web/20120304071510/http://wiki.puredata.info/en/FUDI) (via archive.org)
* [wikipedia: FUDI](https://en.wikipedia.org/wiki/FUDI)
* [Pure Data](http://puredata.info/)
* [Pure Data messages](https://puredata.info/dev/PdMessages)
* [nom](https://github.com/Geal/nom) - parser combinator framework
* [ASCII](https://en.wikipedia.org/wiki/ASCII)
* [graphviz](https://graphviz.org/) for drawing the parsing finite state machine
