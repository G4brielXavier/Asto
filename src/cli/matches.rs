use indexmap::IndexMap;

use crate::cli::commands::Commands;
use crate::cli::filemanager::{ is_exist, read_file, is_asto_type, write_file, write_markdown };

use crate::core::lexer::{ Lexer, LexerF };
use crate::core::parser::{ Parser, ParserF };

use crate::core::nodetrees::{ InputAlias, get_version_string };


fn read_asto_to_json(cont: String) -> String {
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

    json_ast
}

fn read_asto_to_ast(cont: String) -> Vec<IndexMap<String, InputAlias>> {
    let content_bytes: &[u8] = cont.as_bytes();

    let mut lexer: Lexer = Lexer::new(content_bytes);
    let tokens = lexer.tokenizable();
    
    let mut parser: Parser = Parser::new();
    let ast = parser.parseator(tokens);

    ast
}

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

                                    let asto_content = read_asto_to_json(cont);
                                    
                                    let name = filepath.file_stem().and_then(|n| n.to_str()).unwrap_or("default_asto_file.json");
                                    let extension = "json";
                                    
                                    let _ = write_file(name.to_string(), extension.to_string(), asto_content.to_string());
                                    
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
   
        Commands::Version { path } => {

            match path {

                Some(p) => {

                    if !is_asto_type(&p) {
                        eprintln!("Asto CLI Error - {:?} is not a Asto File", {p});
                        std::process::exit(1);
                    }

                    match is_exist(&p) {
                        Some(file) => {

                            if let Some(cont) = read_file(file) {

                                println!("-.- Asto is analyzing your file ...");

                                // This is will Tokenizer, Parseate and return a AST with informations about the Asto File
                                let asto_content = read_asto_to_ast(cont);
                                let mut counter = 0;
                                let mut versions_count: Vec<i32> = vec![0, 0, 0, 0];

                                for cmd in asto_content.iter() {

                                    counter += 1;

                                    for (key, value) in cmd.iter() {

                                        if key == "command" {
                                            println!();
                                            match value {
                                                InputAlias::Text(s) => println!("> {}", s),
                                                _ => println!()
                                            }
                                        }

                                        if key == "version" {

                                            match value {

                                                InputAlias::Text(s) => {
                                                    
                                                    let vers = get_version_string(&s);
                                                    println!(": version = {}", vers);
                                                    
                                                    match &*vers {
                                                        "New" => {
                                                            versions_count[0] += 1;
                                                            println!("note: It's new!")
                                                        },
                                                        "Deprecated" => {
                                                            versions_count[1] += 1;
                                                            println!("note: It's deprecated! Make your changes to improve or solve.")
                                                        },
                                                        "Experimental" => {
                                                            versions_count[2] += 1;
                                                            println!("note: It's experimental! Analyze all possibilities to this command before finish.")
                                                        },
                                                        "Stable" => {
                                                            versions_count[3] += 1;
                                                            println!("note: It's stable! it can be used!")
                                                        },
                                                        _ => {
                                                            println!("Unexpected version")
                                                        }
                                                    }
                                                },
                                                _ => println!()

                                            }
                                            println!();

                                        }

                                    }

                                }

                                println!("Asto found {} command(s) in this file.", counter);
                                println!();
                                println!("-- {} command(s) as 'new'", versions_count[0]);
                                println!("-- {} command(s) as 'deprecated'", versions_count[1]);
                                println!("-- {} command(s) as 'experimental'", versions_count[2]);
                                println!("-- {} command(s) as 'stable'", versions_count[3]);

                            } else {
                                eprintln!("Asto CLI Error - Not possible read the file.");
                                std::process::exit(1);
                            }

                        },
                        None => {

                            eprintln!("Asto CLI Error - {:?} not found or exist.", p.clone());
                            std::process::exit(1);

                        }
                    }

                },
                None => {
                    
                    eprintln!("Asto CLI Error - Path not implemented in Command.");
                    std::process::exit(1);

                }

            }

        }

        Commands::Status { path } => {

            match path {

                Some(p) => {

                    if !is_asto_type(&p) {
                        eprintln!("Asto CLI Error - {:?} is not a Asto File", {p});
                        std::process::exit(1);
                    }

                    match is_exist(&p) {
                        Some(file) => {

                            if let Some(cont) = read_file(file) {

                                println!("-.- Asto is analyzing your file ...");

                                // This is will Tokenizer, Parseate and return a AST with informations about the Asto File
                                let asto_content = read_asto_to_ast(cont);

                                for cmd in asto_content.iter() {

                                    for (key, value) in cmd.iter() {



                                        if key == "command" {
                                            println!();
                                            match value {
                                                InputAlias::Text(s) => {
                                                    println!("> {}", s);
                                                },
                                                _ => println!()
                                            }
                                        }

                                        if key == "description" {

                                            match value {
                                                InputAlias::Text(s) => {
                                                    if *s != "" {
                                                        println!("   OK description")
                                                    } else {
                                                        println!("   WARN missing 'description'")
                                                    }
                                                }
                                                _ => {
                                                    println!("   WARN missing 'description'")
                                                }
                                            }

                                        }

                                        if key == "version" {

                                            match value {
                                                InputAlias::Text(s) => {
                                                    if *s != "" {
                                                        println!("   OK version")
                                                    } else {
                                                        println!("   WARN missing 'version'")
                                                    }
                                                }
                                                _ => {
                                                    println!("   WARN missing 'version'")
                                                }
                                            }

                                        }

                                        if key == "params" {

                                            match value {
                                                InputAlias::List(s) => {
                                                    if s.len() != 0 {
                                                        println!("   OK Params")
                                                    } else {
                                                        println!("   WARN missing 'params'")
                                                    }
                                                }
                                                _ => {
                                                    println!("   WARN missing 'params'")
                                                }
                                            }

                                        }

                                        if key == "params_config" {

                                            match value {
                                                InputAlias::ListParam(s) => {
                                                    if s.len() != 0 {
                                                        println!("   OK params_config")
                                                    } else {
                                                        println!("   WARN missing 'params_config'")
                                                    }
                                                }
                                                _ => {
                                                    println!("   WARN missing 'params_config'")
                                                }
                                            }

                                        }

                                        if key == "output_logs" {

                                            match value {
                                                InputAlias::List(s) => {
                                                    if s.len() != 0 {
                                                        println!("   OK output_logs")
                                                    } else {
                                                        println!("   WARN missing 'output_logs'")
                                                    }
                                                }
                                                _ => {
                                                    println!("   WARN missing 'output_logs'")
                                                }
                                            }

                                        }

                                    }

                                }

                            } else {
                                eprintln!("Asto CLI Error - Not possible read the file.");
                                std::process::exit(1);
                            }

                        },
                        None => {

                            eprintln!("Asto CLI Error - {:?} not found or exist.", p.clone());
                            std::process::exit(1);

                        }
                    }

                },
                None => {
                    
                    eprintln!("Asto CLI Error - Path not implemented in Command.");
                    std::process::exit(1);

                }

            }

        }
    
        Commands::Tree { path } => {

            match path {

                Some(p) => {

                    if !is_asto_type(&p) {
                        eprintln!("Asto CLI Error - {:?} is not a Asto File", {p});
                        std::process::exit(1);
                    }

                    match is_exist(&p) {
                        Some(file) => {

                            if let Some(cont) = read_file(file) {

                                println!("-.- Asto is creating the Tree for you ...");
                                println!();

                                // This is will Tokenizer, Parseate and return a AST with informations about the Asto File
                                let asto_content = read_asto_to_ast(cont);
                                
                                println!("CLI");

                                for cmd in asto_content.iter() {

                                    for (key, value) in cmd.iter() {

                                        if key == "command" {
                                            match value {
                                                InputAlias::Text(s) => {
                                                    println!(" ├── {}", s);
                                                },
                                                _ => println!()
                                            }
                                        }

                                        if key == "description" {
                                            match value {
                                                InputAlias::Text(s) => {
                                                    println!(" │    └── description: {} ", s);
                                                },
                                                _ => println!()
                                            }
                                        }

                                        if key == "version" {
                                            match value {
                                                InputAlias::Text(s) => {
                                                    println!(" │    └── version: {}", get_version_string(&s));
                                                },
                                                _ => println!()
                                            }
                                        }

                                        if key == "params_config" {
                                            match value {
                                                InputAlias::ListParam(s) => {
                                                    println!(" │    └── params:");
                                                    for param in s.iter() {
                                                        println!(" │        └── {} {} {}", param.name, param.typeval, param.desc);
                                                    }
                                                },
                                                _ => println!()
                                            }
                                        }
 
                                        if key == "output_logs" {
                                            match value {
                                                InputAlias::List(s) => {
                                                    println!(" │    └── outputs:");
                                                    for log in s.iter() {
                                                        println!(" │        └── {}", log);
                                                    }
                                                    println!(" │ ");
                                                },
                                                _ => println!()
                                            }
                                        }


                                    }

                                }



                            } else {
                                eprintln!("Asto CLI Error - Not possible read the file.");
                                std::process::exit(1);
                            }

                        },
                        None => {

                            eprintln!("Asto CLI Error - {:?} not found or exist.", p.clone());
                            std::process::exit(1);

                        }
                    }

                },
                None => {
                    
                    eprintln!("Asto CLI Error - Path not implemented in Command.");
                    std::process::exit(1);

                }

            }



        }

    }

}