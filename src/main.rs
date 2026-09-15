mod command;
mod storage;

use command::parse;
use storage::Storage;

fn main() {
    let mut storage = Storage::new();

    let commands = [
        "ADD nome Ze Zinho",
        "GET nome",
        "GET inexistente",
        "EXIT",
    ];

    for line in commands {
        match parse(line) {
            Ok(command) => match command {
                command::Command::Add { key, value } => {
                    storage.add(key, value);
                    println!("OK");
                }

                command::Command::Get { key } => {
                    match storage.get(&key) {
                        Some(value) => println!("{}", value),
                        None => println!("ERRO: chave inexistente"),
                    }
                }

                command::Command::Exit => break,
            },

            Err(error) => {
                println!("ERRO: {}", error);
            }
        }
    }
}