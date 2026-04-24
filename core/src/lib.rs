pub mod types;
use types::{AliceResult};

pub mod security;
use security::{AliceUser, AliceUserRole};

pub mod transactions;
use transactions::Transaction;

pub mod connection;
use connection::Connection;

pub mod database_manager;
use database_manager::DatabaseManager;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user() {
        let user = AliceUser::new("root", "root", AliceUserRole::King).unwrap();
        assert_eq!(user.username, "root".to_string());
    }

    #[test]
    fn escalate_privilegies() {
        let mut user = AliceUser::new("root", "root", AliceUserRole::Pawn).unwrap();
        user.change_role(AliceUserRole::King);
        assert_eq!(user.role, AliceUserRole::King);
    }
}
