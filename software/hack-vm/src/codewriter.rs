use std::fmt::format;
use std::{collections::HashMap, fs::{self, File}, io::{Write}};
use std::fs::OpenOptions;

use crate::parser::{self, *};

#[derive(Clone)]
pub struct Codewriter{
    pub output_file: String,
    pub end_count: i32 // count of end labels, incremented when new end label is added
}


impl Codewriter{
    pub fn createOutputFile(&mut self){
        let output_file_create = File::create(&self.output_file)
            .expect("error");


        let initial_statement = "
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
        ";


        let mut output_file = OpenOptions::new().read(true).append(true).open(&self.output_file)
            .expect("err");

        output_file.write_all(initial_statement.as_bytes());

    }
    pub fn writeArithmetic(&mut self, line: String){
        let mut output: String = "".to_string();
        let operator = Parser::arg1(&line);

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
                M=M-D // add

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
                output="".to_string();
            }
        }



        let mut output_file = OpenOptions::new().read(true).append(true).open(&self.output_file)
            .expect("err");

        output_file.write_all(output.as_bytes());
    }


    pub fn writePushPop(&self, line: String){

        // need hashmap for segments and corresponding pointers
        let mut segmentMap: HashMap<&str, i32> = HashMap::new();

        segmentMap.insert("constant", 256); // this is just a placeholder, it should never be accessed
        segmentMap.insert("local", 1);
        segmentMap.insert("argument", 2);
        segmentMap.insert("temp", 5);
        segmentMap.insert("this", 3);
        segmentMap.insert("that", 4);

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
                let label = format!("{}.{}",self.output_file,item);
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


        let mut output_file = OpenOptions::new().read(true).append(true).open(&self.output_file)
            .expect("err");

        output_file.write_all(output.as_bytes());

    }

}