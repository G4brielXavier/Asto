use std::collections::HashMap;


pub fn to_input(prefix: & str, func: &str) -> HashMap<String, String> {
    let mut structure = HashMap::new();

    let command_s= format!("{} {}", prefix, func);

    structure.insert("node".to_string(), "Input".to_string());
    structure.insert("prefix".to_string(), prefix.to_string());
    structure.insert("function".to_string(), func.to_string());
    structure.insert("command".to_string(), command_s);

    structure
}