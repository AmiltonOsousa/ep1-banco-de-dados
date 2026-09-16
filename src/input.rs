use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex};

use crate::commander::{parse, Command};
use crate::lua::LuaBridge;
use crate::storage::Storage;

pub fn run(
    storage: Arc<Mutex<Storage>>,
    lua: &LuaBridge,
) {
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
                let value = match lua.add(&key, &value) {
                    Ok(value) => value,
                    Err(error) => {
                        println!("ERRO: {}", error);
                        continue;
                    }
                };

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

                let value = match storage.get(&key) {
                    Some(value) => value.clone(),
                    None => {
                        println!("ERRO: chave inexistente");
                        continue;
                    }
                };

                drop(storage);

                match lua.get(&key, &value) {
                    Ok(value) => println!("{}", value),
                    Err(error) => println!("ERRO: {}", error),
                }
            }

            Err(error) => {
                println!("ERRO: {}", error);
            }
        }
    }
}