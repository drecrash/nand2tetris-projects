use regex::Regex;

pub struct JackTokenizer {
    pub whole_input: String,
    pub current_token: String

}

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

/* Routines
- hasMoreTokens
- advance: advances current_token
- tokenType: returns token type of current_token
- keyWord
- symbol
- identifier
- intVal
- stringVal

*/

impl JackTokenizer {

    // Need function that breaks input into a list of tokens

    pub fn tokenize(&self, input_str: String){
        let re = Regex::new(r#"(" [^"]* ")|(?i)\b(class|constructor|function|method|field|static|var|int|char|boolean|void|true|false|null|this|let|do|if|else|while|return)\b|([{}()\[\].+,\-;/*&|<>=~])|(\d+)|([a-zA-Z_][a-zA-Z0-9_]*)"#).unwrap();
        let tokens: Vec<&str> = re.find_iter(&input_str)
            .map(|m| m.as_str())
            .collect();

        println!("{:?}", tokens)


        
    }

}
