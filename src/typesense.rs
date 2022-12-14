use std::path::PathBuf;

use crate::Index;
use clap::Parser;
use rocksdb::{Options, DB};

#[derive(Debug, Parser)]
#[command(
    author = "tamo",
    version,
    about = "Provide helpers to dump data out of your typesense database."
)]
pub struct Opt {
    /// Path of the typesense datatabase.
    db_path: PathBuf,

    /// The index you want to dump.
    #[arg(short, long)]
    index: Option<String>,

    /// Use this flag to dump the document instead of the settings.
    #[arg(short, long)]
    documents: bool,
}

impl Opt {
    pub fn process(self) {
        let db_path = self.db_path.join("db");

        let opt = Options::default();
        let db = DB::open_for_read_only(&opt, db_path, false).unwrap();

        let indexes = Index::from_db(&db);

        if let Some(index) = self.index {
            let index = indexes
                .iter()
                .find(|idx| idx.name == index)
                .expect("The requested index does not exists");

            if self.documents {
                for document in index.get_documents(&db) {
                    let doc =
                        serde_json::to_string(&document).expect("Could not serialize a document.");
                    println!("{doc}");
                }
            } else {
                let settings =
                    serde_json::to_string(index).expect("Could not serialize your settings.");
                println!("{settings}");
            }
        } else {
            for index in indexes {
                println!("========== Dumping index `{}` ==========", index.name);
                println!("==== Settings ====");
                let settings =
                    serde_json::to_string(&index).expect("Could not serialize your settings.");
                println!("{settings}");
                println!("==== Documents ====");
                for document in index.get_documents(&db) {
                    let doc =
                        serde_json::to_string(&document).expect("Could not serialize a document.");
                    println!("{doc}");
                }
                println!();
            }
        }
    }
}
