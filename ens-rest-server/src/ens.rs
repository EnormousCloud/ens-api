use crate::web3sync::{EthClient, RpcSingleResponse};
use cached::proc_macro::cached;
use hex_literal::hex;
use std::str::FromStr;
use tiny_keccak::{Hasher, Keccak};
use web3::types::H160;

// namehash generates a hash from a name that can be used to look up the name in ENS
fn namehash(name: &str) -> Vec<u8> {
    let mut node = vec![0u8; 32];
    if name.is_empty() {
        return node;
    }
    let n = name.clone().to_lowercase();
    let mut labels: Vec<&str> = n.as_str().split(".").collect();
    labels.reverse();
    for label in labels.iter() {
        let mut labelhash = [0u8; 32];

        let mut hasher = Keccak::v256();
        hasher.update(label.as_bytes());
        hasher.finalize(&mut labelhash);

        node.append(&mut labelhash.to_vec());
        labelhash = [0u8; 32];

        let mut hasher = Keccak::v256();
        hasher.update(node.as_slice());
        hasher.finalize(&mut labelhash);
        node = labelhash.to_vec();
    }
    node
}

pub struct Ens {
    pub contract: H160,
    pub client: EthClient,
}

impl Ens {
    pub fn new(rpc_address: &str) -> Self {
        Self {
            contract: hex!("00000000000c2e074ec69a0dfb2997ba6c7d2e1e").into(),
            client: EthClient::new(rpc_address),
        }
    }

    pub fn from_contract(rpc_address: &str, contract: &H160) -> Self {
        Self {
            contract: contract.clone(),
            client: EthClient::new(rpc_address),
        }
    }

    fn get_resolver(&self, address: H160) -> anyhow::Result<H160> {
        let method_str = "0x0178b8bf"; // "resolver(bytes32)"
        let resolver_addr = format!("{:x}.addr.reverse", address);
        let nh: String = namehash(&resolver_addr)
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect();
        let payload = format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"eth_call","params":[{{
"from":"0x0000000000000000000000000000000000000000","data":"{}{}","to":"{:?}"
}},"latest"]}}"#,
            method_str, nh, self.contract
        );
        let res: H160 = match self
            .client
            .execute_str::<RpcSingleResponse<String>>(&payload)
        {
            Ok(x) => {
                let remained: String = x.result.chars().skip(x.result.len() - 40).collect();
                match H160::from_str(&remained) {
                    Ok(a) => a,
                    Err(e) => return Err(anyhow::Error::msg(e)),
                }
            }
            Err(e) => return Err(anyhow::Error::msg(e)),
        };
        Ok(res)
    }

    fn get_name(&self, address: H160, resolver: H160) -> anyhow::Result<String> {
        let method_str = "0x691f3431"; // "name(bytes32)"
        let resolver_addr = format!("{:x}.addr.reverse", address);
        let nh: String = namehash(&resolver_addr)
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect();
        let payload = format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"eth_call","params":[{{
    "from":"0x0000000000000000000000000000000000000000","data":"{}{}","to":"{:?}"
    }},"latest"]}}"#,
            method_str, nh, resolver
        );
        let strx: String = match self
            .client
            .execute_str::<RpcSingleResponse<String>>(&payload)
        {
            Ok(x) => x.result,
            Err(e) => return Err(anyhow::Error::msg(e)),
        };
        let out = match hex::decode(strx.replace("0x", "")) {
            Ok(x) => match crate::hextext::HexReader::new(x) {
                Ok(mut x) => x.text(),
                Err(e) => return Err(anyhow::Error::msg(e)),
            },
            Err(e) => return Err(anyhow::Error::msg(e)),
        };
        Ok(out)
    }
}

#[cached(time = 86400)]
pub fn reverse(rpc_address: String, address: H160) -> String {
    let ens = Ens::new(&rpc_address);
    let resolver = match ens.get_resolver(address) {
        Ok(x) => x,
        Err(_) => return "".to_owned(),
    };
    match ens.get_name(address, resolver) {
        Ok(x) => x,
        Err(_) => "".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use web3::types::H160;

    #[test]
    pub fn it_reads_resolver() {
        // tracing_subscriber::fmt()
        //     .with_env_filter(tracing_subscriber::EnvFilter::new("debug,ureq=warn"))
        //     .init();

        let addr: H160 = hex!("6518c695cdcbefa272a4e5ef73bd46e801983e19").into();
        let resolver: H160 = hex!("a2c122be93b0074270ebee7f6b7292c7deb45047").into();
        let ens = super::Ens::new("http://localhost:8545");
        let result = ens.get_resolver(addr).unwrap();
        assert_eq!(resolver, result);
        let name = ens.get_name(addr, resolver).unwrap();
        assert_eq!(name, "enormouscloud.eth");
    }

    #[test]
    pub fn it_reads_reverse() {
        // tracing_subscriber::fmt()
        //     .with_env_filter(tracing_subscriber::EnvFilter::new("debug,ureq=warn"))
        //     .init();

        let addr: H160 = hex!("6518c695cdcbefa272a4e5ef73bd46e801983e19").into();
        let name = super::reverse("http://localhost:8545/".to_owned(), addr);
        assert_eq!(name, "enormouscloud.eth");
    }
}
