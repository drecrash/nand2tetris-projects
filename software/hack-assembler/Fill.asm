// Handling 	push argument 1         

                        
                        @1 // get index
                        D=A // hold index in D-register

                        @2 // find where the segment starts
                        A=D+M // set the address to the index + segment location
                        D = M // store found value

                        @SP
                        A=M // go to where stack points
                        M=D // set value to previously stored value

                        @SP
                        M = M+1 // increment stack pointer

                        // Handling 	pop pointer 1           


                        @SP
                        M=M-1
                        A=M
                        D=M

                        @4
                        M=D

                        // Handling 	push constant 0         

                    
                        @0 
                        D=A

                        @SP
                        A=M
                        M=D

                        @SP
                        M=M+1

                        // Handling 	pop that 0              


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

                        // Handling 	push constant 1   

                    
                        @1 
                        D=A

                        @SP
                        A=M
                        M=D

                        @SP
                        M=M+1

                        // Handling 	pop that 1              


                        @1
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

                        // Handling 	if-goto COMPUTE_ELEMENT 

                @SP
                A=M-1
                D=M // get top element of stack, hopefully the result of a boolean operation

                @SP
                M=M-1 // decrement the pointer

                @COMPUTE_ELEMENT
                D;JNE // false is implemented as 0

                // Handling 	goto END                

                @END
                0;JMP
                // Handling label COMPUTE_ELEMENT

                (COMPUTE_ELEMENT)
                // Handling     
// Handling 	push that 0

                        
                        @0 // get index
                        D=A // hold index in D-register

                        @4 // find where the segment starts
                        A=D+M // set the address to the index + segment location
                        D = M // store found value

                        @SP
                        A=M // go to where stack points
                        M=D // set value to previously stored value

                        @SP
                        M = M+1 // increment stack pointer

                        // Handling 	push that 1

                        
                        @1 // get index
                        D=A // hold index in D-register

                        @4 // find where the segment starts
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


                // Handling 	pop that 2


                        @2
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

                        // Handling 	
// Handling 	push pointer 1

                    
                        @4 
                        D=M

                        @SP
                        A=M
                        M=D

                        @SP
                        M=M+1

                        // Handling 	push constant 1

                    
                        @1 
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


                // Handling 	pop pointer 1 


                        @SP
                        M=M-1
                        A=M
                        D=M

                        @4
                        M=D

                        // Handling 	
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

                        // Handling 	goto LOOP

                @LOOP
                0;JMP
                // Handling label END

                (END)
                