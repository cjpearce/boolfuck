# Boolfuck
This is a simple Boolfuck interpreter that I implemented in Rust as a solution for a 3kyu [exercise on codewars](https://www.codewars.com/kata/5861487fdb20cff3ab000030).

This solution only uses one bit of memory for each "Cell" of memory described in the language specification, making it more complex than most.

The interpreter's "infinite" tape is stored in a single piece of memory using a Rust `Vec`, with addresses above the start position stored across even indices, and those below the start position stored across odd indices. This was done mostly as an excercise and not for any specific performance benefits.

My Codewars submission can be found here: https://www.codewars.com/kata/reviews/599110e4800bb6b79f001c8a/groups/5dcf25612d2c4c0001211d60
