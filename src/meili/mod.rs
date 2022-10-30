use clap::Parser;

pub mod dump;

#[derive(Debug, Parser)]
#[command(
    author = "tamo",
    version,
    about = "Provide helpers to dump data out of your typesense database."
)]
pub enum Opt {
    Dump(dump::Opt),
}

impl Opt {
    pub fn process(self) {
        match self {
            Opt::Dump(opt) => opt.process(),
        }
    }
}
