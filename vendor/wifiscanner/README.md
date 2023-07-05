# wifiscanner

[![Build Status](https://travis-ci.org/booyaa/wifiscanner.svg?branch=master)](https://travis-ci.org/booyaa/wifiscanner)
[![Crates](https://img.shields.io/crates/v/wifiscanner.svg)](https://crates.io/crates/wifiscanner)
[![docs.rs](https://docs.rs/wifiscanner/badge.svg)](https://docs.rs/wifiscanner)

A crate to list WiFi hotspots in your area.

As of v0.5.x now supports macOS, Linux and Windows. :tada:

Inspired by Maurice Svay's node-wifiscanner (https://github.com/mauricesvay/node-wifiscanner)

Tests shameless pilfered from Christian Kuster's node-wifi-scanner (https://github.com/ancasicolica/node-wifi-scanner)

Full documentation can be found [here](https://docs.rs/wifiscanner).

# Usage

This crate is [on crates.io](https://crates.io/crates/wifiscanner) and can be
used by adding `wifiscanner` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
wifiscanner = "0.5.*"
```

and this to your crate root:

```rust
extern crate wifiscanner;
```

# Example

```rust
use wifiscanner;
println!("{:?}", wifiscanner::scan());
```

Alternatively if you've cloned the Git repo, you can run the above example
using: `cargo run --example scan`.

# Changelog

- 0.5.1 - crates.io metadata update
- 0.5.0 - add window support (props to  @brianjaustin)
- 0.4.0 - replace iwlist with iw (props to @alopatindev)
- 0.3.6 - crates.io metadata update
- 0.3.5 - remove hardcoded path for iwlist (props to @alopatindev)
- 0.3.4 - initial stable release

# How to contribute

see [CONTRIBUTING.md](/CONTRIBUTING.md)
# Contributors

wifiscanner would not be possible without the following folks:

@alopatindev, @bizzu, @bash, @cristicbz, @lpmi-13, @brianjaustin


# Copyright

Copyright 2019 Mark Sta Ana.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> at your option. This file may not
be copied, modified, or distributed except according to those terms.
