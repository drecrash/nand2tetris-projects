mod parser;
use crate::{codewriter::Codewriter, parser::*};
use std::io::{BufWriter, Write};

mod codewriter;


use std::fs;

fn main() {

    let file_path = "test.txt";
    println!("{}", std::env::current_dir().unwrap().display());

    let mut file_contents = fs::read_to_string(file_path)
        .expect("File read error");

    file_contents = clean_file(file_contents);

    let instruction_set: Vec<&str> = file_contents.lines().collect();

    let output_file = "output.asm";

    let mut codewriter = Codewriter {
        output_file: output_file.to_string(),
        end_count: 0
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