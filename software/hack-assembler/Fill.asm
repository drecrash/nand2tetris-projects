// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

//// Replace this comment with your code.





(LOOPEMPTY)


@row 
A = M
M = 0

@row
D = M
@1
D = D + A 
@row
M = D 

@KBD
D = M

@STARTLOOPFILL
D;JGT

@LOOPEMPTY
D;JLE


(STARTLOOPFILL)
@SCREEN 
D=A
@row 
M=D

@LOOPFILL
0;JMP

(STARTLOOPEMPTY)
@SCREEN 
D=A
@row 
M=D

@LOOPEMPTY

0;JMP
(LOOPFILL)


@row 
A = M
M = -1 

@row
D = M
@1
D = D + A 
@row
M = D 


@KBD
D = M

@STARTLOOPEMPTY
D;JLE

@LOOPFILL
D;JGT

@LOOPEMPTY
D;JMP
