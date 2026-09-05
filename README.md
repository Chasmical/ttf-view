# ttf-view

A TrueType/OpenType font parsing/viewing library and a CLI tool.

For information about the Rust library, go to [its docs.rs documentation](https://docs.rs/ttf-view/).

## Installation

```sh
cargo +nightly-2026-09-04 install ttf-view -F cli
```

You can also find an already compiled exe for Windows x86_64 in [Releases](https://github.com/Chasmical/ttf-view/releases/latest).

## Usage examples

<img src="img/ttf-view-help.png" alt="ttf-view --help" width="580" height="346" />

<img src="img/ttf-view-list-tables.png" alt="ttf-view --list-tables" width="580" height="205" />

<img src="img/ttf-view-list-tables-2.png" alt="ttf-view test.ttf --list-tables" width="580" height="493" />

<img src="img/ttf-view-table-head.png" alt="ttf-view test.ttf -t head" width="580" height="371" />

## License

Licensed under either [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
