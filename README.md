# MTIE [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Build Status](https://travis-ci.com/github/robinjpark/mtie.png)](https://travis-ci.com/github/robinjpark/mtie)

Calculate MTIE from a series of TIE values.

## Usage
```
$ mtie --help
mtie 0.1
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
2 1.1999999999999997
3 1.2999999999999998
```

## TODO
1. Round off the MTIE value in the output file, based upon the input (or a specified precision).
1. Improve the documentation.
