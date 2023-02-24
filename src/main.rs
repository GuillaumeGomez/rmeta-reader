#![feature(rustc_private)]
#![feature(once_cell)]

// We need to import them like this otherwise it doesn't work.
pub extern crate rustc_data_structures;
pub extern crate rustc_metadata;

use rustc_data_structures::memmap::Mmap;
use rustc_data_structures::owning_ref::OwningRef;
use rustc_data_structures::rustc_erase_owner;
use rustc_metadata::MetadataBlob;

fn main() -> Result<(), String> {
    for argument in std::env::args() {
        let file = std::fs::File::open(argument).map_err(|_| {
            format!("failed to open rmeta metadata: '{argument}'")
        })?;
        let mmap = unsafe { Mmap::map(file) };
        let mmap = mmap.map_err(|_| {
            format!("failed to mmap rmeta metadata: '{argument}'")
        })?;

        let raw_bytes = rustc_erase_owner!(OwningRef::new(mmap).map_owner_box());
        let blob = MetadataBlob::new(raw_bytes);
        println!("Got it!");
    }

    Ok(())
}
