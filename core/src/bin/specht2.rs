use specht2_core::{
    server::{Server, ServerConfig},
    Result,
};
use std::{
    env,
    fs::read_to_string,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "specht2", about = "CLI version of the Specht2 client")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    flexi_logger::Logger::try_with_env()
        .unwrap()
        .start()
        .unwrap();

    let opt: Opt = Opt::from_args();

    let path: PathBuf = opt
        .input
        .or_else(|| {
            env::var("SNAP_COMMON")
                .map(|p| Path::new(&p).join("config.ron"))
                .ok()
        })
        .or_else(|| {
            env::var("HOME")
                .map(|p| Path::new(&p).join("./.specht2/config.ron"))
                .ok()
        })
        .ok_or(anyhow::anyhow!(
            "Failed to load config file from $SNAP_COMMON and $HOME"
        ))?;

    let config: ServerConfig = ron::de::from_str(&read_to_string(path)?)?;

    Server::new(config).serve().await?;
    Ok(())
}
