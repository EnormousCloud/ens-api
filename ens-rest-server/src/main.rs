pub mod api;
pub mod args;
pub mod ens;
pub mod hextext;
pub mod telemetry;
pub mod web3sync;

#[derive(Debug, Clone)]
pub struct State {
    rpc_endpoint: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let args = match args::parse() {
        Ok(x) => x,
        Err(e) => {
            panic!("Args parsing error: {}", e);
        }
    };

    let state = State {
        rpc_endpoint: args.rpc_endpoint.clone(),
    };
    let mut app = tide::with_state(state);
    app.with(telemetry::TraceMiddleware::new());
    app.at("/reverse").get(api::get);
    tracing::info!("Starting HTTP server at {}", args.listen);
    app.listen(&args.listen).await?;
    Ok(())
}
