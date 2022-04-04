use std::env;
use std::fs;
use std::collections::HashMap;

/*
next features:
1. Trap Vector Table.
2. Remove directives from output.
*/

// Directives and ISA Instructions

// TODO: Turn into enum.
static DIRECTIVES: [&str; 3] = ["orig", "end", "fill"];
static ISA: [&str; 23] = ["ADD", "AND", "NOT", "BR", "BRp", "BRn", "BRz", "BRnz", "BRnp", "BRzp", "BRnzp", "JMP", 
    "JSR", "JSRR", "LD", "LDI", "LDR", "LEA", "ST", "STI", "STR", "TRAP", "RET"];
static BINARY: [&str; 23] = ["0001", "0101", "1001", "0000000", "0000001", "0000100", "0000010", "0000110", "0000101", "0000011", 
    "0000111", "1100", "01001", "01000", "0010", "1010", "0110", "1110", "0011", "1011", "0111", "1111", "1100"];
static ERROR: &str = "XXXX";

// Lexer Modes
const LEXING: u32 = 0;
const COMMENT: u32 = 1;

// Parser Modes
const NORMAL: u32 = 0;
const ORIG: u32 = 1;
const EOL: u32 = 3;
const TO_FILL: u32 = 4;
const FILL: u32 = 5;
const DIR: u32 = 6;

// Error Throwing Function.
// Throws an error, specifying the address and the token at which the error occured. 
// The number is i is for debugging purposes
fn throw_error(pc: u32, cause: &str, i: i32) {
    println!("Error: Illegal Instruction at address {:#0x}: {}. Type {}.", pc - 1, cause, i);
    std::process::exit(1);
}

fn translate_error(cause: &str, i: i32) {
    println!("Error: Malformed Instruction: {}. Type {}.", cause, i);
    std::process::exit(1);
}

fn file_error() {
    println!("Error: Invalid file format. File must be of type .asm.");
    std::process::exit(1);
}

fn parse_file_name(s: &str) {
    const FILE: [char; 3] = ['a', 's', 'm'];
    let mut extension = false;
    let mut i = 0;
    for c in s.chars() {
        if extension {
            if i > 2 {
                file_error();
            }
            if c != FILE[i] {
                file_error();
            }
            i = i + 1;
        } else if c == '.' {
            extension = true;
        }
    }
    if !extension {
        file_error();
    }
}

fn translate(s: &String) -> usize {
    let mut i = 0;
    while i < 23 {
        if ISA[i] == s {
            return i;
        }
        i = i + 1;
    }
    return 24;
}

fn two_comp_imm5(j: u32, toggle: i32) -> String {
    let mut k = j;
    let mut out = String::new();
    let mut dig = 0;
    while k > 0 {
        if k % 2 == 0 {
            out.push_str("0");
        } else {
            out.push_str("1");
        }
        k = k / 2;
        dig = dig + 1;
    }
    while dig < 5 {
        if toggle == -1 {
            out.push_str("1");
        } else {
            out.push_str("0");
        }
        dig = dig + 1;
    }
    return out[0..5].chars().rev().collect::<String>().to_string();
}

fn register_format(j: u32) -> String {
    let mut k = j;
    let mut out = String::new();
    let mut dig_count = 0;
    while k > 0 {
        if k % 2 == 0 {
            out.push_str("0");
        } else {
            out.push_str("1");
        }
        k = k / 2;
        dig_count = dig_count + 1;
    }
    while dig_count < 3 {
        out.push_str("0");
        dig_count = dig_count + 1;
    }
    return out.chars().rev().collect::<String>();
}

