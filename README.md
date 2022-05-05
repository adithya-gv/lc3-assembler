# LC-3 Assembler
This is an LC-3 assembler written in C++.

## What is the LC-3?
The LC-3 is a simple computer procdssor, its corresponding RISC instruction set architecture, and an
assembly language. It's main focus is  to serve as an introductory ISA for undergraduates learning 
low-level computing for the first time and as such focuses on the underlying details rather 
than ease of use in terms of assembly programming. Naturally, this makes it a rather simple assembly 
language to build an assembler for.

## Overview
The following assembler targets LC-3 assembly code and assembles it into the appropriate machine instructions as 
specified by the LC-3 ISA. 

The ISA will be posted here shortly.

## Details 
This is a two-pass assembler. Currently, the assembler performs part of the first pass:
1. It tokenizes each of the lines, flagging any syntax errors if tokens are invalid
2. It builds a symbol table for the assembly code based on any labels. 

The current output file stores the tokenized assembly, removing any directives.

More details coming soon.