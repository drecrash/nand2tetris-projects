/* Routines

CompileClass
CompileClassVarDec
CompileSubroutine

compileParameterList
compileVarDec
compileStatements
compileDo
compileLet
compileWhile
compileReturn
compileIf

CompileExpression
CompileTerm
CompileExpressionList


*/

use std::{fs::{File, OpenOptions}, io::Write};
use crate::jacktokenizer::{JackTokenizer, TOKEN_TYPE};


pub struct CompilationEngine {

    pub file_contents: String,
    pub output_file: String,
    pub tokenizer: JackTokenizer

}

impl CompilationEngine {

    fn loadTokenizer(&mut self){
        let mut jack_tokenizer = JackTokenizer {
            whole_input: self.file_contents.clone(),
            current_token_index: 0,
            tokens: Vec::new()
        };

        jack_tokenizer.tokenize(self.file_contents.clone());

        self.tokenizer = jack_tokenizer;
    }
    
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

    pub fn createOutputFile(&mut self){
        let output_file_create = File::create(&self.output_file)
            .expect("error");
    }


    // Returns current token in the form <terminalType> xxx </terminalType>
    fn format_terminal(&mut self) -> String{

        let mut output = "".to_string();

        match (self.tokenizer.get_token_type()){
            TOKEN_TYPE::KEYWORD =>{
                return format!("<keyword>{}</keyword>\n", self.tokenizer.get_current_token());
            },
            TOKEN_TYPE::IDENTIFIER =>{
                return format!("<identifier>{}</identifier>\n", self.tokenizer.get_current_token());
            },
            TOKEN_TYPE::INT_CONST =>{
                return format!("<integerConstant>{}</integerConstant>\n", self.tokenizer.get_current_token());
            },
            TOKEN_TYPE::STRING_CONST =>{
                return format!("<stringConstant>{}</stringConstant>\n", self.tokenizer.get_current_token());
            },
            TOKEN_TYPE::SYMBOL =>{
                return format!("<symbol>{}</symbol>\n", self.tokenizer.get_current_token());
            },
            _=>{
                return "".to_string();
            }
        }

    }

    pub fn run_compiler(&mut self){
        self.loadTokenizer();
        self.createOutputFile();
        let output = self.CompileClass();

        self.writeToOutput(&output);
    }



    fn CompileClass(&mut self) -> String{
        let mut output = "".to_string();
        output += "<class>\n";
        if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // should be 'class' terminal
            output += "<keyword>\n";

            output += &self.format_terminal();

            self.tokenizer.advance_index();

            if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // should be className

                output += &self.format_terminal();

                self.tokenizer.advance_index();

                if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){ // should be '{'
                    
                    output += &self.format_terminal();

                    self.tokenizer.advance_index();

                    if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // Could be classVarDec or subroutineDec

                        while ((self.tokenizer.get_current_token() == "static") | (self.tokenizer.get_current_token() == "field")){
                            output += &self.compileClassVarDec();
                        };

                        while ((self.tokenizer.get_current_token() == "constructor") | (self.tokenizer.get_current_token() == "function") | (self.tokenizer.get_current_token() == "method")){
                            output += &self.compileSubroutine();
                        }
                        
                        
                    } 

