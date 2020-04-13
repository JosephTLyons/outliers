use num::ToPrimitive;

#[derive(std::cmp::PartialEq, std::fmt::Debug)]
pub enum ErrorMessage {
    ToPrimitiveCast,
    MinimumSetForQuartile,
    MinimumSetForMedian,
    Sort,
}

pub fn get_error_message(error_message: ErrorMessage) -> &'static str {
    match error_message {
        ErrorMessage::ToPrimitiveCast => "Had issues ToPrimitiveCast `T` to `f32`",
        ErrorMessage::MinimumSetForQuartile => {
            "Cannot calculate the quartile values of a data set with less than 2 elements"
        }
        ErrorMessage::MinimumSetForMedian => "Cannot calculate the median of an empty data set",
        ErrorMessage::Sort => "Sorting the data failed",
    }
}

pub fn get_quartile_values<T: ToPrimitive>(
    data_vec: &[T],
) -> Result<(f32, f32, f32), ErrorMessage> {
    let data_vec_length = data_vec.len();

    if data_vec_length < 2 {
        return Err(ErrorMessage::MinimumSetForQuartile);
    }

    let mut halfway = data_vec_length / 2;

    let q1_value = match get_median(&data_vec[0..halfway]) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let q2_value = match get_median(&data_vec) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    if data_vec_length % 2 != 0 {
        halfway += 1;
    }

    let q3_value = match get_median(&data_vec[halfway..data_vec_length]) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    Ok((q1_value, q2_value, q3_value))
}

#[test]
fn get_quartile_values_empty_set() {
    let data: [usize; 0] = [];
    let quartile_values_result = get_quartile_values(&data);

    assert!(quartile_values_result.is_err());
}

#[test]
fn get_quartile_values_set_of_one() {
    let data = [10];
    let quartile_values_result = get_quartile_values(&data);

    assert!(quartile_values_result.is_err());
}

#[test]
fn get_quartile_values_set_of_two() {
    let data = [10, 12];
    let quartile_values_result = get_quartile_values(&data);

    assert_eq!(quartile_values_result, Ok((10.0, 11.0, 12.0)));
}

#[test]
fn get_quartile_values_set_of_three() {
    let data = [10, 11, 12];
    let quartile_values_result = get_quartile_values(&data);

    assert_eq!(quartile_values_result, Ok((10.0, 11.0, 12.0)));
}

// [1   2   3   4]   [5   6   7   8]
//        |        |        |
//        Q1       Q2       Q3
#[test]
fn get_quartile_values_even_set_even_halves() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let quartile_values_result = get_quartile_values(&data);

    assert_eq!(quartile_values_result, Ok((2.5, 4.5, 6.5)));
}

// [1   2   3]   [4   5   6]
//      |      |      |
//      Q1     Q2     Q3
#[test]
fn get_quartile_values_even_set_odd_halves() {
    let data = [1, 2, 3, 4, 5, 6];
    let quartile_values_result = get_quartile_values(&data);

    assert_eq!(quartile_values_result, Ok((2.0, 3.5, 5.0)));
}

// [1   2   3   4]   5   [6   7   8   9]
//        |          |          |
//        Q1         Q2         Q3
#[test]
fn get_quartile_values_odd_set_even_halves() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let quartile_values_result = get_quartile_values(&data);

    assert_eq!(quartile_values_result, Ok((2.5, 5.0, 7.5)));
}

// [1   2   3   4   5]   6   [7   8   9   10   11]
//          |            |            |
//          Q1           Q2           Q3
#[test]
fn get_quartile_values_odd_set_odd_halves() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let quartile_values_result = get_quartile_values(&data);

    assert_eq!(quartile_values_result, Ok((3.0, 6.0, 9.0)));
}

#[test]
fn get_quartile_values_float_set_of_two() {
    let data = [10.27, 12.9];
    let quartile_values_result = get_quartile_values(&data);

    assert_eq!(quartile_values_result, Ok((10.27, 11.585, 12.9)));
}

#[test]
fn get_quartile_values_float_set_of_three() {
    let data = [10.167, 11.917, 12.3];
    let quartile_values_result = get_quartile_values(&data);

    assert_eq!(quartile_values_result, Ok((10.167, 11.917, 12.3)));
}

pub fn get_median<T: ToPrimitive>(data_vec: &[T]) -> Result<f32, ErrorMessage> {
    let data_vec_length = data_vec.len();

    if data_vec_length == 0 {
        return Err(ErrorMessage::MinimumSetForMedian);
    }

    let half_way = data_vec_length / 2;

    let mut result_f32 = match ToPrimitive::to_f32(&data_vec[half_way]) {
        Some(value_f32) => value_f32,
        None => return Err(ErrorMessage::ToPrimitiveCast),
    };

    if data_vec.len() % 2 == 0 {
        let left_middle = match ToPrimitive::to_f32(&data_vec[half_way - 1]) {
            Some(value_f32) => value_f32,
            None => return Err(ErrorMessage::ToPrimitiveCast),
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
