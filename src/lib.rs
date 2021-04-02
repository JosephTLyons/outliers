//! ```
//! let data = [10.0, 12.0, 11.0, 15.0, 11.0, 14.0, 13.0, 17.0, 12.0, 22.0, 14.0, 11.0].to_vec();
//! let outlier_identifier = outliers::OutlierIdentifier::new(data, false);
//! let results_tuple = outlier_identifier.get_outliers().unwrap();
//!
//! assert_eq!(results_tuple.0, [].to_vec()); // Lower outliers
//! assert_eq!(results_tuple.1, [10.0, 11.0, 11.0, 11.0, 12.0, 12.0, 13.0, 14.0, 14.0, 15.0, 17.0].to_vec()); // Non-outliers
//! assert_eq!(results_tuple.2, [22.0].to_vec()); // Upper outliers
//! ```

use statrs::statistics::OrderStatistics;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OutlierError {
    #[error("The data set contains one or more NANs")]
    ContainsNans,
    #[error("K value cannot be negative")]
    NegativeKValue,
}

pub struct OutlierIdentifier {
    data_set: Vec<f64>,
    k_value: f64,
    data_is_sorted: bool,
}

impl OutlierIdentifier {
    /// Creates a new `OutlierIdentifier`.  The default `k_value` is `1.5`, a value in outlier
    /// identification made popular by the mathematician John Tukey.  If the order state of the data
    /// is unknown, then use `false` for `data_is_sorted`.
    pub fn new(data_set: Vec<f64>, data_is_sorted: bool) -> OutlierIdentifier {
        OutlierIdentifier {
            data_set,
            data_is_sorted,
            k_value: 1.5,
        }
    }

    /// Allows for altering the `k_value`.  A larger `k_value` will result in fewer numbers being
    /// identified as outliers, while a smaller `k_value` will result in more numbers being
    /// identified as outliers.  The `k_value` must be non-negative, or `get_outliers()` will return
    /// an `Err`.
    pub fn with_k_value(self, k_value: f64) -> OutlierIdentifier {
        OutlierIdentifier {
            data_set: self.data_set,
            data_is_sorted: self.data_is_sorted,
            k_value,
        }
    }

