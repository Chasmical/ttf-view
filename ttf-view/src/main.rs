#![feature(type_info)]
#![feature(specialization)]
#![allow(incomplete_features)]
use std::{
    fmt::Debug,
    io::{Write, stdout},
};
use termal::{eprintac, printac, printacln};
use ttf_view::{
    tables::*,
    types::{Tag, tags},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    PrintHelp,
    PrintVersion,
    ListTables,
    DumpTable,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Format {
    Debug,
    Binary,
    Json,
    Ttx,
}

macro_rules! error_exit {
    ($($arg:tt)*) => {{
        eprintac!("{'bold r}error{'_}: ");
        eprintln!($($arg)*);
        std::process::exit(1)
    }};
}

fn main() {
    let mut path = None;
    let mut action = Action::PrintHelp;
    let mut format = None;
    let mut table = None;

    let mut args = std::env::args();
    args.next(); // skip executable

    while let Some(arg) = args.next() {
        match &*arg {
            "-h" | "--help" => {
                action = Action::PrintHelp;
            },
            "-v" | "-V" | "--version" => {
                action = Action::PrintVersion;
            },
            "--list-tables" => {
                action = Action::ListTables;
            },
            "-f" | "--format" => {
                if format.is_some() {
                    error_exit!("-f/--format was specified more than once");
                }
                let Some(arg) = args.next() else {
                    error_exit!("Expected a format identifier after -f/--format");
                };

                format = Some(match &*arg {
                    "dbg" | "debug" => Format::Debug,
                    "bin" | "binary" => Format::Binary,
                    "json" => Format::Json,
                    "ttx" => Format::Ttx,
                    unknown => error_exit!("Got an unknown format identifier '{unknown}'"),
                });
            },
            "-t" | "--table" => {
                if table.is_some() {
                    error_exit!("-t/--table was specified more than once");
                }
                let Some(arg) = args.next() else {
                    error_exit!("Expected a table tag after -t/--table");
                };

                if arg == "directory" {
                    table = Some(None);
                } else {
                    table = match Tag::from_str(&arg) {
                        Ok(tag) => Some(Some(tag)),
                        Err(tag_error) => error_exit!("Got an invalid table tag ({tag_error})"),
                    };
                }
            },
            _ => {
                if arg.starts_with('-') {
                    error_exit!("Got an unknown option '{}'", arg);
                }
                if let Some(x) = path {
                    error_exit!("Got more than one path in arguments ('{}', '{}')", &x, arg);
                }
                path = Some(arg);
                action = Action::DumpTable;
            },
        };
    }

    // Set option defaults
    let format = format.unwrap_or(Format::Debug);
    let table = table.unwrap_or(None);

    let mut font_data = None;

    // The font file is loaded only when dumping a table, or showing --list-tables
    let load_font = || {
        font_data = path.map(|path| match std::fs::read(&path) {
            Ok(data) => data,
            Err(err) => error_exit!("{} '{}'", err, &path),
        });
        font_data.as_ref().map(|data| {
            return unsafe { TableDirectoryRepr::new_unchecked(&data) };
        })
    };

    match action {
        Action::PrintVersion => print_version(),
        Action::PrintHelp => print_help(),
        Action::ListTables => print_tables(load_font()),
        Action::DumpTable => {
            let Some(dir) = load_font() else { error_exit!("Font file was not specified") };
            dump(dir, table, format);
        },
    };
}

// 'dir' works as an alias for table directory, but only when there's no table with that tag.
const DIR_TAG: Tag = if let Ok(tag) = Tag::from_str("dir") { tag } else { unreachable!() };

// This macro constructs the dump() fn, and the supported formats table.
macro_rules! implement_tables {
    ($dir:ident { $( $tag:path, $name:expr, $get_table:expr => $Table:ty );* $(;)? }) => {
        fn dump($dir: &TableDirectoryRepr, tag: Option<Tag>, format: Format) {
            // Binary format should always work for any tables
            if format == Format::Binary {
                let bytes = match tag {
                    None => $dir.directory_as_bytes(),
                    Some(DIR_TAG) if $dir.table_record_raw(DIR_TAG).is_none() => $dir.directory_as_bytes(),
                    Some(tag) => $dir.table_record(tag).map_or_default(|t| t.table_as_bytes()),
                };
                stdout().write_all(bytes).unwrap();
                return;
            }

            // Get the table with its vtable (see DumpDebug trait at the end of the file)
            let table: &dyn DumpDebug = match tag {
                None => $dir,
                Some(DIR_TAG) if $dir.table_record_raw(DIR_TAG).is_none() => $dir,

                $(Some($tag) => $get_table,)*

                Some(table_tag @ _) => {
                    if $dir.table_record_raw(table_tag).is_none() {
                        error_exit!("Could not find a table with tag '{table_tag}'");
                    } else {
                        error_exit!("Unknown table '{table_tag}'. Consider exporting in binary format");
                    }
                },
            };

            // Dump the table in specified format
            match format {
                Format::Binary => {
                    unreachable!(); // was handled earlier
                },
                Format::Debug => {
                    if !table.debug_is_supported() {
                        error_exit!("debug format is not yet implemented for '{}'", tag.unwrap());
                    }
                    println!("{:#?}", std::fmt::from_fn(|f| table.debug_dump(f)));
                },
                Format::Json => {
                    error_exit!("json format is not implemented yet");
                },
                Format::Ttx => {
                    error_exit!("ttx format is not implemented yet");
                },
            };
        }

        // Construct a supported formats table for tables
        struct Table {
            tag: Tag,
            name: &'static str,
            // has_impl: bool,
            dbg: bool,
            json: bool,
            ttx: bool,
        }
        const SUPPORTED_TABLES: &[Table] = &[
            $(Table {
                tag: $tag, name: $name,
                // has_impl: stringify!($Table) != "Todo",
                dbg: std::any::TypeId::of::<$Table>().trait_info_of::<dyn Debug>().is_some(),
                json: false,
                ttx: false,
            },)*
        ];
    };
}

struct Todo;

implement_tables!(dir {
    tags::avar, "Axis Variations Table", &Todo => Todo; // dir.avar() => avar::AvarTableRepr;
    tags::BASE, "Baseline Table", &Todo => Todo; // dir.base() => base::BaseTableRepr;
    tags::CBDT, "Color Bitmap Data Table", &Todo => Todo; // dir.cbdt() => cbdt::CbdtTableRepr;
    tags::CBLC, "Color Bitmap Location Table", &Todo => Todo; // dir.cblc() => cblc::CblcTableRepr;
    tags::CFF , "Compact Font Format (Version 1)", &Todo => Todo; // dir.cff() => cff::CffTableRepr;
    tags::CFF2, "Compact Font Format (Version 2)", &Todo => Todo; // dir.cff2() => cff2::Cff2TableRepr;
    tags::cmap, "Character to Glyph Index Mapping", dir.cmap() => cmap::CmapTableRepr;
    tags::COLR, "Color Table", &Todo => Todo; // dir.colr() => colr::ColrTableRepr;
    tags::CPAL, "Color Palette Table", &Todo => Todo; // dir.cpal() => cpal::CpalTableRepr;
    tags::cvar, "CVT Variations Table", &Todo => Todo; // dir.cvar() => cvar::cvarTableRepr;
    tags::cvt , "Control Value Table", &Todo => Todo; // dir.cvt() => cvt::cvtTableRepr;
    tags::DSIG, "Digital Signature Table", &Todo => Todo; // dir.dsig() => dsig::DsigTableRepr;
    tags::EBDT, "Embedded Bitmap Data Table", &Todo => Todo; // dir.ebdt() => ebdt::EbdtTableRepr;
    tags::EBLC, "Embedded Bitmap Location Table", &Todo => Todo; // dir.eblc() => eblc::EblcTableRepr;
    tags::EBSC, "Embedded Bitmap Scaling Table", &Todo => Todo; // dir.ebsc() => ebsc::EbscTableRepr;
    tags::fpgm, "Font Program", &Todo => Todo; // dir.fpgm() => fpgm::FpgmTableRepr;
    tags::fvar, "Font Variations Table", &Todo => Todo; // dir.fvar() => fvar::FvarTableRepr;
    tags::gasp, "Grid-fitting and Scan-conversion", &Todo => Todo; // dir.gasp() => gasp::GaspTableRepr;
    tags::GDEF, "Glyph Definition Table", &Todo => Todo; // dir.gdef() => gdef::GdefTableRepr;
    tags::glyf, "Glyph Data Table", &Todo => Todo; // dir.glyf() => glyf::GlyfTableRepr;
    tags::GPOS, "Glyph Positioning Table", &Todo => Todo; // dir.gpos() => gpos::GposTableRepr;
    tags::GSUB, "Glyph Substitution Table", &Todo => Todo; // dir.gsub() => gsub::GsubTableRepr;
    tags::gvar, "Glyph Variations Table", &Todo => Todo; // dir.gvar() => gvar::GvarTableRepr;
    tags::hdmx, "Horizontal Device Metrics", &Todo => Todo; // dir.hdmx() => hdmx::HdmxTableRepr;
    tags::head, "Font Header Table", dir.head() => head::HeadTableRepr;
    tags::hhea, "Horizontal Header Table", dir.hhea() => hhea::HheaTableRepr;
    tags::hmtx, "Horizontal Metrics Table", &dir.hmtx() => hmtx::HmtxTableHandle;
    tags::HVAR, "Horizontal Metrics Variations Table", &Todo => Todo; // dir.hvar() => hvar::HvarTableRepr;
    tags::JSTF, "Justification Table", &Todo => Todo; // dir.jstf() => jstf::JstfTableRepr;
    tags::kern, "Kerning Table", &Todo => Todo; // dir.kern() => kern::KernTableRepr;
    tags::loca, "Index to Location Table", &Todo => Todo; // dir.loca() => loca::LocaTableRepr;
    tags::LTSH, "Linear Threshold Table", &Todo => Todo; // dir.ltsh() => ltsh::LtshTableRepr;
    tags::MATH, "Mathematical Typesetting Table", &Todo => Todo; // dir.math() => math::MathTableRepr;
    tags::maxp, "Maximum Profile", dir.maxp() => maxp::MaxpTableRepr;
    tags::MERG, "Merge Table", &Todo => Todo; // dir.merg() => merg::MergTableRepr;
    tags::meta, "Metadata Table", &Todo => Todo; // dir.meta() => meta::MetaTableRepr;
    tags::MVAR, "Metrics Variations Table", &Todo => Todo; // dir.mvar() => mvar::MvarTableRepr;
    tags::name, "Naming Table", dir.name() => name::NameTableRepr;
    tags::OS_2, "OS/2 and Windows Metrics Table", &Todo => Todo; // dir.os_2() => os_2::Os_2TableRepr;
    tags::PCLT, "PCL 5 Table", &Todo => Todo; // dir.pclt() => pclt::PcltTableRepr;
    tags::post, "PostScript Table", &Todo => Todo; // dir.post() => post::PostTableRepr;
    tags::prep, "Control Value Program", &Todo => Todo; // dir.prep() => prep::PrepTableRepr;
    tags::sbix, "Standard Bitmap Graphics Table", &Todo => Todo; // dir.sbix() => sbix::SbixTableRepr;
    tags::STAT, "Style Attributes Table", &Todo => Todo; // dir.stat() => stat::StatTableRepr;
    tags::SVG , "Scalable Vector Graphics Table", &Todo => Todo; // dir.svg() => svg::SvgTableRepr;
    tags::VDMX, "Vertical Device Metrics Table", &Todo => Todo; // dir.vdmx() => vdmx::VdmxTableRepr;
    tags::vhea, "Vertical Header Table", &Todo => Todo; // dir.vhea() => vhea::VheaTableRepr;
    tags::vmtx, "Vertical Metrics Table", &Todo => Todo; // dir.vmtx() => vmtx::VmtxTableRepr;
    tags::VORG, "Vertical Origin Table", &Todo => Todo; // dir.vorg() => vorg::VorgTableRepr;
    tags::VVAR, "Vertical Metrics Variations Table", &Todo => Todo; // dir.vvar() => vvar::VvarTableRepr;
});

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
  {'bold}-t, --table{'_} <TAG>      The table to dump [default: table directory]

  {'bold}-f, --format{'_} <FORMAT>  The format to dump the data in [default: debug]
                         [options: dbg/debug, bin/binary]

  {'bold}    --list-tables{'_}      List all supported OpenType tables and formats
                         [with <FONT>, only shows ones in the font file]

  {'bold}-h, --help{'_}             Print help
  {'bold}-V, --version{'_}          Print version"#
    );
}

