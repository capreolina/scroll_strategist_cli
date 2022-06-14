use clap::Parser;

#[derive(Parser)]
#[clap(version, author, about)]
pub struct Opts {
    #[clap(
        help = "The path to a JSON file to use for equipment & scroll \
information.",
        short,
        long,
        value_parser
    )]
    pub json: Option<String>,
    #[clap(
        long_help = "Non-interactive mode will not interactively query for \
equipment & scroll information, and will not drop you into a quasi-shell \
after generating a scrolling strategy. Instead, it will exit immediately \
after emitting the first scroll choice. This option is invalid without \
`--json` because otherwise, there would be no source of equipment & scroll \
information.",
        help = "Non-interactive mode exits after initial output.",
        short,
        long,
        value_parser,
        requires = "json"
    )]
    pub noninteractive: bool,
}
