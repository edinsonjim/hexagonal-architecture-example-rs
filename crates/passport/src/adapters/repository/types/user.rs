use passport_core::user::{GrantedAuthority, UserDetails};
use portal_schema::user;

pub struct UserSelector {
    pub user_id: i32,
    pub username: String,
    pub password: String,
}

impl From<user::Model> for UserSelector {
    fn from(value: user::Model) -> Self {
        Self {
            user_id: value.id,
            username: value.username.to_owned(),
            password: value.password.to_owned(),
        }
    }
}

impl UserDetails for UserSelector {
    fn id(&self) -> i32 {
        self.user_id
    }

    fn username(&self) -> String {
        self.username.to_owned()
    }

    fn password(&self) -> String {
        self.password.to_owned()
    }

    fn is_enabled(&self) -> bool {
        true
    }

    fn authorities(&self) -> Vec<Box<dyn GrantedAuthority>> {
        Vec::new()
    }
}
