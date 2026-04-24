use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AliceResult;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AliceTransactionStatus {
    Pending,
    Commited,
    Error,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transaction {
    pub uid: Uuid,
    pub cmd: String,
    pub status: AliceTransactionStatus, 
}


impl Transaction {
    pub fn new(cmd: &str) -> AliceResult<Self> {
        Ok(Self {
            uid: Uuid::new_v4(),
            cmd: cmd.to_string(),
            status: AliceTransactionStatus::Pending,
        })
    }

    pub fn change_status(&mut self, status: AliceTransactionStatus) {
        self.status = status;
    }
}
