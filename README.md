Study-RustyCap
==============

An experimental project to implement pcap processing in Rust language.
Implements processing of file structure described by wireshark wiki: http://wiki.wireshark.org/Development/LibpcapFileFormat

**Status:** Early Alpha. Expect to be rough around the edges.

**Compile:** rustc src/rustycap.rs

**Usage:** ./rcap dumpfile

LICENSE
-------
All code in repo under BSD 3-Clause license.

Work completed
--------------
* Command line argument processing and file reading for pcap files
* Hex pretty printer (for examining read out code)
* PCap global header decoding
* Packet header decoding

Work in progress
----------------
* Subcommand processing

TODO
----

###Major###
* Packet data decoding
* To be defined

###Minor###
* Add description for link types
* Documentation/License
* Test case generation

###Testing###
* Decoding of Big Endian file
