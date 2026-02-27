
pub struct Parser {
    pub command: String
}


impl Parser {
    // All commands are A (A-instruction), L (label), or C (C-instruction)
    pub fn command_type(&self) -> &str{
        let first_char = self.command.chars().nth(0)
            .expect("Error");
        match (first_char){
            '@' => "A",
            '(' => "L",
            _ => "C"
        }
    }


    // A-instructions and labels have symbols "Xxx" associated with them [ @Xxx or (Xxx) ]
    pub fn symbol(&self) -> &str{
        let command_type = self.command_type();
        let mut symbol = "";
        if command_type == "A"{
            symbol = &self.command[1..]; // remove leading @
        } else if command_type == "L"{
            let command_length = self.command.len();
            symbol = &self.command[1..(command_length-1)]; // remove leading and following parentheses
        }

        symbol
    }

    
    // C-instructions have the form "dest=comp;jump", where dest and jump are optional
    pub fn dest(&self) -> &str{
        if (self.command_type() != "C" || !self.command.contains("=")){
            return ""
        }
        let command_parts: Vec<&str> = (self.command).split("=").collect();// break into ["dest", "comp;jump"]
        command_parts[0]
    }

    pub fn comp(&self) -> &str{
        let mut command_parts: Vec<&str>;
        let mut broken_command: &str = &self.command;
        if (self.command_type() != "C"){
            return ""
        }
        if self.command.contains("="){
            command_parts = (self.command).split("=").collect(); 
            broken_command = command_parts[1];
        }
        if self.command.contains(";"){ 
            command_parts = (broken_command).split(";").collect(); // break into ["comp", "jump"]
            broken_command = command_parts[0];
        }

        broken_command

      
    }

    pub fn jump(&self) -> &str{
        let mut command_parts: Vec<&str>;
        let mut broken_command: &str = &self.command;
        if (self.command_type() != "C" || !self.command.contains(";")){
            return ""
        }
        if self.command.contains("="){
            command_parts = (self.command).split("=").collect(); 
            broken_command = command_parts[1];
        }
        if self.command.contains(";"){ 
            command_parts = (broken_command).split(";").collect(); // break into ["comp", "jump"]
            broken_command = command_parts[1];
        }

        broken_command    
    }

}
