# About Rusty-SPAdes
Rusty-SPAdes is a toolkit designed for accurate assembly of Illumina sequencing data. As the name implies, this project was no only inspired by SPAdes, but utilizes the same algorithms and graph assembly strategies; this project is not intended to offer an alternative strategy to genome assembly, but rather port SPAdes, prioritizing the following metrics:

1. Memory Usage: SPAdes is well-known to consume large amounts of memory, reaching hundreds of GB or even TBs of RAM when assembling large genomes. By storing some data in an intermediate database, Rusty-SPAdes attempts to reduce memory consumption at the cost of some speed.
2. Ease of Refactoring: I initially wanted to fork the SPAdes project, but at least to me, the code is quite confusing. This is what prompted me to do a Rust port instead of a fork; it should not only be easy to find the entry point in code, but reuse parts of the codebase including error handling when one part of the pipeline needs to be adapted/replaced.
3. Speed/Multithreading: After factoring for accuracy and other optimizations, Rusty-SPAdes should be as fast as possible. SPAdes is already known to take an incredibly long time doing sequencing and performing reads and writes to disk to optimize for memory should in theory be slower than keeping everything in memory; optimizations should thus be made to minimize this slowness.


# Usage
For instructions on running this program, clone this repository with:
    git clone https://github.com/aw-andre/rusty-spades
and run
    cargo run -- --help
to compile the program and display commandline flags.

Alternatively, run
    cargo run --release -- --help
to compile the program with maximum optimizations.

Rusty-SPAdes currently supports input of two separate paired-end libraries; support for arbitrary numbers of pairs and interleaved pairs is planned.


# Additional Information
Rusty-SPAdes is a WIP; this repo is an incomplete rewrite of a previous iteration that was abandoned as the foundation for implementation led to excessive consumption of RAM.
