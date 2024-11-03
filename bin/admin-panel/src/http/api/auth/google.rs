use serde::Deserialize;

#[derive(Deserialize)]
pub struct GoogleUserResult {
    pub sub: String,
    pub email: String,
    pub name: String,
    #[allow(unused)]
    pub given_name: String,
    #[allow(unused)]
    pub family_name: String,
    pub picture: String,
}
