use std::fs;

use crate::{compilationengine::CompilationEngine, jacktokenizer::JackTokenizer};

mod jacktokenizer;

mod compilationengine;

fn main() {
    let file_path = "test.txt";

    let mut file_contents = fs::read_to_string(file_path)
        .expect("File read error");

    file_contents = clean_file(file_contents);

    
    let mut jack_tokenizer = JackTokenizer {
        whole_input: file_contents.clone(),
        current_token_index: 0,
        tokens: Vec::new()
    };

    let mut compiler = CompilationEngine {
        file_contents: file_contents,
        output_file: "output.txt".to_string(),
        tokenizer: jack_tokenizer
    };

    compiler.run_compiler();

    /*

    let mut jack_tokenizer = JackTokenizer {
        whole_input: file_contents.clone(),
        current_token_index: 0,
        tokens: Vec::new()
    };

    jack_tokenizer.tokenize(file_contents);

    while (jack_tokenizer.has_more_tokens()){
        println!("{}: {:?}", jack_tokenizer.get_current_token(), jack_tokenizer.get_token_type());
        jack_tokenizer.advance_index();
    }

    */

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