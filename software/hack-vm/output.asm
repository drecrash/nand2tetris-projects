
@256 // initialize stack pointer
D=A
@0
M=D

@300 // initialize local pointer
D=A
@1
M=D

@400 // initialize argument pointer
D=A
@2
M=D

@3000 // initialize this pointer
D=A
@3
M=D

@3010 // initialize that pointer
D=A
@4
M=D


@7 
D=A

@SP
A=M
M=D

@SP
M=M+1

@8 
D=A

@SP
A=M
M=D

@SP
M=M+1

@SP
M=M-1 // decrement stack pointer
A=M 
D=M // save top element of stack

@SP
A=M-1 // go to new top element of stack
M=D+M // add


                