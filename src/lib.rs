//! The library crate for the MTIE application.

#[cfg(test)]
#[macro_use]
extern crate time_test;

use anyhow::Context;
use std::io::Read;

/// The entry point for the "library", which implements the mtie application.
pub fn run() -> anyhow::Result<()> {
    let input_filename = parse_arguments_for_filename();
    let input = get_tie_input_data(input_filename).context("failed to get TIE input data")?;
    let tie = parse_tie_input_data(input);

    let sample_count = tie.len();
    let mtie = if sample_count <= 100_000 {
        mtie_complete(&tie)
    } else {
        mtie_fast(&tie)
    };

    print_mtie(&mtie);

    Ok(())
}

// Parses the command line arguments, returning the input filename, if specified
fn parse_arguments_for_filename() -> Option<String> {
    let long_about = "Calculates MTIE from a series of TIE input data.\n\n\
                      The TIE input data is expected to be in text format, with one number per line.\n\
                      It is assumed that the input data was sampled at a uniform rate.\n\
                      The MTIE calculation is unaware of the sampling rate of the data,\n\
                      or the units of the TIE measurement.\n\n\
                      The MTIE is printed to standard output, with each line containing:\n\
                      - an interval\n\
                      - the MTIE for that interval";
    let input_help = "Specifies the file containing the TIE input data.\n\
                      If this option is not given, TIE input data is taken from standard input.";
    let matches = clap::App::new("mtie")
        .version("0.1.0")
        .author("Robin Park <robin.j.park@gmail.com>")
        .about("Calculates MTIE from a set of TIE data.")
        .long_about(long_about)
        .arg(
            clap::Arg::with_name("input")
                .help(input_help)
                .short("i")
                .long("input")
                .takes_value(true),
        )
        .get_matches();
    let input_file = matches.value_of("input");
    input_file.map(str::to_string)
}

// Reads the TIE input data from the given filename (or standard input),
// returning the data in one giant String
fn get_tie_input_data(input_filename: Option<String>) -> anyhow::Result<String> {
    let buffer = match input_filename {
        Some(input_filename) => std::fs::read_to_string(&input_filename)
            .with_context(|| format!("Could not read file '{}'", input_filename))?,
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        }
    };
    Ok(buffer)
}

// Parses the TIE input data, converting from a big giant string,
// into a vector of TIE values.
fn parse_tie_input_data(input: String) -> Vec<f64> {
    let mut tie_values = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Ignore comments, which start with a "#" or "//"
        if trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        if !trimmed.is_empty() {
            let line_number = line_number + 1; // enumerate starts at 0, but we think of files as starting at line 1.
            let parse_result = trimmed.parse::<f64>();
            match parse_result {
                Ok(number) => tie_values.push(number),

                // TODO: Is this error handling sufficient?
                // It currently simply ignores any invalid input, outputting an error message to standard error.
                Err(_error) => eprintln!(
                    "Ignoring line {} '{}': it does not contain a valid number",
                    line_number, line
                ),
            }
        }
    }

    tie_values
}

// Prints the MTIE for each interval, in two columns:
// <interval> <mtie_value>
fn print_mtie(mtie: &[(u32, f64)]) {
    for (interval, val) in mtie {
        println!("{} {}", interval, val);
    }
}

// Calculates the "complete" MTIE for a series of TIE values
pub fn mtie_complete(samples: &[f64]) -> Vec<(u32, f64)> {
    const MAX_DATA_SET_SIZE: usize = 100_000; // Data sets bigger than this take too long to process!
    let count = samples.len();
    if count > MAX_DATA_SET_SIZE {
        panic!("Data set is too large for this MTIE algorithm, which is O(n^2).  This algorithm will not attempt to calculate MTIE on an input of more than {} samples!  The input data size was {} samples.", MAX_DATA_SET_SIZE, count);
    }

    let mut mtie = Vec::new();

    let mut prev_maximum = 0.0;
    for tau in 1..count {
        let mut maximum = 0.0;
        for interval_start in 0..count - tau {
            let left_value = samples[interval_start];
            let right_value = samples[interval_start + tau];
            let difference = (right_value - left_value).abs();
            if difference > maximum {
                maximum = difference;
            }
        }
        if prev_maximum > maximum {
            maximum = prev_maximum;
        }
        mtie.push((tau as u32, maximum));
        prev_maximum = maximum;
    }

    check_monotomically_increasing(&mtie);
    mtie
}

