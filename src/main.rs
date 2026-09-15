mod storage;

use storage::Storage;

fn main() {
    let mut storage = Storage::new();

    storage.add(
        "nome".to_string(),
        "Amilton".to_string(),
    );

    match storage.get("nome") {
        Some(value) => println!("{}", value),
        None => println!("chave inexistente"),
    }
}