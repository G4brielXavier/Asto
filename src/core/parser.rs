
use std::collections::HashMap;
use std::mem;
use std::str::from_utf8;


use crate::core::{nodetrees::to_input, token::Token};
use crate::core::types::AstoStructure;


type Node = HashMap<String, String>;
type NodeVec = Vec<HashMap<String, String>>;


pub trait ParserF<'a> {

    fn new() -> Self;

    fn see(&self, toks: &Vec<Token<'a>>) -> Result<Token<'a>, ()>;
    fn next(&self, toks: &Vec<Token<'a>>) -> Result<Token<'a>, ()>; 
    fn eat(&mut self, toks: &Vec<Token<'a>>) -> Option<Token<'a>>;
    fn expected(&mut self, type_expc: AstoStructure, toks: &Vec<Token<'a>>) -> Result<Token<'a>, String>;

    fn parse_primary(&mut self, toks: &Vec<Token>) -> Node;
    fn parseator(&mut self, toks: Vec<Token>) -> NodeVec;

}


pub struct Parser {
    pub idx: usize,
    }




impl<'a> ParserF<'a> for Parser {

    fn new() -> Self { Self { idx: 0 } }


    fn see(&self, toks: &Vec<Token<'a>>) -> Result<Token<'a>, ()> {
    if self.idx < toks.len() {
        Ok(toks[self.idx])
    } else {
        Err(())
    }
}


    fn next(&self, toks: &Vec<Token<'a>>) -> Result<Token<'a>, ()> {
        if self.idx + 1 < toks.len() {
            Ok(toks[self.idx + 1])
        } else {
            Err(())
        }
    }


    fn eat(&mut self, toks: &Vec<Token<'a>>) -> Option<Token<'a>> {
        
        let tok: Token = toks[self.idx];
        self.idx += 1;
        
        Some(tok)

    }


    fn expected(&mut self, type_expc: AstoStructure, toks: &Vec<Token<'a>>) -> Result<Token<'a>, String> {

        match self.see(&toks) {
            Ok(b) => {
                if mem::discriminant(&type_expc) == mem::discriminant(&b.typeval) {
                    
                    if let Some(tok) = self.eat(toks) {
                        Ok(tok)
                    } else {
                        Err("Asto Parser Error - Occurred an error while Eat".to_string())
                    }

                } else {
                    let msg = format!("Asto Parser Error - The Token's AstoStructure is different of {:?}", type_expc);
                    Err(msg)
                }
            },
            Err(_) => {
                Err("Asto Parser Error - Occurred an error while Match".to_string())
            }
        }

    }






    fn parse_primary(&mut self, toks: &Vec<Token>) -> Node {

        let tok: Token = match self.eat(&toks) {
            Some(t) => t,
            None => { 
                eprintln!("Asto CLI Error - Ocurred an Error in Parser");
                std::process::exit(1)
            }
        };


        match tok.typeval {

            // > cmd command
            AstoStructure::Input => {

                // Get Token "cmd"
                let prefx_token = match self.expected(AstoStructure::Info, &toks) {
                    Ok(b) => b,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1)
                    }
                };

                // Get "cmd"
                let prefx_value = match from_utf8(&prefx_token.value) {
                    Ok(b) => b,
                    Err(e) => {
                        println!("{:?}", e);
                        std::process::exit(1)
                    }
                };

                // Get Token "command"
                let func_token = match self.expected(AstoStructure::Info, &toks) {
                    Ok(b) => b,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1)
                    }
                };

                // Get "command"
                let func_value = match from_utf8(&func_token.value) {
                    Ok(b) => b,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1)
                    }
                };


    
                to_input(prefx_value, func_value)


            },

            AstoStructure::Info => {
                HashMap::new()
            },

            AstoStructure::Version => {
                HashMap::new()
            },

            AstoStructure::Output => {
                HashMap::new()
            },

            AstoStructure::Description => {
                HashMap::new()
            },

            AstoStructure::Param => {
                HashMap::new()  
            },

            AstoStructure::Type => {
                HashMap::new()
            },

            _ => {
                HashMap::new()
            }
        }


    }








    fn parseator(&mut self, toks: Vec<Token>) -> NodeVec {
        
        let mut tree: NodeVec = Vec::new();

        while self.idx < toks.len() {
            let node = self.parse_primary(&toks);
            tree.push(node)
        }

        tree

    }


}