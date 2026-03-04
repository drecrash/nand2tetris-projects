
@256 // initialize stack pointer
D=A
@0
M=D
// Handling call Sys.init 0
@CALL_0 // generate a label and save it
D=A

@SP // push current address onto the stack
A=M
M=D

@SP // increment stack pointer
M=M+1

@1
D=M

@SP
A=M
M=D

@SP
M=M+1

@2
D=M

@SP
A=M
M=D

@SP
M=M+1

@3
D=M

@SP
A=M
M=D

@SP
M=M+1

@4
D=M

@SP
A=M
M=D

@SP
M=M+1

// set ARG = SP - 5 - nArgs
@SP
D=M
@5
D=D-A
@0 // nArgs
D=D-A
@ARG // arg segment pointer
M=D

// set LCL = SP
@SP
D=M
@1 // lcl segment pointer
M=D
// Handling goto Sys.init
@Sys.init
0;JMP
(CALL_0)// Handling function Main.main 0
(Main.main)
// Handling     push constant 7

@7
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling     push constant 10

@10
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling     add
                @SP
                M=M-1 // decrement stack pointer
                A=M 
                D=M // save top element of stack

                @SP
                A=M-1 // go to new top element of stack
                M=D+M // add


                // Handling function Sys.init 0
(Sys.init)
// Handling 	call Main.main 0
@CALL_1 // generate a label and save it
D=A

@SP // push current address onto the stack
A=M
M=D

@SP // increment stack pointer
M=M+1

@1
D=M

@SP
A=M
M=D

@SP
M=M+1

@2
D=M

@SP
A=M
M=D

@SP
M=M+1

@3
D=M

@SP
A=M
M=D

@SP
M=M+1

@4
D=M

@SP
A=M
M=D

@SP
M=M+1

// set ARG = SP - 5 - nArgs
@SP
D=M
@5
D=D-A
@0 // nArgs
D=D-A
@ARG // arg segment pointer
M=D

// set LCL = SP
@SP
D=M
@1 // lcl segment pointer
M=D
// Handling goto Main.main
@Main.main
0;JMP
(CALL_1)// Handling     pop static 0

@SP
M=M-1
A=M
D=M

@Sys.vm.0
M=D

// Handling label END
(END)
// Handling 	goto END
@END
0;JMP
