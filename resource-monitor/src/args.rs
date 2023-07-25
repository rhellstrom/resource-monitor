use clap:: {Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The frequency which the system metrics are updated. Given in milliseconds
    #[arg(short('r'), long("update"), default_value = "2000")]
    pub update_frequency: u64,

    /// The port to listen for incoming requests
    #[arg(short, long("port"), default_value = "3000")]
    pub port: u16,

}