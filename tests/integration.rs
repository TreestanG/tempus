use tempus::{Tempus, AggregateType};
use std::collections::HashMap;

#[test]
fn test_basic_operations() {
    let mut ts = Tempus::new();
    
    // Test insert
    let mut tags = HashMap::new();
    tags.insert("sensor".to_string(), "temperature".to_string());
    ts.insert(1234567890, 23.5, tags);
    
    // Test get
    assert_eq!(ts.get(1234567890), Some(23.5));
    
    // Test range query
    let range = ts.range_query(1234567880, 1234567900);
    assert_eq!(range.len(), 1);
    
    // Test aggregation
    let avg = ts.aggregate(AggregateType::Average, 1234567880, 1234567900);
    assert_eq!(avg, 23.5);
}
