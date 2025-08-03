use std::collections::BTreeMap;

type RID = usize;

#[derive(Debug, Eq, Clone, PartialEq, Ord, PartialOrd)]
enum FieldType {
    Int(i64),
    String(String),
}

#[derive(Debug)]
struct Row {
    id: RID,
    fields: Vec<FieldType>,
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
}

fn main() {
    let row_1 = Row {
        id: 0,
        fields: vec![FieldType::String(String::from("Jake"))],
    };

    let mut index = Index::new();
    index.insert(row_1, 0);
}
