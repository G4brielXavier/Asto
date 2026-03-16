use std::str::from_utf8;

use crate::core::types::{ AstoStructure };

use crate::core::utilities::Utilities;

use crate::core::token::Token;

pub trait LexerF<'a> {
	fn new(code: &'a [u8]) -> Self;

	// Scanner
	fn step(&mut self) -> u8;
	fn see(&self) -> u8;
	fn next(&self) -> u8;
	fn is_eof(&self) -> bool;
	fn is_alpha_extended(&self, byte: u8) -> bool;

	// Tokenizer Core
	fn tokenizable(&mut self) -> Vec<Token<'a>>;

	// Tokenizer Extras
	fn tok_string(&mut self) -> Result<Token<'a>, String>;
	fn tok_symbol(&mut self) -> Result<Token<'a>, String>;
	
	fn tok_info(&mut self, utils: &Utilities) -> Result<Token<'a>, String>;
	fn tok_param(&mut self) -> Result<Token<'a>, String>;
	// fn tok_output(&mut self) -> Result<Token<'a>, String>;


	fn tok_comment(&mut self) -> Result<(), String>;
}


pub struct Lexer<'a> {
	pub index: usize,
	pub code: &'a [u8],
	pub ln: u32,
	pub col: u16 
}


impl<'a> LexerF<'a> for Lexer<'a> {
	
	fn new(code: &'a [u8]) -> Self {
		Self {
			index: 0,
			code: code,
			ln: 1,
			col: 1
		}
	}


	fn tokenizable(&mut self) -> Vec<Token<'a>> {
		
		let mut tokens: Vec<Token> = Vec::new();
		let utils = Utilities::new();

		
		while !self.is_eof() {

			let byte = self.see();
			
			if byte == b'\r' {
				self.step();
				continue;
			}




			else if byte == b'\n' {

				self.ln += 1;
				self.col = 0;

				let tok: Token = Token {
					value: b"\n",
					typeval: AstoStructure::NewLine,
					start_ln: self.ln,
					start_col: self.col,
					end_ln: self.ln,
					end_col: self.col   
				};

				tokens.push(tok);
				self.step();
				continue;

			}




			else if byte == b'\t' {

				self.col = 4;

				let tok: Token = Token {
					value: b"\n",
					typeval: AstoStructure::Tab,
					start_ln: self.ln,
					start_col: self.col,
					end_ln: self.ln,
					end_col: self.col   
				};

				tokens.push(tok);
				self.step();
				self.step();
				self.step();
				self.step();
				continue;
			}



			// Capture tabs
			else if self.code.get(self.index..self.index+4) == Some(b"    ") {

				self.index += 4;
				self.col += 4;

				let tok: Token = Token {
					value: b"\t",
					typeval: AstoStructure::Tab,
					start_ln: self.ln,
					start_col: self.col,
					end_ln: self.ln,
					end_col: self.col   
				};

				tokens.push(tok);
				continue;
			}



			
			else if byte == b' ' {
				self.step();
				continue;
			}



			// Capture "Strings"
			else if byte == b'\"' {
				match self.tok_string() {
					Ok(t) => {
						tokens.push(t);
						continue;
					},    
					Err(e) => {
						println!("{}", e);
						std::process::exit(1);
					}    
				}    
			}    



			// Capture INFO and specific KEYWORDS
			else if self.is_alpha_extended(byte) || byte == b'_' {
				match self.tok_info(&utils) {
					Ok(t) => {
						tokens.push(t);
						continue;
					},    
					Err(e) => {
						println!("{}", e);
						std::process::exit(1)
					}    
				};    
			}    



			// Capture --param
			else if byte == b'-' && self.next() == b'-' {
				match self.tok_param() {
					Ok(t) => {
						tokens.push(t);
						continue;
					},    
					Err(e) => {
						println!("{}", e);
						std::process::exit(1)
					}    
				}    
			}    



			// Capture output line ($)
			// else if byte == b'$' {
			// 	match self.tok_output() {
			// 		Ok(t) => {
			// 			tokens.push(t);
			// 			continue;
			// 		},
			// 		Err(e) => {
			// 			println!("{}", e);	
			// 			std::process::exit(1)
			// 		}
			// 	}
			// }



			// Capture specific symbols
			else if utils.symbols.contains(&byte) {
				match self.tok_symbol() {
					Ok(t) => {
						tokens.push(t);
						continue;
					},
					Err(e) => {
						println!("{}", e);
						std::process::exit(1)
					}
				};
			}

			// Capture ## comments
			else if byte == b'#' && self.next() == b'#' {
				match self.tok_comment() {
					Ok(_t) => { continue; },
					Err(e) => {
						println!("{}", e);
						std::process::exit(1)
					}
				};
			}

			self.step();
			// println!("Unexpected Character: {}", byte as char);
		}


		tokens
	}




	// Tokenizer Functions Extras

	fn tok_string(&mut self) -> Result<Token<'a>, String> {
		
		let start_ln = self.ln;
		let start_col = self.col;

		self.step();

		let initialindex = self.index;

		while !self.is_eof() && self.see() != b'\"' {
			self.step();
		}

		self.step();

		let bytes_ident = &self.code[initialindex..self.index-1];

		Ok(Token {
			value: bytes_ident,
			typeval: AstoStructure::Description,
			start_ln: start_ln,
			start_col: start_col,
			end_ln: self.ln,
			end_col: self.col
		})

	}


	fn tok_info(&mut self, utils: &Utilities) -> Result<Token<'a>, String> {
		let start_ln = self.ln;
		let start_col = self.col;

		let initialindex = self.index;

		while !self.is_eof() && (self.is_alpha_extended(self.see()) || self.see().is_ascii_digit()) {
			self.step();
		}

		let bytes_ident = &self.code[initialindex..self.index];
		let value = match from_utf8(&bytes_ident) {
			Ok(s) => s,
			Err(e) => {
				println!("{}", e);
				std::process::exit(1)
			}
		};

		let toktype = if utils.valtype.contains(&value) {
			AstoStructure::Type
		} else if utils.valstatus.contains(&value) {
			AstoStructure::Version
		} else {
			AstoStructure::Info
		};


		Ok(Token {
			value: bytes_ident,
			typeval: toktype,
			start_ln: start_ln,
			start_col: start_col,
			end_ln: self.ln,
			end_col: self.col
		})
	}


	fn tok_symbol(&mut self) -> Result<Token<'a>, String> {
		
		let start_ln = self.ln;
		let start_col = self.col;

		let initialindex = self.index;
		
		let toktype: AstoStructure = match self.see() {
			b'>' => {
				AstoStructure::Input
			},
			b'/' => {
				AstoStructure::Description
			},
			b':' => {
				AstoStructure::Version
			},
			// b'$' => {
			// 	AstoStructure::OutputLine
			// },
			b'-' => {
				AstoStructure::Param
			},
			b'{' => {
				AstoStructure::ParamBox
			},
			b'}' => {
				AstoStructure::ParamBox
			},
			// b'[' => {
			// 	AstoStructure::OutputBox
			// },
			// b']' => {
			// 	AstoStructure::OutputBox
			// },
			_ => {
				AstoStructure::Info
			}
		};
		
		self.step();
		
		Ok(Token {
			value: &self.code[initialindex..self.index],
			typeval: toktype,
			start_ln: start_ln,
			start_col: start_col,
			end_ln: self.ln,
			end_col: self.col
		})


	}


	fn tok_comment(&mut self) -> Result<(), String> {
		
		self.step();

		while self.see() != b'\n' {
			self.step();
		}

		Ok(())

	}


	fn tok_param(&mut self) -> Result<Token<'a>, String> {
		
		let start_ln = self.ln;
		let start_col = self.col;

		let initindex = self.index;

		self.step();

		while !self.is_eof() && self.see() != b'\x20' && self.see() != b'\n' && self.see() != b'\r' {
			self.step();
		}

		Ok(Token {
			value: &self.code[initindex..self.index],
			typeval: AstoStructure::Param,
			start_ln: start_ln,
			start_col: start_col,
			end_ln: self.ln,
			end_col: self.col   
		})

	}


	// fn tok_output(&mut self) -> Result<Token<'a>, String> {

	// 	let start_ln = self.ln;
	// 	let start_col = self.col;

		
	// 	// consume '"'
	// 	self.step();
	// 	self.step();

	// 	let initindex = self.index;

	// 	while !self.is_eof() && self.see() != b'"' {
	// 		self.step();
	// 	}

	// 	let value = &self.code[initindex..self.index];

	// 	self.step();

	// 	Ok(Token {
	// 		value: value,
	// 		typeval: AstoStructure::OutputLine,
	// 		start_ln: start_ln,
	// 		start_col: start_col,
	// 		end_ln: self.ln,
	// 		end_col: self.col   
	// 	})

	// }

	// Scanner Functions

	// return current byte
	fn see(&self) -> u8 {
		if self.is_eof() {
			return b'\0'
		}
		self.code[self.index]
	}

	// return next byte
	fn next(&self) -> u8 {
		if self.index + 1 < self.code.len() {
			self.code[self.index + 1]
		} else {
			b'\0'
		}
	}

	// step to next byte
	fn step(&mut self) -> u8 {
		let byte = self.see();

		if byte == b'\n' {
			self.ln += 1;
			self.col = 0
		} else {
			self.col += 1;
		}

		self.index += 1;
		byte
	}

	// verify if it out
	fn is_eof(&self) -> bool {
		self.index >= self.code.len()
	}





	fn is_alpha_extended(&self, byte: u8) -> bool {
		byte.is_ascii_alphabetic() || byte == b'_' || byte > 127
	}

}