// Calculates the "fast" MTIE for a series of TIE values.
//
// See "Fast Algorithms for TVAR and MTIE Computation in Characterization of Network Synchronization Performance"
// https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.10.3746&rep=rep1&type=pdf
#[allow(non_snake_case)] // to allow the variable names to match the reference algorithm
pub fn mtie_fast(samples: &[f64]) -> Vec<(u32, f64)> {
    let N = samples.len() as u32;
    let k_max = (N as f64).log2() as u32;

    let mut a_M: std::vec::Vec<std::vec::Vec<f64>> = Vec::new();
    let mut a_m: std::vec::Vec<std::vec::Vec<f64>> = Vec::new();

    // Push a dummy row into each to allow indexing a_M and a_m by k (which starts at 1)
    a_M.push(Vec::new());
    a_m.push(Vec::new());

    for k in 1..k_max + 1 {
        let k = k as usize;
        let mut a_M_k = Vec::new();
        let mut a_m_k = Vec::new();
        a_M_k.push(0.0); // push dummy value to allow indexing by i, which starts at 1, not 0
        a_m_k.push(0.0); // push dummy value to allow indexing by i, which starts at 1, not 0
        if k == 1 {
            let i_max = N - 2_u32.pow(k as u32) + 1;
            for i in 1..i_max + 1 {
                let i = i as usize;
                let val1 = samples[i - 1]; // samples are indexes by 0, not 1
                let val2 = samples[i];
                let max = if val1 > val2 { val1 } else { val2 };
                let min = if val1 < val2 { val1 } else { val2 };
                a_M_k.push(max);
                a_m_k.push(min);
            }
        } else {
            let i_max = N - 2_u32.pow(k as u32) + 1;
            let p = 2_u32.pow((k as u32) - 1) as usize;
            for i in 1..i_max + 1 {
                let i = i as usize;
                let max1 = a_M[k - 1][i];
                let max2 = a_M[k - 1][i + p];
                let max = if max1 > max2 { max1 } else { max2 };
                let min1 = a_m[k - 1][i];
                let min2 = a_m[k - 1][i + p];
                let min = if min1 < min2 { min1 } else { min2 };
                a_M_k.push(max);
                a_m_k.push(min);
            }
        }
        a_M.push(a_M_k);
        a_m.push(a_m_k);
    }

    let mut mtie = Vec::new();
    for k in 1..k_max + 1 {
        let i_max = N - 2_u32.pow(k) + 1;
        let tau = (2_u32.pow(k) - 1) as u32;
        let k = k as usize;
        let mut mtie_k = a_M[k][1] - a_m[k][1];
        for i in 2..i_max + 1 {
            let i = i as usize;
            if a_M[k][i] - a_m[k][i] > mtie_k {
                mtie_k = a_M[k][i] - a_m[k][i];
            }
        }
        mtie.push((tau, mtie_k));
    }

    check_monotomically_increasing(&mtie);
    mtie
}