fn print_tables(dir: Option<&TableDirectoryRepr>) {
    if dir.is_some() {
        println!("The specified font contains the following tables:");
    } else {
        println!("Currently only the following OpenType tables are implemented:");
    }

    // Print the table header, and the table directory row
    printacln!(
        r#"
{'bold u}OT Tag{'_}  {'bold u}OT Table Name{'_}                        {'bold u}Supported Formats{'_}
{'bold i} dir  {'_}  Table directory [alias {'bold i}directory{'_}]    bin,dbg"#
    );

    // Collect a vec of all tags to be shown
    let tags = if let Some(dir) = dir {
        Vec::from_iter(dir.table_records_raw().iter().map(|x| x.table_tag))
    } else {
        Vec::from_iter(SUPPORTED_TABLES.iter().map(|x| x.tag))
    };

    for tag in tags {
        let known = SUPPORTED_TABLES.iter().find(|x| x.tag == tag);

        // If not showing tables from a font file, skip tables without a Debug impl
        if dir.is_none() && known.is_some_and(|x| !x.dbg) {
            continue;
        }

        let name = known.map_or("UNKNOWN TABLE", |x| x.name);

        // Tables with formats list their formats (Debug is usually the first to be implemented)
        if let Some(known) = known
            && known.dbg
        {
            printac!("{'bold}{:?}{'_bold}  {:35}  bin,dbg", tag, name);
            known.json.then(|| print!(",json"));
            known.ttx.then(|| print!(",ttx"));
            println!();
        }
        // Tables without any format impls are greyed out
        else {
            // Unknown tables are highlighted in red
            if known.is_none() {
                printac!("{'r}");
            }
            printacln!("{'bold f}{:?}{'_bold}  {'f}{:35}{'_fg}  bin{'_}", tag, name);
        }
    }

    printacln!("\nNote: {'bold}binary{'_} format is always supported, even for unknown tables.");
}

trait DumpDebug {
    fn debug_is_supported(&self) -> bool;
    fn debug_dump(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}
impl<T> DumpDebug for T {
    default fn debug_is_supported(&self) -> bool {
        false
    }
    default fn debug_dump(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Err(std::fmt::Error)
    }
}
impl<T: Debug> DumpDebug for T {
    fn debug_is_supported(&self) -> bool {
        true
    }
    fn debug_dump(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
