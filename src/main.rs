use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::Mutex;

pub type RID = usize;

#[derive(Debug, Eq, Clone, PartialEq, Ord, PartialOrd)]
pub enum FieldType {
    Int(i64),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    id: RID,
    fields: Vec<FieldType>,
}

// Pretend like this is a bufferpool that can "store" all of the rows.
// I put store in quotes, because it will keep some rows in memory, and
// others will be put on disk
static BUFFERPOOL_MOCK: Lazy<Mutex<Vec<Row>>> = Lazy::new(|| Mutex::new(Vec::new()));

impl Row {
    pub fn new(id: RID, fields: Vec<FieldType>) -> Self {
        let row = Row { id, fields };
        let mut b = BUFFERPOOL_MOCK.lock().unwrap();
        b.push(row.clone());
        row
    }
}

#[derive(Debug)]
pub struct Index {
    index: BTreeMap<FieldType, Vec<RID>>,
}

impl Index {
    pub fn new() -> Self {
        return Index {
            index: BTreeMap::new(),
        };
    }

    pub fn insert(&mut self, row: Row, index_on_col: usize) {
        let key = row.fields[index_on_col].clone();
        let ids_node: Option<&mut Vec<usize>> = self.index.get_mut(&key);

        if let Some(ids_vec) = ids_node {
            ids_vec.push(row.id);
        } else {
            self.index.insert(key, vec![row.id]);
        }
    }

    pub fn get(&self, key: FieldType) -> Option<Vec<Row>> {
        let ids_node = self.index.get(&key);

        let bp = BUFFERPOOL_MOCK.lock().unwrap();
        let mut rows = vec![];

        if let Some(ids_vec) = ids_node {
            for id in ids_vec {
                rows.push(bp[*id].clone());
            }

            return Some(rows);
        }

        None
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_insert_test() {
        let mut index = Index::new();

        let row_1 = Row::new(0, vec![FieldType::String(String::from("Jake"))]);
        index.insert(row_1, 0);

        let fetched_rows = index.get(FieldType::String(String::from("Jake")));

        assert_eq!(
            fetched_rows.unwrap()[0].fields[0],
            FieldType::String(String::from("Jake"))
        );
    }

    #[test]
    fn duplicate_insert_test() {
        let mut index = Index::new();

        let row_2 = Row::new(1, vec![FieldType::String(String::from("Foo"))]);
        let row_3 = Row::new(2, vec![FieldType::String(String::from("Foo"))]);
        index.insert(row_2, 0);
        index.insert(row_3, 0);

        let fetched_rows_opt_2 = index.get(FieldType::String(String::from("Foo")));
        let fetched_rows_2 = fetched_rows_opt_2.unwrap();

        assert_eq!(
            fetched_rows_2[0].fields[0],
            FieldType::String(String::from("Foo"))
        );

        assert_eq!(
            fetched_rows_2[1].fields[0],
            FieldType::String(String::from("Foo"))
        );
    }
}
