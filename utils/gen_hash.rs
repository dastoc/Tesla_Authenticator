//! ```cargo
//! [dependencies]
//! bcrypt = "0.13"
//! ```

use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let password = "password123";
    match hash(password, DEFAULT_COST) {
        Ok(hashed) => println!("Generated hash: {}", hashed),
        Err(e) => eprintln!("Error: {}", e),
    }
}