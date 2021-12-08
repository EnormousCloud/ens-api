use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "ens-rest-server", about = "ENS REST API Server")]
pub struct Args {
    /// Net listening address of HTTP server
    #[structopt(long, default_value = "0.0.0.0:8000", env = "LISTEN")]
    pub listen: String,
    /// Ethereum JSON+RPC HTTP address
    #[structopt(long, default_value = "http://localhost:8545", env = "RPC_ENDPOINT")]
    pub rpc_endpoint: String,
    /// ENS contract address
    #[structopt(long, default_value = "", env = "ENS_CONTRACT")]
    pub ens_contract: String,
}

pub fn parse() -> anyhow::Result<Args> {
    dotenv::dotenv().ok();
    let log_level: String = std::env::var("LOG_LEVEL").unwrap_or("debug".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(log_level))
        .init();
    let res = Args::from_args();
    tracing::debug!("{:?}", res);
    Ok(res)
}
