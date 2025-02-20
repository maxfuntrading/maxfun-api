use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct NonceRsp {
    pub nonce: String,
}

#[derive(Deserialize, Debug)]
pub struct VerifyReq {
    pub message: String,
    pub signature: String,
    #[allow(dead_code)]
    pub chain_id: Option<u32>,
    #[allow(dead_code)]
    pub source: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct VerifyResp {
    pub user_exists: bool,
    pub auth_type: String,
    pub auth_token: String,
}
