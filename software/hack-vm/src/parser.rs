pub struct Parser {
    pub instruction_set: String,
    pub cur_line: i32
}

#[derive(PartialEq)]
pub enum COMMAND_TYPES {
    PUSH,
    POP,
    ARITHMETIC,

    LABEL,
    GOTO,
    IF,
    FUNCTION,
    RETURN,
    CALL
}


impl Parser {

    // Get command type (arithmetic, push, pop)
    pub fn commandType(line: String) -> COMMAND_TYPES{
        if (line.contains("push")){ // push/pop command types
            return COMMAND_TYPES::PUSH;
        } else if (line.contains("pop")){
            return COMMAND_TYPES::POP;
        } else if (line.contains("label")){ // branching commands
            return COMMAND_TYPES::LABEL;
        } else if (line.contains("goto")){
            return COMMAND_TYPES::GOTO;
        } else if (line.contains("if-goto")){
            return COMMAND_TYPES::IF
        } else { // to-do: account for improper syntax
            return COMMAND_TYPES::ARITHMETIC;
        }
    }

    /* get first argument

    If ARITHMETIC type, returns command
    If branch related command, returns label
    Otherwise returns segment (local, static, etc.)

    */
    pub fn arg1(line: &String) -> &str{


        let command_parts:Vec<&str> = line.split(" ").collect();

        let mut index = 1; // if push/pop, structure is [push/pop] [arg1] [arg2]

        if (Self::commandType(line.clone()) == COMMAND_TYPES::ARITHMETIC){ // if arithmetic, structure is [arg1]
            index = 0;
        }

        println!("{line}");
        
        let arg1 = command_parts.get(index)
            .expect("error getting arg1");

        arg1

    }

    pub fn arg2(line: &String) -> &str{
        let command_parts:Vec<&str> = line.split(" ").collect();

        if (Self::commandType(line.clone()) == COMMAND_TYPES::ARITHMETIC){ // if arithmetic, no arg2
            return "String::new();"
        }
        
        let arg2 = command_parts.get(2)
            .expect("error getting arg2");

        arg2       
    }

}