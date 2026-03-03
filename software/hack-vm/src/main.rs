mod parser;
use crate::{codewriter::Codewriter, parser::*};
use std::io::{BufWriter, Write};

mod codewriter;


use std::fs;

fn main() {

    let file_path = "input.vm";
    println!("{}", std::env::current_dir().unwrap().display());

    let mut file_contents = fs::read_to_string(file_path)
        .expect("File read error");

    file_contents = clean_file(file_contents);

    let instruction_set: Vec<&str> = file_contents.lines().collect();

    let output_file = "output.asm";

    let mut codewriter = Codewriter {
        output_file: output_file.to_string(),
        end_count: 0,
        call_count: 0,
        toggle_bootstrap: false
    };

    codewriter.createOutputFile();

    for mut line in instruction_set{
        match (Parser::commandType(line.to_string())){
            parser::COMMAND_TYPES::ARITHMETIC =>{
                codewriter.writeArithmetic(line.to_string());
            },
            parser::COMMAND_TYPES::POP =>{
                codewriter.writePushPop(line.to_string());
            },
            parser::COMMAND_TYPES::PUSH =>{
                 codewriter.writePushPop(line.to_string());
            },
            parser::COMMAND_TYPES::CALL =>{
                codewriter.writeCall(line.to_string());
            },
            parser::COMMAND_TYPES::FUNCTION =>{
                codewriter.writeFunction(line.to_string());
            },
            parser::COMMAND_TYPES::RETURN =>{
                codewriter.writeReturn(line.to_string());
            },
            parser::COMMAND_TYPES::GOTO =>{
                codewriter.writeBranch(line.to_string());
            },
            parser::COMMAND_TYPES::LABEL=>{
                codewriter.writeBranch(line.to_string());
            },
            parser::COMMAND_TYPES::IF =>{
                codewriter.writeBranch(line.to_string());
            },       
            _=>{ // Other command types are yet to be implemented
                print!("{:?}", Parser::commandType(line.to_string()));
                panic!("Something has gone awry")
            }
        }

    }



    println!("Hello, world!");
}


fn clean_file(mut contents: String) -> String{
    //contents = contents.replace(" ", ""); // get rid of white space

    let all_lines: Vec<&str> = contents.lines().collect();
    let mut updated_contents: Vec<&str> = Vec::new();

    for mut line in all_lines{ // remove comments
        let mut parts: Vec<&str>;
        parts = line.split("//").collect();
        line = parts[0];

        if !(line == ""){
            updated_contents.push(line);
        }
        
    }


    let mut fin_contents: String = updated_contents.join("\n");
    //fin_contents = fin_contents.replace(" ", "");
    fin_contents

}