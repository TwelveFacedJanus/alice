mod database;
use database::Database;
use crate::{AliceUser, AliceUserRole};
use crate::AliceResult;
use crate::Connection;
use crate::Transaction;

#[derive(Debug)]
pub struct DatabaseManager {
    pub databases: Vec<Database>,
    pub users: Vec<AliceUser>,
    pub connections: Vec<Connection>,   // references must outlive 'a
    pub max_connections_count: usize,
    pub current_connections_count: usize,
    pub transactions: Vec<Transaction>,
}

impl DatabaseManager {
    pub fn start() -> AliceResult<Self> {
        let databases = vec![];
        let mut users = vec![];
        let connections = vec![];
        let transactions = vec![];

        let root_user = AliceUser::new("root", "root", AliceUserRole::King)
            .expect("Failed to create root user");

        users.push(root_user);

        Ok(Self {
            databases,
            users,
            connections,
            transactions,
            max_connections_count: 10,
            current_connections_count: 0,
        })
    }

    pub fn debug(&self) {
        println!("{:#?}", self);
    }

    // Use the struct's lifetime 'a, not a new one
    pub fn connect(&mut self, conn: Connection) {
        self.connections.push(conn);
        self.current_connections_count += 1;
    }

    pub fn disconnect(&mut self, _conn: Connection) {
        // TODO: proper removal logic
        // For now, just a placeholder
        for _conn_i in &self.connections {
            // ...
        }
    }

    // Check if we can accept another connection
    fn check_connection(&mut self, _conn: &Connection) -> bool {
        self.current_connections_count < self.max_connections_count
    }

    fn execute_transaction(&mut self, transaction: Transaction) -> AliceResult<()> {
        todo!()
    }
}