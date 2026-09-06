//! A TrueType/OpenType font parsing/viewing library and a CLI tool.
//!
//! # Features
//!
//! Not much is implemented yet, but here's a few features that [`ttf-parser`] and [`read-fonts`]
//! don't have:
//!
//! - Two-byte codepoint mappings (other crates only support Unicode).
//! - Supports more character encodings (such as Shift JIS, GB 18030, Big5, EUC-KR).
//! - Faster and easier access to metrics in the 'hmtx' table.
//! - Actually implements 'cmap' format 2 subtable.
//!
//! Also, while not implemented yet, validation of the table data will only happen once - when you
//! load the font, instead doing bounds-checks on every single field access like `ttf-parser` and
//! `read-fonts` do, for some reason.
//!
//! The documentation will also be better, more descriptive and with examples and stuff. I want the
//! entire crate to be comprehensible through documentation alone, without the need to look through
//! what the language server shows you a type can do.
//!
//! # Reading a font file
//!
//! Loading fonts in a safe manner is not implemented yet, so you'll need to use `unsafe`.
//! It will not perform any validation or bounds checks, and may cause memory access violation.
//! But, as long as the font file is well-formed, there shouldn't be any issues.
//!
//! ```no_run
//! use ttf_view::tables::TableDirectoryRepr;
//!
//! let data = std::fs::read("MyFont.ttf").unwrap();
//! let dir = unsafe { TableDirectoryRepr::new_unchecked(&data) };
//!
//! if let Some(cmap) = dir.cmap() {
//!     // ...
//! }
//! ```
//!
//! [`TableDirectoryRepr`] is the entry point and the main hub for OpenType tables.
//!
//! See the [`tables`] module for more information about tables that you can access.
//!
//! [`TableDirectoryRepr`]: tables::TableDirectoryRepr
//! [`ttf-parser`]: https://docs.rs/ttf-parser/latest/ttf_parser/
//! [`read-fonts`]: https://docs.rs/read-fonts/latest/read_fonts/

#![feature(const_trait_impl)]
#![feature(const_result_trait_fn)]
#![feature(const_slice_from_ptr_range)]
#![feature(const_slice_make_iter)]
#![feature(const_option_ops)]
#![feature(const_convert)]
#![feature(const_default)]
#![feature(const_clone)]
#![feature(const_index)]
#![feature(const_iter)]
#![feature(const_cmp)]
#![feature(const_try)]
#![feature(derive_const)]
#![feature(bstr)]
#![feature(debug_closure_helpers)]
#![feature(formatting_options)]
#![feature(slice_from_ptr_range)]
#![feature(try_trait_v2)]
#![allow(clippy::missing_safety_doc)] // TODO: remove when adding docs

pub mod platform;
pub mod tables;
pub mod types;

mod util;
