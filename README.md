# outliers

[A Rust crate used to identify outliers in a data set](https://crates.io/crates/outliers)

```rust
use outliers::OutlierIdentifier; 

let data = [10.0, 12.0, 11.0, 15.0, 11.0, 14.0, 13.0, 17.0, 12.0, 22.0, 14.0, 11.0].to_vec();
let outlier_identifier = outliers::OutlierIdentifier::new(data, false);
let results_tuple = outlier_identifier.get_outliers().unwrap();

assert_eq!(results_tuple.0, [].to_vec()); // Lower outliers
assert_eq!(results_tuple.1, [10.0, 11.0, 11.0, 11.0, 12.0, 12.0, 13.0, 14.0, 14.0, 15.0, 17.0].to_vec()); // Non-outliers
assert_eq!(results_tuple.2, [22.0].to_vec()); // Upper outliers
```

```rust
use outliers::OutlierIdentifier;

let data = [0.53, 0.57, 0.51, 0.60, 0.09, 12.75].to_vec();
let has_outliers = OutlierIdentifier::new(data, false).has_outliers().unwrap();

assert!(has_outliers);
```
