use std::fs;

use anyhow::Context;
use cargo_metadata::{CargoOpt, Metadata, MetadataCommand};
use clap::Parser;
use toml_edit::DocumentMut;

use crate::util::Result;

#[derive(Debug, Parser)]
pub struct List {}

impl List {
    pub fn exec(&self) -> Result {
        let mut cmd = MetadataCommand::new();
        cmd.no_deps();
        cmd.features(CargoOpt::AllFeatures);
        match cmd.exec() {
            Ok(meta) => {
                self.show(&meta)?;
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
        Ok(())
    }

    fn show(&self, meta: &Metadata) -> Result {
        let path = meta.workspace_root.join("Cargo.toml");
        let contents = fs::read_to_string(path)?;
        let mut doc = contents.parse::<DocumentMut>()?;
        let Some(ws_doc) = doc.get_mut("workspace") else {
            return Ok(());
        };

        println!("workspace members: ");
        if let Some(members) = ws_doc
            .get_mut("members")
            .and_then(|members| members.as_array_mut()) {
            
            for member in members.iter() {
                let m = member
                    .as_str()
                    .with_context(|| format!("invalid non-string member '{}'", member))?;
                println!("\t{}", m);
            }
        }
        Ok(())
    }
}
