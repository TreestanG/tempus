use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataPoint {
    timestamp: u64,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct Tempus {
    data_series: Vec<DataPoint>,
    index: usize,
}

impl Tempus {
    fn new() -> Self {
        Self {
            data_series: Vec::new(),
            index: 0,
        }
    }       

    fn insert<T: Serialize>(&mut self, timestamp: u64, value: T) {
        let data_point = DataPoint { timestamp, value: serde_json::to_string(&value).unwrap() };
        self.data_series.push(data_point);
    }

    fn get<'a, T: Deserialize<'a>>(&'a self, timestamp: u64) -> Option<T> {
        self.data_series.iter().find(|db| db.timestamp == timestamp).map(|db| serde_json::from_str(&db.value).unwrap())
    }
}

fn main() {
    let mut tempus = Tempus::new();
    tempus.insert(1, "Hello");
    tempus.insert(2, vec![1, 2, 3]);
    println!("{:?}", tempus.get::<Vec<i32>>(2));
}