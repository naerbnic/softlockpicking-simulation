# Rust implementation of simulation of Softlock Picking scenario

This is a suggestion from the [YouTube video](https://youtu.be/M8C8dHQE2Ro?si=8uoZzFPP9SXqBFtl) from Shoddycast.
In that video, he implemented a pretty trivial version of a simulation of a die roll for 231 attempts
on a four-sided die, and getting over 171 ones. His was written in python, and took several days to run.
This is an attempt to rewrite the basic algorithm with additional optimizations in Rust, a fully-
compiled language.

## Optimizations

- Fully compiled/optimized program

  Rust is compiled to machine code, and takes advantage of the LLVM compiler
  and optimizer to avoid extra instructions that are ultimately unnecessary.
  This can have large effects on the performance of the final program.

- No memory allocation.

  Python allocates memory each time a list or other non-trivial object is
  created. This code avoids any heap memory allocations, depending on only stack-allocated
  memory.

- Use multithreading

  Most machines have multiple CPU cores available, and this task is almost entirely CPU
  bound. We use the `rayon` crate to take advantage of parallelism. I also used
  atomic memory primitives to be able to minimize the amount of overhead due
  to synchronization

## How to run

You can run by installing Rust/Cargo, then running this from a terminal in
the project root directory:

```shell
$ cargo run --release
```

This will download dependencies, compile the program, then run it. The program
is currently configured to run 1,000,000,000 trials, as in the video.

## Results

The max ones I achieved with this program is 102. This program took about 444 seconds,
or 7 minutes, 24 seconds. The output from the run is in the `test_output.txt` file.