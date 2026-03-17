use indexmap::IndexMap;
use serde::{Deserialize, Serialize};




#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputAlias {
    Text(String),
    List(Vec<String>),
    ListParam(Vec<ParamStruct>)
}



#[derive(Serialize, Deserialize, Debug)]
pub struct InputStruct {
    pub command: String,
    pub desc: String,

    pub prefix: String,
    pub func: String,
    pub version: String,
    
    #[serde(rename = "param_config")]
    pub params_config: Vec<ParamStruct>,

    // #[serde(rename = "output_logs")]
    // pub output_logs: Vec<String>,
    
    pub params: Vec<String>
    
}

impl InputStruct {
    pub fn new(prefix: String, func: String, command: String, desc: String, version: String, params: Vec<String>, params_config: Vec<ParamStruct>) -> Self {
        Self {
            command: command,
            desc: desc,
            prefix: prefix,
            func: func,
            version: version,
            params: params,
            params_config: params_config,
            // output_logs: output_logs
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ParamStruct {
    pub name: String,
    pub typeval: String,
    pub desc: String
}


// FUNCTIONS 


pub fn get_version_string(version: &str) -> String {
    match version {
        "new" => String::from("🟡 New"),
        "depre" => String::from("🔴 Deprecated"),
        "expm" => String::from("🔵 Experimental"),
        "stable" => String::from("🟢 Stable"),
        _ => {
            String::from("No Implemented")
        }
    }
}




pub fn to_input(input: InputStruct) -> IndexMap<String, InputAlias> {
    let mut structure: IndexMap<String, InputAlias> = IndexMap::new();

    let node_name: InputAlias = InputAlias::Text("Input".to_string());

    structure.insert("node".to_string(), node_name);
    structure.insert("command".to_string(), InputAlias::Text(input.command));
    structure.insert("description".to_string(), InputAlias::Text(input.desc));
    structure.insert("prefix".to_string(), InputAlias::Text(input.prefix));
    structure.insert("function".to_string(), InputAlias::Text(input.func));
    structure.insert("version".to_string(), InputAlias::Text(input.version));
    structure.insert("params".to_string(), InputAlias::List(input.params));
    structure.insert("params_config".to_string(), InputAlias::ListParam(input.params_config));
    // structure.insert("output_logs".to_string(), InputAlias::List(input.output_logs));

    structure
}