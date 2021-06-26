use serde::Deserialize;
#[derive(Deserialize)]
pub struct User {
    pub bio: Option<String>,
    pub banner: Option<String>,
}
#[derive(Deserialize)]
pub struct UserApiResponse {
    pub user: User,
}
// TODO: Esperando o discord liberar o endpoint profile para poder fazer isso....
pub async fn get_user_info(_token: &str, _id: u64) {}
