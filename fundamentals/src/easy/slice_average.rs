/*
  Problem 11: Slice Average

  Write a function that takes a slice of f64 values and returns the arithmetic mean.
  If the slice is empty, return None.

  Run the tests for this problem with:
    cargo test --test slice_average_test
*/


pub fn average(values: &[f64]) -> Option<f64> {
   if values.is_empty() {
    return None;
   }
   else {
    let mut sum = 0.0;
    for num in values {
      sum += *num;
    }

    return Some(sum as f64 / values.len() as f64);
    
   }
}
