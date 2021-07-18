use clickhouse::{Row};

pub struct Batch<T> {
    pub rows: Vec<T>
}

impl<T> Batch<T> {
    pub fn new(rows: Vec<T>) -> Batch<T> where T:Row {
        Batch{rows}
    }
}