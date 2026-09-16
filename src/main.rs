mod commander;
mod input;
mod lua;
mod storage;

use std::sync::{Arc, Mutex};

use lua::LuaBridge;
use storage::Storage;

fn main() {
    let storage = Arc::new(Mutex::new(Storage::new()));

    let lua = match LuaBridge::new(Arc::clone(&storage)) {
        Ok(lua) => lua,
        Err(error) => {
            println!("ERRO: {}", error);
            return;
        }
    };

    input::run(storage, &lua);
}