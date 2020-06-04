use std::collections::HashMap;

use super::account::Account;
use super::super::database_models::User;
use super::super::properties::UserRole;

pub struct SuperuserAccount {
    user: User
}

impl SuperuserAccount {
    pub fn from_user(user: User) -> Result<SuperuserAccount, String> {
        if user.get_role() == UserRole::Superuser {
            return Ok(SuperuserAccount{user})
        }
        Err(String::from("Invalid role"))
    }
}


impl Account for SuperuserAccount {
    fn has_superuser_access(&self) -> bool {
        true
    }

    fn has_admin_access(&self) -> bool {
        true
    }

    fn get_username(&self) -> String {
        self.user.username.clone()
    }

    fn generate_meta(&self) -> HashMap<String, String> {
        let mut meta: HashMap<String, String> = HashMap::new();
        meta.insert(String::from("username"), self.user.username.clone());
        meta.insert(String::from("display_name"), self.user.display_name.clone());
        meta.insert(String::from("role"), self.user.get_role().to_string());
        meta
    }
}

#[cfg(test)]
mod tests {
    mod test_admin_creation {
        use crate::account::account::Account;
        use crate::account::account_superuser::SuperuserAccount;
        use crate::database_models::User;
        use crate::properties::UserRole;
        use crate::utils;

        #[test]
        fn test_superuser_create_admin() {
            let test_connection = utils::get_test_connection();

            let admin_username = utils::generate_random_string(20);
            let admin_display_name = utils::generate_random_string(20);
            let admin_password = utils::generate_random_string(30);
            let admin_hashed_password = utils::hash(&admin_password);
            let _ = User::create(
                &admin_username,
                &admin_display_name,
                &admin_hashed_password,
                UserRole::Superuser,
                &test_connection
            );
            let user_admin = User::get(&admin_username, &test_connection).unwrap();
            let account = SuperuserAccount{user: user_admin};

            let username = utils::generate_random_string(20);
            let display_name = utils::generate_random_string(20);
            let password = utils::generate_random_string(30);
            let hashed_password = utils::hash(&password);
            let result = account.create_new_admin(&username, &display_name, &hashed_password, &test_connection);
            assert_eq!(result.is_ok(), true);

            let user_created = User::get(&username, &test_connection).unwrap();
            assert_eq!(user_created.username, username)
        }
    }
}
