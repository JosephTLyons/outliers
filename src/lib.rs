/// This function uses the Tukey method, which uses a multiplier value of 1.5. It returns a tuple
/// that contains three vectors.  The first vector contains any lower outliers, the third vector
/// contains any upper outliers, and the second vector contains the non-outliers.
/// ```
/// let data = [10, 12, 11, 15, 11, 14, 13, 17, 12, 22, 14, 11].to_vec();
/// let results_tuple = outliers::get_tukeys_outliers(data, false);
///
/// assert_eq!(results_tuple.0, [].to_vec());
/// assert_eq!(results_tuple.1, [10, 11, 11, 11, 12, 12, 13, 14, 14, 15, 17].to_vec());
/// assert_eq!(results_tuple.2, [22].to_vec());
/// ```
pub fn get_tukeys_outliers(
    mut data_vec: Vec<usize>,
    is_sorted: bool,
) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    if !is_sorted {
        data_vec.sort();
    }

    let mut lower_outliers: Vec<usize> = Vec::new();
    let mut upper_outliers: Vec<usize> = Vec::new();

    if let Some((q1_value, _, q3_value)) = get_quartile_values(&data_vec) {
        let interquartile_range = q3_value - q1_value;

        let intermediate_value = 1.5 * interquartile_range;
        let lower_range = q1_value - intermediate_value;
        let upper_range = q3_value + intermediate_value;

        let mut non_outliers: Vec<usize> = Vec::new();

        for data in data_vec {
            if (data as f32) < lower_range {
                lower_outliers.push(data);
            } else if (data as f32) > upper_range {
                upper_outliers.push(data);
            } else {
                non_outliers.push(data);
            }
        }

        data_vec = non_outliers;
    }

    (lower_outliers, data_vec, upper_outliers)
}

#[test]
fn get_tukeys_outliers_empty_data_set() {
    let data = [].to_vec();
    let results_tuple = get_tukeys_outliers(data, true);

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_tukeys_outliers_set_of_one() {
    let data = [30].to_vec();
    let results_tuple = get_tukeys_outliers(data, true);

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [30].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_tukeys_outliers_set_of_two() {
    let data = [30, 90].to_vec();
    let results_tuple = get_tukeys_outliers(data, true);

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [30, 90].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_tukeys_outliers_none() {
    let data = [1, 2, 4, 10].to_vec();
    let results_tuple = get_tukeys_outliers(data, true);

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(results_tuple.1, [1, 2, 4, 10].to_vec());
    assert_eq!(results_tuple.2, [].to_vec());
}

#[test]
fn get_tukeys_outliers_1() {
    let data = [10, 12, 11, 15, 11, 14, 13, 17, 12, 22, 14, 11].to_vec();
    let results_tuple = get_tukeys_outliers(data, false);

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(
        results_tuple.1,
        [10, 11, 11, 11, 12, 12, 13, 14, 14, 15, 17].to_vec()
    );
    assert_eq!(results_tuple.2, [22].to_vec());
}

#[test]
fn get_tukeys_outliers_2() {
    let data = [0, 3, 3, 3, 11, 12, 13, 15, 19, 20, 29, 40, 79].to_vec();
    let results_tuple = get_tukeys_outliers(data, false);

    assert_eq!(results_tuple.0, [].to_vec());
    assert_eq!(
        results_tuple.1,
        [0, 3, 3, 3, 11, 12, 13, 15, 19, 20, 29, 40].to_vec()
    );
    assert_eq!(results_tuple.2, [79].to_vec());
}

fn get_quartile_values(data_vec: &[usize]) -> Option<(f32, f32, f32)> {
    let data_vec_length = data_vec.len();

    if data_vec_length < 2 {
        return None;
    }

    let mut halfway = data_vec_length / 2;

    let q1_value = get_median(&data_vec[0..halfway]);
    let q2_value = get_median(&data_vec);

    if data_vec_length % 2 != 0 {
        halfway += 1;
    }

    let q3_value = get_median(&data_vec[halfway..data_vec_length]);

    Some((q1_value.unwrap(), q2_value.unwrap(), q3_value.unwrap()))
}

#[test]
fn get_quartile_values_empty_set() {
    let data = [];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, None);
}

#[test]
fn get_quartile_values_set_of_one() {
    let data = [10];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, None);
}

#[test]
fn get_quartile_values_set_of_two() {
    let data = [10, 12];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Some((10.0, 11.0, 12.0)));
}

#[test]
fn get_quartile_values_set_of_three() {
    let data = [10, 11, 12];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Some((10.0, 11.0, 12.0)));
}

// [1   2   3   4]   [5   6   7   8]
//        |        |        |
//        Q1       Q2       Q3
#[test]
fn get_quartile_values_even_set_even_halves() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Some((2.5, 4.5, 6.5)));
}

// [1   2   3]   [4   5   6]
//      |      |      |
//      Q1     Q2     Q3
#[test]
fn get_quartile_values_even_set_odd_halves() {
    let data = [1, 2, 3, 4, 5, 6];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Some((2.0, 3.5, 5.0)));
}

// [1   2   3   4]   5   [6   7   8   9]
//        |          |          |
//        Q1         Q2         Q3
#[test]
fn get_quartile_values_odd_set_even_halves() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Some((2.5, 5.0, 7.5)));
}

// [1   2   3   4   5]   6   [7   8   9   10   11]
//          |            |            |
//          Q1           Q2           Q3
#[test]
fn get_quartile_values_odd_set_odd_halves() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let quartile_values_option = get_quartile_values(&data);

    assert_eq!(quartile_values_option, Some((3.0, 6.0, 9.0)));
}

fn get_median(data_vec: &[usize]) -> Option<f32> {
    let data_vec_length = data_vec.len();

    if data_vec_length == 0 {
        return None;
    }

    let half_way = data_vec_length / 2;

    if data_vec.len() % 2 == 0 {
        return Some((data_vec[half_way - 1] as f32 + data_vec[half_way] as f32) / 2.0);
    }

    Some(data_vec[half_way] as f32)
}

#[test]
fn get_median_no_elements() {
    assert!(get_median(&[]).is_none());
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
