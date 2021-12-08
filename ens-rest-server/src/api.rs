use crate::ens::reverse;
use crate::State;
use serde::Deserialize;
use tide::{Request, Response, Result};
use web3::types::H160;

#[derive(Default, Deserialize)]
struct ReverseSingleRequest {
    pub address: H160,
}

pub async fn get(req: Request<State>) -> Result {
    let rq: ReverseSingleRequest = match req.query() {
        Ok(a) => a,
        Err(e) => {
            let mut res = Response::new(400);
            res.set_content_type("text/plain");
            res.set_body(format!("ERROR: {}", e));
            return Ok(res);
        }
    };
    let mut res = Response::new(200);
    let rpc_endpoint = req.state().rpc_endpoint.clone();
    let out = reverse(rpc_endpoint, rq.address);
    res.set_content_type("text/plain");
    res.set_body(out);
    Ok(res)
}
