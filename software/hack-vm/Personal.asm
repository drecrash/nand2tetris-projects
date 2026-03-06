
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

@LCL
D=M

@SP
A=M
M=D

@SP
M=M+1

@ARG
D=M

@SP
A=M
M=D

@SP
M=M+1

@THIS
D=M

@SP
A=M
M=D

@SP
M=M+1

@THAT
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
@LCL // lcl segment pointer
M=D
// Handling goto Sys.init
@Sys.init
0;JMP
(CALL_0)// Handling function Main.main 0
(Main.main)
// Handling// Handling     push constant 16384

@16384
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling// Handling// Handling// Handling     pop pointer 1

@SP
M=M-1
A=M
D=M

@4
M=D

// Handling// Handling// Handling// Handling     push constant 1

@1
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling     neg
                @SP
                A=M-1
                M=-M    
                // Handling// Handling// Handling// Handling     pop that 0

@0
D=A // store index in D-reg

@4
D=M+D // store address of index i in segment in D-reg

@13 // general purpose register
M=D // store address of index i in segment in GPR

@SP
M=M-1 // decrement stack pointer
A=M // go to where stack pointer points
D=M // store top value of stack in D-reg

@13
A=M // travel to index i of segment
M=D // set value

// Handling// Handling label END
(END)
// Handling     goto END
@END
0;JMP
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

@LCL
D=M

@SP
A=M
M=D

@SP
M=M+1

@ARG
D=M

@SP
A=M
M=D

@SP
M=M+1

@THIS
D=M

@SP
A=M
M=D

@SP
M=M+1

@THAT
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
@LCL // lcl segment pointer
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
