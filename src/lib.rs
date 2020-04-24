use num::ToPrimitive;

mod helper_items;
use helper_items::{get_error_message, get_quartile_values, ErrorMessage};

type VectorTuple<T> = (Vec<T>, Vec<T>, Vec<T>);

/// This function uses the Tukey method, which uses a multiplier value of 1.5. In the case that is
/// does not return an `Err`, it returns a tuple of `Vec<T>`s.  The first vector contains any lower
/// outliers and the third vector contains any upper outliers.  Additionally, the second vector
/// returned contains all the non-outliers, so that the data set passed in is returned, in its
/// entirety, as partitioned subsets.
/// ```
/// let data = [10, 12, 11, 15, 11, 14, 13, 17, 12, 22, 14, 11].to_vec();
/// let results_tuple = outliers::get_tukeys_outliers(data, false).unwrap();
///
/// assert_eq!(results_tuple.0, [].to_vec()); // Lower outliers
/// assert_eq!(results_tuple.1, [10, 11, 11, 11, 12, 12, 13, 14, 14, 15, 17].to_vec()); // Non-outliers
/// assert_eq!(results_tuple.2, [22].to_vec()); // Upper outliers
/// ```
#[allow(clippy::eq_op)]
pub fn get_tukeys_outliers<T: std::cmp::PartialOrd + ToPrimitive>(
    mut data_vec: Vec<T>,
    data_is_sorted: bool,
) -> Result<VectorTuple<T>, &'static str> {
    // Tests for NaNs in floats, should catch cases where the next `unwrap()` would panic, see:
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
    if data_vec.iter().any(|x| !(x == x)) {
        return Err(get_error_message(ErrorMessage::NanError));
    }

    if !data_is_sorted {
        data_vec.sort_by(|a, b| {
            a.partial_cmp(b).unwrap()
        });
    }

    let mut lower_outliers: Vec<T> = Vec::new();
    let mut upper_outliers: Vec<T> = Vec::new();

    match get_quartile_values(&data_vec) {
        Ok((q1_value, _, q3_value)) => {
            let interquartile_range: f32 = q3_value - q1_value;

            let intermediate_value: f32 = 1.5 * interquartile_range;
            let lower_range: f32 = q1_value - intermediate_value;
            let upper_range: f32 = q3_value + intermediate_value;

            let mut non_outliers: Vec<T> = Vec::new();

            for data in data_vec {
                let data_f32 = match ToPrimitive::to_f32(&data) {
                    Some(value_f32) => value_f32,
                    None => return Err(get_error_message(ErrorMessage::ToPrimitiveCast)),
                };

                if (data_f32) < lower_range {
                    lower_outliers.push(data);
                } else if (data_f32) > upper_range {
                    upper_outliers.push(data);
                } else {
                    non_outliers.push(data);
                }
            }

            data_vec = non_outliers;
        }
        Err(error) => {
            if let ErrorMessage::ToPrimitiveCast = error {
                return Err(get_error_message(ErrorMessage::ToPrimitiveCast));
            }
        }
    }

    Ok((lower_outliers, data_vec, upper_outliers))
}

#[allow(clippy::eq_op)]
#[allow(clippy::zero_divided_by_zero)]
#[test]
fn get_tukeys_outliers_needs_sorted_nan_set() {
    let data: Vec<f64> = [f64::NAN, f64::NAN].to_vec();
    let results_tuple = get_tukeys_outliers(data, false);

    assert!(results_tuple.is_err());
}

#[allow(clippy::eq_op)]
#[allow(clippy::zero_divided_by_zero)]
#[test]
fn get_tukeys_outliers_is_sorted_nan_set() {
    let data: Vec<f64> = [3.0, 2.9, 2.8, 33.3, f64::NAN, f64::NAN].to_vec();
    let results_tuple = get_tukeys_outliers(data, true);

    assert!(results_tuple.is_err());
}

#[test]
fn get_tukeys_outliers_empty_data_set() {
    let data: Vec<usize> = [].to_vec();
    let results_tuple = get_tukeys_outliers(data, true).unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_tukeys_outliers_set_of_one() {
    let data = [30].to_vec();
    let results_tuple = get_tukeys_outliers(data, true).unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [30].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_tukeys_outliers_set_of_two() {
    let data = [30, 90].to_vec();
    let results_tuple = get_tukeys_outliers(data, true).unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [30, 90].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_tukeys_outliers_none() {
    let data = [1, 2, 4, 10].to_vec();
    let results_tuple = get_tukeys_outliers(data, true).unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [1, 2, 4, 10].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_tukeys_outliers_1() {
    let data = [0, 3, 3, 3, 11, 12, 13, 15, 19, 20, 29, 40, 79].to_vec();
    let results_tuple = get_tukeys_outliers(data, true).unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(
        results_tuple.1,
        [0, 3, 3, 3, 11, 12, 13, 15, 19, 20, 29, 40].to_vec()
    );
    assert_eq!(results_tuple.2, [79].to_vec());
}

#[test]
fn get_tukeys_outliers_float_negative_1() {
    let data = [
        29.5, -3.79, 15.0, 11.47, 3.6, 3.6, 19.0, 79.37, 40.7, -23.3, 12.0, 20.113, 13.39,
    ]
    .to_vec();
    let results_tuple = get_tukeys_outliers(data, false).unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(
        results_tuple.1,
        [-23.3, -3.79, 3.6, 3.6, 11.47, 12.0, 13.39, 15.0, 19.0, 20.113, 29.5, 40.7].to_vec()
    );
    assert_eq!(results_tuple.2, [79.37].to_vec());
}

#[test]
fn get_tukeys_outliers_float_negative_2() {
    let data = [-62.3, 67.9, 71.02, 43.3, 51.7, 65.43, 67.23].to_vec();
    let results_tuple = get_tukeys_outliers(data, false).unwrap();

    assert_eq!(results_tuple.0, [-62.3].to_vec());
    assert_eq!(
        results_tuple.1,
        [43.3, 51.7, 65.43, 67.23, 67.9, 71.02].to_vec()
    );
    assert_eq!(results_tuple.2, [].to_vec());
}
