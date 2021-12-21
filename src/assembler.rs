use std::env;
use std::fs;

/*
next features:
1. labels
*/

static DIRECTIVES: [&str; 2] = ["orig", "end"];
const NORMAL: u32 = 0;
const ORIG: u32 = 1;
const PC: u32 = 2;
const EOL: u32 = 3;

const PARSING: u32 = 0;
const COMMENT: u32 = 1;

fn throw_error(pc: u32, cause: &str, i: i32) {
    println!("Exception: Illegal Instruction at address {:#0x}: {}. Type {}", pc, cause, i);
    std::process::exit(1);
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let code = fs::read_to_string(file)
        .expect("Critical Error: Unable to read the file.");

    let mut start = 0;
    let mut end = 0;
    let mut tokens: Vec<String> = vec![];
    let mut eol = false;
    let mut out = String::new();
    let mut pc: u32 = 0x0;
    let mut parse_mode = PARSING;
    for c in code.chars() {
        if parse_mode == COMMENT {
            if c == '\n' {
                parse_mode = PARSING;
                start = end + 1;
                eol = true;
            }
            end = end + 1;
            continue;
        }
        if c == ';' {
            parse_mode = COMMENT;
            end = end + 1;
            continue;
        }
        if c == ' ' || c == '\n' {
            if end - start <= 0 {
                start = end + 1;
            } else {
                tokens.push(code[start..end].to_string());
                start = end + 1;
            }
        }
        end = end + 1;
        if c == '\n' {
            eol = true;
        }
        if end == code.len() {
            if c != ' ' && parse_mode == PARSING {
                tokens.push(code[start..end].to_string());
            }
            eol = true;
        }
        if eol {
            let mut lexed_tokens: Vec<String> = vec![];
            let mut token_num = 0;
            let mut dir_mode = NORMAL;
            while token_num < tokens.len() {
                let bound = tokens[token_num].len() - 1;
                let mut token_index = 0;
                let mut start = 0;
                for c in tokens[token_num].chars() {
                    if dir_mode == EOL {
                        throw_error(pc, &tokens[token_num], 0);
                    }
                    if c == '.' && token_index == 0 {
                        dir_mode = ORIG;
                        start = token_index + 1;
                    }
                    if dir_mode == NORMAL {
                        if c == ',' && (token_index != bound || token_num == tokens.len() - 1 || token_num == 0)  {
                            throw_error(pc, &tokens[token_num], 1);
                        }
                        if c != ',' && token_index == bound && token_num != tokens.len() - 1 && token_num != 0 {
                            throw_error(pc, &tokens[token_num], 2);
                        }
                    } else if dir_mode == ORIG {
                        if c == ','  {
                            throw_error(pc, &tokens[token_num], 3);
                        }
                        if token_index == bound {
                            let mut direct = String::new();
                            direct.push_str(&tokens[token_num][start..token_index + 1]);
                            let mut found = false;
                            for d in DIRECTIVES {
                                if d.eq(&direct) {
                                    found = true;
                                    if d.eq("orig") {
                                        dir_mode = PC;
                                        pc = 0x0;
                                    } else if d.eq("end") {
                                        dir_mode = EOL;
                                    }
                                    break;
                                }
                            }
                            if !found {
                                throw_error(pc, &tokens[token_num], 4);
                            }
                        }
                    } else if dir_mode == PC {
                        if c == 'x' {
                            continue;
                        } else if !(c >= '0' && c <= '9') {
                            throw_error(pc, &tokens[token_num], 5);
                        } else {
                            let digit = c.to_digit(10);
                            let mut val = 0;
                            if digit == None {
                                throw_error(pc, &tokens[token_num], 6);
                            } else {
                                val = digit.unwrap();
                            }
                            pc = pc * 16 + val;
                        }
                    }
                    token_index = token_index + 1;
                    if token_index == bound + 1 && dir_mode == NORMAL {
                        if c == ',' {
                            lexed_tokens.push(tokens[token_num][0..tokens[token_num].len() - 1].to_string());
                        } else {
                            lexed_tokens.push(tokens[token_num][0..tokens[token_num].len()].to_string());
                        }
                    }
                }
                token_num = token_num + 1;
            }
            token_num = 0;
            let mut wrote = false;
            for l in lexed_tokens {
                if token_num != 0 {
                    out.push_str(" ");
                }
                out.push_str(&l);
                wrote = true;
                token_num = token_num + 1;
            }
            if wrote {
                out.push_str("\n");
            }
            tokens.clear();
            pc = pc + 1;
            eol = false;
        }
        
        
    }
    fs::write("output.txt", out)
        .expect("Critical Error: Cannot produce machine code output file.");
    
}
