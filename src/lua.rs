use std::fs;
use std::path::Path;

use mlua::{Function, Lua, Table};

pub struct LuaBridge {
    lua: Lua,
}

impl LuaBridge {
    pub fn new() -> Result<Self, String> {
        let lua = Lua::new();

        let bridge = Self { lua };

        bridge.load_extensions("utils")?;

        Ok(bridge)
    }

    fn load_extensions(&self, directory: &str) -> Result<(), String> {
        let entries = fs::read_dir(Path::new(directory)).map_err(|error| {
            format!(
                "não foi possível abrir o diretório de extensões: {}",
                error
            )
        })?;

        for entry in entries {
            let entry = entry
                .map_err(|error| format!("erro ao ler extensão: {}", error))?;

            let path = entry.path();

            if path.extension().and_then(|extension| extension.to_str()) != Some("lua") {
                continue;
            }

            let source = fs::read_to_string(&path).map_err(|error| {
                format!("erro ao ler extensão {:?}: {}", path, error)
            })?;

            let extension: Table = self
                .lua
                .load(&source)
                .eval()
                .map_err(|error| {
                    format!("erro ao carregar extensão {:?}: {}", path, error)
                })?;

            let prefix: String = extension.get("prefix").map_err(|error| {
                format!("extensão {:?} sem prefixo: {}", path, error)
            })?;

            let _: Function = extension.get("add").map_err(|error| {
                format!("extensão {:?} sem operação ADD: {}", path, error)
            })?;

            let _: Function = extension.get("get").map_err(|error| {
                format!("extensão {:?} sem operação GET: {}", path, error)
            })?;

            println!("Extensão carregada: {}", prefix);
        }

        Ok(())
    }
}