                    if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){ // should be '}'

                        output += &self.format_terminal();

                        self.tokenizer.advance_index();

                    }

                }

            }





        }

        output += "</class>\n";

        return output;
    }


    fn compileClassVarDec(&mut self) -> String{
        let mut output = "".to_string();

        output += "<classVarDec>\n";

        if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // 'static' or 'field'
            output += &self.format_terminal();

            self.tokenizer.advance_index();

            if ((self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD) | (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER)){ // 'int', 'char', 'bool', or className
                output += &self.format_terminal();

                self.tokenizer.advance_index();

                if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname 
                    output += &self.format_terminal();

                    self.tokenizer.advance_index();

                    while (self.tokenizer.get_current_token() == ","){
                        output += &self.format_terminal();
                        self.tokenizer.advance_index();

                        if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname

                            output += &self.format_terminal();
                            self.tokenizer.advance_index();

                        }
                    }

                    if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){// ;

                            output += &self.format_terminal();
                            self.tokenizer.advance_index();

                    }

                }
            

            }

        }

        output += "</classVarDec>\n";


        return output;

    }


    fn compileSubroutine(&mut self) -> String{

        let mut output = "".to_string();

        output += "<subroutineDec>\n";

        if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // 'constructor', 'function', or 'method'
            output += &self.format_terminal();
            self.tokenizer.advance_index();

            if ((self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER) | (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD)){ // 'void', 'int', 'char', 'bool', or className
                output += &self.format_terminal();

                self.tokenizer.advance_index();

                if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ //subroutine name
                    output += &self.format_terminal();

                    self.tokenizer.advance_index();

                    if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){ // should be open parentheses 
                        output += &self.format_terminal();
                        self.tokenizer.advance_index(); 

                        if ((self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER) | (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD)){ //'int', 'char', 'bool', or className
                            output += &self.compileParameterList();

                            if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){ // should be close parentheses
                                output += &self.format_terminal();
                                self.tokenizer.advance_index(); 

                                if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){ // should be '{'
                                    output += &self.format_terminal();
                                    self.tokenizer.advance_index(); 

                                    while (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){
                                        output += &self.compileVarDec();
                                    }
                                    if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){
                                        output += &self.compileStatements();

                                        if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){ // should be '}'
                                            output += &self.format_terminal();
                                            self.tokenizer.advance_index(); 

                                        }
                                    }
                                }
                            }


                        }   

                    }
                }


            }
        }


        output += "</subroutineDec>\n";


        return output;

    }



    fn compileVarDec(&mut self) -> String{
        let mut output = "".to_string();

        output += "<varDec>\n";

        self.tokenizer.advance_index(); 
        if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // should be var
            output += &self.format_terminal();
            self.tokenizer.advance_index(); 


            if ((self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER) | (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD)){ //'int', 'char', 'bool', or className
                output += &self.format_terminal();
                self.tokenizer.advance_index(); 

                if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname
                    output += &self.format_terminal();
                    self.tokenizer.advance_index();       

                    while (self.tokenizer.get_current_token() == ","){
                        output += &self.format_terminal();
                        self.tokenizer.advance_index();

                        if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname

                            output += &self.format_terminal();
                            self.tokenizer.advance_index();

                        }
                    }

                    if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){
                        output += &self.format_terminal();
                        self.tokenizer.advance_index();
                    }              

                }
            }



        }


        output += "</varDec>\n";

        return output;
    }



    fn compileParameterList(&mut self) -> String{
        let mut output = "".to_string();

        output += "<parameterList>\n";

         
        if ((self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER) | (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD)){ //'int', 'char', 'bool', or className
            output += &self.format_terminal();
            self.tokenizer.advance_index();
            if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname
                output += &self.format_terminal();
                self.tokenizer.advance_index();


                while (self.tokenizer.get_current_token() == ","){
                    output += &self.format_terminal();
                    self.tokenizer.advance_index();

                    if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname

                        output += &self.format_terminal();
                        self.tokenizer.advance_index();

                    }
                }
            }
        }

        output += "</parameterList>\n";

        return output;



    }


    fn compileStatements(&mut self) -> String{
        let mut output = "".to_string();

        output += "<statements>\n";


        while (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){

            output += "<statement>\n";
            

            match(self.tokenizer.get_current_token().as_str()){
                "let" => {
                    output += &self.compileLet();
                },
                "if" => {
                    output += &self.compileIf();
                },
                "while" => {
                    output += &self.compileWhile();
                },
                "do" => {
                    output += &self.compileDo();
                },
                "return" =>{
                    output += &self.compileReturn();
                }
                _=>{
                    output += "";
                }
            }

            output += "</statement>\n";

        }

        return output
    }

    fn compileLet(&mut self) -> String{
        let mut output = "".to_string();
        

        output += "<letStatement>\n";

        if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // let
            output += &self.format_terminal();
            self.tokenizer.advance_index();

            if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname
                output += &self.format_terminal();
                self.tokenizer.advance_index();

                if (self.tokenizer.get_current_token() == "["){
                    output += &self.format_terminal();
                    self.tokenizer.advance_index();        

                    output += &self.compileExpression();


                    if (self.tokenizer.get_current_token() == "]"){
                        output += &self.format_terminal();
                        self.tokenizer.advance_index();  
                    }   
                }

                if (self.tokenizer.get_current_token() == "="){
                    output += &self.format_terminal();
                    self.tokenizer.advance_index();   

                    output += &self.compileExpression();

                    if (self.tokenizer.get_current_token() == ";"){
                        output += &self.format_terminal();
                        self.tokenizer.advance_index();   
                    }
                }
            }

        }


        output += "</letStatement>\n";

        return output;
    }


    fn compileIf(&mut self) -> String{
        let mut output = "".to_string();
        //
        return output;
    }


    fn compileWhile(&mut self) -> String{
        let mut output = "".to_string();
        //
        return output;
    }


    fn compileDo(&mut self) -> String{
        let mut output = "".to_string();
        //
        return output;
    }


    fn compileReturn(&mut self) -> String{
        let mut output = "".to_string();
        //
        return output;
    }

    fn compileExpression(&mut self) -> String{
        let mut output = "".to_string();
        //
        return output;    
    }
}