
use indexmap::IndexMap;
use std::mem::{self, discriminant};
use std::str::from_utf8;

use crate::core::token::Token;
use crate::core::nodetrees::{
    InputStruct, to_input, InputAlias, ParamStruct
};
use crate::core::types::AstoStructure;


type Node = IndexMap<String, InputAlias>;
pub type ParamVec = Vec<String>;


pub trait ParserF<'a> {

    fn new() -> Self;

    fn see(&self, toks: &Vec<Token<'a>>) -> Result<Token<'a>, ()>;
    fn _next(&self, toks: &Vec<Token<'a>>) -> Result<Token<'a>, ()>; 
    fn eat(&mut self, toks: &Vec<Token<'a>>) -> Option<Token<'a>>;
    fn expected(&mut self, type_expc: AstoStructure, toks: &Vec<Token<'a>>) -> Result<Token<'a>, String>;
    fn get(&mut self, type_expc: AstoStructure, toks: &Vec<Token<'a>>) -> &'a str;
    fn is_enum_next(&mut self, toks: &Vec<Token<'a>>, enum_test: AstoStructure) -> bool;

    fn parse_primary(&mut self, toks: &Vec<Token>) -> Node;
    fn parseator(&mut self, toks: Vec<Token>) -> Vec<Node>;

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


    fn _next(&self, toks: &Vec<Token<'a>>) -> Result<Token<'a>, ()> {
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
                    let msg = format!("Asto Parser Error - The Type '{:?}' (AstoStructure) is different of '{:?}'", b.typeval, type_expc);
                    Err(msg)
                }
            },
            Err(_) => {
                Err("Asto Parser Error - Occurred an error while Match".to_string())
            }
        }

    }


    fn get(&mut self, type_expc: AstoStructure, toks: &Vec<Token<'a>>) -> &'a str {

        let token = match self.expected(type_expc, &toks) {
            Ok(b) => b,
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        };

        let value: &str = match from_utf8(&token.value) {
            Ok(b) => b,
            Err(e) => {
                println!("{:?}", e);
                std::process::exit(1);
            }
        };

        value

    }


    fn is_enum_next(&mut self, toks: &Vec<Token<'a>>, enum_test: AstoStructure) -> bool {
        if let Ok(param) = self.see(&toks) {
            if discriminant(&param.typeval) == discriminant(&enum_test) {
                true
            } else {
                false
            }
        } else { 
            false 
        }
    }



    fn parse_primary(&mut self, toks: &Vec<Token>) -> Node {

        while self.is_enum_next(&toks, AstoStructure::NewLine) {
            self.eat(&toks);
        }

        let tok: Token = match self.eat(&toks) {
            Some(t) => t,
            None => { 
                eprintln!("Asto CLI Error - Ocurred an Error in Parser");
                std::process::exit(1)
            }
        };

        // println!("{:?} - {:?}", from_utf8(&toks[self.idx].value), self.see(&toks).unwrap().typeval);

        match tok.typeval {

            // > cmd command --param1 --param2
            //  / "It does..."
            //  : 0
            AstoStructure::Input => {

                let mut prefx_val: String = "".to_string();
                let mut cmd_val: String = "".to_string();
                let mut command_val: String = "".to_string();
                let mut params: ParamVec = Vec::new();
                let mut desc_val: String = "".to_string();
                let mut vers_val: String = "".to_string();
                let mut params_config: Vec<ParamStruct> = Vec::new();
 

                // Get "cmd"
                prefx_val = self.get(AstoStructure::Info, &toks).to_string();
                
                // Get "command"
                cmd_val = self.get(AstoStructure::Info, &toks).to_string();
                
                // Get all --params until \n
                while self.is_enum_next(&toks, AstoStructure::Param) {
                    // println!("{:?}", from_utf8(&toks[self.idx].value));
                    params.push(self.get(AstoStructure::Param, &toks).to_string());
                }

                // Get \n
                if self.is_enum_next(&toks, AstoStructure::NewLine) {
                    self.eat(&toks);
                } else {
                    println!("Asto Syntax Error - Expected 'NewLine'");
                    std::process::exit(1)
                }

                // Verify if next is TAB to Description

                if self.is_enum_next(&toks, AstoStructure::Tab) {

                    // Eat "\t"
                    self.eat(&toks);

                    // Verify if next is Description
                    if self.is_enum_next(&toks, AstoStructure::Description) {
                        // Eat "/"
                        self.eat(&toks);

                        // Get Description
                        desc_val = self.get(AstoStructure::Description, &toks).to_string();
                    }

                }

                // Get \n
                if self.is_enum_next(&toks, AstoStructure::NewLine) {
                    self.eat(&toks);
                }


                // Verify Tab for Version

                if self.is_enum_next(&toks, AstoStructure::Tab) {
                    // Eat "\t"

                    self.eat(&toks);

                    if self.is_enum_next(&toks, AstoStructure::Version) {
                        // Eat ":"
                        self.eat(&toks);

                        // Get Version
                        vers_val = self.get(AstoStructure::Version, &toks).to_string();
                    }

                } 

                // Get \n
                if self.is_enum_next(&toks, AstoStructure::NewLine) {
                    self.eat(&toks);
                }


                // Get {  } (if has)
                if self.is_enum_next(&toks, AstoStructure::Tab) {
                    // Eat TAB
                    self.eat(&toks);

                    if self.is_enum_next(&toks, AstoStructure::ParamBox) {
                        // Eat "{"
                        self.eat(&toks);


                        // Eat or Store everything that is inside a ParamBox
                        while !self.is_enum_next(&toks, AstoStructure::ParamBox) {
                            
                            // Get \n
                            if self.is_enum_next(&toks, AstoStructure::NewLine) {
                                self.eat(&toks);
                            } else {
                                println!("Asto Syntax Error - Expected 'NewLine'");
                                std::process::exit(1)
                            }


                            // Get \t
                            if self.is_enum_next(&toks, AstoStructure::Tab) {
                                self.eat(&toks);
                            } else {
                                println!("Asto Syntax Error - Expected 'Tab' inside ({}{})", "{", "}");
                                std::process::exit(1)
                            }   

                            if self.is_enum_next(&toks, AstoStructure::ParamBox) {
                                break
                            }

                            // Get \t
                            if self.is_enum_next(&toks, AstoStructure::Tab) {
                                self.eat(&toks);
                            } else {
                                println!("oi Asto Syntax Error - Expected 'Tab' inside ({}{})", "{", "}");
                                std::process::exit(1)
                            }   

                            let name = self.get(AstoStructure::Param, &toks).to_string();
                            let typeval = self.get(AstoStructure::Type, &toks).to_string();
                            let desc = self.get(AstoStructure::Description, &toks).to_string();

                            params_config.push(
                                ParamStruct {
                                    name: name,
                                    typeval: typeval,
                                    desc: desc
                                }
                            );                       

                            continue

                        }

                        // Eat "}"
                        if self.is_enum_next(&toks, AstoStructure::ParamBox) {
                            self.eat(&toks);
                        } else {
                            println!("Asto Syntax Error - Expected '}}' to close ParamBox");
                            std::process::exit(1)
                        } 

                    }

                } 
                


                command_val = format!("{} {}", prefx_val, cmd_val);
                
                for p in params.iter() {
                    command_val.push_str(format!(" {}", p).as_str())
                }

                let input_arch: InputStruct = InputStruct::new(
                    prefx_val,
                    cmd_val, 
                    command_val, 
                    desc_val, 
                    vers_val, 
                    params,
                    params_config
                );

                
                to_input(input_arch)

            },

            _ => {
                println!("Asto Parser Error - The first structure must be a INPUT ('>').");
                println!("Asto Tip - Starts with '> pre command_name' to test.");
                std::process::exit(1)
            }
        }


    }








    fn parseator(&mut self, toks: Vec<Token>) -> Vec<Node> {
        
        let mut tree: Vec<Node> = Vec::new();

        while self.idx < toks.len() {
            let node = self.parse_primary(&toks);
            tree.push(node)
        }

        tree

    }


}