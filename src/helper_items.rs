use num::ToPrimitive;
use stats::median;

#[derive(std::cmp::PartialEq, std::fmt::Debug)]
pub enum ErrorMessage {
    MedianFunctionFailed,
    MinimumSetForQuartile,
    NanError,
    ToPrimitiveCast,
}

pub fn get_error_message(error_message: ErrorMessage) -> &'static str {
    match error_message {
        ErrorMessage::MedianFunctionFailed => "Cannot calculate median of data set",
        ErrorMessage::MinimumSetForQuartile => {
            "Cannot calculate the quartile values of a data set with less than 2 elements"
        }
        ErrorMessage::NanError => "The data set contains one or more NaNs",
        ErrorMessage::ToPrimitiveCast => "Had issues using ToPrimitiveCast to cast `T` to `f32`",
    }
}

pub fn get_quartile_values<T: ToPrimitive + PartialOrd + Clone>(
    data_vec: &[T],
) -> Result<(f64, f64, f64), ErrorMessage> {
    let data_vec_length = data_vec.len();

    if data_vec_length < 2 {
        return Err(ErrorMessage::MinimumSetForQuartile);
    }

    let mut halfway = data_vec_length / 2;

    let first_half_iter = data_vec.iter().take(halfway).cloned();
    let full_iter = data_vec.iter().cloned();

    if data_vec_length % 2 != 0 {
        halfway += 1;
    }

    let second_half_iter = data_vec.iter().skip(halfway).cloned();

    let q1_value = median(first_half_iter).ok_or(ErrorMessage::MedianFunctionFailed)?;
    let q2_value = median(full_iter).ok_or(ErrorMessage::MedianFunctionFailed)?;
    let q3_value = median(second_half_iter).ok_or(ErrorMessage::MedianFunctionFailed)?;

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
