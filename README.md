# Bluebox Assembler

## Description
This is an assembler for the Bluebox computer which I built in The Ultimate Nerd Game (TUNG) It takes in an input `.txt` file and creates an `output.txt` file which has a byte in binary on each line. Your job as the programmer is to then input this into the computer through the memory interface panel. The default program provided, `program.txt`, is a fibonacci program. The circuit board files for the computer are located [here](https://github.com/Broyojo/Capstone-Project).

## How To Run It
To run the program, simply run
```
$ cargo run -- path/to/program.txt
```
The program should run, and assuming you have no errors in your code, it will compile correctly and create `output.txt` in the current directory.

## Future Work
In future work, I plan to investigate inconsistencies in the ISA standard for the Bluebox computer and find a way to input the machine code into the computer easily (probably through the use of an automated clicker script). 

## Examples
```
$ cargo run -- src/program.txt
```