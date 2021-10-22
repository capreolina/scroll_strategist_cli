use clap::Parser;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "deer <capreolina@protonmail.ch>")]
pub struct Opts {
    #[clap(short, long)]
    pub json: Option<String>,
    #[clap(short, long)]
    pub noninteractive: bool,
}