fn handle_arithmetic(s: &String, instruction: u32) -> String {
    let mut out = String::new();
    if instruction == 0 || instruction == 1 {
        let mut i = 0;
        for c in s.chars() {
            if i > 1 {
                translate_error(&s, 1);
                return ERROR.to_string();
            }
            if i == 0 && c != 'R' {
                translate_error(&s, 0);
                return ERROR.to_string();
            } else if i == 1 {
                let digit = c.to_digit(10);
                if digit == None {
                    translate_error(&s, 0);
                } else {
                    out.push_str(&register_format(digit.unwrap()));
                }
            }
            i = i + 1
        }
    } else if instruction == 2 {
        let mut j = 0;
        let mut toggle = 1;
        let mut i = 0;
        let mut mode = 0;
        for c in s.chars() {
            if i == 0 {
                if c == '#' {
                    out.push_str("1");
                    mode = 1;
                } else if c == 'R' {
                    out.push_str("000");
                    mode = 2;
                } else {
                    translate_error(&s, 2);
                    return ERROR.to_string();
                }
            } else {
                if c == '-' {
                    toggle = -1;
                } else if mode == 1 {
                    let digit = c.to_digit(10);
                    if digit == None {
                        translate_error(&s, 3);
                    } else {
                        j = j * 10 + digit.unwrap();
                    }
                } else if mode == 2 {
                    let digit = c.to_digit(10);
                    if digit == None {
                        translate_error(&s, 4);
                    } else {
                        out.push_str(&register_format(digit.unwrap()));
                    }
                }
            }
            i = i + 1;
        }
        if mode == 1 {
            if (j > 15 && toggle == 1) || (j > 16 && toggle == -1) {
                translate_error(&s, 5);
            } else {
                out.push_str(&two_comp_imm5(j, toggle));
            }
        }
    }
    return out;
}

fn handle_not(s: &String, instruction: u32) -> String {
    let mut out = String::new();
    let mut i = 0;
    for c in s.chars() {
        if i > 1 {
            translate_error(&s, 1);
            return ERROR.to_string();
        }
        if i == 0 && c != 'R' {
            translate_error(&s, 0);
            return ERROR.to_string();
        } else if i == 1 {
            let digit = c.to_digit(10);
            if digit == None {
                translate_error(&s, 0);
            } else {
                out.push_str(&register_format(digit.unwrap()));
            }
        }
        i = i + 1
    }
    if instruction == 1 {
        out.push_str("111111");
    }
    return out;
}

fn handle_branch(diff: u32) -> String {
    let mut out = String::new();
    return out;
}

