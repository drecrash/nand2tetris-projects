use std::{collections::HashMap, hash::Hash, fmt};


#[derive(Debug)]
#[derive(Clone)]
pub struct Symbol {
    pub type_: String,
    pub kind: String,
    pub id: i32
}


pub struct SymbolTable {
    class_scope: HashMap<String, Symbol>,
    subroutine_scope: HashMap<String, Symbol>,

    class_kinds: HashMap<String, i32>, // Keeps a count of the current # of the different kinds in the symbol table
    /* EG:
    {
    "static": 1,
    "field": 2,
    "argument": 1,
    ...
    }
     */
    subroutine_kinds: HashMap<String, i32>
}

impl SymbolTable {

    pub fn get_class_symbol(&self, name: String) -> &Symbol{
        return self.class_scope.get(&name)
            .expect("error");
    }

    pub fn get_subroutine_symbol(&self, name: String) -> &Symbol{
        return self.subroutine_scope.get(&name)
            .expect("error");
    }

    pub fn get_subroutine_kinds(&self) -> HashMap<String, i32>{
        return self.subroutine_kinds.clone();
    }

    pub fn get_class_symbol_table(&self) -> HashMap<String, Symbol>{
        return self.class_scope.clone();
    }

    pub fn get_subroutine_symbol_table(&self) -> HashMap<String, Symbol>{
        return self.subroutine_scope.clone();
    }



    // class_level is a boolean toggle indicating if changes are being pushed to the class scope symbol table, or the subroutine level
    pub fn push_symbol(&mut self, class_level: bool, name: String, type_: String, kind: String){
        let mut id = 0;

        if (class_level){
            id = self.class_kinds.get(&kind)
                .expect("Error getting kind count for class")
                .to_owned();

        }

        if (!class_level){
            println!("{kind}");
            id = self.subroutine_kinds.get(&kind)
                .expect("Error getting kind count for subroutine")
                .to_owned();

        }


        let new_symbol: Symbol = Symbol {
            type_: type_,
            kind: kind.clone(),
            id: id
        };

        if (class_level){
            self.class_scope.insert(name, new_symbol);
            *self.class_kinds.entry(kind).or_default() += 1; // https://stackoverflow.com/questions/73837235/add-or-increment-value-in-hashmap-in-rust
        } else {
            self.subroutine_scope.insert(name, new_symbol);
            *self.subroutine_kinds.entry(kind).or_default() += 1;
        }
        
    }

    pub fn clear_class_scope(&mut self){

        self.display_symbol_table(true);

        self.class_kinds.clear();
        self.class_kinds.insert("arg".to_string(), 0);
        self.class_kinds.insert("field".to_string(), 0);
        self.class_kinds.insert("static".to_string(), 0);
        self.class_kinds.insert("local".to_string(), 0);

        self.class_scope = HashMap::new();
    }

    pub fn clear_subroutine_scope(&mut self){

        self.display_symbol_table(false);

        self.subroutine_kinds.clear();
        self.subroutine_kinds.insert("arg".to_string(), 0);
        self.subroutine_kinds.insert("field".to_string(), 0);
        self.subroutine_kinds.insert("static".to_string(), 0);
        self.subroutine_kinds.insert("local".to_string(), 0);

        self.subroutine_scope = HashMap::new();
    }

    pub fn initialize_method(&mut self, class_name: String){

        self.push_symbol(false, "this".to_string(),class_name, "arg".to_string());

    }

    pub fn new() -> Self{
        Self {
            class_kinds: HashMap::new(),
            subroutine_kinds: HashMap::new(),

            class_scope: HashMap::new(),
            subroutine_scope: HashMap::new()
        }

    }

    pub fn display_symbol_table(&mut self, class_level: bool){
        if (class_level){

            for (key, val) in &self.class_scope{
                println!("{key}: {:?}", val);
            }
        } else {

            for (key, val) in &self.subroutine_scope{
                println!("{key}: {:?}", val)
            }
        }
    }


}