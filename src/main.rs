use clap::Parser;

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