// Assembler
fn main() {

    // Setup Lablers Lookup Table
    let mut labels: HashMap<String, u32> = HashMap::new();
    let mut orig: Vec<u32> = Vec::new();

    // Read code from ASM file.
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    parse_file_name(&file);
    let code = fs::read_to_string(file)
        .expect("Critical Error: Unable to read the file.");

    // Setup Values for Lexing
    let mut start = 0;
    let mut end = 0;
    let mut tokens: Vec<String> = vec![];
    let mut eol = false;

    // Initialize Output Variables
    let mut formatted_tokens: Vec<String> = vec![];
    let mut out = String::new();

    // Initialize Program Counter
    let mut pc: u32 = 0x0;

    // Start Lexing
    let mut parse_mode = LEXING;
    for c in code.chars() {
        // Check if we're in a comment
        if parse_mode == COMMENT {
            // Check if we're out of a comment. If we are, switch back to Lexing. 
            if c == '\n' {
                parse_mode = LEXING;
                start = end + 1;
                eol = true;
            }
            // Otherwise, ignore. 
            end = end + 1;
            continue;
        }   
        // Check if we're at the start of a comment.
        // If we are, swtich to Comment mode (ignore text).
        if c == ';' {
            parse_mode = COMMENT;
            end = end + 1;
            continue;
        }
        // Check if whether we're at the end of a token. 
        if c == ' ' || c == '\n' {
            if end - start <= 0 { // Edge case to handle extraneous spaces.
                start = end + 1;
            } else { // else we have a valid token, add to list. 
                tokens.push(code[start..end].to_string());
                start = end + 1;
            }
        }
        // Iterate one character along
        end = end + 1;
        // If we're at a newline character, end the lexing and parse the line. 
        if c == '\n' {
            eol = true;
        }
        // If we are at the end of the code file and there aren't any extraneous spaces
        // add the extra character to the token list and begin parsing. 
        if end == code.len() {
            if c != ' ' && parse_mode == LEXING {
                tokens.push(code[start..end].to_string());
            }
            eol = true;
        }
        // EOL being true means we are at the end of the line => parse the line. 
        if eol {
            // Setup Parsing Variables
            let mut parsed_tokens: Vec<String> = vec![];
            let mut token_num = 0;

            // Dir Mode is set to the type of character we're parsing
            // Behaves differently based on directives. 
            let mut dir_mode = NORMAL;
            // Parse each token in the list.
            while token_num < tokens.len() {
                // Setup to parse each token. 
                let bound = tokens[token_num].len() - 1;
                let mut token_index = 0;
                let mut start = 0;
                // Go through each character in the token, looking for errors. 
                for c in tokens[token_num].chars() {
                    // If we're parsing an .end directive and we find more non-soace text in the line 
                    // this is a syntax error, so throw it. 
                    if dir_mode == EOL {
                        throw_error(pc, &tokens[token_num], 0);
                    }
                    // If we find a period, we're parsing a directive, switch mode and continue.
                    if c == '.' && token_index == 0 {
                        dir_mode = DIR;
                        start = token_index + 1;
                    }
                    // We're parsing a regular instruction, check for errors. 
                    if dir_mode == NORMAL {
                        // If we find a comma mid token, this is a syntax error. 
                        if c == ',' && (token_index != bound || token_num == tokens.len() - 1 || token_num == 0)  {
                            throw_error(pc, &tokens[token_num], 1);
                        }
                        // If we find a comma at any point in a instruction signifier, this is a syntax error. 
                        if c == ',' && ISA.contains(&&tokens[token_num][0..tokens[token_num].len()]) {
                            throw_error(pc, &tokens[token_num], 9);
                        }
                        // otherwise if we don't find a comma at the end of a non instruction-signifier, 
                        // this is a syntax error,  
                        else if c != ',' && (token_index == bound && token_num != tokens.len() - 1 && token_num != 0 
                            && !ISA.contains(&&tokens[token_num][0..tokens[token_num].len()])) {
                            throw_error(pc, &tokens[token_num], 2);
                        }
                    }
                    // We're parsing a directive.
                    else if dir_mode == DIR {
                        // If we find a comma at any point in a directive, this is a syntax error. 
                        if c == ','  {
                            throw_error(pc, &tokens[token_num], 3);
                        }
                        // If we've reached the end of the token, decipher the token and set mode appropriately. 
                        if token_index == bound {
                            // build the directive string. 
                            let mut direct = String::new();
                            direct.push_str(&tokens[token_num][start..token_index + 1]);
                            // search through directives list to find a match. 
                            let mut found = false;
                            for d in DIRECTIVES {
                                if d.eq(&direct) {
                                    found = true;
                                    if d.eq("orig") {
                                        dir_mode = ORIG;
                                        pc = 0x0;
                                    } else if d.eq("end") {
                                        dir_mode = EOL;
                                    } else if d.eq("fill") {
                                        dir_mode = TO_FILL;
                                    }
                                    break;
                                }
                            }
                            // no match found, throw error. 
                            if !found {
                                throw_error(pc, &tokens[token_num], 4);
                            }
                        }
                    }
                    // We're parsing the values for an .orig directive.  
                    else if dir_mode == ORIG {
                        // hex signifier. 
                        if c == 'x' {
                            continue;
                        } 
                        // check to see if number is outside hex, if it is, this is a syntax error. 
                        else if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) {
                            throw_error(pc, &tokens[token_num], 5);
                        } 
                        // turn hex string into hex number digit by digit. 
                        else {
                            let mut temp = 0;
                            // convert a-f into digits.
                            if c >= 'a' && c <= 'f' {
                                if c == 'a' {
                                    temp = 10;
                                } else if c == 'b' {
                                    temp = 11;
                                } else if c == 'c' {
                                    temp = 12;
                                } else if c == 'd' {
                                    temp = 13;
                                } else if c == 'e' {
                                    temp = 14;
                                } else if c == 'f' {
                                    temp = 15;
                                } else {
                                    temp = 0;
                                }
                            }
                            // perform char to digit conversion. 
                            let digit = c.to_digit(10);
                            let mut val = 0;
                            // bounds checking
                            if digit == None && temp == 0 {
                                throw_error(pc, &tokens[token_num], 6);
                            } else {
                                if temp != 0 {
                                    val = temp;
                                } else {
                                    val = digit.unwrap();
                                }
                            }
                            // increment PC. 
                            pc = pc * 16 + val;
                            orig.push(pc);
                        }
                    }
                    // We're parsing a fill directive in some way.  
                    else if dir_mode == TO_FILL || dir_mode == FILL {
                        dir_mode = FILL;
                        // Convert string to digit. If it fails, this is a syntax error. 
                        let digit = c.to_digit(10);
                        if digit == None {
                            throw_error(pc, &tokens[token_num], 7);
                        }
                    }
                    // Handle the end of a fill directive's argument
                    if dir_mode == FILL && token_index == bound {
                        // If there's still more tokens ahead this is a syntax error. 
                        if token_num != tokens.len() - 1 {
                            throw_error(pc, &tokens[token_num], 8);
                        }
                        // if there is no error, switch back to normal handling. 
                        dir_mode = NORMAL;
                    }
                    // Add the parsed, corrected token to the parsed tokens list. 
                    token_index = token_index + 1;
                    if token_index == bound + 1 && (dir_mode == NORMAL || dir_mode == TO_FILL) {
                        if c == ',' {
                            parsed_tokens.push(tokens[token_num][0..tokens[token_num].len() - 1].to_string());
                        } else {
                            // If we're handling a label, add the label to the lookup table, along with its address. 
                            // Remember, PC is always one ahead of the current instruction address.  
                            if token_num == 0 && !ISA.contains(&&tokens[token_num][0..tokens[token_num].len()]) {
                                labels.insert(tokens[token_num][0..tokens[token_num].len()].to_string(), pc - 1);
                            }
                            parsed_tokens.push(tokens[token_num][0..tokens[token_num].len()].to_string());
                        }
                    }
                }
                // Move to next token. 
                token_num = token_num + 1;
            }
            // We're done parsing the line.

            // Add each parsed token to the string.
            for l in parsed_tokens {
                formatted_tokens.push(l.to_string());
            }
            // If there were tokens to parse, increment PC by 1, else this was just an empty line. 
            if tokens.len() > 0 {
                pc = pc + 1;
            }
            // Reset variables before moving to lexing next line. 
            tokens.clear();
            eol = false;
        }
    }
    
    let mut prev_res = 0;
    let mut instruction: u32 = 0;
    let mut index = 0;
    // Reinitialize PC
    pc = *orig.get(index).unwrap();
    index = index + 1;
    for token in formatted_tokens {
        let result = translate(&token);
        if result == 24 {
            if prev_res == 0 || prev_res == 1 {
                out.push_str(&handle_arithmetic(&token, instruction));
            } else if prev_res == 2 {
                out.push_str(&handle_not(&token, instruction));
            } else if prev_res >= 3 && prev_res <= 10 {
                out.push_str(&handle_branch(0));
            }
            instruction = instruction + 1;
        } else {
            out.push_str(&BINARY[result]);
            prev_res = result;
            instruction = 0;
            pc = pc + 1;
        }
    }

    // Once we've parsed the entire code string, write the output string to the file. 
    fs::write("output.txt", out)
        .expect("Critical Error: Cannot produce machine code output file.");
    
    
}
