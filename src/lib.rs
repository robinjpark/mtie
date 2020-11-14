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
    const MAX_DATA_SET_SIZE: usize = 100_000; // Data sets bigger than this take too long to process!
    let count = samples.len();
    if count > MAX_DATA_SET_SIZE {
        panic!("Data set is too large for this MTIE algorithm, which is O(n^2).  This algorithm will not attempt to calculate MTIE on an input of more than {} samples!  The input data size was {} samples.", MAX_DATA_SET_SIZE, count);
    }

    let mut mtie = Vec::new();

    for tau in 1..count {
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

    mtie
}

#[allow(dead_code)]
#[allow(non_snake_case)] // to allow the variable names to match the reference algorithm
// See "Fast Algorithms for TVAR and MTIE Computation in Characterization of Network Synchronization Performance"
// https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.10.3746&rep=rep1&type=pdf
pub fn mtie_fast (samples: &[f64]) -> Vec<f64>
{
    let N = samples.len() as u32;
    let k_max = (N as f64).log2() as u32;

    let mut a_M : std::vec::Vec<std::vec::Vec<f64>> = Vec::new();
    let mut a_m : std::vec::Vec<std::vec::Vec<f64>> = Vec::new();

    // Push a dummy row into each to allow indexing a_M and a_m by k (which starts at 1)
    a_M.push(Vec::new());
    a_m.push(Vec::new());

    for k in 1..k_max+1 {
        let k = k as usize;
        let mut a_M_k = Vec::new();
        let mut a_m_k = Vec::new();
        a_M_k.push(0.0); // push dummy value to allow indexing by i, which starts at 1, not 0
        a_m_k.push(0.0); // push dummy value to allow indexing by i, which starts at 1, not 0
        if k == 1 {
            let i_max = N-2_u32.pow(k as u32)+1;
            for i in 1..i_max+1 {
                let i = i as usize;
                let val1 = samples[i-1]; // samples are indexes by 0, not 1
                let val2 = samples[i];
                let max = if val1 > val2 { val1 } else { val2 };
                let min = if val1 < val2 { val1 } else { val2 };
                a_M_k.push(max);
                a_m_k.push(min);
            }
        } else {
            let i_max = N-2_u32.pow(k as u32)+1;
            let p = 2_u32.pow((k as u32)-1) as usize;
            for i in 1..i_max+1 {
                let i = i as usize;
                let max1 = a_M[k-1][i];
                let max2 = a_M[k-1][i+p];
                let max = if max1 > max2 { max1 } else { max2 };
                let min1 = a_m[k-1][i];
                let min2 = a_m[k-1][i+p];
                let min = if min1 < min2 { min1 } else { min2 };
                a_M_k.push(max);
                a_m_k.push(min);
            }
        }
        a_M.push(a_M_k);
        a_m.push(a_m_k);
    }

    let mut mtie = Vec::new();
    for k in 1..k_max+1 {
        let i_max = N-2_u32.pow(k)+1;
        let k = k as usize;
        let mut mtie_k = a_M[k][1] - a_m[k][1];
        for i in 2..i_max+1 {
            let i = i as usize;
            if a_M[k][i] - a_m[k][i] > mtie_k {
                mtie_k = a_M[k][i] - a_m[k][i];
            }
        }
        mtie.push(mtie_k);
    }

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
        let input = vec![0.0; 100_000];
        let expected = vec![0.0; 99_999];
        let output = mtie(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    #[should_panic(expected = "Data set is too large for this MTIE algorithm, which is O(n^2).  This algorithm will not attempt to calculate MTIE on an input of more than 100000 samples!  The input data size was 100001 samples.")]
    pub fn test_too_large() {
        let input = vec![0.0; 100_001];
        let _output = mtie(&input);
    }

    #[test]
    pub fn test_fast_constant() {
        let input = vec![1.0, 1.0, 1.0, 1.0];
        let expected = vec![0.0, 0.0];
        let output = mtie_fast(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    pub fn test_fast_slope() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let expected = vec![1.0, 3.0, 7.0];
        let output = mtie_fast(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    #[ignore]
    pub fn test_fast_large() {
        let input = vec!(0.0; 10_000_000);
        let output = mtie_fast(&input);
        assert_eq!(output.len(), 23, "mtie is {:?}", output);
    }

}

