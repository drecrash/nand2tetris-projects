use std::fs;

use crate::jacktokenizer::JackTokenizer;

mod jacktokenizer;

fn main() {
    let file_path = "test.txt";

    let mut file_contents = fs::read_to_string(file_path)
        .expect("File read error");

    file_contents = clean_file(file_contents);

    let jack_tokenizer = JackTokenizer {
        whole_input: file_contents.clone(),
        current_token: String::new()
    };

    jack_tokenizer.tokenize(file_contents);

    //let instruction_set: Vec<&str> = file_contents.lines().collect();
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


/* NOTES:

Jack Non-Terminals:
- class, classVarDec, subroutineDec, parameterList, subroutineBody, varDec;
- statements, whileSatement, ifStatement, returnStatement, letStatement, doStatement;
- expression, term, expressionList.

Jack Terminals:
- keyword, symbol, integerConstant, stringConstant, or identifier




*/