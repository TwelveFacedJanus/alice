pub enum AliceValue { Text(String), Integer(i32), Boolean(bool) }

pub struct Row {
    pub name: String,
    pub values: Vec<AliceValue>,
}

pub struct Table {
    pub name: String,
    pub rows: Vec<Row>,
}

pub struct InMemEngine {
    pub tables: Vec<Table>,
}

pub trait BaseEngine<T> {
    pub fn new() -> AliceResult<T>;
    pub fn execute_task(&mut self, transaction: &str) -> AliceResult<()>;

    fn debug(&self);
    fn as_json(&self);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
