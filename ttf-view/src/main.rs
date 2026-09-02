use std::fs;
use ttf_view::tables::TableDirectoryRepr;

fn main() {
    let font_data = fs::read("test.ttf").unwrap();

    let dir = unsafe { TableDirectoryRepr::new_unchecked(&font_data) };

    println!("{:#?}", dir);
}
