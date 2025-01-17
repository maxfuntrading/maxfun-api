use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Tag {
    pub name: String,
    pub sort: i32,
}

#[derive(Debug, Serialize)]
pub struct TagListResp {
    pub list: Vec<Tag>,
}

#[derive(Debug, Serialize)]
pub struct RaisedToken {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimal: i32,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RaisedTokenListResp {
    pub list: Vec<RaisedToken>,
}

#[derive(Debug, Serialize)]
pub struct UploadImageResp {
    pub url: String,
}