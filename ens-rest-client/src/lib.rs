use ethereum_types::H160;
use std::collections::BTreeMap as Map;

pub enum EnsApiError {}

pub trait EnsApiClient {
    fn reverse(address: &H160) -> Result<String, EnsApiError>;
    fn bulk_reverse(addresses: Vec<H160>) -> Result<Map<H160, String>, EnsApiError>;
}
