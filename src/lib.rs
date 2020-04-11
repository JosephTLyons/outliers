use num::ToPrimitive;

/// This function uses the Tukey method, which uses a multiplier value of 1.5. In the case that is
/// does not return an `Err`, it returns a tuple of `Vec<T>`.  The first vector contains any lower
/// outliers and the third vector contains any upper outliers.  Additionally, the second vector
/// returned contains all the non-outliers; so that the data set passed in is returned in its
/// entirety as partitioned subsets.
/// ```
/// let data = [10, 12, 11, 15, 11, 14, 13, 17, 12, 22, 14, 11].to_vec();
/// let results_tuple = outliers::get_tukeys_outliers(data, false).unwrap();
///
/// assert_eq!(results_tuple.0, [].to_vec()); // Lower outliers
/// assert_eq!(results_tuple.1, [10, 11, 11, 11, 12, 12, 13, 14, 14, 15, 17].to_vec()); // Non-outliers
/// assert_eq!(results_tuple.2, [22].to_vec()); // Upper outliers
/// ```
pub fn get_tukeys_outliers<T: std::cmp::PartialOrd + ToPrimitive>(
    mut data_vec: Vec<T>,
    is_sorted: bool,
) -> Result<(Vec<T>, Vec<T>, Vec<T>), &'static str> {
    if !is_sorted {
        // TODO: Error handle this unwrap
        data_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    let mut lower_outliers: Vec<T> = Vec::new();
    let mut upper_outliers: Vec<T> = Vec::new();

    if let Ok((q1_value, _, q3_value)) = get_quartile_values(&data_vec) {
        let interquartile_range: f32 = q3_value - q1_value;

        let intermediate_value: f32 = 1.5 * interquartile_range;
        let lower_range: f32 = q1_value - intermediate_value;
        let upper_range: f32 = q3_value + intermediate_value;

        let mut non_outliers: Vec<T> = Vec::new();

        for data in data_vec {
            let data_f32 = match ToPrimitive::to_f32(&data) {
                Some(value_f32) => value_f32,
                None => return Err("Had issues casting T to f32"),
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

    Ok((lower_outliers, data_vec, upper_outliers))
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

// TODO: Should there be a float test for each case that integers were tested on?

fn get_quartile_values<T: ToPrimitive>(data_vec: &[T]) -> Result<(f32, f32, f32), &'static str> {
    let data_vec_length = data_vec.len();

    if data_vec_length < 2 {
        return Err("Cannot calculate the quartile values of a data set with less than 2 elements");
    }

    let mut halfway = data_vec_length / 2;

    let q1_value = get_median(&data_vec[0..halfway]);
    let q2_value = get_median(&data_vec);

    if data_vec_length % 2 != 0 {
        halfway += 1;
    }

    let q3_value = get_median(&data_vec[halfway..data_vec_length]);

    Ok((q1_value.unwrap(), q2_value.unwrap(), q3_value.unwrap()))
}

#[test]
fn get_quartile_values_empty_set() {
    let data: [usize; 0] = [];
    let quartile_values_option = get_quartile_values(&data);

    assert!(quartile_values_option.is_err());
}

#[test]
fn get_quartile_values_set_of_one() {
    let data = [10];
    let quartile_values_option = get_quartile_values(&data);

    assert!(quartile_values_option.is_err());
}

#[test]
fn get_quartile_values_set_of_two() {
    let data = [10, 12];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Ok((10.0, 11.0, 12.0)));
}

#[test]
fn get_quartile_values_set_of_three() {
    let data = [10, 11, 12];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Ok((10.0, 11.0, 12.0)));
}

// [1   2   3   4]   [5   6   7   8]
//        |        |        |
//        Q1       Q2       Q3
#[test]
fn get_quartile_values_even_set_even_halves() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Ok((2.5, 4.5, 6.5)));
}

// [1   2   3]   [4   5   6]
//      |      |      |
//      Q1     Q2     Q3
#[test]
fn get_quartile_values_even_set_odd_halves() {
    let data = [1, 2, 3, 4, 5, 6];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Ok((2.0, 3.5, 5.0)));
}

// [1   2   3   4]   5   [6   7   8   9]
//        |          |          |
//        Q1         Q2         Q3
#[test]
fn get_quartile_values_odd_set_even_halves() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Ok((2.5, 5.0, 7.5)));
}

// [1   2   3   4   5]   6   [7   8   9   10   11]
//          |            |            |
//          Q1           Q2           Q3
#[test]
fn get_quartile_values_odd_set_odd_halves() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Ok((3.0, 6.0, 9.0)));
}

#[test]
fn get_quartile_values_float_set_of_two() {
    let data = [10.27, 12.9];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Ok((10.27, 11.585, 12.9)));
}

#[test]
fn get_quartile_values_float_set_of_three() {
    let data = [10.167, 11.917, 12.3];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Ok((10.167, 11.917, 12.3)));
}

// TODO: Should there be a float test for each case that integers were tested on?

fn get_median<T: ToPrimitive>(data_vec: &[T]) -> Result<f32, &'static str> {
    let data_vec_length = data_vec.len();

    if data_vec_length == 0 {
        return Err("Cannot calculate the median of an empty data set");
    }

    let half_way = data_vec_length / 2;
    let error_message: &'static str = "Had issues casting T to f32";

    let mut result_f32 = match ToPrimitive::to_f32(&data_vec[half_way]) {
        Some(value_f32) => value_f32,
        None => return Err(error_message),
    };

    if data_vec.len() % 2 == 0 {
        let left_middle = match ToPrimitive::to_f32(&data_vec[half_way - 1]) {
            Some(value_f32) => value_f32,
            None => return Err(error_message),
        };

        result_f32 = (result_f32 + left_middle) / 2.0;
    }

    Ok(result_f32)
}

#[test]
fn get_median_no_elements() {
    let data: Vec<usize> = [].to_vec();
    assert!(get_median(&data).is_err());
}

#[test]
fn get_median_one_element() {
    assert!((get_median(&[3]).unwrap() - 3.0).abs() < 0.0001);
}

#[test]
fn get_median_even_set() {
    assert!((get_median(&[1, 2, 3, 4, 5, 6]).unwrap() - 3.5).abs() < 0.0001);
}

#[test]
fn get_median_odd_set() {
    assert!((get_median(&[1, 2, 3, 4, 5]).unwrap() - 3.0).abs() < 0.0001);
}

#[test]
fn get_median_random_numbers_even_set() {
    assert!((get_median(&[1, 11, 34, 66, 209, 353, 1067, 10_453]).unwrap() - 137.5).abs() < 0.0001);
}

#[test]
fn get_median_random_numbers_odd_set() {
    assert!((get_median(&[1, 23, 24, 45, 200, 343, 1001]).unwrap() - 45.0).abs() < 0.0001);
}

#[test]
fn get_median_float_negative_even_set() {
    assert!((get_median(&[-1.32, 32.2]).unwrap() - 15.44).abs() < 0.0001);
}

#[test]
fn get_median_float_negative_odd_set() {
    assert!((get_median(&[-1.32, 32.2, 40.1]).unwrap() - 32.2).abs() < 0.0001);
}

// TODO: Should there be a float test for each case that integers were tested on?
