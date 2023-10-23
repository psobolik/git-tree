use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Clargs {
    #[arg(default_value = ".")]
    pub folder: String,

    #[arg(long, help = "Use ASCII instead of extended characters")]
    pub ascii: bool,
}