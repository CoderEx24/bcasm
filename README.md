# bcasm

This is an assmebler the produces machine code for the Basic Computer,
described in Morris Mano's Computer System Architecture textbook, from its symbolic form

## Assembly File Structure
The assembly file needs to follow the following template
```asm
// this is a comment
data:
    a 12  // a label with the value of decimal 12
    b 012 // a label with the value of octal 12 (decimal 9)
    c 0xF // a label with the value of hex A (decimal 15)
    d 'A' // a label with the ASCII value of 'A' (decimal 65)

text:
    lda a   // load label a into accumlator
    add b   // add b to accumlator
    sta c i // store the value in accumlator into the address in c (i for indirect addressing)
    lda d   // load d into accumlator
    out     // output accumlator
    hlt     // halt
```
In essence, the assembly file needs to start with a data section, 
which contains all the constants used in the program.
the labels can contain only alphanumeric letters and underscores and cannot begin with a number.
Following the data section is the text section, which contains the code of the program.

the total size of the program may not exceed 8192 bytes, due to the design of the basic computer.

## Ouput Format
The assembler outputs machine code in the following format
```
branch to start of program
--------------------------

      Data Section

--------------------------

        Program

--------------------------
```

the first 2 bytes of the program is an unconditional branch to the start of the program.
Following it is the values from the data section, following that is the program.
