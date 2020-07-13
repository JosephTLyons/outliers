use statrs::statistics::OrderStatistics;

type VectorTuple<T> = (Vec<T>, Vec<T>, Vec<T>);

/// This function uses the Tukey method, which uses a multiplier value of 1.5. In the case that is
/// does not return an `Err`, it returns a tuple of `Vec<f64>`s.  The first vector contains any lower
/// outliers and the third vector contains any upper outliers.  Additionally, the second vector
/// returned contains all the non-outliers, so that the data set passed in is returned, in its
/// entirety, as partitioned subsets.
/// ```
/// let data = [10.0, 12.0, 11.0, 15.0, 11.0, 14.0, 13.0, 17.0, 12.0, 22.0, 14.0, 11.0].to_vec();
/// let results_tuple = outliers::get_tukeys_outliers(data, false).unwrap();
///
/// assert_eq!(results_tuple.0, [].to_vec()); // Lower outliers
/// assert_eq!(results_tuple.1, [10.0, 11.0, 11.0, 11.0, 12.0, 12.0, 13.0, 14.0, 14.0, 15.0, 17.0].to_vec()); // Non-outliers
/// assert_eq!(results_tuple.2, [22.0].to_vec()); // Upper outliers
/// ```
pub fn get_tukeys_outliers(
    mut data_vec: Vec<f64>,
    data_is_sorted: bool,
) -> Result<VectorTuple<f64>, &'static str> {
    // Tests for NaNs in floats, should catch cases where the next `unwrap()` would panic, see:
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
    if data_vec.iter().any(|x| !(x == x)) {
        return Err("The data set contains one or more NaNs");
    }

    if !data_is_sorted {
        data_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    let q1_value = data_vec.lower_quartile();
    let q3_value = data_vec.upper_quartile();
    let interquartile_range = q3_value - q1_value;

    let intermediate_value = 1.5 * interquartile_range;
    let lower_range = q1_value - intermediate_value;
    let upper_range = q3_value + intermediate_value;

    let mut lower_outliers: Vec<f64> = Vec::new();
    let mut upper_outliers: Vec<f64> = Vec::new();
    let mut non_outliers: Vec<f64> = Vec::new();

    for data in data_vec {
        if (data) < lower_range {
            lower_outliers.push(data);
        } else if (data) > upper_range {
            upper_outliers.push(data);
        } else {
            non_outliers.push(data);
        }
    }

    Ok((lower_outliers, non_outliers, upper_outliers))
}

#[test]
fn get_tukeys_outliers_needs_sorted_nan_set() {
    let data: Vec<f64> = [f64::NAN, f64::NAN].to_vec();
    let results_tuple = get_tukeys_outliers(data, false);

    assert!(results_tuple.is_err());
}

#[test]
fn get_tukeys_outliers_is_sorted_nan_set() {
    let data: Vec<f64> = [3.0, 2.9, 2.8, 33.3, f64::NAN, f64::NAN].to_vec();
    let results_tuple = get_tukeys_outliers(data, true);

    assert!(results_tuple.is_err());
}

#[test]
fn get_tukeys_outliers_empty_data_set() {
    let data: Vec<f64> = [].to_vec();
    let results_tuple = get_tukeys_outliers(data, true).unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_tukeys_outliers_set_of_one() {
    let data: Vec<f64> = [30.0].to_vec();
    let results_tuple = get_tukeys_outliers(data, true).unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [30.0].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_tukeys_outliers_set_of_two() {
    let data: Vec<f64> = [30.0, 90.0].to_vec();
    let results_tuple = get_tukeys_outliers(data, true).unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [30.0, 90.0].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_tukeys_outliers_none() {
    let data: Vec<f64> = [1.0, 2.0, 4.0, 10.0].to_vec();
    let results_tuple = get_tukeys_outliers(data, true).unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [1.0, 2.0, 4.0, 10.0].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_tukeys_outliers_1() {
    let data = [
        0.0, 3.0, 3.0, 3.0, 11.0, 12.0, 13.0, 15.0, 19.0, 20.0, 29.0, 40.0, 79.0,
    ]
    .to_vec();
    let results_tuple = get_tukeys_outliers(data, true).unwrap();

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(
        results_tuple.1,
        [0.0, 3.0, 3.0, 3.0, 11.0, 12.0, 13.0, 15.0, 19.0, 20.0, 29.0, 40.0].to_vec()
    );
    assert_eq!(results_tuple.2, [79.0].to_vec());
}

#[test]
fn get_tukeys_outliers_negative_1() {
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
fn get_tukeys_outliers_negative_2() {
    let data = [-62.3, 67.9, 71.02, 43.3, 51.7, 65.43, 67.23].to_vec();
    let results_tuple = get_tukeys_outliers(data, false).unwrap();

    assert_eq!(results_tuple.0, [-62.3].to_vec());
    assert_eq!(
        results_tuple.1,
        [43.3, 51.7, 65.43, 67.23, 67.9, 71.02].to_vec()
    );
    assert_eq!(results_tuple.2, [].to_vec());
}
