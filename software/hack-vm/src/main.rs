mod parser;
use crate::{codewriter::Codewriter, parser::*};
use std::io::{BufWriter, Write};
use std::{fs, io, path::Path};

mod codewriter;

// https://medium.com/@trivajay259/listing-files-in-a-directory-in-rust-the-practical-guide-with-python-parallels-25f499e663b6
fn get_files_in_dir(directory: &str)  -> io::Result<Vec<String>> {

    let mut all_files: Vec<String> = fs::read_dir(directory)?
            .map(|res| {
                let entry = res?;
                let path = entry
                    .file_name()                
                    .to_string_lossy() 
                    .into_owned();
                Ok(path)
            })
            .collect::<io::Result<Vec<_>>>()?;

    Ok(all_files)
}
fn main() {

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)
        .expect("failed get user input");

    println!("{user_input}");

    let directory = user_input.as_str().trim();
    let all_files = get_files_in_dir(directory).unwrap();

    let output_file = &format!("{}.asm", directory);

    let mut codewriter = Codewriter {
        output_file: output_file.to_string(),
        end_count: 0,
        call_count: 0,
        toggle_bootstrap: true,
        input_file: all_files[0].clone()
    };

    codewriter.createOutputFile();


    for file_path in all_files{

        if (!file_path.contains(".vm")){
            continue;
        }

        codewriter.input_file = file_path.clone();
        println!("{file_path}");

        let mut file_contents = fs::read_to_string(format!("{}/{}",directory,file_path))
            .expect("File read error");

        file_contents = clean_file(file_contents);

        let instruction_set: Vec<&str> = file_contents.lines().collect();

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