//! This module provides user credentials data.

use serde::{Deserialize, Serialize};
use crate::AliceResult;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AliceUserRole {
    King,
    Bishop,
    Pawn,
}


/// AliceUser structure that provides user credentials
///
/// # Fields
/// * `username` - String username;
/// * `password` - String password md5 hash;
/// * `role` - Alice user role;
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AliceUser {
    pub username: String,
    pub password_hash: String,
    pub role: AliceUserRole,
}


impl AliceUser {
    /// Create new user.
    ///
    /// # Arguments
    /// * `username` - &str username;
    /// * `password` - &str raw password;
    /// * `role` - AliceUserRole for user;
    ///
    /// # Returns
    /// * `AliceUser` - structure.
    pub fn new(username: &str, password: &str, role: AliceUserRole) -> AliceResult<Self> {
        Ok(AliceUser {
            username: username.to_string(),
            password_hash: password.to_string(),
            role,
        })
    }
    
    /// Encode user password.
    /// 
    /// # Arguments
    /// * `&mut self` - mutable self;
    ///
    /// # Returns
    /// * `AliceResult` - result of encoding;
    fn encode_password(&mut self) -> AliceResult<()> {
        todo!()
    }

    /// Change user role
    ///
    /// # Arguments
    /// * `&mut self` - mutable self;
    /// * `role` - new AliceUserRole;
    ///
    /// # Returns
    /// * `AliceResult` - result of escalation.
    pub fn change_role(&mut self, role: AliceUserRole) -> AliceResult<()> {
        self.role = role;
        Ok(())
    }
}
