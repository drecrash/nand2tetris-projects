
        @256 // initialize stack pointer
        D=A
        @0
        M=D
        
                        
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

                        
                    
                        @2 
                        D=A

                        @SP
                        A=M
                        M=D

                        @SP
                        M=M+1

                        
                @N_LT_2
                0;JMP
                
                @N_GE_2
                0;JMP
                
                (N_LT_2)
                
                        
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

                        
                (N_GE_2)
                
                        
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

                        
                    
                        @2 
                        D=A

                        @SP
                        A=M
                        M=D

                        @SP
                        M=M+1

                        
                        
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

                        
                    
                        @1 
                        D=A

                        @SP
                        A=M
                        M=D

                        @SP
                        M=M+1

                        