//! The library crate.
//!
//! It implements the entire application.
//!
//! Although this library is only meant for the single binary,
//! the application is split into a binary and library to overcome
//! limitations on doc-tests, which can only run in library crates.

/// The entry point for the "library", which implements the game.
pub fn libmain() {
    println!("Library main function!");
}

#[cfg(test)]
mod tests {

    #[test]
    #[should_panic]
    pub fn test_panic() {
        panic!("Unit test");
    }
}

