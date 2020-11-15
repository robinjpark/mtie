//! The binary crate.
//! It does nothing except call into the library.
extern crate mtielib;

/// The main entry point of the binary.
///
/// It simply calls the main() function in the associated library.
fn main() {
    if let Err(err) = mtielib::run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