    /// Performs the outlier identification.  In the case that is does not return an `Err`, it
    /// returns a tuple of `Vec<f64>`s.  The first vector contains any lower outliers and the third
    /// vector contains any upper outliers.  Additionally, the second vector returned contains all
    /// the non-outliers, so that the data set passed in is returned, in its entirety, as
    /// partitioned subsets.  `get_outliers()` will return an `Err` if the `data_set` contains one
    /// or more `NAN`s or if the `k_value` is a negative number.
    #[allow(clippy::type_complexity)]
    pub fn get_outliers(mut self) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>), OutlierError> {
        let (lower_fence, upper_fence) = self.get_fences()?;

        let mut lower_outliers: Vec<f64> = Vec::new();
        let mut upper_outliers: Vec<f64> = Vec::new();
        let mut non_outliers: Vec<f64> = Vec::new();

        for data in self.data_set {
            if data < lower_fence {
                lower_outliers.push(data);
            } else if data > upper_fence {
                upper_outliers.push(data);
            } else {
                non_outliers.push(data);
            }
        }

        Ok((lower_outliers, non_outliers, upper_outliers))
    }

    /// Indicates whether the data set has outliers.  This method is useful when one only needs to
    /// know if a data set has outliers and isn't concerned with the details of the outliers.  This
    /// method short circuits; if any outliers exist, the moment the first one is found, the method
    /// immediately returns with `true`, else, it returns `false`.
    pub fn has_outliers(mut self) -> Result<bool, OutlierError> {
        let (lower_fence, upper_fence) = self.get_fences()?;

        for data in self.data_set {
            if data < lower_fence || data > upper_fence {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn get_fences(&mut self) -> Result<(f64, f64), OutlierError> {
        if self.k_value < 0.0 {
            return Err(OutlierError::NegativeKValue);
        }

        // This should catch cases where the next `unwrap()` would panic, see:
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
        let data_set_has_nans = self.data_set.iter().any(|x| x.is_nan());

        if data_set_has_nans {
            return Err(OutlierError::ContainsNans);
        }

        if !self.data_is_sorted {
            self.data_set.sort_by(|a, b| a.partial_cmp(b).unwrap());
            self.data_is_sorted = true;
        }

        let lower_quartile = self.data_set.lower_quartile();
        let upper_quartile = self.data_set.upper_quartile();
        let interquartile_range = upper_quartile - lower_quartile;

        let intermediate_value = self.k_value * interquartile_range;
        let lower_fence = lower_quartile - intermediate_value;
        let upper_fence = upper_quartile + intermediate_value;

        Ok((lower_fence, upper_fence))
    }
}

#[test]
fn get_outliers_needs_sorted_nan_set() {
    let data: Vec<f64> = [f64::NAN, f64::NAN].to_vec();
    let outlier_identifier = OutlierIdentifier::new(data, false);
    let results_tuple = outlier_identifier.get_outliers();

    assert!(matches!(results_tuple, Err(OutlierError::ContainsNans)));
}

#[test]
fn get_outliers_is_sorted_nan_set() {
    let data: Vec<f64> = [3.0, 2.9, 2.8, 33.3, f64::NAN, f64::NAN].to_vec();
    let outlier_identifier = OutlierIdentifier::new(data, true);
    let results_tuple = outlier_identifier.get_outliers();

    assert!(matches!(results_tuple, Err(OutlierError::ContainsNans)));
}

#[test]
fn get_outliers_empty_data_set() {
    let data: Vec<f64> = [].to_vec();
    let outlier_identifier = OutlierIdentifier::new(data, true);
    let results_tuple = outlier_identifier.get_outliers().unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_outliers_set_of_one() {
    let data: Vec<f64> = [30.0].to_vec();
    let outlier_identifier = OutlierIdentifier::new(data, true);
    let results_tuple = outlier_identifier.get_outliers().unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [30.0].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_outliers_set_of_two() {
    let data: Vec<f64> = [30.0, 90.0].to_vec();
    let outlier_identifier = OutlierIdentifier::new(data, true);
    let results_tuple = outlier_identifier.get_outliers().unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [30.0, 90.0].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_outliers_none() {
    let data: Vec<f64> = [1.0, 2.0, 4.0, 10.0].to_vec();
    let outlier_identifier = OutlierIdentifier::new(data, true);
    let results_tuple = outlier_identifier.get_outliers().unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [1.0, 2.0, 4.0, 10.0].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_outliers_1() {
    let data = [
        0.0, 3.0, 3.0, 3.0, 11.0, 12.0, 13.0, 15.0, 19.0, 20.0, 29.0, 40.0, 79.0,
    ]
    .to_vec();
    let outlier_identifier = OutlierIdentifier::new(data, true);
    let results_tuple = outlier_identifier.get_outliers().unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(
        results_tuple.1,
        [0.0, 3.0, 3.0, 3.0, 11.0, 12.0, 13.0, 15.0, 19.0, 20.0, 29.0, 40.0].to_vec()
    );
    assert_eq!(results_tuple.2, [79.0].to_vec());
}

#[test]
fn get_outliers_negative_1() {
    let data = [
        29.5, -3.79, 15.0, 11.47, 3.6, 3.6, 19.0, 79.37, 40.7, -23.3, 12.0, 20.113, 13.39,
    ]
    .to_vec();
    let outlier_identifier = OutlierIdentifier::new(data, false);
    let results_tuple = outlier_identifier.get_outliers().unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(
        results_tuple.1,
        [-23.3, -3.79, 3.6, 3.6, 11.47, 12.0, 13.39, 15.0, 19.0, 20.113, 29.5, 40.7].to_vec()
    );
    assert_eq!(results_tuple.2, [79.37].to_vec());
}

#[test]
fn get_outliers_negative_2() {
    let data = [-62.3, 67.9, 71.02, 43.3, 51.7, 65.43, 67.23].to_vec();
    let outlier_identifier = OutlierIdentifier::new(data, false);
    let results_tuple = outlier_identifier.get_outliers().unwrap();

    assert_eq!(results_tuple.0, [-62.3].to_vec());
    assert_eq!(
        results_tuple.1,
        [43.3, 51.7, 65.43, 67.23, 67.9, 71.02].to_vec()
    );
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn negative_k_value_error() {
    let data = [30.0].to_vec();
    let outlier_identifier = OutlierIdentifier::new(data, true).with_k_value(-3.0);
    let results_tuple = outlier_identifier.get_outliers();

    assert!(matches!(results_tuple, Err(OutlierError::NegativeKValue)));
}

#[test]
fn has_outliers_false() {
    let data: Vec<f64> = [1.0, 2.0, 4.0, 10.0].to_vec();
    let has_outliers = OutlierIdentifier::new(data, true).has_outliers().unwrap();

    assert!(!has_outliers);
}

#[test]
fn has_outliers_true() {
    let data = [-62.3, 67.9, 71.02, 43.3, 51.7, 65.43, 67.23].to_vec();
    let has_outliers = OutlierIdentifier::new(data, true).has_outliers().unwrap();

    assert!(has_outliers);
}
