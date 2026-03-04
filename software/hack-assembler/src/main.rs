use std::{collections::HashMap, fs::{self, File}, io::{Write}};
mod code_mod;
use crate::code_mod::*;

mod parser;
use crate::parser::*;
fn main() {

    let file_path = "Personal.asm"; // assembly file to parse

    let mut file_contents = fs::read_to_string(file_path)
        .expect("File error");

    

    file_contents = clean_file(file_contents);

    let symbol_table = build_symbol_table(&file_contents); // this approach requires 2 initial passes to build the symbol table

    fs::remove_file("output.hack")
        .expect("error");


    let mut output_file = File::create("output.hack")
        .expect("error");

    

    for line in file_contents.lines(){
        //println!("{line}");
        let processed_command = process_command(line.to_string(), &symbol_table);

        if (processed_command != "".to_string()){
            writeln!(output_file, "{}", processed_command)
                .expect("error");
        }

    }


}

fn clean_file(mut contents: String) -> String{
    contents = contents.replace(" ", ""); // get rid of white space

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
    fin_contents = fin_contents.replace(" ", "");
    fin_contents

}

fn build_symbol_table(contents: &String) -> HashMap<String, i32>{
    let mut symbol_table: HashMap<String, i32> = HashMap::new();
    let mut iterator = 0; // iterate over ROM
    let mut ram_address = 16; // iterate over available memory addresses

    // There are predefined symbols
    symbol_table.insert("SP".to_string(), 0);
    symbol_table.insert("LCL".to_string(), 1);
    symbol_table.insert("ARG".to_string(), 2);
    symbol_table.insert("THIS".to_string(), 3);
    symbol_table.insert("THAT".to_string(), 4);
    symbol_table.insert("SCREEN".to_string(), 16384);
    symbol_table.insert("KBD".to_string(), 24576);
    
    for i in 0..16 {
        let r = format!("R{}",i);
        symbol_table.insert(r, i);
    }

    // Get user-defined labels
    for line in contents.lines(){
        let mut parser = Parser {
            command: line.to_string()
        };

        let command_type = parser.command_type();
        let symbol = parser.symbol();

        if (command_type == "L" && !(symbol_table.contains_key(symbol))){
            symbol_table.insert(symbol.to_string(), iterator.clone());
        } else if (command_type != "L"){
            iterator += 1;
        }

    }

    // Get user-defined variables
    for line in contents.lines(){
        let mut parser = Parser {
            command: line.to_string()
        };

        let command_type = parser.command_type();
        let symbol = parser.symbol();
   

        if (command_type == "A"){

            if !(symbol_table.contains_key(symbol)){
                symbol_table.insert(symbol.to_string(), ram_address);
                ram_address += 1;
            }

            
        }
    }

    symbol_table

}


fn process_command(command: String, symbol_table: &HashMap<String, i32>) -> String{
    
    let parser = Parser {
        command: command
    };

    let command_type = parser.command_type();
    let symbol = parser.symbol();


    if (command_type == "C"){

        let dest = parser.dest();
        let comp = parser.comp();
        let jmp = parser.jump();

        let coded_dest = code_dest(dest);
        let coded_comp = code_mod::code_comp(comp);
        let coded_jmp = code_mod::code_jump(jmp);

        // C-instructions begin with '1', followed by two irrelevant '11'
        return format!("{}{}{}{}{}", 1, 11, coded_comp, coded_dest, coded_jmp);

    }

    else if (command_type == "A"){
        let coded_a = code_mod::code_ainstruct(symbol, symbol_table);
        return coded_a;
    }

    else {
        return "".to_string();
    }



}

