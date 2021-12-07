pub mod api;
pub mod args;
pub mod ens;
pub mod hextext;
pub mod web3sync;

pub fn main() -> anyhow::Result<()> {
    let _args = match args::parse() {
        Ok(x) => x,
        Err(e) => return Err(anyhow::Error::msg(format!("Args parsing error {}", e))),
    };

    println!("ens-rest-server");
    Ok(())
}
