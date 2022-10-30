use rocksdb::DB;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Index {
    #[serde(with = "time::serde::timestamp")]
    pub created_at: OffsetDateTime,
    pub default_sorting_field: String,
    pub fallback_field_type: String,
    pub fields: Vec<Field>,
    pub id: usize,
    pub name: String,
    pub num_memory_shards: usize,
    pub symbols_to_index: Vec<String>,
    pub token_separators: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Field {
    pub facet: bool,
    pub index: bool,
    pub infix: bool,
    pub locale: String,
    pub name: String,
    pub optional: bool,
    pub sort: bool,
    #[serde(rename = "type")]
    pub kind: Kind,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    String,
    #[serde(rename = "string[]")]
    StringArray,
    Int32,
    #[serde(rename = "int32[]")]
    Int32Array,
    Int64,
    #[serde(rename = "int64[]")]
    Int64Array,
    Float,
    #[serde(rename = "float[]")]
    FloatArray,
    Bool,
    #[serde(rename = "bool[]")]
    BoolArray,
}

impl Index {
    pub fn from_db(db: &DB) -> Vec<Index> {
        let mut indexes = Vec::new();

        // The databases starts with this prefix.
        let db_prefix = "$CM_";
        let databases = db.prefix_iterator(db_prefix.as_bytes());
        for (db_name, index) in databases {
            let db_name = std::str::from_utf8(&db_name).unwrap();
            if !db_name.starts_with(db_prefix) {
                // I don't know why rocksdb iterates over non prefix keys but let's ignore this shit
                break;
            }
            let index = serde_json::from_slice::<Index>(&index).unwrap();
            if !db_name.ends_with(&index.name) {
                println!("inconsistencies in the database");
            }
            indexes.push(index);
        }

        indexes
    }

    pub fn get_documents(&self, db: &DB) -> Vec<Value> {
        let mut documents = Vec::new();

        let documents_prefix = format!("{}_$SI_", self.id);
        let all_documents = db.prefix_iterator(documents_prefix.as_bytes());
        for (key, document) in all_documents {
            if !key.starts_with(documents_prefix.as_bytes()) {
                // I don't know why rocksdb iterates over non prefix keys but let's ignore this shit
                break;
            }
            let document = serde_json::from_slice::<Value>(&document).unwrap();
            documents.push(document);
        }

        documents
    }
}
