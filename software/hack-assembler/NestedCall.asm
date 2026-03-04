
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
(CALL_0)// Handling function Sys.init 0
(Sys.init)
// Handling 	push constant 4000

@4000
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	pop pointer 0

@SP
M=M-1
A=M
D=M

@3
M=D

// Handling 	push constant 5000

@5000
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	pop pointer 1

@SP
M=M-1
A=M
D=M

@4
M=D

// Handling 	call Sys.main 0
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
// Handling goto Sys.main
@Sys.main
0;JMP
(CALL_1)// Handling 	pop temp 1

@SP
M=M-1
A=M
D=M

@6
M=D

// Handling 	label LOOP
(LOOP)
// Handling 	goto LOOP
@LOOP
0;JMP
// Handling function Sys.main 5
(Sys.main)
// Handling push constant 0


@0
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling push constant 0


@0
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling push constant 0


@0
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling push constant 0


@0
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling push constant 0


@0
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	push constant 4001

@4001
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	pop pointer 0

@SP
M=M-1
A=M
D=M

@3
M=D

// Handling 	push constant 5001

@5001
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	pop pointer 1

@SP
M=M-1
A=M
D=M

@4
M=D

// Handling 	push constant 200

@200
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	pop local 1

@1
D=A // store index in D-reg

@1
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

// Handling 	push constant 40

@40
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	pop local 2

@2
D=A // store index in D-reg

@1
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

// Handling 	push constant 6

@6
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	pop local 3

@3
D=A // store index in D-reg

@1
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

// Handling 	push constant 123

@123
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	call Sys.add12 1
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
// Handling goto Sys.add12
@Sys.add12
0;JMP
(CALL_2)// Handling 	pop temp 0

@SP
M=M-1
A=M
D=M

@5
M=D

// Handling 	push local 0

@0 // get index
D=A // hold index in D-register

@1 // find where the segment starts
A=D+M // set the address to the index + segment location
D = M // store found value

@SP
A=M // go to where stack points
M=D // set value to previously stored value

@SP
M = M+1 // increment stack pointer

// Handling 	push local 1

@1 // get index
D=A // hold index in D-register

@1 // find where the segment starts
A=D+M // set the address to the index + segment location
D = M // store found value

@SP
A=M // go to where stack points
M=D // set value to previously stored value

@SP
M = M+1 // increment stack pointer

// Handling 	push local 2

@2 // get index
D=A // hold index in D-register

@1 // find where the segment starts
A=D+M // set the address to the index + segment location
D = M // store found value

@SP
A=M // go to where stack points
M=D // set value to previously stored value

@SP
M = M+1 // increment stack pointer

// Handling 	push local 3

@3 // get index
D=A // hold index in D-register

@1 // find where the segment starts
A=D+M // set the address to the index + segment location
D = M // store found value

@SP
A=M // go to where stack points
M=D // set value to previously stored value

@SP
M = M+1 // increment stack pointer

// Handling 	push local 4

@4 // get index
D=A // hold index in D-register

@1 // find where the segment starts
A=D+M // set the address to the index + segment location
D = M // store found value

@SP
A=M // go to where stack points
M=D // set value to previously stored value

@SP
M = M+1 // increment stack pointer

// Handling 	add
                @SP
                M=M-1 // decrement stack pointer
                A=M 
                D=M // save top element of stack

                @SP
                A=M-1 // go to new top element of stack
                M=D+M // add


                // Handling 	add
                @SP
                M=M-1 // decrement stack pointer
                A=M 
                D=M // save top element of stack

                @SP
                A=M-1 // go to new top element of stack
                M=D+M // add


                // Handling 	add
                @SP
                M=M-1 // decrement stack pointer
                A=M 
                D=M // save top element of stack

                @SP
                A=M-1 // go to new top element of stack
                M=D+M // add


                // Handling 	add
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



@ENDFRAME
M=M-1
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


@RETURN_ADDR
A=M
0;JMP
// Handling function Sys.add12 0
(Sys.add12)
// Handling 	push constant 4002

@4002
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	pop pointer 0

@SP
M=M-1
A=M
D=M

@3
M=D

// Handling 	push constant 5002

@5002
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	pop pointer 1

@SP
M=M-1
A=M
D=M

@4
M=D

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

// Handling 	push constant 12

@12
D=A

@SP
A=M
M=D

@SP
M=M+1

// Handling 	add
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



@ENDFRAME
M=M-1
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


@RETURN_ADDR
A=M
0;JMP
