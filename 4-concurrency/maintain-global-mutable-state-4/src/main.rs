extern crate lazy_static;
extern crate error_chain;

use error_chain::error_chain;
use lazy_static::lazy_static;
use std::sync::Mutex;

error_chain!{ }

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
}

fn main() -> Result<()> {
    // lazy_static creates a globally available static ref which requires
    // a Mutex to allow mutation

    // The Mutex wrap ensures the state cannot be simultaneously accessed
    // by multiple threads, preventing race conditions

    // A MutexGuard must be acquired to read or mutate the value stored
    // in a Mutex
    
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;

        db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i, item));
    }
    insert("grape")?;
    Ok(())
}
