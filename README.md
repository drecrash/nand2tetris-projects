# Nand2Tetris Part 2 \- Software

## Overview

This repo holds the projects that I’ve built for Part 2 of the NandToTetris course, which focuses on the software elements of system architecture.  
I chose to build the projects in Rust to get more exposure to low-level languages, learn more about memory management, and because I thought it would be fun to experiment with a new language.

## The Assembler

I loosely followed the NandToTetris API guidelines when designing and writing both the assembler and the virtual machine translator. I took many artistic liberties in organizing the various functions and structures.

The assembler utilizes a multiple-pass approach. It initially cleans the file by removing whitespace and comments. It then scans the file for labels, and then once more for variables. The latter two passes build a symbol table that is referred to throughout the final translation of assembly to binary.

Any statement of the form @Xxx referred to, without a corresponding (Xxx) label, is deemed a user-defined variable.

Within the parser, A-instructions and L-instructions (technically pseudoinstructions) have their symbols extracted, whereas C-instructions have their comp, dest, and jump fields extracted.

The coder is simple, and directly translates the fields provided by the parser into binary using the provided NandToTetris tables

A custom binary-to-decimal translator function is used to process A-instructions.

![Hack Specifications for assembly to binary](https://github.com/drecrash/nand2tetris-projects/blob/master/images/hackBinaryTable.png)

## The Virtual Machine Translator

*Please note that Project 8 is **in progress**; the VM Translator is still being built.*

The VM Translator translates virtual machine language into assembly language, which can then be processed by the assembler into binary. 

Different portions of memory are allocated to various segments, and pointers at the top of the memory (RAM\[0\] through RAM\[4\]) point to where these portions of memory are located. Note that RAM\[5\] through RAM\[12\] are apportioned to fixed memory.

The most important portion of memory is the stack, where the values actively being worked with are stored. Arithmetic and Boolean logic are done by pushing values onto the stack and calling the appropriate functions. Elements can also be popped off the stack and pushed onto the other memory portions.

Each instruction (push, pop, and the various operators) has a specific set of assembly code associated with it. I manually derived the appropriate assembly for each instruction, and implemented them in the Codewriter.

The “static” segment is especially interesting, as index ‘i’ of the static segment is stored as the label (Xxx.i), where Xxx is the name of the output file. (Note: this should be changed to the input file, I have it set as the output file for the sake of simplicity while development is in progress).

![Hack Specifications for memory segmentation](https://github.com/drecrash/nand2tetris-projects/blob/master/images/hackRAM.png)