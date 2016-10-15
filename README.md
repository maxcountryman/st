# st
Fast and simple statistics on the command line.

`st` is a dead-simple tool for displaying simple statistics. It's a simplified
port of the epnonymous Perl program and a faster, more light weight
implementation of the Go variants.

Why another command line stats tool?

* Fast
* Lightweight
* Can handle larger datasets as compared to its cousins

## Installation

If you have Cargo available, then simply:

```
$ cargo install st
```

...or you can clone this repository and build with `rustc`:

```
rustc -O src/main.rs
```

## Usage

`st` takes input from stdin. It expects numbers to be separated by whitespace.
For example:

```
$ echo '1 2 3 4 5 6' | st
N      6
min    1
max    6
sum    21
median 3.5
mean   3.5
σ      1.8708286933869707
```
