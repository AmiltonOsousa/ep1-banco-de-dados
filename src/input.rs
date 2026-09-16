use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex};

use crate::commander::{parse, Command};
use crate::storage::Storage;

pub fn run(storage: Arc<Mutex<Storage>>) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut line = String::new();

        match stdin.lock().read_line(&mut line) {
            Ok(0) => break,

            Ok(_) => {}

            Err(error) => {
                println!("ERRO: falha ao ler entrada: {}", error);
                continue;
            }
        }

        match parse(&line) {
            Ok(Command::Exit) => break,

            Ok(Command::Add { key, value }) => {
                let mut storage = match storage.lock() {
                    Ok(storage) => storage,
                    Err(_) => {
                        println!("ERRO: falha ao acessar banco de dados");
                        continue;
                    }
                };

                storage.add(key, value);

                println!("OK");
            }

            Ok(Command::Get { key }) => {
                let storage = match storage.lock() {
                    Ok(storage) => storage,
                    Err(_) => {
                        println!("ERRO: falha ao acessar banco de dados");
                        continue;
                    }
                };

                match storage.get(&key) {
                    Some(value) => println!("{}", value),
                    None => println!("ERRO: chave inexistente"),
                }
            }

            Err(error) => {
                println!("ERRO: {}", error);
            }
        }
    }
}