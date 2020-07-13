# CHANGELOG:

## July 13, 2020 - v0.3.0

- General code cleanup
- Now using [statrs](https://crates.io/crates/statrs) to calculate upper and
  lower quartiles.  This allowed for a large portion of the code to be deleted.
  **Note**: This causes a breaking change, as the input to
  `get_tukeys_outliers()` no longer allows for generic values, and requires
  `f64`.

## April 14, 2020 - v0.2.1

- Improvements to error handling

## April 11, 2020 - v0.2.0

- NOTE: This update introduces some breaking changes, such as
  `get_tukeys_outliers()` now returns a `Result`
- `get_tukeys_outliers()` is now generic and can accept more than just `usize`
  data types

## April 11, 2020 - v0.1.0

- Initial release
