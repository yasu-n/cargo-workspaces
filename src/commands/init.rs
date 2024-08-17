use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

use clap::Parser;
use git2::Repository;
use toml_edit::{Array, DocumentMut, Item, Table, Value};

use crate::util::Result;

#[derive(Debug, Parser)]
pub struct Init {}

impl Init {
    pub fn exec(&self) -> Result {
        let path = env::current_dir()?;

        let mut menifest = File::create(path.join("Cargo.toml"))?;

        let mut doc = DocumentMut::new();
        let mut members = Array::new();

        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let meta = entry.metadata()?;
            if meta.is_dir() {
                members.push(entry.file_name().into_string().unwrap());
            }
        }

        doc["workspace"] = Item::Table(Table::new());
        doc["workspace"]["members"] = Item::Value(Value::Array(members));

        write!(menifest, "{}", doc.to_string())?;

        // git initialized
        if !Path::new(&path.join(".git")).exists() {
            Repository::init(&path)?;
        }

        println!(
            "initialized workspace: {}",
            &path.file_name().unwrap().to_string_lossy()
        );

        Ok(())
    }
}
