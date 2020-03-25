use super::JWTMediator;
use super::super::models::User;
use super::super::utils;

pub struct AccessControl {
    user: User
}

impl AccessControl {
    pub fn from_user(user: User) -> AccessControl {
        AccessControl {
            user
        }
    }

    pub fn get_visitor_access() -> AccessControl {
        let user = User::get_dummy_visitor();
        AccessControl::from_user(user)
    }

    pub fn from_jwt(jwt: String, services: &utils::ExternalServices) -> AccessControl {
        if let Ok(user) = JWTMediator::get_user_from_jwt(jwt, services) {
            return AccessControl::from_user(user)
        }
        AccessControl::get_visitor_access()
    }

    pub fn generate_jwt(&self) -> Result<String, String> {
        JWTMediator::generate_jwt_from_user(&self.user)
    }
}
