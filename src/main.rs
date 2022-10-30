use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use clap::Parser;
use clap::Subcommand;
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
    #[clap(subcommand)]
    inner: meilisense::Opt,
}

fn main() {
    let options = Opt::parse();
    options.inner.process();
}
