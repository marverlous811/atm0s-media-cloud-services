use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ListQuery {
    pub offset: u32,
    pub limit: u32,
}
