use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::Mutex;

type RID = usize;

#[derive(Debug, Eq, Clone, PartialEq, Ord, PartialOrd)]
enum FieldType {
    Int(i64),
    String(String),
}

#[derive(Debug, Clone)]
struct Row {
    id: RID,
    fields: Vec<FieldType>,
}

// Pretend like this is a bufferpool that can "store" all of the rows.
// I put store in quotes, because it will keep some rows in memory, and
// others will be put on disk
static BUFFERPOOL_MOCK: Lazy<Mutex<Vec<Row>>> = Lazy::new(|| Mutex::new(Vec::new()));

impl Row {
    fn new(id: RID, fields: Vec<FieldType>) -> Self {
        let r = Row { id, fields };
        let mut b = BUFFERPOOL_MOCK.lock().unwrap();
        b.push(r.clone());
        r
    }
}

struct Index {
    index: BTreeMap<FieldType, RID>,
}

impl Index {
    fn new() -> Self {
        return Index {
            index: BTreeMap::new(),
        };
    }

    fn insert(&mut self, row: Row, index_on_col: usize) {
        let key = row.fields[index_on_col].clone();
        self.index.insert(key, row.id);
    }

    fn get(&self, key: FieldType) -> Option<Row> {
        let i = self.index.get(&key);

        match i {
            Some(a) => {
                let b = BUFFERPOOL_MOCK.lock().unwrap();
                return Some(b[*a].clone());
            }
            None => None,
        }
    }
}

fn main() {
    let row_1 = Row::new(0, vec![FieldType::String(String::from("Jake"))]);

    let mut index = Index::new();
    index.insert(row_1, 0);

    let fetched_row = index.get(FieldType::String(String::from("Jake")));

    assert_eq!(
        fetched_row.unwrap().fields[0],
        FieldType::String(String::from("Jake"))
    );
}
