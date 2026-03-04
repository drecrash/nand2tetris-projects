
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
(CALL_0)// Handling function Main.fibonacci 0
(Main.fibonacci)
// Handling 	push argument 0

@0 // get index
D=A // hold index in D-register

@2 // find where the segment starts
A=D+M // set the address to the index + segment location
D = M // store found value

@SP
A=M // go to where stack points
M=D // set value to previously stored value

@SP
M = M+1 // increment stack pointer

// Handling 	push constant 2

@2
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	lt
                @SP
                M=M-1
                A=M // travel to the last added element of stack
                D=M // store value

                @SP
                A=M-1 // travel to second to last item in stack
                D=M-D
                M=-1

                @END_0
                D;JLT // jump to end if less than

                @SP
                A=M-1
                M=0

                (END_0)
                // Handling 	if-goto N_LT_2
@SP
A=M-1
D=M // get top element of stack, hopefully the result of a boolean operation

@SP
M=M-1 // decrement the pointer

@N_LT_2
D;JNE // false is implemented as 0

// Handling 	goto N_GE_2
@N_GE_2
0;JMP
// Handling label N_LT_2
(N_LT_2)
// Handling 	push argument 0

@0 // get index
D=A // hold index in D-register

@2 // find where the segment starts
A=D+M // set the address to the index + segment location
D = M // store found value

@SP
A=M // go to where stack points
M=D // set value to previously stored value

@SP
M = M+1 // increment stack pointer

// Handling 	return

@LCL
D=M

@ENDFRAME // store the endframe location
M=D


@ENDFRAME
D=M-1 // segment pointers are stored one ram address above local

D=D-1
D=D-1
D=D-1
D=D-1 // go back 5 in order to get the return address

A=D
D=M // D is now the return address

@RETURN_ADDR
M=D




@ENDFRAME
A=M // go to the endframe and store the top value
D=M

@THAT
M=D

@ENDFRAME // repeat for this, arg, and local
M=M-1
A=M
D=M

@THIS
M=D

@ENDFRAME
M=M-1
A=M
D=M

@ARG
M=D


@ENDFRAME
M=M-1
A=M
D=M

@LCL
M=D


@SP
A=M-1
D=M // save returned value

@ARG
A=M
M=D // place returned value whereever ARG points

@ARG
A=M+1
D=A

@SP // set stack pointer to right below argument; recycle allocated memory
M=D

@RETURN_ADDR
A=M
0;JMP
// Handling label N_GE_2
(N_GE_2)
// Handling 	push argument 0

@0 // get index
D=A // hold index in D-register

@2 // find where the segment starts
A=D+M // set the address to the index + segment location
D = M // store found value

@SP
A=M // go to where stack points
M=D // set value to previously stored value

@SP
M = M+1 // increment stack pointer

// Handling 	push constant 2

@2
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	sub
                @SP
                M=M-1
                A=M // travel to the last added element of stack
                D=M // store value
                
                @SP
                A=M-1 // travel to second to last added element of stack
                M=M-D // sub

                // Handling 	call Main.fibonacci 1
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
@1 // nArgs
D=D-A
@ARG // arg segment pointer
M=D

// set LCL = SP
@SP
D=M
@1 // lcl segment pointer
M=D
// Handling goto Main.fibonacci
@Main.fibonacci
0;JMP
(CALL_1)// Handling 	push argument 0

@0 // get index
D=A // hold index in D-register

@2 // find where the segment starts
A=D+M // set the address to the index + segment location
D = M // store found value

@SP
A=M // go to where stack points
M=D // set value to previously stored value

@SP
M = M+1 // increment stack pointer

// Handling 	push constant 1

@1
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	sub
                @SP
                M=M-1
                A=M // travel to the last added element of stack
                D=M // store value
                
                @SP
                A=M-1 // travel to second to last added element of stack
                M=M-D // sub

                // Handling 	call Main.fibonacci 1
@CALL_2 // generate a label and save it
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
@1 // nArgs
D=D-A
@ARG // arg segment pointer
M=D

// set LCL = SP
@SP
D=M
@1 // lcl segment pointer
M=D
// Handling goto Main.fibonacci
@Main.fibonacci
0;JMP
(CALL_2)// Handling 	add
                @SP
                M=M-1 // decrement stack pointer
                A=M 
                D=M // save top element of stack

                @SP
                A=M-1 // go to new top element of stack
                M=D+M // add


                // Handling 	return

@LCL
D=M

@ENDFRAME // store the endframe location
M=D


@ENDFRAME
D=M-1 // segment pointers are stored one ram address above local

D=D-1
D=D-1
D=D-1
D=D-1 // go back 5 in order to get the return address

A=D
D=M // D is now the return address

@RETURN_ADDR
M=D




@ENDFRAME
A=M // go to the endframe and store the top value
D=M

@THAT
M=D

@ENDFRAME // repeat for this, arg, and local
M=M-1
A=M
D=M

@THIS
M=D

@ENDFRAME
M=M-1
A=M
D=M

@ARG
M=D


@ENDFRAME
M=M-1
A=M
D=M

@LCL
M=D


@SP
A=M-1
D=M // save returned value

@ARG
A=M
M=D // place returned value whereever ARG points

@ARG
A=M+1
D=A

@SP // set stack pointer to right below argument; recycle allocated memory
M=D

@RETURN_ADDR
A=M
0;JMP
// Handling function Sys.init 0
(Sys.init)
// Handling// Handling 	push constant 4

@4
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling// Handling 	call Main.fibonacci 1
@CALL_3 // generate a label and save it
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
@1 // nArgs
D=D-A
@ARG // arg segment pointer
M=D

// set LCL = SP
@SP
D=M
@1 // lcl segment pointer
M=D
// Handling goto Main.fibonacci
@Main.fibonacci
0;JMP
(CALL_3)// Handling label END
(END)
// Handling 	goto END
@END
0;JMP
