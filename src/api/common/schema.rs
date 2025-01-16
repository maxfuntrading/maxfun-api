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