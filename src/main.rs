use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataPoint {
    timestamp: u64,
    value: f64,
    tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
enum Value {
    String(String),
    HashMap(HashMap<String, String>),
    Float(f32),
    Bool(bool),
    Int(i32),
    None,
    Vec(Vec<i32>),
    VecString(Vec<String>),
    VecFloat(Vec<f32>),
    VecBool(Vec<bool>),
    VecInt(Vec<i32>),
    VecHashMap(Vec<HashMap<String, String>>),
}

#[derive(Serialize, Deserialize)]
struct Tempus {
    data_series: Vec<DataPoint>,
    index: usize,
}

enum AggregateType {
    Sum,
    Average,
    Min,
    Max,
    Count,
    First,
    Last,
    /* Median,
    Mode,
    Percentile,
    StdDev,
    Variance, */
}

impl Tempus {
    fn new() -> Self {
        Self {
            data_series: Vec::new(),
            index: 0,
        }
    }

    fn insert(&mut self, timestamp: u64, value: f64, tags: HashMap<String, String>) {
        let data_point: DataPoint = DataPoint {
            timestamp,
            value,
            tags,
        };
        self.data_series.push(data_point);
    }

    fn bulk_insert(&mut self, data: Vec<(u64, f64, HashMap<String, String>)>) {
        let data_points = data.iter().map(|(timestamp, value, tags)| DataPoint {
            timestamp: *timestamp,
            value: *value,
            tags: tags.clone(),
        });
        self.data_series.extend(data_points);
    }

    fn update(&mut self, timestamp: u64, value: f64, tags: HashMap<String, String>) {
        let data_point: &mut DataPoint = self.data_series.iter_mut().find(|dp: &&mut DataPoint| dp.timestamp == timestamp).unwrap();
        data_point.value = value;
        data_point.tags = tags;
    }

    fn get<'a>(&'a self, timestamp: u64) -> Option<f64> {
        self.data_series
            .iter()
            .find(|dp: &&DataPoint| dp.timestamp == timestamp)
            .map(|dp: &DataPoint| dp.value)
    }

    fn range_query(&self, start: u64, end: u64) -> Vec<DataPoint> {
        self.data_series
            .iter()
            .filter(|dp: &&DataPoint| dp.timestamp >= start && dp.timestamp <= end)
            .cloned()
            .collect()
    }

    fn find_by_tag(&self, tag: &str) -> Vec<DataPoint> {
        self.data_series
            .iter()
            .filter(|dp: &&DataPoint| dp.tags.contains_key(tag))
            .cloned()
            .collect()
    }

    fn find_by_tag_value(&self, tag: &str, value: &str) -> Vec<DataPoint> {
        self.data_series
            .iter()
            .filter(|dp: &&DataPoint| dp.tags.get(tag).unwrap() == value)
            .cloned()
            .collect()
    }

    fn delete(&mut self, timestamp: u64) {
        self.data_series.retain(|dp: &DataPoint| dp.timestamp != timestamp);
    }

    fn delete_by_tag(&mut self, tag: &str) {
        self.data_series.retain(|dp: &DataPoint| !dp.tags.contains_key(tag));
    }

    fn aggregate(&self, aggregate_type: AggregateType, start:u64, end:u64) -> f64 {
        let data: Vec<DataPoint> = self.range_query(start, end);

        match aggregate_type {
            AggregateType::Sum => data.iter().map(|dp: &DataPoint| dp.value).sum::<f64>(),
            AggregateType::Average => {
                data.iter().map(|dp: &DataPoint| dp.value).sum::<f64>() / data.len() as f64
            },
            AggregateType::Count => data.len() as f64,
            AggregateType::First => data.first().map(|dp: &DataPoint| dp.value).unwrap_or(0.0),
            AggregateType::Last => data.last().map(|dp: &DataPoint| dp.value).unwrap_or(0.0),
            AggregateType::Max => data.iter().map(|dp: &DataPoint| dp.value).max_by(|a, b| a.total_cmp(b)).unwrap_or(0.0),
            AggregateType::Min => data.iter().map(|dp: &DataPoint| dp.value).min_by(|a, b| a.total_cmp(b)).unwrap_or(0.0)
            
        }
    }


}

fn main() {
    let mut tempus: Tempus = Tempus::new();
    let mut scores: HashMap<String, String> = HashMap::new();

    scores.insert("Hello".to_string(), "1".to_string());
    scores.insert("World".to_string(), "2".to_string());

    tempus.bulk_insert(vec![(1, 1.0, scores), (2, 2.0, HashMap::new())]);
    tempus.insert(3, 3.0, HashMap::new());

    tempus.update(1, 1.0, HashMap::new());
}
