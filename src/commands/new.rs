use std::env;
use std::fs::{self, File};
use std::io::Write;

use clap::{Parser, ValueEnum};

use git2::Repository;
use toml_edit::{value, Array, DocumentMut, Item, Table, Value};

use crate::util::error::Error;
use crate::Result;

#[derive(Debug, Clone, ValueEnum)]
enum Resolver {
    #[clap(name = "1")]
    V1,
    #[clap(name = "2")]
    V2,
}

/// create new crate
#[derive(Debug, Parser)]
pub struct Create {
    /// name of crate
    name: String,

    /// crate resolver
    #[clap(long, value_enum)]
    resolver: Option<Resolver>,
}

impl Create {
    pub fn exec(&self) -> Result {
        let path = env::current_dir()?;
        let path = path.join(&self.name);

        if path.exists() {
            return Err(Error::AlreadyExistsCrate(self.name.clone()));
        }

        fs::create_dir(&path)?;

        // create contents for Cargo.toml
        let mut doc = DocumentMut::new();
        doc["workspace"] = Item::Table(Table::new());
        doc["workspace"]["members"] = Item::Value(Value::Array(Array::new()));
        match &self.resolver {
            Some(version) => {
                doc["workspace"]["resolver"] = match version {
                    Resolver::V1 => value("1"),
                    Resolver::V2 => value("2"),
                }
            }
            None => {}
        }

        doc["workspace.package"] = Item::Table(Table::new());
        doc["workspace.package"] = value("");
        doc["workspace.dependencies"] = Item::Table(Table::new());

        // create Cargo.toml
        // writing contents in Cargo.toml
        let mut manifest = File::create(path.join("Cargo.toml"))?;
        write!(manifest, "{}", doc.to_string())?;

        // git init
        Repository::init(path)?;

        println!("create new workspace: {}", &self.name);

        Ok(())
    }
}
