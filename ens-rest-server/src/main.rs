pub mod api;
pub mod args;
pub mod web3sync;

pub fn main() -> anyhow::Result<()> {
    let args = match args::parse() {
        Ok(x) => x,
        Err(e) => return Err(anyhow::Error::msg(format!("Args parsing error {}", e))),
    };

    println!("ens-rest-server");
    Ok(())
}
