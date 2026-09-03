use std::{
    fmt::Debug,
    io::{Write, stdout},
};
use termal::{eprintac, printacln};
use ttf_view::{
    tables::{TableDirectoryRepr, TableRecordRepr},
    types::{Tag, tags},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Help,
    Version,
    ListTables,
    Dump,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Format {
    Debug,
    Binary,
}

macro_rules! error_exit {
    ($($arg:tt)*) => {{
        eprintac!("{'bold r}error{'_}: ");
        eprintln!($($arg)*);
        std::process::exit(1);
    }};
}

fn main() {
    let mut font_path = None;
    let mut action = Action::Help;
    let mut format = Format::Debug;
    let mut table_tag = None;

    let mut args = std::env::args();
    args.next(); // skip executable

    while let Some(arg) = args.next() {
        match &*arg {
            "-h" | "--help" => {
                action = Action::Help;
            },
            "-v" | "-V" | "--version" => {
                action = Action::Version;
            },
            "--list-tables" => {
                action = Action::ListTables;
            },
            "-f" | "--format" => {
                match args.next() {
                    Some(arg) => match &*arg {
                        "dbg" | "debug" => format = Format::Debug,
                        "bin" | "binary" => format = Format::Binary,
                        format => error_exit!("Got an unknown format identifier '{format}'"),
                    },
                    None => error_exit!("Expected a format identifier after '-f'/'--format'"),
                };
            },
            "-t" | "--table" => {
                match args.next() {
                    Some(arg) => match Tag::from_str(&arg) {
                        Ok(tag) => table_tag = Some(tag),
                        Err(tag_error) => error_exit!("Got an invalid table tag ({tag_error})"),
                    },
                    None => error_exit!("Expected a table tag after '-t'/'--table'"),
                };
            },
            _ => {
                if let Some(x) = font_path {
                    error_exit!("Got more than one path in arguments ('{}', '{}')", &x, arg);
                }
                font_path = Some(arg);
                action = Action::Dump;
            },
        };
    }

    match action {
        Action::Version => {
            print_version();
        },
        Action::Help => {
            print_help();
        },
        Action::ListTables => {
            print_tables();
        },
        Action::Dump => {
            let Some(font_path) = font_path else { error_exit!("The font file was not specified") };

            let font_data = match std::fs::read(&font_path) {
                Ok(font_data) => font_data,
                Err(err) => error_exit!("{} '{}'", err, &font_path),
            };
            let dir = unsafe { TableDirectoryRepr::new_unchecked(&font_data) };

            match format {
                Format::Binary => {
                    let data = dump_binary(&font_data, dir, table_tag);
                    stdout().write_all(data).unwrap();
                },
                Format::Debug => {
                    println!("{:#?}", dump_debug(dir, table_tag));
                },
            };
        },
    };
}

fn dump_binary<'a>(data: &'a Vec<u8>, dir: &'a TableDirectoryRepr, tag: Option<Tag>) -> &'a [u8] {
    match tag {
        Some(tag) => dir.table_record(tag).map_or_default(|t| t.data(dir)),
        None => {
            let dir_size = size_of::<TableDirectoryRepr>()
                + dir.table_records().len() * size_of::<TableRecordRepr>();
            &data[..dir_size]
        },
    }
}

fn dump_debug(dir: &TableDirectoryRepr, tag: Option<Tag>) -> &dyn Debug {
    match tag {
        None => dir,

        // Some(tags::cmap) => dir.cmap(),
        Some(tags::head) => dir.head(),
        Some(tags::hhea) => dir.hhea(),
        // Some(tags::hmtx) => dir.hmtx(),
        Some(tags::maxp) => dir.maxp(),
        Some(tags::name) => dir.name(),

        Some(table_tag @ _) => {
            if Tag::KNOWN_TAGS.contains(&table_tag) {
                error_exit!("error: Dumping the table '{table_tag}' is not supported yet")
            } else {
                error_exit!("error: Could not find a table with tag '{table_tag}'")
            }
        },
    }
}

fn print_version() {
    println!("ttf-view {}", env!("CARGO_PKG_VERSION"));
}

fn print_help() {
    printacln!(
        r#"A TrueType/OpenType font parsing/viewing Rust library and a CLI tool.
The project's GitHub repository: https://github.com/Chasmical/ttf-view

{'bold u}Usage:{'_u} ttf-view{'_bold} [OPTIONS] <FONT>

{'bold u}Arguments:{'_}
  <FONT>  Path to the OpenType font file to view (.ttf, .otf)

{'bold u}Options:{'_}
  {'bold}-f, --format{'_} <FORMAT>  The format to dump the table data in (possible values: dbg/debug, bin/binary)
  {'bold}-t, --table{'_} <TAG>      The table to dump (omit to dump the table directory)
  {'bold}    --list-tables{'_}      List all supported OpenType tables (binary format always works)
  {'bold}-h, --help{'_}             Print help
  {'bold}-V, --version{'_}          Print version"#
    );
}

fn print_tables() {
    printacln!(
        r#"Currently only the following OpenType tables can be exported:

{'bold}cmap{'_}  Character Mapping Table   bin
{'bold}head{'_}  Font Header Table         bin,dbg
{'bold}hhea{'_}  Horizontal Header Table   bin,dbg
{'bold}hmtx{'_}  Horizontal Metrics Table  bin
{'bold}maxp{'_}  Maximum Profile           bin,dbg
{'bold}name{'_}  Naming Table              bin,dbg

Note: {'bold}bin{'_} format is always available for any tables."#
    );
}
