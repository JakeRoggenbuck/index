use std::collections::BTreeMap;

type RID = usize;

#[derive(Debug)]
enum FieldType {
    Int(i64),
    String(String),
}

#[derive(Debug)]
struct Row {
    id: RID,
    fields: Vec<FieldType>,
}

struct Index<T> {
    index: BTreeMap<T, RID>,
}

impl<T> Index<T> {
    fn new() -> Self {
        return Index {
            index: BTreeMap::new(),
        };
    }

    fn insert(row: Row) {}
}

fn main() {
    let row_1 = Row {
        id: 0,
        fields: vec![FieldType::String(String::from("Jake"))],
    };

    let index = Index::<String>::new();

    println!("{:?}", row_1);
}
