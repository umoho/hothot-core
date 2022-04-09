use clap::Parser;

#[derive(Debug)]
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Config file path
    #[clap(short, long, value_name = "PATH", default_value = "config.toml")]
    pub config: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
