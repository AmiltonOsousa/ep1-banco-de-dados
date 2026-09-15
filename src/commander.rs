pub enum Command {
    Add { key: String, value: String },
    Get { key: String },
    Exit,
}

pub fn parse(input: &str) -> Result<Command, String> {
    let input = input.trim();

    if input.is_empty() {
        return Err("linha vazia".to_string());
    }

    if input == "EXIT" {
        return Ok(Command::Exit);
    }

    if let Some(rest) = input.strip_prefix("ADD ") {
        let mut parts = rest.splitn(2, ' ');

        let key = parts
            .next()
            .filter(|key| !key.is_empty())
            .ok_or_else(|| "comando ADD incompleto".to_string())?;

        let value = parts
            .next()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| "comando ADD incompleto".to_string())?;

        return Ok(Command::Add {
            key: key.to_string(),
            value: value.to_string(),
        });
    }

    if let Some(rest) = input.strip_prefix("GET ") {
        let key = rest.trim();

        if key.is_empty() || key.contains(' ') {
            return Err("comando GET incompleto".to_string());
        }

        return Ok(Command::Get {
            key: key.to_string(),
        });
    }

    Err("comando desconhecido".to_string())
}