use uuid::Uuid;
use crate::AliceUser;
use crate::AliceResult;
use serde::{Serialize, Deserialize};
use crate::Transaction;

#[derive(Debug, Deserialize, Serialize)]
pub struct Connection {
    uid: Uuid,
    user: AliceUser,
    transactions: Vec<Transaction>
}

impl Connection {
    pub fn new(user: AliceUser) -> AliceResult<Self> {
        let transactions = vec![];

        Ok(Self {
            uid: Uuid::new_v4(),
            user,
            transactions,
        })
    }

    pub fn execute(&mut self, input: &str) -> AliceResult<()> {
        let mut transaction = Transaction::new(input)?;
        self.transactions.push(transaction);
        Ok(())
    }

    pub fn close_connection(&mut self) -> AliceResult<()> {
        Ok(())
    }
}
