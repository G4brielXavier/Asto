use std::fs;
use std::io::Write;
use std::fs::File;
use std::path::PathBuf;

use indexmap::IndexMap;

use crate::core::nodetrees::InputAlias;


// Verify if the path exists
pub fn is_exist(path: &PathBuf) -> Option<&PathBuf> {
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

// Read the file else Error
pub fn read_file(path: &PathBuf) -> Option<String> {
    match fs::read_to_string(&path) {
        Ok(content) => Some(content),
        Err(err) => {
            eprintln!("Asto CLI Error - Error while read {:?}. {}", path, err);
            std::process::exit(1);
        }
    }
} 


pub fn write_file(name: String, extension: String, content: String) -> std::io::Result<()> {

    let filename = format!("{}.{}", name, extension);

    let mut file = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        }
    };

    let _ = file.write_all(content.as_bytes());

    Ok(())
}

// Verify if the file received is from the extension Asto
pub fn is_asto_type(path: &PathBuf) -> bool {
    match path.extension() {
        Some(ext) => ext == "asto",
        None => false
    }
}





pub fn write_markdown(content: Vec<IndexMap<String, InputAlias>>) -> String {

    let default_msg = "# Project Name \n\n**By:** YOUR_NAME_HERE<br>\n\nInformations about project...\n\n\n## Commands\n";
    let mut markdown_content: String = default_msg.to_string(); 

    for cont in content.iter() {

        for (key, value) in cont.iter() {

            match &*key.as_str() {
                "command" => {
                    let mut val = match value {
                        InputAlias::Text(s) => s.trim_matches('"'),
                        _ => "default"
                    };

                    if val == "" {
                        val = "No Implemented"
                    }

                    let data = &format!("\n\n### `{}`\n\n", val);
                    markdown_content.push_str(data)
                },

                "description" => {
                    let mut val = match value {
                        InputAlias::Text(s) => s.trim_matches('"'),
                        _ => "default"
                    };

                    if val == "" {
                        val = "No Implemented"
                    }

                    let data = &format!("{} \n\n\n", val);
                    markdown_content.push_str(data)
                },

                "version" => {
                    let mut val = match value {
                        InputAlias::Text(s) => s.trim_matches('"'),
                        _ => "default"
                    };

                    if val == "" {
                        val = "No Implemented"
                    } else {
                        match val {
                            "0" => {
                                val = "New"
                            },
                            "0.5" => {
                                val = "Experimental"
                            },
                            "1" => {
                                val = "Solid"
                            }

                            &_ => {
                                val = "No Implemented"
                            }
                        }
                    }

                    let data = &format!("**Version**: {} \n\n\n", val);
                    markdown_content.push_str(data)
                },

                "params_config" => {
                    markdown_content.push_str("### Params \n\n");
                    
                    let val = match value {
                        InputAlias::ListParam(s) => s,
                        _ => &Vec::new()
                    };

                    for param in val.iter() {
                        markdown_content.push_str(&format!("`{}` `{}` *{}* <br>\n", param.name, param.typeval, param.desc))
                    }

                    markdown_content.push_str(&format!("<br>\n\n"))
                    
                },

                _ => {

                }
            }

        }

    }

    markdown_content

}