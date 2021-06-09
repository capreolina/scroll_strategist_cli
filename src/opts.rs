use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "deer <capreolina@protonmail.ch>")]
pub struct Opts {
    #[clap(short, long)]
    pub json: Option<String>,
    #[clap(short, long)]
    pub noninteractive: bool,
}
