# ttf-view

A TrueType/OpenType font parsing/viewing library and a CLI tool.

### Installation

```sh
cargo +nightly-2026-09-04 install ttf-view -F cli
```

### Usage examples

```
> ttf-view --help

A TrueType/OpenType font parsing/viewing Rust library and a CLI tool.
The project's GitHub repository: https://github.com/Chasmical/ttf-view

Usage: ttf-view [OPTIONS] <FONT>

Arguments:
  <FONT>  Path to the OpenType font file to view (.ttf, .otf)

Options:
  -f, --format <FORMAT>  The format to dump the table data in [possible values: dbg/debug, bin/binary]
  -t, --table <TAG>      The table to dump (omit to dump the table directory)
      --list-tables      List all supported OpenType tables (binary format always works)
  -h, --help             Print help
  -V, --version          Print version
```

```
> ttf-view test.ttf

TableDirectory {
    sfnt_version: 0x00010000,
    num_tables: 21,
    search_range: 256,
    entry_selector: 4,
    range_shift: 80,
    table_records: [
        TableRecord { table_tag: 'COLR', checksum: 0x4823DB3B, offset: 0x00576994, length: 0x006D01BA },
        TableRecord { table_tag: 'CPAL', checksum: 0x3CC6B60F, offset: 0x00C46B50, length: 0x0003FE92 },
        TableRecord { table_tag: 'GDEF', checksum: 0x0ADC054D, offset: 0x00C869E4, length: 0x000000F4 },
        TableRecord { table_tag: 'GPOS', checksum: 0x7376E302, offset: 0x00C86AD8, length: 0x00001806 },
        TableRecord { table_tag: 'GSUB', checksum: 0xA4D814D5, offset: 0x00C882E0, length: 0x00006828 },
        TableRecord { table_tag: 'OS/2', checksum: 0x49DD8A15, offset: 0x000001D8, length: 0x00000060 },
        TableRecord { table_tag: 'cmap', checksum: 0xBF9CA30A, offset: 0x0003E018, length: 0x00001F91 },
        TableRecord { table_tag: 'cvt ', checksum: 0x377532F4, offset: 0x00040E30, length: 0x000001EA },
        TableRecord { table_tag: 'fpgm', checksum: 0xBD3C2AFF, offset: 0x0003FFAC, length: 0x00000983 },
        TableRecord { table_tag: 'gasp', checksum: 0x0008001B, offset: 0x00576988, length: 0x0000000C },
        TableRecord { table_tag: 'glyf', checksum: 0x1B2CBBF3, offset: 0x0007FD68, length: 0x004F3F9D },
        TableRecord { table_tag: 'head', checksum: 0x3182EE04, offset: 0x0000015C, length: 0x00000036 },
        TableRecord { table_tag: 'hhea', checksum: 0x0EDCF927, offset: 0x00000194, length: 0x00000024 },
        TableRecord { table_tag: 'hmtx', checksum: 0x4197269D, offset: 0x00000238, length: 0x0003DDDE },
        TableRecord { table_tag: 'kern', checksum: 0xB17BC637, offset: 0x00573D08, length: 0x000020BE },
        TableRecord { table_tag: 'loca', checksum: 0x64FF0ECB, offset: 0x0004101C, length: 0x0003ED4C },
        TableRecord { table_tag: 'maxp', checksum: 0x00631DF5, offset: 0x000001B8, length: 0x00000020 },
        TableRecord { table_tag: 'meta', checksum: 0x4C48FD9E, offset: 0x00C8EB08, length: 0x00000042 },
        TableRecord { table_tag: 'name', checksum: 0xEBC6470F, offset: 0x00575DC8, length: 0x00000BA0 },
        TableRecord { table_tag: 'post', checksum: 0xFF510077, offset: 0x00576968, length: 0x00000020 },
        TableRecord { table_tag: 'prep', checksum: 0x20E2343E, offset: 0x00040930, length: 0x000004FF },
    ],
}
```

```
> ttf-view test.ttf -t head

HeadTable {
    major_version: 1,
    minor_version: 0,
    font_revision: 1.6,
    checksum_adjustment: 0x62E47DE8,
    magic_number: 0x5F0F3CF5,
    flags: 0b000000000010001,
    units_per_em: 2048,
    created: 2025-02-12T02:03:10Z,
    modified: 2026-07-31T13:46:51Z,
    x_min: -717,
    y_min: -501,
    x_max: 2758,
    y_max: 1975,
    mac_style: 0b0000000,
    lowest_rec_ppem: 9,
    font_direction_hint: 2,
    index_to_loc_format: 1,
    glyph_data_format: 0,
}
```



## License

Licensed under either [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
