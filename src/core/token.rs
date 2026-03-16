use crate::core::types::AstoStructure;

#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    
    pub value: &'a [u8],
    pub typeval: AstoStructure,

    pub start_ln: u32,
    pub start_col: u16,
    pub end_ln: u32, 
    pub end_col: u16

}