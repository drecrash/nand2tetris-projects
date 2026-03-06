use std::fmt::format;
use std::{collections::HashMap, fs::{self, File}, io::{Write}};
use std::fs::OpenOptions;

use crate::parser::{self, *};

#[derive(Clone)]
pub struct Codewriter{
    pub output_file: String,
    pub end_count: i32, // count of end labels, incremented when new end label is added
    pub call_count: i32, // count of how many times functions are called, used to instantiate return labels, incremented on each call
    pub toggle_bootstrap: bool, // if testing a single file, don't use the bootstrap code
    pub input_file: String // current file being read
}


impl Codewriter{


    fn trimOutput(&self, mut contents: &str) -> String{
        let all_lines: Vec<&str> = contents.lines().collect();
        let mut updated_contents: Vec<&str> = Vec::new();

        for mut line in all_lines{ // remove comments
            line = line.trim();
            updated_contents.push(line);
        }


        let mut fin_contents: String = updated_contents.join("\n");

        return fin_contents
    }

    pub fn writeToOutput(&self, mut output: &str){
        println!("Writing: {output}");
        let mut output_file = OpenOptions::new().read(true).append(true).open(&self.output_file)
            .expect("err");

        let written_output = self.trimOutput(output);

        output_file.write_all(&written_output.as_bytes());
    }

    pub fn buildSegmentMap(&self) -> HashMap<&str, i32>{
        let mut segmentMap: HashMap<&str, i32> = HashMap::new();

        segmentMap.insert("constant", 256); // this is just a placeholder, it should never be accessed
        segmentMap.insert("static", 100); // this is just a placeholder, it should never be accessed
        segmentMap.insert("local", 1); // by default, LCL is set to RAM[1]
        segmentMap.insert("argument", 2); // default is RAM[2]
        segmentMap.insert("temp", 5);
        segmentMap.insert("this", 3);
        segmentMap.insert("that", 4);

        return segmentMap;
    }

    pub fn createOutputFile(&mut self){
        let output_file_create = File::create(&self.output_file)
            .expect("error");


        let initial_statement = "
        @256 // initialize stack pointer
        D=A
        @0
        M=D
        ";

        if (self.toggle_bootstrap){
        
            self.writeToOutput(initial_statement);

            self.writeCall("call Sys.init 0".to_string());
            

        }

    }



