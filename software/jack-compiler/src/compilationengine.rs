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

use std::{fs::{File, OpenOptions}, hash::Hash, io::Write};
use crate::{jacktokenizer::{JackTokenizer, TOKEN_TYPE}, symboltable};
use crate::symboltable::{SymbolTable};


pub struct CompilationEngine {

    pub file_contents: String,
    pub output_file: String,
    pub vm_output_file: String,
    pub tokenizer: JackTokenizer,
    pub symbol_table: SymbolTable,
    pub cur_class_name: String
    

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

    fn initializeSymbolTables(&mut self){
        let mut symbol_table = SymbolTable::new();
        symbol_table.clear_class_scope();
        symbol_table.clear_subroutine_scope();
        self.symbol_table = symbol_table;

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

    pub fn writeToXMLOutput(&self, mut output: &str){
        //println!("Writing: {output}");
        let mut output_file = OpenOptions::new().read(true).append(true).open(&self.output_file)
            .expect("err");

        let written_output = self.trimOutput(output);

        output_file.write_all(&written_output.as_bytes());
    }

    pub fn writeToVMOutput(&self, mut output: &str){
        let mut output_file = OpenOptions::new().read(true).append(true).open(&self.vm_output_file)
            .expect("err");

        let written_output = self.trimOutput(output);

        output_file.write_all(&written_output.as_bytes());
    }

    pub fn createOutputFiles(&mut self){
        let output_file_create = File::create(&self.output_file)
            .expect("error");

        let vm_file_create = File::create(&self.vm_output_file)
            .expect("error");
    }

    // Get the # of fields in the current class
    fn getFieldCount(&mut self) -> i32{
        let mut field_count = 0;
        for (key, val) in self.symbol_table.get_class_symbol_table(){
            if (val.kind=="field"){
                field_count += 1;
            }
        }
        return field_count;
    }


    // Returns current token in the form <terminalType> xxx </terminalType>
    fn format_terminal(&mut self) -> String{


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
        self.createOutputFiles();
        self.initializeSymbolTables();
        let mut output = self.CompileClass();

        output = output.replace("<symbol>&", "<symbol>&amp;");
        output = output.replace("<symbol><", "<symbol>&lt;");
        output = output.replace("<symbol>>", "<symbol>&gt;");
        output = output.replace("<symbol>\"", "<symbol>&quot;");
        

        self.writeToXMLOutput(&output);
        self.symbol_table.display_symbol_table(true);
        self.symbol_table.display_symbol_table(false);
    }



    fn CompileClass(&mut self) -> String{
        let mut output = "".to_string();
        let mut vm_output = "".to_string();
        output += "<class>\n";

        let mut class_name = String::new();
        if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // should be 'class' terminal

            self.symbol_table.clear_class_scope();

            output += &self.format_terminal();

            self.tokenizer.advance_index();

            if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // should be className

                class_name = self.tokenizer.get_current_token();
                self.cur_class_name = class_name;

                output += &self.format_terminal();

                self.tokenizer.advance_index();

                if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){ // should be '{'
                    
                    output += &self.format_terminal();

                    self.tokenizer.advance_index();

                    while (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // Could be classVarDec or subroutineDec

                        while ((self.tokenizer.get_current_token() == "static") | (self.tokenizer.get_current_token() == "field")){
                            output += &self.compileClassVarDec();
                        };

                        while ((self.tokenizer.get_current_token() == "constructor") | (self.tokenizer.get_current_token() == "function") | (self.tokenizer.get_current_token() == "method")){
                            output += &self.compileSubroutine(class_name.clone());
                        }

                        self.tokenizer.advance_index();
                        
                        
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
        let mut vm_output = "".to_string();

        output += "<classVarDec>\n";

        let mut var_kind = String::new();
        let mut var_name = String::new();
        let mut var_type = String::new();

        if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // 'static' or 'field'

            var_kind = self.tokenizer.get_current_token();

            output += &self.format_terminal();

            self.tokenizer.advance_index();

            if ((self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD) | (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER)){ // 'int', 'char', 'bool', or className

                var_type = self.tokenizer.get_current_token();

                output += &self.format_terminal();

                self.tokenizer.advance_index();

                if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname 


                    var_name = self.tokenizer.get_current_token();
                    self.symbol_table.push_symbol(true, var_name.clone(), var_type.clone(), var_kind.clone());

                    output += &self.format_terminal();

                    self.tokenizer.advance_index();

                    while (self.tokenizer.get_current_token() == ","){
                        output += &self.format_terminal();
                        self.tokenizer.advance_index();

                        if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname

                            var_name = self.tokenizer.get_current_token();
                            self.symbol_table.push_symbol(true, var_name.clone(), var_type.clone(), var_kind.clone());

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


    fn compileSubroutine(&mut self, class_name: String) -> String{

        let mut output = "".to_string();
        let mut vm_output = "".to_string();

        let mut subroutine_name = "".to_string();
        let mut subroutine_kind = "".to_string();

        output += "<subroutineDec>\n";


        if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // 'constructor', 'function', or 'method'

            self.symbol_table.clear_subroutine_scope();

            subroutine_kind = self.tokenizer.get_current_token();

            if (subroutine_kind == "method"){
                self.symbol_table.initialize_method(class_name.clone()); // pushes 'this' into the symbol table
            }

            output += &self.format_terminal();
            self.tokenizer.advance_index();

            if ((self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER) | (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD)){ // 'void', 'int', 'char', 'bool', or className
                output += &self.format_terminal();

                self.tokenizer.advance_index();

                if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ //subroutine name
                    subroutine_name = self.tokenizer.get_current_token();
                    output += &self.format_terminal();

                    self.tokenizer.advance_index();

                    if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){ // should be open parentheses 
                        output += &self.format_terminal();
                        self.tokenizer.advance_index(); 

                        output += "<parameterList>\n";
                        if ((self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER) | (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD)){ //'int', 'char', 'bool', or className
                            output += &self.compileParameterList();

                        }
                        output += "</parameterList>\n";



                        if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL) { // close parentheses
                            output += &self.format_terminal();
                            self.tokenizer.advance_index(); 

                            output += "<subroutineBody>\n";

                            if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){ // should be '{'
                                output += &self.format_terminal();
                                self.tokenizer.advance_index(); 

                                while (self.tokenizer.get_current_token() == "var"){
                                    output += &self.compileVarDec();
                                }

                                let subroutine_kinds = self.symbol_table.get_subroutine_kinds();

                                let nLcl = (subroutine_kinds).get("local")
                                    .expect("error");

                               vm_output += format!("function {}.{} {}\n", class_name.clone(), subroutine_name, nLcl).as_str(); // function className.subroutineName nLcl


                                match (subroutine_kind.as_str()){
                                    "method"=>{
                                        vm_output += "push argument 0\npop pointer 0";
                                    },
                                    "constructor"=>{
                                        vm_output += format!("push constant {}\ncall Memory.alloc 1\npop pointer 0", self.getFieldCount()).as_str();
                                    },
                                    _=>{
                                        // do nothing
                                    }
                                }
                            
                                if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){
                                    output += &self.compileStatements();

                                    if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){ // should be '}'
                                        output += &self.format_terminal();
                                        self.tokenizer.advance_index(); 

                                    }
                                }
                            }

                            output += "</subroutineBody>\n";
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


        let mut var_name = String::new();
        let mut var_type = String::new();

        if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // should be var
            output += &self.format_terminal();
            self.tokenizer.advance_index(); 


            if ((self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER) | (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD)){ //'int', 'char', 'bool', or className


                var_type = self.tokenizer.get_current_token();


                output += &self.format_terminal();
                self.tokenizer.advance_index(); 

                if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname

                    var_name = self.tokenizer.get_current_token();


                    self.symbol_table.push_symbol(false, var_name, var_type.clone(), "local".to_string());
                    output += &self.format_terminal();
                    self.tokenizer.advance_index();       

                    while (self.tokenizer.get_current_token() == ","){
                        output += &self.format_terminal();
                        self.tokenizer.advance_index();

                        if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname

                            var_name = self.tokenizer.get_current_token();
                            self.symbol_table.push_symbol(false, var_name, var_type.clone(), "local".to_string());

                            output += &self.format_terminal();
                            self.tokenizer.advance_index();

                        }
                    }

                    if (self.tokenizer.get_token_type() == TOKEN_TYPE::SYMBOL){
                        output += &self.format_terminal();
                        self.tokenizer.advance_index();
                    }    
                    else {
                        println!("NOT A SYMBOL: {:?} is {:?}", self.tokenizer.get_current_token(), self.tokenizer.get_token_type());
                    }          

                }
            }



        }


        output += "</varDec>\n";

        return output;
    }



