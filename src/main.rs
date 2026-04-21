
use std::ops::AddAssign;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use pest::Parser;
use pest_derive::Parser;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs;
use rustyline::{Editor, error::ReadlineError, history::DefaultHistory};

const WELCOME_MESSAGE: &str = "
|===============================================|
|                                               |
|                ALICE CLI                      |
|                                               |
|===============================================|
";

#[derive(Debug, Deserialize, Serialize)]
pub enum Value {
    Text(String),
    Integer(i32),
    Timestamp(DateTime<Utc>),
}
#[derive(Debug, Deserialize, Serialize)]
pub enum EngineType {
    RowEngine,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FormatType {
    AutoTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub datetime: DateTime<Utc>,
    pub data: Vec<Value>,
}

#[derive(Parser)]
#[grammar = "global_commands.pest"]
pub struct CommandsParser;

impl Row {
    pub fn new(data: Vec<Value>) -> Self {
        Self {
            data,
            datetime: Utc::now(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {
    pub name: String,
    pub rows: Vec<Row>,
    pub engine_type: EngineType,
    pub format_type: FormatType,
}

impl Table {
    fn new(name: &str, engine_type: EngineType) -> Table {
        Table {
            name: name.to_string(),
            engine_type,
            rows: Vec::new(),
            format_type: FormatType::AutoTime,
        }
    }
}

impl AddAssign<Row> for Table {
    fn add_assign(&mut self, value: Row) { self.rows.push(value); }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TType {
    AddRow,
    CreateTable,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum TStatus {
    Pending,
    Executed,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub uid: Uuid,
    pub ttype: TType,
    pub status: TStatus,
    pub created_at: DateTime<Utc>,
}

impl Transaction {
    pub fn new(ttype: TType) -> Self {
        Self {
            ttype,
            uid: Uuid::new_v4(),
            status: TStatus::Pending,
            created_at: Utc::now(),
        }
    }

    pub fn execute(&mut self) { self.status = TStatus::Executed; }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    pub name: String,
    pub tables: Vec<Table>,
    pub transactions: Vec<Transaction>,
    pub latest_commit_id: String,
}

impl Database {
    pub fn new(name: &str) -> Self {
        Database {
            name: name.to_string(),
            tables: Vec::new(),
            transactions: Vec::new(),
            latest_commit_id: "".to_string()
        }
    }

    pub fn init(&mut self) {
        match fs::read_to_string("alice.dbmscore") {
            Ok(contents) => {
                match serde_json::from_str::<Database>(&contents) {
                    Ok(loaded_db) => {
                        self.name = loaded_db.name;
                        self.tables = loaded_db.tables;
                        self.transactions = loaded_db.transactions;
                        self.latest_commit_id = loaded_db.latest_commit_id;
                        println!("[ALICE]: Database loaded from archive.");
                    }
                    Err(e) => eprintln!("Error deserializing database: {}", e),
                }
            }
            Err(e) => eprintln!("Error reading archive file: {}", e),
        }
    }

    pub fn get_table(&mut self, name: &str) -> Option<&mut Table> {
        self.tables.iter_mut().find(|t| t.name == name)
    }

    pub fn commit(&mut self, mut transaction: Transaction) {
        transaction.execute();
        self.transactions.push(transaction);
    }

    pub fn print(&self) {
        println!("{:#?}", self);
    }

    fn start_transaction(&mut self, ttype: TType) -> Uuid {
        let transaction = Transaction::new(ttype);
        let uuid = transaction.uid;
        self.transactions.push(transaction);
        uuid
    }

    fn complete_transaction(&mut self, uuid: Uuid) -> Result<(), &'static str> {
        if let Some(tr) = self.transactions.iter_mut().find(|t| t.uid == uuid) {
            tr.status = TStatus::Executed;
            Ok(())
        } else {
            Err("Transaction not found")
        }
    }

    pub fn add_row(&mut self, table_name: &str, row: Row) {
        let tx_uuid = self.start_transaction(TType::AddRow);

        if let Some(table) = self.get_table(table_name) {
            table.rows.push(row);
            self.complete_transaction(tx_uuid).unwrap();
        } else {
            self.transactions.retain(|t| t.uid != tx_uuid);
        }
    }

    fn parse_value(pair: pest::iterators::Pair<'_, Rule>) -> Value {
        match pair.as_rule() {
            Rule::string => {
                let s = pair.as_str();
                let inner = &s[1..s.len() - 1];
                Value::Text(inner.to_string())
            }
            Rule::integer => {
                let n = pair.as_str().parse::<i32>().unwrap();
                Value::Integer(n)
            }
            Rule::iso_date => {
                let dt = DateTime::parse_from_rfc3339(pair.as_str())
                    .unwrap()
                    .with_timezone(&Utc);
                Value::Timestamp(dt)
            }
            _ => unreachable!("Unexpected rule in parse_value: {:?}", pair.as_rule()),
        }
    }

    pub fn process_command(&mut self, input: &str) -> io::Result<()> {
        let command_pairs = CommandsParser::parse(Rule::command, input)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        for command in command_pairs {
            let inner = command
                .into_inner()
                .next()
                .expect("command should have inner rule");
            match inner.as_rule() {
                Rule::create_table => {
                    let table_name = inner.into_inner().nth(0).unwrap().as_str();
                    *self += Table::new(table_name, EngineType::RowEngine);
                    println!("[ALICE]: Table '{}' created.", table_name);
                }
                
                Rule::add_row => {
                    let mut tokens = inner.into_inner();
                    let table_name = tokens.next().unwrap().as_str();
                    let values = match tokens.next() {
                        Some(vlist) if vlist.as_rule() == Rule::value_list => {
                            vlist.into_inner()
                                .map(|v| {
                                    let inner_value = v.into_inner().next().unwrap();
                                    Self::parse_value(inner_value)
                                })
                                .collect()
                        }
                        _ => Vec::new(),
                    };
                    println!("{:#?}", values);
                    let row = Row::new(values);
                    self.add_row(table_name, row);
                    println!("[ALICE]: Row added to table '{}'.", table_name);
                }
                Rule::exit => {
                    println!("[ALICE]: Goodbye!");
                    std::process::exit(0);
                }

                Rule::show_table => {
                    let mut tokens = inner.into_inner();
                    let table_name = tokens.next().unwrap().as_str();
                    match self.get_table(table_name) {
                        Some(table) => println!("{:#?}", table.rows),
                        _ => println!("Table doesnt exists!")
                    }
                }

                Rule::debug => { self.print() }
                Rule::view_transactions => { println!("{:#?}", self.transactions); }

                Rule::archive_myself => {
                    let mut tokens = inner.into_inner();
                    let file_path = tokens.next().unwrap().as_str();
                    let json_string = serde_json::to_string(self)?;
                    let mut file = File::create(file_path)?;
                    file.write_all(json_string.as_bytes())?;
                }

                Rule::load_archive => {
                    let mut tokens = inner.into_inner();
                    let file_path = tokens.next().unwrap().as_str();
                    match fs::read_to_string(file_path) {
                        Ok(contents) => {
                            match serde_json::from_str::<Database>(&contents) {
                                Ok(loaded_db) => {
                                    self.name = loaded_db.name;
                                    self.tables = loaded_db.tables;
                                    self.transactions = loaded_db.transactions;
                                    println!("[SYSTEM]: Database loaded from archive.");
                                }
                                Err(e) => eprintln!("Error deserializing database: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Error reading archive file: {}", e),
                    }
                }

                Rule::commit => {
                    self.latest_commit_id = Uuid::new_v4().to_string();;
                    let json_string = serde_json::to_string(self)?;
                    let mut file = File::create("alice.dbmscore")?;
                    file.write_all(json_string.as_bytes())?;
                }

                _ => unreachable!("Unexpected rule: {:?}", inner.as_rule()),
            }
        }
        Ok(())
    }

    pub fn shell(&mut self) {
        let mut rl = match Editor::<(), DefaultHistory>::new() {
            Ok(editor) => editor,
            Err(e) => {
                eprintln!("Error initializing line editor: {}", e);
                return;
            }
        };

        if let Err(_) = rl.load_history("history.txt") {
            todo!();
        }

        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    let _ = rl.add_history_entry(line);
                    let _ = rl.save_history("history.txt");

                    if let Err(e) = self.process_command(line) {
                        eprintln!("Error: {}", e);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("Goodbye!");
                    break;
                }
                Err(err) => {
                    eprintln!("Error reading input: {}", err);
                    break;
                }
            }
        }
    }
}

impl AddAssign<Table> for Database {
    fn add_assign(&mut self, value: Table) {
        let tx_uuid = self.start_transaction(TType::CreateTable);
        self.tables.push(value);
        self.complete_transaction(tx_uuid).unwrap();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseManager {
    pub databases: Vec<Database>,
}

fn main() -> io::Result<()> {
    println!("{}", WELCOME_MESSAGE);
    let mut database = Database::new("X Service");
    database.init();
    database.shell();
    Ok(())
}
