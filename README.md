# MTIE [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Build Status](https://travis-ci.com/robinjpark/mtie.svg?token=K5UsvTeLCfHUcRwSY7ts&branch=main)](https://travis-ci.com/github/robinjpark/mtie)

Calculate MTIE from a series of TIE values.

For a "small" set of TIE input data (100,000 samples or less), it will calculate the MTIE for all possible intervals.<br>
For a "large" set of TIE input data (more than 100,000 samples), it will only calculate the MTIE for intervals 1, 3, 7, 15, 31, ...<br>
Calculating all possible intervals is not feasible beyond a certain point.

## Acknowledgments
[Fast Algorithms for TVAR and MTIE Computation in Characterization of Network Synchronization Performance](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.10.3746&rep=rep1&type=pdf)

## Usage
```
$ mtie --help
mtie 0.1.0
Robin Park <robin.j.park@gmail.com>
Calculates MTIE from a series of TIE input data.

The TIE input data is expected to be in text format, with one number per line.
It is assumed that the input data was sampled at a uniform rate.
The MTIE calculation is unaware of the sampling rate of the data,
or the units of the TIE measurement.

The MTIE is printed to standard output, with each line containing:
- an interval
- the MTIE for that interval

USAGE:
    mtie [OPTIONS]

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
    -i, --input <input>
            Specifies the file containing the TIE input data.
            If this option is not given, TIE input data is taken from standard input.
```

## Input

The input file (or standard input) is a series of TIE values, one per line.  For example:
```
# Comment lines are allowed.  Any lines starting with a '#' character is a comment line and is ignored.
// Any lines starting with '//' are also deemed comments and are ignored.
1.1
2.2
2.3
2.4
```

### Output
The output consists of two columns, with the first column being the interval,
and the second column containing the MTIE for that interval.  For example:
```
1 1.1
2 1.2
3 1.3
```
