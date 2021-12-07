use crate::web3sync::{EthClient, RpcSingleResponse};
use hex_literal::hex;
use std::str::FromStr;
use tiny_keccak::{Hasher, Keccak};
use web3::types::H160;

const ENS_REVERSE_REGISTRAR_DOMAIN: &str = "addr.reverse";

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

    pub fn get_resolver(&self, address: H160) -> anyhow::Result<H160> {
        let method_str = "0x0178b8bf";
        let resolver_addr = format!("{:x}.{}", address, ENS_REVERSE_REGISTRAR_DOMAIN);
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
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use web3::types::H160;

    #[test]
    pub fn it_reads_resolver() {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::new("debug,ureq=warn"))
            .init();

        let addr: H160 = hex!("6518c695cdcbefa272a4e5ef73bd46e801983e19").into();
        let expected: H160 = hex!("a2c122be93b0074270ebee7f6b7292c7deb45047").into();
        let ens = super::Ens::new("http://localhost:8545");
        let result = ens.get_resolver(addr).unwrap();
        assert_eq!(expected, result);
    }
}
