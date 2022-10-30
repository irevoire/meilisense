use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::Index;
use clap::Parser;
use dump::{DumpWriter, IndexMetadata};
use meilisearch_types::{milli::update::Setting, settings::Settings};
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

    /// Path to write the dump.
    #[clap(default_value = "./typesense.dump")]
    dump_path: PathBuf,
}

impl Opt {
    pub fn process(self) {
        let db_path = self.db_path.join("db");

        let opt = Options::default();
        let db = DB::open_for_read_only(&opt, db_path, false).unwrap();
        let dump = DumpWriter::new(None).unwrap();

        dump.create_keys().unwrap().flush().unwrap();
        dump.create_tasks_queue().unwrap().flush().unwrap();

        let indexes = Index::from_db(&db);
        for index in indexes {
            println!("Dumping the index `{}`.", index.name);
            println!("Lost the primary key, let's hope it's id or somethin'");
            let metadata = IndexMetadata {
                uid: index.name.to_string(),
                primary_key: None,
                created_at: index.created_at,
                updated_at: index.created_at,
            };
            let mut dump_index = dump.create_index(&index.name, &metadata).unwrap();
            let settings = Settings {
                displayed_attributes: Setting::NotSet,
                searchable_attributes: Setting::Set(
                    index
                        .fields
                        .iter()
                        .filter(|field| field.index)
                        .map(|field| field.name.to_string())
                        .collect(),
                ),
                filterable_attributes: Setting::Set(
                    index
                        .fields
                        .iter()
                        .filter(|field| field.facet)
                        .map(|field| field.name.to_string())
                        .collect(),
                ),
                sortable_attributes: Setting::Set(
                    index
                        .fields
                        .iter()
                        .filter(|field| field.sort)
                        .map(|field| field.name.to_string())
                        .collect(),
                ),
                ranking_rules: Setting::NotSet,
                stop_words: Setting::NotSet,
                synonyms: Setting::NotSet,
                distinct_attribute: Setting::NotSet,
                typo_tolerance: Setting::NotSet,
                faceting: Setting::NotSet,
                pagination: Setting::NotSet,
                _kind: std::marker::PhantomData,
            };

            for document in index.get_documents(&db) {
                dump_index
                    .push_document(document.as_object().expect("Malformed document"))
                    .unwrap();
            }

            dump_index.flush().unwrap();
            dump_index.settings(&settings).unwrap();
        }

        let out_file = File::create(self.dump_path).unwrap();
        let mut writer = BufWriter::new(out_file);

        dump.persist_to(&mut writer).unwrap();

        writer.flush().unwrap();
    }
}
