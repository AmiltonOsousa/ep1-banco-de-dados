use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use mlua::{Function, Lua, Table};

use crate::storage::Storage;

pub struct LuaBridge {
    lua: Lua,
}

impl LuaBridge {
    pub fn new(storage: Arc<Mutex<Storage>>) -> Result<Self, String> {
        let lua = Lua::new();

        Self::register_database_query(&lua, storage)?;

        let bridge = Self { lua };

        bridge.load_extensions("utils")?;

        Ok(bridge)
    }

    fn register_database_query(
        lua: &Lua,
        storage: Arc<Mutex<Storage>>,
    ) -> Result<(), String> {
        let globals = lua.globals();

        let db = lua
            .create_table()
            .map_err(|error| format!("erro ao criar objeto db: {}", error))?;

        let storage_clone = Arc::clone(&storage);

        let get = lua
            .create_function(move |_, key: String| {
                let storage = storage_clone.lock().map_err(|_| {
                    mlua::Error::RuntimeError(
                        "erro ao acessar banco de dados".to_string(),
                    )
                })?;

                match storage.get(&key) {
                    Some(value) => Ok(Some(value.clone())),
                    None => Ok(None),
                }
            })
            .map_err(|error| format!("erro ao criar db.get: {}", error))?;

        db.set("get", get)
            .map_err(|error| format!("erro ao registrar db.get: {}", error))?;

        globals
            .set("db", db)
            .map_err(|error| format!("erro ao registrar objeto db: {}", error))?;

        Ok(())
    }

    pub fn add(
        &self,
        key: &str,
        value: &str,
    ) -> Result<String, String> {
        let extensions = self
            .lua
            .globals()
            .get::<Table>("_extensions")
            .map_err(|error| error.to_string())?;

        let extension = self.find_extension(&extensions, key)?;

        let Some(extension) = extension else {
            return Ok(value.to_string());
        };

        let function: Function = extension
            .get("add")
            .map_err(|error| {
                format!("erro ao obter operação ADD: {}", error)
            })?;

        let result: Table = function
            .call((key, value))
            .map_err(|error| {
                format!("erro na extensão ADD: {}", error)
            })?;

        let ok: bool = result
            .get("ok")
            .map_err(|error| {
                format!("resultado ADD inválido: {}", error)
            })?;

        if !ok {
            let error: String = result
                .get("error")
                .unwrap_or_else(|_| {
                    "valor rejeitado pela extensão".to_string()
                });

            return Err(error);
        }

        let transformed: String = result
            .get("value")
            .map_err(|error| {
                format!("resultado ADD inválido: {}", error)
            })?;

        Ok(transformed)
    }

    pub fn get(
        &self,
        key: &str,
        value: &str,
    ) -> Result<String, String> {
        let extensions = self
            .lua
            .globals()
            .get::<Table>("_extensions")
            .map_err(|error| error.to_string())?;

        let extension = self.find_extension(&extensions, key)?;

        let Some(extension) = extension else {
            return Ok(value.to_string());
        };

        let function: Function = extension
            .get("get")
            .map_err(|error| {
                format!("erro ao obter operação GET: {}", error)
            })?;

        let result: Table = function
            .call((key, value))
            .map_err(|error| {
                format!("erro na extensão GET: {}", error)
            })?;

        let ok: bool = result
            .get("ok")
            .map_err(|error| {
                format!("resultado GET inválido: {}", error)
            })?;

        if !ok {
            let error: String = result
                .get("error")
                .unwrap_or_else(|_| {
                    "erro na extensão".to_string()
                });

            return Err(error);
        }

        let transformed: String = result
            .get("value")
            .map_err(|error| {
                format!("resultado GET inválido: {}", error)
            })?;

        Ok(transformed)
    }

    fn find_extension(
        &self,
        extensions: &Table,
        key: &str,
    ) -> Result<Option<Table>, String> {
        let mut index = 1;

        loop {
            let extension: Option<Table> = extensions
                .get(index)
                .map_err(|error| error.to_string())?;

            let Some(extension) = extension else {
                break;
            };

            let prefix: String = extension
                .get("prefix")
                .map_err(|error| error.to_string())?;

            if key.starts_with(&prefix) {
                return Ok(Some(extension));
            }

            index += 1;
        }

        Ok(None)
    }

    fn load_extensions(
        &self,
        directory: &str,
    ) -> Result<(), String> {
        let entries = fs::read_dir(Path::new(directory))
            .map_err(|error| {
                format!(
                    "não foi possível abrir o diretório de extensões: {}",
                    error
                )
            })?;

        let extensions = self
            .lua
            .create_table()
            .map_err(|error| {
                format!(
                    "erro ao criar lista de extensões: {}",
                    error
                )
            })?;

        let mut index = 1;

        for entry in entries {
            let entry = entry
                .map_err(|error| {
                    format!("erro ao ler extensão: {}", error)
                })?;

            let path = entry.path();

            if path.extension()
                .and_then(|extension| extension.to_str())
                != Some("lua")
            {
                continue;
            }

            let source = fs::read_to_string(&path)
                .map_err(|error| {
                    format!(
                        "erro ao ler extensão {:?}: {}",
                        path, error
                    )
                })?;

            let extension: Table = self
                .lua
                .load(&source)
                .eval()
                .map_err(|error| {
                    format!(
                        "erro ao carregar extensão {:?}: {}",
                        path, error
                    )
                })?;

            let prefix: String = extension
                .get("prefix")
                .map_err(|error| {
                    format!(
                        "extensão {:?} sem prefixo: {}",
                        path, error
                    )
                })?;

            let _: Function = extension
                .get("add")
                .map_err(|error| {
                    format!(
                        "extensão {:?} sem operação ADD: {}",
                        path, error
                    )
                })?;

            let _: Function = extension
                .get("get")
                .map_err(|error| {
                    format!(
                        "extensão {:?} sem operação GET: {}",
                        path, error
                    )
                })?;

            extensions
                .set(index, extension)
                .map_err(|error| {
                    format!(
                        "erro ao registrar extensão {:?}: {}",
                        path, error
                    )
                })?;

            println!("Extensão carregada: {}", prefix);

            index += 1;
        }

        self.lua
            .globals()
            .set("_extensions", extensions)
            .map_err(|error| {
                format!(
                    "erro ao registrar extensões: {}",
                    error
                )
            })?;

        Ok(())
    }
}