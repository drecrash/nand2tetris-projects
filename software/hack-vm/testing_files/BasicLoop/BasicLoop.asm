// Handling 	push constant 0    

                    
                        @0 
                        D=A

                        @SP
                        A=M
                        M=D

                        @SP
                        M=M+1

                        // Handling 	pop local 0         


                        @0
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

                        // Handling label LOOP

                (LOOP)
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

                        // Handling 	add

                @SP
                M=M-1 // decrement stack pointer
                A=M 
                D=M // save top element of stack

                @SP
                A=M-1 // go to new top element of stack
                M=D+M // add


                // Handling 	pop local 0	        


                        @0
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
                M=M-D // add

                // Handling 	pop argument 0      


                        @0
                        D=A // store index in D-reg

                        @2
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

                        // Handling 	if-goto LOOP        

                @SP
                A=M-1
                D=M // get top element of stack, hopefully the result of a boolean operation

                @SP
                M=M-1 // decrement the pointer

                @LOOP
                D;JNE // false is implemented as 0

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

                        