//! The library crate.
//!
//! It implements the entire application.
//!
//! Although this library is only meant for the single binary,
//! the application is split into a binary and library to overcome
//! limitations on doc-tests, which can only run in library crates.

/// The entry point for the "library", which implements the game.
pub fn libmain() {
    println!("Library main function!");
    panic!("Write me!");
}

/// Calculates the MTIE of a set of evenly spaced samples
///
/// # Arguments
/// * samples
///
/// A slice of f64 values, containing the TIE samples.
/// This function assumes that the samples are evenly spaced in time.
///
/// # Output
/// A vector containing the calculated MTIE values.
///
/// If the input contained N samples, the output will contains N-1 values:
/// * The first value contains the calculated MTIE for an interval of one unit.
/// * The last value contains the MTIE for the maximum interval.
#[allow(dead_code)]
pub fn mtie (samples: &[f64]) -> Vec<f64>
{
    const MAX_DATA_SET_SIZE: usize = 50000; // Data sets bigger than this take too long to process!
    let count = samples.len();
    if count > MAX_DATA_SET_SIZE {
        panic!("Data set is too large for this MTIE algorithm, which is O(n^2).  This algorithm will not attempt to calculate MTIE on an input of more than {} samples!  The input data size was {} samples.", MAX_DATA_SET_SIZE, count);
    }

    let mut mtie = Vec::new();
    //println!("Input has {} samples", count);

    for tau in 1..count {
        //println!("tau {}", tau);
        let mut maximum = 0.0;
        for interval_start in 0..count-tau {
            let left_value = samples[interval_start];
            let right_value = samples[interval_start + tau];
            let difference = (right_value - left_value).abs();
            if difference > maximum {
                maximum = difference;
            }
        }
        mtie.push(maximum);
    }

    //println!("mtie {:?}", mtie);
    mtie
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_mtie_output_size() {
        let input = vec![1.0; 10];
        let output = mtie(&input);
        assert_eq!(output.len(), input.len() - 1);

        let input = vec![0.0; 99];
        let output = mtie(&input);
        assert_eq!(output.len(), input.len() - 1);
    }

    #[test]
    pub fn test_single_input() {
        let input = vec![1.0; 1];
        let output = mtie(&input);
        let expected = Vec::new();
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    pub fn test_flat_line() {
        let input = vec![0.0; 10];
        let expected = vec![0.0; 9];
        let output = mtie(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);

        let input = vec![1234.5678; 10];
        let output = mtie(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);

        let input = vec![-1000.0; 10];
        let output = mtie(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    pub fn test_constant_increase() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = vec![1.0, 2.0, 3.0, 4.0];
        let output = mtie(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    pub fn test_constant_decrease() {
        let input = vec![100.0, 90.0, 80.0, 70.0, 60.0];
        let expected = vec![10.0, 20.0, 30.0, 40.0];
        let output = mtie(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    pub fn test_step() {
        let input = vec![100.0, 100.0, 100.0, 150.0, 150.0];
        let expected = vec![50.0, 50.0, 50.0, 50.0];
        let output = mtie(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    pub fn test_two_steps() {
        let input = vec![100.0, 100.0, 150.0, 150.0, 200.0];
        let expected = vec![50.0, 50.0, 100.0, 100.0];
        let output = mtie(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    #[ignore]
    pub fn test_large() {
        let input = vec![0.0; 50000];
        let expected = vec![0.0; 49999];
        let output = mtie(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    #[should_panic(expected = "Data set is too large for this MTIE algorithm, which is O(n^2).  This algorithm will not attempt to calculate MTIE on an input of more than 50000 samples!  The input data size was")]
    pub fn test_too_large() {
        let input = vec![0.0; 50001];
        let _output = mtie(&input);
    }
}

