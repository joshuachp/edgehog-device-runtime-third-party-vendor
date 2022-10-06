# edgehog-device-runtime-third-party-vendor

This repository contains all the vendored Rust dependencies that are required in
order to build
[edgehog-device-runtime](https://github.com/edgehog-device-manager/edgehog-device-runtime).

## To use vendored sources, add this to your .cargo/config.toml for the edgehog-device-runtime project:

``` toml
[source.crates-io]
replace-with = "vendored-sources"

[source."https://github.com/astarte-platform/astarte-device-sdk-rust.git"]
git = "https://github.com/astarte-platform/astarte-device-sdk-rust.git"
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "./edgehog-device-runtime-third-party-vendor/vendor"
```