    fn compileParameterList(&mut self) -> String{
        let mut output = "".to_string();

        let mut var_type = String::new();
        let mut var_name = String::new();
         
        if ((self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER) | (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD)){ //'int', 'char', 'bool', or className

            var_type = self.tokenizer.get_current_token();

            output += &self.format_terminal();
            self.tokenizer.advance_index();

            if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname

                var_name = self.tokenizer.get_current_token();

                output += &self.format_terminal();
                self.tokenizer.advance_index();

                self.symbol_table.push_symbol(false, var_name, var_type, "arg".to_string());


                while (self.tokenizer.get_current_token() == ","){
                    output += &self.format_terminal();
                    self.tokenizer.advance_index();

                    if ((self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER) | (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD)){ //'int', 'char', 'bool', or className
                        var_type = self.tokenizer.get_current_token();
                        output += &self.format_terminal();
                        self.tokenizer.advance_index();


                        var_name = self.tokenizer.get_current_token();
                        output += &self.format_terminal(); //varname
                        self.tokenizer.advance_index();

                        self.symbol_table.push_symbol(false, var_name, var_type, "arg".to_string());

                    }

                }
            }
        }


        return output;



    }


    fn compileStatements(&mut self) -> String{
        let mut output = "".to_string();

        output += "<statements>\n";


        while (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){
            
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


        }

        output += "</statements>\n";

        return output
    }



    /*

    returns identifier in form: {kind} {id}
    (e.g., local 2)
    
     */
    fn translate_identifier(&self, identifier: String) -> String{ // give priority to subroutine identifiers, then move to class scope
        if (self.symbol_table.get_subroutine_symbol_table().contains_key(&identifier)){
            return format!("{} {}", self.symbol_table.get_subroutine_symbol(identifier.clone()).kind, self.symbol_table.get_subroutine_symbol(identifier.clone()).id);
        }
        else if (self.symbol_table.get_class_symbol_table().contains_key(&identifier)){
            return format!("{} {}", self.symbol_table.get_class_symbol(identifier.clone()).kind, self.symbol_table.get_class_symbol(identifier.clone()).id);
        }
        else{
            return String::new();
        }
    }

    fn compileLet(&mut self) -> String{
        let mut output = "".to_string();
        let mut vm_output = "".to_string();

        output += "<letStatement>\n";



        if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // let
            output += &self.format_terminal();
            self.tokenizer.advance_index();

            if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varname
                output += &self.format_terminal();

                vm_output += format!("push {} \n", self.translate_identifier(self.tokenizer.get_current_token())).as_str();

                self.tokenizer.advance_index();

                if (self.tokenizer.get_current_token() == "["){ // TODO: HANDLE ARRAYS
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

        output += "<ifStatement>\n";

        if (self.tokenizer.get_token_type() == TOKEN_TYPE::KEYWORD){ // if
            output += &self.format_terminal();
            self.tokenizer.advance_index();

            if (self.tokenizer.get_current_token() == "("){
                output += &self.format_terminal();
                self.tokenizer.advance_index();

                output += &self.compileExpression();

                if (self.tokenizer.get_current_token() == ")"){
                    output += &self.format_terminal();
                    self.tokenizer.advance_index();

                    if (self.tokenizer.get_current_token() == "{"){
                        output += &self.format_terminal();
                        self.tokenizer.advance_index();

                        output += &self.compileStatements();

                        if (self.tokenizer.get_current_token() == "}"){
                            output += &self.format_terminal();
                            self.tokenizer.advance_index();     


                            if (self.tokenizer.get_current_token() == "else"){ // this part is optional
                                output += &self.format_terminal();
                                self.tokenizer.advance_index();     
   
                                if (self.tokenizer.get_current_token() == "{"){
                                    output += &self.format_terminal();
                                    self.tokenizer.advance_index();

                                    output += &self.compileStatements();

                                    if (self.tokenizer.get_current_token() == "}"){
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


        
        output += "</ifStatement>\n";

        return output;
    }


    fn compileWhile(&mut self) -> String{
        let mut output = "".to_string();

        output += "<whileStatement>\n";

        if (self.tokenizer.get_current_token() == "while"){
            output += &self.format_terminal();
            self.tokenizer.advance_index();   

            if (self.tokenizer.get_current_token() == "("){
                output += &self.format_terminal();
                self.tokenizer.advance_index();   

                output += &self.compileExpression();

                if (self.tokenizer.get_current_token() == ")"){
                    output += &self.format_terminal();
                    self.tokenizer.advance_index(); 

                    if (self.tokenizer.get_current_token() == "{"){
                        output += &self.format_terminal();
                        self.tokenizer.advance_index(); 

                        output += &self.compileStatements();

                        if (self.tokenizer.get_current_token() == "}"){

                            output += &self.format_terminal();
                            self.tokenizer.advance_index(); 

                        }


                    }


                }  
            }

        }
        //
        output += "</whileStatement>\n";
        return output;
    }


    fn compileDo(&mut self) -> String{
        let mut output = "".to_string();
        //
        output += "<doStatement>\n";

        if (self.tokenizer.get_current_token() == "do"){

            output += &self.format_terminal();
            self.tokenizer.advance_index(); 

            if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // (subroutineName | className | varName)
                output += &self.format_terminal();

                let header = self.tokenizer.get_current_token();
                self.tokenizer.advance_index();         

                output += &self.compileSubroutineCall(header);

                if (self.tokenizer.get_current_token() == ";"){
                    output += &self.format_terminal();
                    self.tokenizer.advance_index();           
                }       
            }


        }

        output += "</doStatement>\n";
        return output;
    }


    fn compileReturn(&mut self) -> String{
        let mut output = "".to_string();
        //

        output += "<returnStatement>\n";

        if (self.tokenizer.get_current_token() == "return"){
            output += &self.format_terminal();
            self.tokenizer.advance_index();   

            if (self.tokenizer.get_current_token() != ";"){
                output += &self.compileExpression();
            }
            if (self.tokenizer.get_current_token() == ";"){
                output += &self.format_terminal();
                self.tokenizer.advance_index();           
            }     
        }

        output += "</returnStatement>\n";
        return output;
    }

    // Grammar: term (op term)*
    fn compileExpression(&mut self) -> String{
        let mut output = "".to_string();
        let mut vm_output = "".to_string();

        let mut tmp_output = "".to_string();

        output += "<expression>\n";

        //output += &self.compileTerm();
        vm_output += &self.compileTerm();

        // "+" , "-" , "*" , "/" , "&" , "," , "<" , ">" , "=", "|"
        while (self.tokenizer.is_op() && self.tokenizer.get_current_token() != ","){ // ',' is handled by compileExpressionList
            let op = self.tokenizer.get_current_token();

            self.tokenizer.advance_index();   

            vm_output += &self.compileTerm();


            match (op.as_str()){
                "+"=>{
                    vm_output += "add\n";
                },
                "-"=>{
                    vm_output += "sub\n";
                },
                "*"=>{
                    vm_output += "call Math.multiply 2\n";
                },
                "/"=>{
                    vm_output += "call Math.divide 2\n";
                },
                "&"=>{
                    vm_output += "and\n";
                },
                "<"=>{
                    vm_output += "lt\n";
                },
                ">"=>{
                    vm_output += "gt\n";
                },
                "="=>{
                    vm_output += "eq\n"
                },
                "|"=>{
                    vm_output += "or\n"
                }
                _=>{
                    // do nothing
                }

            }

            /*output += &self.format_terminal();
            self.tokenizer.advance_index();   

            output += &self.compileTerm();*/
        }

        output += "</expression>\n";



        return vm_output;    
    }

    fn compileTerm(&mut self) -> String{
        let mut output = "".to_string();
        let mut vm_output = "".to_string();
        
        output += "<term>\n";

        if ((self.tokenizer.get_token_type() == TOKEN_TYPE::INT_CONST)){

            output += &self.format_terminal();


            vm_output += format!("push constant {}\n", self.tokenizer.get_current_token()).as_str();

            self.tokenizer.advance_index();   
        }

        else if (self.tokenizer.get_token_type() == TOKEN_TYPE::STRING_CONST){
            let str_const = self.tokenizer.get_current_token();
            vm_output += format!("push constant {}\ncall String.new 1", str_const.len()).as_str();

            for char in str_const.chars(){
                vm_output += format!("push constant {}\ncall String.append 2", char as u32).as_str();

            }

        }

        else if (self.tokenizer.is_keyword_const()){
            output += &self.format_terminal();

            let keyword_const = self.tokenizer.get_current_token();

            match(keyword_const.as_str()){
                "true" =>{
                    vm_output += "push constant 1\nnot" // there is no "-1", so push 1 and then invert
                },
                "null" =>{
                    vm_output += "push constant 0\n"
                },
                "false"=>{
                    vm_output += "push constant 0\n";
                },
                "this"=>{
                    vm_output += "push pointer 0\n"
                }
                _=>{
                    println!("error message here");
                }

            }
            self.tokenizer.advance_index();   

        } else if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // varName | varName[expression] | subroutineCall
            let consumed_value = &self.tokenizer.get_current_token();
            output += &self.format_terminal(); // consumes either varName or (subroutineName | className | varName) depending on which part of the grammar is being used

            vm_output += format!("push {} \n", self.translate_identifier(self.tokenizer.get_current_token())).as_str();

            self.tokenizer.advance_index();   

            if (self.tokenizer.get_current_token() == "["){ // varName[expression] // TODO: HANDLE THIS FOR VM TRANSLATION
                //output += &self.format_terminal();
                self.tokenizer.advance_index();       

                //output += &self.compileExpression(); // WARNING: recursion    
                vm_output += &self.compileExpression();

                vm_output += "add\npop pointer 1\npush that 0\n";

                //output += &self.format_terminal(); // handle close bracket
                self.tokenizer.advance_index();          

            } else {
                
                //output += &self.compileSubroutineCall(consumed_value.to_string());
                vm_output += &self.compileSubroutineCall(consumed_value.to_string());
            }

        } else if (self.tokenizer.is_unary_op()){ // unaryOp term 

            let unary_op = self.tokenizer.get_current_token();

            //output += &self.format_terminal();
            self.tokenizer.advance_index();   

            //output += &self.compileTerm(); 
            vm_output += &self.compileTerm(); 

            if (unary_op == "-"){
                vm_output += "neg\n";
            }
            if (unary_op == "~"){
                vm_output += "not\n";
            }

        } else if (self.tokenizer.get_current_token() == "("){ // (expression)
            output += &self.format_terminal();
            self.tokenizer.advance_index();   

            output += &self.compileExpression();

            output += &self.format_terminal(); // handle close parentheses
            self.tokenizer.advance_index();   

        }


        output += "</term>\n";



        return vm_output; 


    }



    // this compilation function will handle the parentheses processing
    fn compileExpressionList(&mut self) -> (String, i32){
        let mut output = "".to_string();
        
        
        let mut vm_output = "".to_string();

        let mut total_expressions = 0;

        if (self.tokenizer.get_current_token() == "("){ 
            output += &self.format_terminal();
            self.tokenizer.advance_index(); 
        }

        output += "<expressionList>\n";

        if (self.tokenizer.get_current_token() != ")"){
            output += &self.compileExpression();

            vm_output += &self.compileExpression();  

            total_expressions += 1;  

            while (&self.tokenizer.get_current_token() == ","){
                output += &self.format_terminal();
                self.tokenizer.advance_index(); 

                vm_output += &self.compileExpression();  

                total_expressions += 1;    
            }
        }

        output += "</expressionList>\n";

        output += &self.format_terminal();
        self.tokenizer.advance_index(); 



        

        return (vm_output, total_expressions);
    }


    // assumes subroutineName | className | varName have already been consumed
    fn compileSubroutineCall(&mut self, header: String) -> String{
        let mut output = "".to_string();
        let mut vm_output = "".to_string();

        if (self.tokenizer.get_current_token() == "("){ // subroutineName (expressionList)

            


            let expression_list = &self.compileExpressionList();

            vm_output += "push pointer 0\n";
            vm_output += &expression_list.0;


            vm_output += format!("call {}.{} {}\n", self.cur_class_name, header, &expression_list.1+1).as_str(); // call className.subroutineCall nArgs // nArgs is +1 because it's a method

            //output += &self.format_terminal(); // handle close parentheses
            self.tokenizer.advance_index(); 

        } else if (self.tokenizer.get_current_token() == "."){ // (className|varName).subroutineName(expressionList)

            let mut class_name = "".to_string();
            let mut is_varname = false;
            // Check if it is varName or className

            // in these cases, it is varName, not className
            if (self.symbol_table.get_class_symbol_table().contains_key(&header)){ 
                class_name = (self.symbol_table.get_class_symbol(header.clone()).type_).clone();
                vm_output += format!("push {}\n", self.translate_identifier(header)).as_str();
                is_varname = true;

            }
            else if (self.symbol_table.get_subroutine_symbol_table().contains_key(&header)){
                class_name = (self.symbol_table.get_subroutine_symbol(header.clone()).type_).clone();
                vm_output += format!("push {}\n", self.translate_identifier(header)).as_str();
                is_varname = true;
            }



            //output += &self.format_terminal();
            self.tokenizer.advance_index();  

            if (self.tokenizer.get_token_type() == TOKEN_TYPE::IDENTIFIER){ // subroutineName
                let subroutine_name = self.tokenizer.get_current_token();
                output += &self.format_terminal();
                self.tokenizer.advance_index();   


                if (self.tokenizer.get_current_token() == "("){ // subroutineName (expressionList) 

                    //output += &self.compileExpressionList().0;         
                    vm_output += &self.compileExpressionList().0;  

                    let mut nArgs = self.compileExpressionList().1;
                    if (is_varname){
                        nArgs = (nArgs + 1); // pass in object pointer
                    }
                    vm_output += format!("call {}.{} {}", class_name, subroutine_name, nArgs).as_str();

                }
            } 

        }


        return vm_output;

    }
}