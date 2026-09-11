//! A TrueType/OpenType font parsing/viewing library and a CLI tool.
//!
//! # Features
//!
//! Not much is implemented yet, but here's a few features that [`ttf-parser`] and [`read-fonts`]
//! don't have:
//!
//! - Two-byte codepoint mappings (other crates only support Unicode).
//! - Supports more character encodings (such as Shift JIS, GB 18030, Big5, EUC-KR).
//! - Provides raw low-level access to all structures (with &, and not just getters).
//! - Faster and easier access to metrics in the 'hmtx' table.
//! - Actually implements the 'cmap' format 2 subtable (deprecated, but still used by some fonts).
//!
//! Just like `ttf-parser` and `read-fonts`, this is zero-copy and no-alloc too. Also, validation of
//! the table data only happens when you access the table (`dir.hmtx()`), instead of doing bounds
//! checks on every single field access like `ttf-parser` and `read-fonts` do, for some reason.
//!
//! The documentation will also be better, more descriptive and with examples. I want the entire
//! crate to be comprehensible through documentation alone, without the need to look through what
//! the language server shows you a type can do.
//!
//! # Reading a font file
//!
//! [`TableDirectory`][tables::TableDirectory] is the entry point and main hub for OpenType tables.
//! Use [`TableDirectory::new(bytes)`][tables::TableDirectory::new] to read a font file. Note that
//! the validation of specific tables happens separately and is not cached.
//!
//! ```no_run
//! use ttf_view::tables::TableDirectory;
//!
//! let data = std::fs::read("MyFont.ttf").unwrap();
//! let dir = TableDirectory::new(&data).expect("font should be well-formed");
//!
//! if let Ok(cmap) = dir.cmap() {
//!     // ...
//! }
//! ```
//!
//! See the [`tables`] module for more information about tables that you can access.
//!
//! # Notes on how to access tables
//!
//! A table should generally be accessed through a type like [`Os_2<'_>`](tables::os_2::Os_2). This
//! type automatically dereferences to [`Os_2Base`][tables::os_2::Os_2Base], which contains fields
//! that are available in all versions of `'OS/2'` table. You can access specific versions of the
//! table through methods like `v0`, `v1`, `v4` on it, which return references to the raw data with
//! fields exactly as specified by OpenType. Additionally, each version of the table automatically
//! dereferences to the previous version, e.g. `Os_2V1` to `Os_2V0`, `Os_2V0` to `Os_2Base`.
//!
//! ```no_run
//! # use ttf_view::tables::{TableDirectory, os_2::Os_2};
//! # let data = std::fs::read("MyFont.ttf").unwrap();
//! # let dir = unsafe { TableDirectory::new_unchecked(&data) };
//! #
//! let os_2: Os_2<'_> = dir.os_2().unwrap();
//! println!("Vendor ID: {}", os_2.ach_vend_id); // a field in Os_2Base
//!
//! if let Some(v4) = os_2.v4() { // v4 is &Os_2V4
//!     println!("Last char idx: {}", v4.us_last_char_index); // a field in Os_2Base
//!     println!("Default char: {}", v4.us_default_char); // a field in Os_2V4
//! }
//! ```
//!
//! Note about types: if a type is suffixed with `Raw`, then it means it doesn't provide much info
//! on its own, and you should generally use a higher-level type without this suffix. For example,
//! [`TableRecordRaw`][tables::TableRecordRaw] only specifies an offset from the font's root, so it
//! can't provide the actual table's data. But [`TableRecord`](tables::TableRecord) is a
//! higher-level wrapper which does have a reference to the font, and can provide its table's data.
//!
//! ```no_run
//! # use ttf_view::{
//! #     tables::{TableDirectory, TableRecord, TableRecordRaw, head::Head},
//! #     types::tags,
//! # };
//! # let data = std::fs::read("MyFont.ttf").unwrap();
//! # let dir = unsafe { TableDirectory::new_unchecked(&data) };
//! #
//! let record_raw: &TableRecordRaw = dir.table_record_raw(tags::head).unwrap();
//! // can access fields just fine
//! println!("'head' length: {}", record_raw.length);
//! println!("'head' offset: {}", record_raw.offset);
//! // but can't access the actual table
//!
//! let record: TableRecord<'_> = dir.table_record(tags::head).unwrap();
//! // auto-derefs to &TableRecordRaw for field access
//! println!("'head' length: {}", record_raw.length);
//! println!("'head' offset: {}", record_raw.offset);
//! // and can also access the actual table
//! let bytes = record.table_as_bytes();
//! let head = record.table_as::<Head<'_>>().unwrap();
//! ```
//!
//! [`ttf-parser`]: https://docs.rs/ttf-parser/latest/ttf_parser/
//! [`read-fonts`]: https://docs.rs/read-fonts/latest/read_fonts/

#![feature(const_trait_impl)]
#![feature(const_result_trait_fn)]
#![feature(const_slice_from_ptr_range)]
#![feature(const_slice_make_iter)]
#![feature(const_option_ops)]
#![feature(const_closures)]
#![feature(const_convert)]
#![feature(const_default)]
#![feature(const_clone)]
#![feature(const_index)]
#![feature(const_bool)]
#![feature(const_iter)]
#![feature(const_cmp)]
#![feature(const_ops)]
#![feature(const_try)]
#![feature(derive_const)]
#![feature(bstr)]
#![feature(debug_closure_helpers)]
#![feature(formatting_options)]
#![feature(slice_from_ptr_range)]
#![feature(try_trait_v2)]
#![allow(clippy::manual_non_exhaustive)]
#![allow(clippy::missing_safety_doc)] // TODO: remove when adding docs

pub mod platform;
pub mod tables;
pub mod types;

mod util;