fn check_monotomically_increasing(mtie: &[(u32, f64)]) {
    for (index, window) in mtie.windows(2).enumerate() {
        if window[1] < window[0] {
            panic!(
                "MTIE is not monotomically increasing! indices {}-{} contains {} and {}.",
                index,
                index + 1,
                window[0].1,
                window[1].1
            );
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_valid_input() {
        // Well formatted input
        let input = "1.0\n2.0\n3.0".to_string();
        let numbers = parse_tie_input_data(input);
        assert_eq!(numbers, vec![1.0, 2.0, 3.0]);

        // Same as above, with trailing newline
        let input = "1.0\n2.0\n3.0\n".to_string();
        let numbers = parse_tie_input_data(input);
        assert_eq!(numbers, vec![1.0, 2.0, 3.0]);

        // Blank lines
        let input = "1.0\n\n\n\n2.0".to_string();
        let numbers = parse_tie_input_data(input);
        assert_eq!(numbers, vec![1.0, 2.0]);

        // Lines with whitespace
        let input = "1.0\n    \n2.0".to_string();
        let numbers = parse_tie_input_data(input);
        assert_eq!(numbers, vec![1.0, 2.0]);
    }

    #[test]
    pub fn test_invalid_input() {
        let input = "1\nnot_a_number".to_string();
        let _numbers = parse_tie_input_data(input);
    }

    #[test]
    pub fn test_mtie_output_size() {
        let input = vec![1.0; 10];
        let output = mtie_complete(&input);
        assert_eq!(output.len(), input.len() - 1);

        let input = vec![0.0; 99];
        let output = mtie_complete(&input);
        assert_eq!(output.len(), input.len() - 1);
    }

    #[test]
    pub fn test_single_input() {
        let input = vec![1.0; 1];
        let output = mtie_complete(&input);
        let expected = Vec::new();
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    fn test_slow_algo_values(input: Vec<f64>, expected: Vec<f64>) {
        let output = mtie_complete(&input);
        let values: Vec<f64> = output
            .clone()
            .into_iter()
            .map(|(_tau, mtie)| mtie)
            .collect();
        assert_eq!(values, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    pub fn test_flat_line() {
        let input = vec![0.0; 10];
        let expected = vec![0.0; 9];
        test_slow_algo_values(input, expected);

        let input = vec![1234.5678; 10];
        let expected = vec![0.0; 9];
        test_slow_algo_values(input, expected);

        let input = vec![-1000.0; 10];
        let expected = vec![0.0; 9];
        test_slow_algo_values(input, expected);
    }

    #[test]
    pub fn test_constant_increase() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = vec![1.0, 2.0, 3.0, 4.0];
        test_slow_algo_values(input, expected);
    }

    #[test]
    pub fn test_constant_decrease() {
        let input = vec![100.0, 90.0, 80.0, 70.0, 60.0];
        let expected = vec![10.0, 20.0, 30.0, 40.0];
        test_slow_algo_values(input, expected);
    }

    #[test]
    pub fn test_step() {
        let input = vec![100.0, 100.0, 100.0, 150.0, 150.0];
        let expected = vec![50.0, 50.0, 50.0, 50.0];
        test_slow_algo_values(input, expected);
    }

    #[test]
    pub fn test_two_steps() {
        let input = vec![100.0, 100.0, 150.0, 150.0, 200.0];
        let expected = vec![50.0, 50.0, 100.0, 100.0];
        test_slow_algo_values(input, expected);
    }

    #[test]
    pub fn test_oscillating() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let expected = vec![1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0, 4.0];
        test_slow_algo_values(input, expected);
    }

    #[test]
    #[ignore] // Normally ignore, because it takes a long time in debug builds.
    pub fn test_large() {
        time_test!();
        let input = vec![0.0; 100_000];
        let expected = vec![0.0; 99_999];
        test_slow_algo_values(input, expected);
    }

    #[test]
    #[should_panic(
        expected = "Data set is too large for this MTIE algorithm, which is O(n^2).  This algorithm will not attempt to calculate MTIE on an input of more than 100000 samples!  The input data size was 100001 samples."
    )]
    pub fn test_too_large() {
        let input = vec![0.0; 100_001];
        let _output = mtie_complete(&input);
    }

    #[test]
    pub fn test_fast_constant() {
        let input = vec![1.0, 1.0, 1.0, 1.0];
        let expected = vec![(1, 0.0), (3, 0.0)];
        let output = mtie_fast(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    pub fn test_fast_slope() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let expected = vec![(1, 1.0), (3, 3.0), (7, 7.0)];
        let output = mtie_fast(&input);
        assert_eq!(output, expected, "mtie for {:?} is {:?}", input, output);
    }

    #[test]
    #[ignore] // Normally ignore, because it takes a long time in debug builds.
    pub fn test_fast_large() {
        time_test!();
        let input = vec![0.0; 20_000_000];
        let output = mtie_fast(&input);
        assert_eq!(output.len(), 24, "mtie is {:?}", output);
    }
}
