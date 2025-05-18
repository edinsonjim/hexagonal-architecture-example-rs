use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsernamePasswordAuthenticationParams {
    pub username: String,
    pub password: String,
}
