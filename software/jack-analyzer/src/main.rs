use std::{fs, io};

use crate::{compilationengine::CompilationEngine, jacktokenizer::JackTokenizer};

mod jacktokenizer;

mod compilationengine;

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

    for file_path in all_files{


        if (!file_path.contains(".jack")){
            continue;
        }

        let mut file_path_parts: Vec<&str>;
        file_path_parts = file_path.split(".").collect();
        let file_path_root = file_path_parts[0];

        let mut file_contents = fs::read_to_string(format!("{}/{}",directory,file_path))
            .expect("File read error");

        let output_file = format!("{}.xml", file_path_root);

        file_contents = clean_file(file_contents);

        
        let mut jack_tokenizer = JackTokenizer {
            whole_input: file_contents.clone(),
            current_token_index: 0,
            tokens: Vec::new()
        };

        let mut compiler = CompilationEngine {
            file_contents: file_contents,
            output_file: output_file.to_string(),
            tokenizer: jack_tokenizer
        };

        compiler.run_compiler();
    }


}

// TO DO: need to remove multi-line comments
fn clean_file(mut contents: String) -> String{

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




    let mut joined_contents: String = updated_contents.join("\n");


    
    let mut fin_contents: String = joined_contents;

    fin_contents

}


/* NOTES:

Jack Non-Terminals:
- class, classVarDec, subroutineDec, parameterList, subroutineBody, varDec;
- statements, whileSatement, ifStatement, returnStatement, letStatement, doStatement;
- expression, term, expressionList.

Jack Terminals:
- keyword, symbol, integerConstant, stringConstant, or identifier


*/