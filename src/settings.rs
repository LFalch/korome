extern crate toml;

use toml::Table;
use toml::Value::*;

use std::path::Path;
use std::io::{Read, Result as IOResult};
use std::fs::File;

/// Struct for gettings settings with default properties
pub struct Settings{
    inner: Table
}

#[allow(missing_docs)]
impl Settings{
    /// Creates a new `Settings` instance from the given file
    pub fn new<P: AsRef<Path>>(path: P) -> IOResult<Self>{
        let mut buffer = String::new();

        File::open(path)
            .and_then(|mut file| file.read_to_string(&mut buffer))
            .and(Ok(Self::from_string(&buffer).unwrap()))
    }

    /// Returns an optional `Settings` that is `None` if parsing failed
    pub fn from_string(toml: &str) -> Option<Self>{
        toml::Parser::new(toml).parse().map(|table| Settings{
            inner: table
        })
    }

    pub fn get_string(&mut self, key: &str, default: &str) -> Option<String>{
        match self.inner.get(key){
            Some(x) => return x.as_str().map(|s| s.to_string()),
            None => ()
        }

        self.inner.insert(key.to_string(), String(default.to_string()));

        Some(default.to_string())
    }

    pub fn get_int(&mut self, key: &str, default: i64) -> Option<i64>{
        match self.inner.get(key){
            Some(x) => return x.as_integer(),
            None => ()
        }

        self.inner.insert(key.to_string(), Integer(default));

        Some(default)
    }

    pub fn get_float(&mut self, key: &str, default: f64) -> Option<f64>{
        match self.inner.get(key){
            Some(x) => return x.as_float(),
            None => ()
        }

        self.inner.insert(key.to_string(), Float(default));

        Some(default)
    }

    pub fn get_bool(&mut self, key: &str, default: bool) -> Option<bool>{
        match self.inner.get(key){
            Some(x) => return x.as_bool(),
            None => ()
        }

        self.inner.insert(key.to_string(), Boolean(default));

        Some(default)
    }
}
