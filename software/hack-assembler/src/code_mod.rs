
use std::{collections::HashMap, fs::{self, File}, io::{Write}};

// Code Module

// C-instruction structure: 111 acccccc ddd jjj
// if M in instr, then toggle 'a' bit

pub fn code_comp(comp: &str) -> &str{
    match comp{
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",
        "!D"=>"0001101",
        "!A"=>"0110001",
        "!M"=>"1110001",
        "-D"=>"0001111",
        "-A"=>"0110011",
        "-M"=>"1110011",
        "D+1"=>"0011111",
        "A+1"=>"0110111",
        "M+1"=>"1110111",
        "D-1"=>"0001110",
        "A-1"=>"0110010",
        "M-1"=>"1110010",
        "D+A"=>"0000010",
        "D+M"=>"1000010",
        "D-A"=>"0010011",
        "D-M"=>"1010011",
        "A-D"=>"0000111",
        "M-D"=>"1000111",
        "D&A"=>"0000000",
        "D&M"=>"1000000",
        "D|A"=>"0010101",
        "D|M"=>"1010101",
        _=>""
    }
}

pub fn code_dest(dest: &str) -> &str{
    match dest {
        "" => "000",
        "M" => "001",
        "D" => "010",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD"=>"110",
        "AMD"=>"111",
        _=>"000"
    }
}

pub fn code_jump(jump: &str) -> &str{
    match jump {
        "" => "000",
        "JGT"=>"001",
        "JEQ"=>"010",
        "JGE"=>"011",
        "JLT"=>"100",
        "JNE"=>"101",
        "JLE"=>"110",
        "JMP"=>"111",
        _=>"000"
    }
}

// Convert decimal to binary for A instructions and labels
fn dec_to_bin(mut dec: i32) -> String{
    let mut binary = String::new();

    while dec > 0{
        let dec_rem = dec % 2;
        let mut ch = '0';

        if dec_rem == 1{
            ch = '1';
        }

        binary.insert(0, ch);

        dec = dec / 2
        

    }

    binary = format!("{:0>15}", binary); // pad string

    binary


}

pub fn code_ainstruct(symbol: &str, symbol_table: &HashMap<String, i32>) -> String{
    let mut binary: String = "".to_string();

    // if it is a direct integer reference
    if (symbol.parse::<f64>().is_ok()){

        binary = dec_to_bin(symbol.parse::<i32>()
            .expect("error"));
    
    } else { // if it is a reference to a variable or label
        let symbol_option = symbol_table.get(&symbol.to_string());
        match symbol_option {
            Some(symbol_dec) => binary = dec_to_bin(*symbol_dec),
            None => println!("Panic")
        }
        
    }

    // acknowledge it is an A-instruction by prepending a '0' bit
    binary.insert(0, '0');

    binary
}

