mod commander;
mod input;
mod lua;
mod storage;

use lua::LuaBridge;
use storage::Storage;

fn main() {
    let mut storage = Storage::new();

    let _lua = match LuaBridge::new() {
        Ok(lua) => lua,
        Err(error) => {
            println!("ERRO: {}", error);
            return;
        }
    };

    input::run(&mut storage);
}