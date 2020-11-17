//! The binary crate.
//! It does nothing except call into the library.

extern crate mtielib;

fn main() {
    if let Err(err) = mtielib::run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
