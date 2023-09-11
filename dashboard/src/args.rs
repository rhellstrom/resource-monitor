use clap:: {Parser};

#[derive(Parser)]
#[group(multiple = true)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path(s) for loading endpoints from file
    #[arg(default_value = "-")]
    pub files: Vec<String>,

    /// The UI tick rate
    #[arg(short, long("tick rate"), value_name = "milliseconds", default_value = "250")]
    pub tick_rate: u64,

    /// How often we fetch new data from server endpoints
    #[arg(short, long, value_name = "milliseconds", default_value = "1000")]
    pub update_frequency: u64,
}