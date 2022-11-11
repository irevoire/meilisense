use clap::Parser;

pub mod dump;
pub mod import;

#[derive(Debug, Parser)]
#[command(
    author = "tamo",
    version,
    about = "Provide helpers to dump data out of your typesense database into your meilisearch database."
)]
pub enum Opt {
    Dump(dump::Opt),
    Import(import::Opt),
}

impl Opt {
    pub fn process(self) {
        match self {
            Opt::Dump(opt) => opt.process(),
            Opt::Import(opt) => opt.process(),
        }
    }
}
