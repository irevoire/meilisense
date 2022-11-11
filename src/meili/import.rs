use std::path::PathBuf;

use crate::Index;
use clap::Parser;
use meilisearch_sdk::{settings::Settings, Client};
use rocksdb::{Options, DB};

#[derive(Debug, Parser)]
#[command(
    author = "tamo",
    version,
    about = "Fill an already running meilisearch instance with the document in a typesense database."
)]
pub struct Opt {
    /// Path of the typesense datatabase.
    db_path: PathBuf,

    /// Path of the meilisearch instance.
    #[clap(default_value = "http://localhost:7700")]
    addr: String,

    /// The api key if there is one.
    #[clap(long, short)]
    key: Option<String>,
}

impl Opt {
    pub fn process(self) {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self._process())
    }

    async fn _process(self) {
        let db_path = self.db_path.join("db");

        let opt = Options::default();
        let db = DB::open_for_read_only(&opt, db_path, false).unwrap();
        let client = Client::new(self.addr, self.key.unwrap_or(String::from("lol")));

        let indexes = Index::from_db(&db);
        for index in indexes {
            println!("Creating the index `{}`.", index.name);
            let remote_index = client
                .create_index(index.name.clone(), Some("id"))
                .await
                .unwrap()
                .wait_for_completion(&client, None, None)
                .await
                .unwrap()
                .try_make_index(&client)
                .expect("Could not create index");

            let settings = Settings {
                displayed_attributes: None,
                searchable_attributes: Some(
                    index
                        .fields
                        .iter()
                        .filter(|field| field.index)
                        .map(|field| field.name.to_string())
                        .collect(),
                ),
                filterable_attributes: Some(
                    index
                        .fields
                        .iter()
                        .filter(|field| field.facet)
                        .map(|field| field.name.to_string())
                        .collect(),
                ),
                sortable_attributes: Some(
                    index
                        .fields
                        .iter()
                        .filter(|field| field.sort)
                        .map(|field| field.name.to_string())
                        .collect(),
                ),
                ranking_rules: None,
                stop_words: None,
                synonyms: None,
                distinct_attribute: None,
                faceting: None,
                pagination: None,
            };

            if remote_index
                .set_settings(&settings)
                .await
                .unwrap()
                .wait_for_completion(&client, None, None)
                .await
                .unwrap()
                .is_failure()
            {
                println!("The import of the settings failed.");
            }

            if remote_index
                .add_documents(&index.get_documents(&db), None)
                .await
                .unwrap()
                .wait_for_completion(&client, None, None)
                .await
                .unwrap()
                .is_failure()
            {
                println!("The import of the documents failed.");
            }
        }
    }
}