    pub fn writeArithmetic(&mut self, line: String){
        let mut output: String = "".to_string();
        let operator = Parser::arg1(&line);

        
        self.writeToOutput(&format!("// Handling {line}\n"));

        match (operator){
            "add"=>{

                output = "
                @SP
                M=M-1 // decrement stack pointer
                A=M 
                D=M // save top element of stack

                @SP
                A=M-1 // go to new top element of stack
                M=D+M // add


                ".to_string();

                

            },
            "sub"=>{
                output = "
                @SP
                M=M-1
                A=M // travel to the last added element of stack
                D=M // store value
                
                @SP
                A=M-1 // travel to second to last added element of stack
                M=M-D // sub

                ".to_string();
                

            },
            "neg"=>{
                output = "
                @SP
                A=M-1
                M=-M    
                ".to_string();
                
            },
            "eq"=>{
                output = format!("
                @SP
                M=M-1
                A=M // travel to the last added element of stack
                D=M // store value

                @SP
                A=M-1 // travel to second to last item in stack
                D=D-M
                M=0 // set to 0

                @END_{}
                D;JNE // jump if not equal to zero

                @SP
                A=M-1 // travel to top of stack
                M=-1 // set to true

                (END_{})
                ", self.end_count, self.end_count).to_string();

                self.end_count = self.end_count + 1;

                
            },
            "gt" =>{
                output = format!("
                @SP
                M=M-1
                A=M // travel to the last added element of stack
                D=M // store value

                @SP
                A=M-1 // travel to second to last item in stack
                D=M-D
                M=-1

                @END_{}
                D;JGT // jump to end if greater than

                @SP
                A=M-1
                M=0

                (END_{})
                ", self.end_count, self.end_count).to_string();
                self.end_count = self.end_count + 1;

                                
            },
            "lt" => {
                output = format!("
                @SP
                M=M-1
                A=M // travel to the last added element of stack
                D=M // store value

                @SP
                A=M-1 // travel to second to last item in stack
                D=M-D
                M=-1

                @END_{}
                D;JLT // jump to end if less than

                @SP
                A=M-1
                M=0

                (END_{})
                ", self.end_count, self.end_count).to_string();
                self.end_count = self.end_count + 1;

                                  
            },
            "and" => {
                output = "      
                @SP
                M=M-1
                A=M
                D=M
                
                @SP
                A=M-1 
                M=D&M 

                ".to_string();

                
            },
            "or"=>{
                output = "      
                @SP
                M=M-1
                A=M
                D=M
                
                @SP
                A=M-1 
                M=D|M 

                ".to_string();
             
            },
            "not"=>{
                output = "
                @SP
                A=M-1
                M=!M
                ".to_string();

            }


            _=>{
                println!("OUTPUT FAIL: {operator}");
                output="".to_string();
            }
        }



        let mut output_file = OpenOptions::new().read(true).append(true).open(&self.output_file)
            .expect("err");

        output_file.write_all(output.as_bytes());
    }


    pub fn writePushPop(&self, line: String){

        self.writeToOutput(&format!("// Handling {line}\n"));

        // need hashmap for segments and corresponding pointers
        let mut segmentMap = self.buildSegmentMap();

        let mut segment = Parser::arg1(&line);
        let mut item = Parser::arg2(&line);

        // adhere to 'push/pop pointer 0/1 notation
        if segment == "pointer"{

             // this is a hacky workaround that prevents writing more code
             // "temp" maps to 5 + item, so if you do 5 + (-2) you go to 3, and 5 + (-1) goes to 1
            if (item == "0"){
                segment = "temp";
                item = "-2";
            }
            if (item == "1"){
                segment = "temp";
                item = "-1";
            }
        }
        println!("seg: {segment}");
        let segment_pointer = segmentMap.get(segment)
            .expect("Error retrieving segment pointer");

        
        let command_type = Parser::commandType(line.clone());

        let mut output: String = String::new();


        match (segment) {

            "temp"=>{
                match (command_type){
                    COMMAND_TYPES::PUSH => {
                        output = format!("
                    
                        @{} 
                        D=M

                        @SP
                        A=M
                        M=D

                        @SP
                        M=M+1

                        ",(item.parse::<i32>().unwrap()+5));
                    },
                    COMMAND_TYPES::POP =>{
                        output = format!("

                        @SP
                        M=M-1
                        A=M
                        D=M

                        @{}
                        M=D

                        ",(item.parse::<i32>().unwrap()+5));                
                    }
                    _=>{
                        output = "".to_string();
                    }
                }             
            }

            "constant"=>{
                match (command_type){
                    COMMAND_TYPES::PUSH => {
                        output = format!("
                    
                        @{} 
                        D=A

                        @SP
                        A=M
                        M=D

                        @SP
                        M=M+1

                        ",item);
                    },
                    COMMAND_TYPES::POP =>{ // this wont actually work for constant
                        output = format!("

                        @SP
                        M=M-1
                        A=M
                        D=M

                        @{}
                        M=D
                        // THIS SHOULD NEVER BE SEEN

                        ",item);                
                    }
                    _=>{
                        output = "".to_string();
                    }
                }
       
            }

            "static" =>{
                let label = format!("{}.{}",self.input_file,item);
                match (command_type){
                    COMMAND_TYPES::PUSH => {
                        output = format!("
                        
                        @{} 
                        D=M

                        @SP
                        A=M
                        M=D

                        @SP
                        M=M+1

                        ",label);
                    },
                    COMMAND_TYPES::POP =>{
                        output = format!("

                        @SP
                        M=M-1
                        A=M
                        D=M

                        @{}
                        M=D

                        ",label);                
                    }
                    _=>{
                        output = "".to_string();
                    }
                }





            },
            _=>{
                match (command_type){
                    COMMAND_TYPES::PUSH => {
                        output = format!("
                        
                        @{} // get index
                        D=A // hold index in D-register

                        @{} // find where the segment starts
                        A=D+M // set the address to the index + segment location
                        D = M // store found value

                        @SP
                        A=M // go to where stack points
                        M=D // set value to previously stored value

                        @SP
                        M = M+1 // increment stack pointer

                        ",item,segment_pointer);
                    },
                    COMMAND_TYPES::POP =>{
                        output = format!("

                        @{}
                        D=A // store index in D-reg

                        @{}
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

                        ",item,segment_pointer);                
                    }
                    _=>{
                        output = "".to_string();
                    }
                }                
            }


        }


        self.writeToOutput(&output);

    }


    pub fn writeBranch(&self, line: String){

        self.writeToOutput(&format!("// Handling {line}\n"));

        
        let command_type = Parser::commandType(line.clone());
        let mut output: String = String::new();

        let label = Parser::arg1(&line);

        match (command_type){
            COMMAND_TYPES::GOTO => {


                output = format!("
                @{}
                0;JMP
                ", label)

            },
            COMMAND_TYPES::LABEL => {
                output = format!("
                ({})
                ", label)
            },
            COMMAND_TYPES::IF =>{
                output = format!("
                @SP
                A=M-1
                D=M // get top element of stack, hopefully the result of a boolean operation

                @SP
                M=M-1 // decrement the pointer

                @{}
                D;JNE // false is implemented as 0

                ", label)
            }
            _=>{panic!("Something has gone awry")}
        }



        self.writeToOutput(&output);
        
    }


    pub fn writeCall(&mut self, line: String){

        self.writeToOutput(&format!("// Handling {line}\n"));

        let mut reposition_pointers_output: String = String::new();
        let mut save_state_output: String = String::new();

        let n_args = Parser::arg2(&line);
        
        let function_name = Parser::arg1(&line);

        let segment_map = self.buildSegmentMap();

        // need to save return address
        // save segment pointer states
        // reposition 'arg'
        // reposition LCL


        // save the current state (stack pointer, local pointer, arg pointer, this, that)
        // TODO: make the label follow convention
        save_state_output += &format!("
        @CALL_{} // generate a label and save it
        D=A

        @SP // push current address onto the stack
        A=M
        M=D
        
        @SP // increment stack pointer
        M=M+1
        ", self.call_count); 

        self.writeToOutput(&save_state_output);

        save_state_output = "".to_string();

        // TODO: consolidate the following four statements
        save_state_output += &format!("
        @LCL
        D=M

        @SP
        A=M
        M=D

        @SP
        M=M+1
        ");


        save_state_output += &format!("
        @ARG
        D=M

        @SP
        A=M
        M=D

        @SP
        M=M+1
        ");


        save_state_output += &format!("
        @THIS
        D=M

        @SP
        A=M
        M=D

        @SP
        M=M+1
        ");


        save_state_output += &format!("
        @THAT
        D=M

        @SP
        A=M
        M=D

        @SP
        M=M+1
        ");

        self.writeToOutput(&save_state_output);



        // reposition the pointers
        println!("n_args: {line}");
        reposition_pointers_output += &format!("
        // set ARG = SP - 5 - nArgs
        @SP
        D=M
        @5
        D=D-A
        @{} // nArgs
        D=D-A
        @ARG // arg segment pointer
        M=D
        ", n_args.parse::<i32>().unwrap());



        reposition_pointers_output += &format!("
        // set LCL = SP
        @SP
        D=M
        @LCL // lcl segment pointer
        M=D
        ");

        self.writeToOutput(&reposition_pointers_output);

        self.writeBranch(format!("goto {}", function_name));

        self.writeToOutput(&format!("(CALL_{})", self.call_count));

        self.call_count = self.call_count + 1;
    }



    pub fn writeReturn(&self, line: String){
        self.writeToOutput(&format!("// Handling {line}\n"));

        let mut output: String = String::new();
        /* STEPS
        - Store location of endframe (wherever local points)
        - Store return address (stored 5 addresses above ENDFRAME)
        - Save value returned by function (the top of the current stack)
        - Replace function arguments in RAM with the value returned by function
        - Recycle memory allocated to function by setting stack pointer right below argument

        - Restore caller's segment pointers
        
        - Jump to return address
        
         */
        
        output += "

        @LCL
        D=M

        @ENDFRAME // store the endframe location
        M=D

        ";


        
        // save return address

        output += "
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

        ";


        output += "
        @SP
        A=M-1
        D=M // save returned value

        @ARG
        A=M
        M=D // place returned value whereever ARG points
        ";

        output += "
        @ARG
        A=M+1
        D=A

        @SP // set stack pointer to right below argument; recycle allocated memory
        M=D
        ";

        // restore segment pointers
        output += "


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

        ";



        output += "
        @RETURN_ADDR
        A=M
        0;JMP
        ";

        self.writeToOutput(&output);

        

        
    }


    pub fn writeFunction(&self, line: String){
        self.writeToOutput(&format!("// Handling {line}\n\n"));

        let function_name = Parser::arg1(&line);
        println!("FUNCTION NAME: {function_name}");

        let n_lcl = (Parser::arg2(&line)).parse::<i32>().unwrap();

            self.writeToOutput(&format!("({})\n\n", function_name));
            
            for n in 0..n_lcl{
                self.writePushPop("push constant 0\n".to_string());

            }


        
    }
}