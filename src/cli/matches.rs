use crate::cli::commands::Commands;
use crate::cli::filemanager::{ is_exist, read_file, is_asto_type, write_file, write_markdown };

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
                                
                                if let Some(cont) = read_file(&filepath) {

                                    if !*silent {
                                        println!("...Asto is being exported to JSON -.-");
                                    }

                                    let content_bytes: &[u8] = cont.as_bytes();

                                    let mut lexer: Lexer = Lexer::new(content_bytes);
                                    let tokens = lexer.tokenizable();
                                    
                                    let mut parser: Parser = Parser::new();
                                    let ast = parser.parseator(tokens);

                                    let json_ast = match serde_json::to_string_pretty(&ast) {
                                        Ok(j) => j,
                                        Err(e) => {
                                            println!("Asto JSON Error - {}", e);
                                            std::process::exit(1)
                                        }
                                    };
                                    
                                    let name = filepath.file_stem().and_then(|n| n.to_str()).unwrap_or("default_asto_file.json");
                                    let extension = "json";

                                    let _ = write_file(name.to_string(), extension.to_string(), json_ast.to_string());
                                    
                                    if !*silent {
                                        println!("Asto CLI - \"{}.{}\" exported with successfully!", name, extension)
                                    } 

                                } else {
                                    eprintln!("Asto CLI Error - Not possible read the file.");
                                    std::process::exit(1);
                                }
                                
                            }
                            
                            // asto export file.asto --md
                            else if *md {
                                if !*silent {
                                    println!("...Asto is being exported to MARKDOWN -.-");
                                }

                                if let Some(cont) = read_file(&filepath) {

                                    let content_bytes: &[u8] = cont.as_bytes();

                                    let mut lexer: Lexer = Lexer::new(content_bytes);
                                    let tokens = lexer.tokenizable();
                                    
                                    let mut parser: Parser = Parser::new();
                                    let ast = parser.parseator(tokens);
                                    
                                    let md_content = write_markdown(ast);

                                    let name = filepath.file_stem().and_then(|n| n.to_str()).unwrap_or("default_asto_file.json");
                                    let extension = "md";

                                    let _ = write_file(name.to_string(), extension.to_string(), md_content);

                                    if !*silent {
                                        println!("Asto CLI - \"{}.{}\" exported with successfully!", name, extension)
                                    }


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