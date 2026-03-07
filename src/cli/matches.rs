use crate::cli::commands::Commands;
use crate::cli::filemanager::{ is_exist, read_file, is_asto_type };

use crate::core::lexer::{ Lexer, LexerF };
use crate::core::parser::{ Parser, ParserF };

pub fn matches_command(command: &Commands) {

    match command {

        // if is "asto export"
        Commands::Export { path, json, md, silent } => {

            // if <FILE> was implemented
            match path {

                Some(ph) => {

                    // file extension is asto
                    if !is_asto_type(&ph) {
                        eprintln!("Asto CLI Error - {:?} is not a Asto File", {ph});
                        std::process::exit(1);
                    }


                    match is_exist(&ph) {
                        Some(filepath) => {

                            // asto export file.asto --json
                            if *json {
                                if !*silent {
                                    println!("{:?}", filepath);
                                }
                                
                                if let Some(cont) = read_file(&filepath) {

                                    let content_bytes: &[u8] = cont.as_bytes();

                                    let mut lexer: Lexer = Lexer::new(content_bytes);
                                    let tokens = lexer.tokenizable();
                                    
                                    let mut parser: Parser = Parser::new();
                                    let _ast = parser.parseator(tokens);

                                    // for tok in tokens.iter() {
                                    //     println!("{:?}: {:?}", std::str::from_utf8(&tok.value), &tok.typeval)
                                    // }
                                    

                                } else {
                                    eprintln!("Asto CLI Error - Not possible read the file.");
                                    std::process::exit(1);
                                }
                            }
                            
                            // asto export file.asto --md
                            else if *md {
                                if !*silent {
                                    println!("{:?}", filepath);
                                }

                                if let Some(cont) = read_file(&filepath) {
                                    
                                    let content_bytes: &[u8] = cont.as_bytes();

                                    let mut lexer: Lexer = Lexer::new(content_bytes);
                                    let tokens = lexer.tokenizable();
                                    
                                    let mut parser: Parser = Parser::new();
                                    let _ast = parser.parseator(tokens);

                                    // for tok in tokens.iter() {
                                    //     println!("{:?}", tok)
                                    // }


                                } else {
                                    eprintln!("Asto CLI Error - Not possible read the file.");
                                    std::process::exit(1);
                                }
                            }

                            // <FILE> not used
                            else {
                                eprintln!("Asto CLI Error - Expected <EXPORT_TYPE> after <FILE>");
                                std::process::exit(1)
                            }

                        },
                        None => {

                            if !*silent {
                                eprintln!("Asto CLI Error - {:?} not found or exist.", ph.clone());
                            }

                            std::process::exit(1);

                        }
                    }

                }, 
                None => {
                    
                    if !*silent {
                        eprintln!("Asto CLI Error - Path not implemented in Command.");
                    }

                    std::process::exit(1);

                }
            
            }



        }
   
    }

}