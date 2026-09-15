mod commander;
mod input;
mod storage;

use input::run;
use storage::Storage;

fn main() {
    let mut storage = Storage::new();

    run(&mut storage);
}