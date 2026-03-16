use regex::Regex;

pub struct JackTokenizer {
    pub whole_input: String,
    pub current_token_index: i32,
    pub tokens: Vec<String>

}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TOKEN_TYPE {
    KEYWORD,
    IDENTIFIER,
    INT_CONST,
    STRING_CONST,
    SYMBOL
}

pub enum KEYWORD {
    CLASS, METHOD,
    FUNCTION,
    CONSTRUCTOR, INT,
    BOOLEAN, CHAR, VOID,
    VAR, STATIC, FIELD,
    LET, DO, IF, ELSE,
    WHILE, RETURN, TRUE,
    FALSE, NULL, THIS
}



impl JackTokenizer {

    fn all_keywords(&self) -> Vec<String>{
        let keywords= vec!["class" , "constructor" , "function" , "method" , "field" , "static" , "var" , "int" , "char" , "boolean" , "void" , "true" , "false" , "null" , "this" , "let" , "do" , "if" , "else" , "while" , "return"]
            .into_iter()
            .map(String::from)
            .collect();

        keywords
    }

    fn all_symbols(&self) -> Vec<String>{
        let symbols = vec!["{" , "}" , "(" , ")" , "[" , "]" , "." , "," , ";" , "+" , "-" , "*" , "/" , "&" , "," , "<" , ">" , "=" , "~"]
            .into_iter()
            .map(String::from)
            .collect();

        symbols
    }

    fn is_integer(&self, input: String) -> bool{
        return input.parse::<i64>().is_ok()
    }


    // Uses regex to break into tokens
    pub fn tokenize(&mut self, input_str: String) -> Vec<String>{
        let re = Regex::new(r#"("[^"]*")|(?i)\b(class|constructor|function|method|field|static|var|int|char|boolean|void|true|false|null|this|let|do|if|else|while|return)\b|([{}()\[\].+,\-;/*&|<>=~])|(\d+)|([a-zA-Z_][a-zA-Z0-9_]*)"#).unwrap();
        let tokens: Vec<String> = re.find_iter(&input_str)
            .map(|m| m.as_str().to_string())
            .collect();

        //println!("{:?}", tokens);
        self.tokens = tokens.clone();
        return tokens;
    }

    pub fn get_current_token(&self) -> String{
        let tokens = &self.tokens;

        return tokens.get(self.current_token_index as usize)
            .expect("error getting token")
            .to_owned();
        
    }

    pub fn has_more_tokens(&self) -> bool{
        let total_tokens = self.tokens.len() as i32;
        self.current_token_index < total_tokens - 1

    }

    pub fn advance_index(&mut self){
        if (self.has_more_tokens()){
            self.current_token_index = self.current_token_index + 1;
        }
        
    }

    pub fn get_token_type(&mut self) -> TOKEN_TYPE{

        let current_token = self.get_current_token();

        if (self.all_keywords().contains(&current_token.to_lowercase())){
            return TOKEN_TYPE::KEYWORD;
        } else if (self.all_symbols().contains(&current_token.to_lowercase())){
            return TOKEN_TYPE::SYMBOL;
        } else if (self.is_integer(current_token.clone().to_lowercase())){
            return TOKEN_TYPE::INT_CONST;
        } else if ((&current_token).contains("\"")){
            return TOKEN_TYPE::STRING_CONST;
        } else {
            return TOKEN_TYPE::IDENTIFIER;
        }

    }

    pub fn is_unary_op(&mut self) -> bool{
        let unary_ops: Vec<String> = vec!["-", "~"]
            .into_iter()
            .map(String::from)
            .collect();

        return (unary_ops.contains(&self.get_current_token()));
    }

    
    pub fn is_op(&mut self) -> bool{
        let ops: Vec<String> = vec!["+" , "-" , "*" , "/" , "&" , "," , "<" , ">" , "=", "|"]
            .into_iter()
            .map(String::from)
            .collect();

        return (ops.contains(&self.get_current_token()));
    }

    pub fn is_keyword_const(&mut self)->bool{
        let ops: Vec<String> = vec!["true", "false", "null", "this"]
            .into_iter()
            .map(String::from)
            .collect();

        return (ops.contains(&self.get_current_token())); 
    }



}
