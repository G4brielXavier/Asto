
use indexmap::IndexMap;
use core::error;
use std::mem::{self, discriminant};
use std::str::from_utf8;

use crate::core::token::Token;
use crate::core::nodetrees::{
    InputStruct, to_input, InputAlias, ParamStruct
};
use crate::core::types::AstoStructure;
use crate::core::errors::AstoError;
use crate::core::utilities::Utilities;


type Node = IndexMap<String, InputAlias>;
pub type ParamVec = Vec<String>;


pub trait ParserF<'a> {

    fn new() -> Self;

    fn see(&self, toks: &Vec<Token<'a>>) -> Result<Token<'a>, AstoError>;
    fn _next(&self, toks: &Vec<Token<'a>>) -> Result<Token<'a>, AstoError>; 
    fn eat(&mut self, toks: &Vec<Token<'a>>) -> Option<Token<'a>>;
    fn expected(&mut self, type_expc: AstoStructure, toks: &Vec<Token<'a>>) -> Result<Token<'a>, AstoError>;
    fn get(&mut self, type_expc: AstoStructure, toks: &Vec<Token<'a>>) -> Result<&'a str, AstoError>;
    fn is_enum_next(&mut self, toks: &Vec<Token<'a>>, enum_test: AstoStructure) -> bool;

    fn parse_primary(&mut self, toks: &Vec<Token>) -> Result<Node, AstoError>;
    fn parseator(&mut self, toks: Vec<Token>) -> Vec<Node>;

}


pub struct Parser {
    pub idx: usize,
}




impl<'a> ParserF<'a> for Parser {

    fn new() -> Self { Self { idx: 0 } }


    fn see(&self, toks: &Vec<Token<'a>>) -> Result<Token<'a>, AstoError> {
    if self.idx < toks.len() {
        Ok(toks[self.idx])
    } else {
        Err(AstoError::SyntaxError("EOF".to_string()))
    }
}


    fn _next(&self, toks: &Vec<Token<'a>>) -> Result<Token<'a>, AstoError> {
        if self.idx + 1 < toks.len() {
            Ok(toks[self.idx + 1])
        } else {
            Err(AstoError::SyntaxError("EOF".to_string()))
        }
    }


    fn eat(&mut self, toks: &Vec<Token<'a>>) -> Option<Token<'a>> {
        
        let tok: Token = toks[self.idx];
        self.idx += 1;
        
        Some(tok)

    }


    fn expected(&mut self, type_expc: AstoStructure, toks: &Vec<Token<'a>>) -> Result<Token<'a>, AstoError> {

        match self.see(&toks) {
            Ok(b) => {
                if mem::discriminant(&type_expc) == mem::discriminant(&b.typeval) {
                    
                    if let Some(tok) = self.eat(toks) {
                        Ok(tok)
                    } else {
                        Err(AstoError::SyntaxError("Occurred an error while consume token.".to_string()))
                    }

                } else {
                    let msg = format!("The Type \"{:?}\" is different of \"{:?}\". Unexpected syntax.", b.typeval, type_expc);
                    Err(AstoError::SyntaxError(msg))
                }
            },
            Err(_) => {
                Err(AstoError::SyntaxError("Occurred an error while see tokens.".to_string()))
            }
        }

    }


    fn get(&mut self, type_expc: AstoStructure, toks: &Vec<Token<'a>>) -> Result<&'a str, AstoError> {

        let token = self.expected(type_expc, &toks);

        match token {
            Ok(t) => {
                let value: &str = match from_utf8(&t.value) {
                    Ok(b) => b,
                    Err(e) => {
                        println!("{}", e);
                        return Err(AstoError::SyntaxError("UTF8".to_string()))
                    }
                };

                Ok(value)
            }
            Err(e) => {
                Err(e)
            }
        }



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


    fn parse_primary(&mut self, toks: &Vec<Token>) -> Result<Node, AstoError> {

        let utils: Utilities = Utilities::new();

        while self.is_enum_next(&toks, AstoStructure::NewLine) {
            self.eat(&toks);
        }

        let tok: Token = self.eat(&toks).expect("Ocurred an Error in Parser");

        // println!("{:?} - {:?}", from_utf8(&toks[self.idx].value), self.see(&toks).unwrap().typeval);

        match tok.typeval {

            // > cmd command --param1 --param2
            //  / "It does..."
            //  : 0
            AstoStructure::Input => {

                let mut prefx_val: String = Default::default();
                let mut cmd_val: String = Default::default();
                let mut command_val: String = Default::default();
                let mut params: ParamVec = Vec::new();
                let mut desc_val: String = Default::default();
                let mut vers_val: String = Default::default();
                let mut params_config: Vec<ParamStruct> = Vec::new();
                // let mut output_logs: Vec<String> = Vec::new();
 
                let control: bool = false;
                if control  {
                    println!("{:?}", prefx_val);
                    println!("{:?}", cmd_val);
                    println!("{:?}", command_val);
                }
                

                // Get "cmd"
                match self.get(AstoStructure::Info, &toks) {
                    Ok(e) => prefx_val = e.to_string(),
                    Err(e) => return Err(e)
                }
                
                // Get "command"
                match self.get(AstoStructure::Info, &toks) {
                    Ok(e) => cmd_val = e.to_string(),
                    Err(e) => return Err(e)
                }
                

                // Get all --params until \n
                while self.is_enum_next(&toks, AstoStructure::Param) {

                    let params_getted = self.get(AstoStructure::Param, &toks);
                    
                    match params_getted {
                        Ok(p) => params.push(p.to_string()),
                        Err(e) => return Err(e)
                    }

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
                        match self.get(AstoStructure::Description, &toks) {
                            Ok(e) => desc_val = e.to_string(),
                            Err(e) => return Err(e)
                        }

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
                        match self.get(AstoStructure::Version, &toks) {
                            Ok(e) => vers_val = e.to_string(),
                            Err(e) => return Err(e) 
                        }
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

                            let name = self.get(AstoStructure::Param, &toks);

                            match name {
                                Ok(_) => {}
                                Err(e) => return Err(e)
                            }

                            let typeval = self.get(AstoStructure::Type, &toks);
                            
                            match typeval {
                                Ok(_) => {}
                                Err(e) => return Err(e)
                            }

                            let desc = self.get(AstoStructure::Description, &toks);

                            match desc {
                                Ok(e) => {
                                    if !utils.valtype.contains(&e) {
                                        return Err(AstoError::KeywordError(format!("\"{}\" not exists as type value.", e)))
                                    }
                                }
                                Err(e) => return Err(e)
                            }

                            params_config.push(
                                ParamStruct {
                                    name: name.expect("Error").to_string(),
                                    typeval: typeval.expect("Error").to_string(),
                                    desc: desc.expect("Error").to_string()
                                }
                            );                       

                            continue

                        }

                        // Eat "}"
                        if self.is_enum_next(&toks, AstoStructure::ParamBox) {
                            self.eat(&toks);
                        } else {
                            return Err(AstoError::SyntaxError("Expected '}}' to close ParamBox".to_string()))
                        } 

                    }

                } 
                


                // Get \n
                if self.is_enum_next(&toks, AstoStructure::NewLine) {
                    self.eat(&toks);
                }


                // Get [ ] (if has)
                // if self.is_enum_next(&toks, AstoStructure::Tab) {
                //     // Eat TAB
                //     self.eat(&toks);

                //     if self.is_enum_next(&toks, AstoStructure::OutputBox) {
                //         // Eat "{"
                //         self.eat(&toks);


                //         // Eat or Store everything that is inside a ParamBox
                //         while !self.is_enum_next(&toks, AstoStructure::OutputBox) {
                            
                //             // Get \n
                //             if self.is_enum_next(&toks, AstoStructure::NewLine) {
                //                 self.eat(&toks);
                //             } else {
                //                 println!("Asto Syntax Error - Expected 'NewLine' ln {} : {}", self.see(&toks).unwrap().start_ln, self.see(&toks).unwrap().start_col);
                //                 std::process::exit(1)
                //             }


                //             // Get \t
                //             if self.is_enum_next(&toks, AstoStructure::Tab) {
                //                 self.eat(&toks);
                //             } else {
                //                 println!("Asto Syntax Error - Expected 'Tab' inside ({{}})ln {} : {}", self.see(&toks).unwrap().start_ln, self.see(&toks).unwrap().start_col);
                //                 std::process::exit(1)
                //             }   

                //             if self.is_enum_next(&toks, AstoStructure::OutputBox) {
                //                 break
                //             }

                //             // Get \t
                //             if self.is_enum_next(&toks, AstoStructure::Tab) {
                //                 self.eat(&toks);
                //             } else {
                //                 println!("oi Asto Syntax Error - Expected 'Tab' inside ({{}}). ln {} : {}", self.see(&toks).unwrap().start_ln, self.see(&toks).unwrap().start_col);
                //                 std::process::exit(1)
                //             }   

                //             let outputline = self.get(AstoStructure::OutputLine, &toks).to_string();
                            
                //             output_logs.push(outputline);

                //             continue

                //         }

                //         // Eat "]"
                //         if self.is_enum_next(&toks, AstoStructure::OutputBox) {
                //             self.eat(&toks);
                //         } else {
                //             println!("Asto Syntax Error - Expected ']' to close OutputBox. ln {} : {}", self.see(&toks).unwrap().start_ln, self.see(&toks).unwrap().start_col);
                //             std::process::exit(1)
                //         } 

                //     }

                // } 
                
                

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
                    params_config,
                    // output_logs
                );

                
                Ok(to_input(input_arch))

            },

            _ => {
                println!("{:?}", tok);
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
            match node {
                Ok(e) => {
                    tree.push(e)
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1)
                }
            }
        }

        tree

    }


}