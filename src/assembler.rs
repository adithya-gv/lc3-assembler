use std::env;
use std::fs;

/*
next features:
1. comments
2. labels
*/

static DIRECTIVES: [&str; 2] = ["orig", "end"];

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
    let mut v: Vec<String> = vec![];
    let mut eol = false;
    let mut out = String::new();
    let mut pc: u32 = 0x0;
    for c in code.chars() {
        if c == ' ' || c == '\n' {
            if end - start <= 0 {
                start = end + 1;
            } else {
                v.push(code[start..end].to_string());
                start = end + 1;
            }
        }
        end = end + 1;
        if c == '\n' {
            eol = true;
        }
        if end == code.len() {
            if c != ' ' {
                v.push(code[start..end].to_string());
            }
            eol = true;
        }
        if eol {
            let mut lexed: Vec<String> = vec![];
            let mut i = 0;
            let mut dir = 0;
            while i < v.len() {
                let j = v[i].len() - 1;
                let mut k = 0;
                let mut start = 0;
                for c in v[i].chars() {
                    if dir == 3 {
                        throw_error(pc, &v[i], 0);
                    }
                    if c == '.' && k == 0 {
                        dir = 1;
                        start = k + 1;
                    }
                    if dir == 0 {
                        if c == ',' && (k != j || i == v.len() - 1 || i == 0)  {
                            throw_error(pc, &v[i], 1);
                        }
                        if c != ',' && k == j && i != v.len() - 1 && i != 0 {
                            throw_error(pc, &v[i], 2);
                        }
                    } else if dir == 1 {
                        if c == ','  {
                            throw_error(pc, &v[i], 3);
                        }
                        if k == j {
                            let mut direct = String::new();
                            direct.push_str(&v[i][start..k + 1]);
                            let mut found = false;
                            for d in DIRECTIVES {
                                if d.eq(&direct) {
                                    found = true;
                                    if d.eq("orig") {
                                        dir = 2;
                                        pc = 0x0;
                                    } else if d.eq("end") {
                                        dir = 3;
                                    }
                                    break;
                                }
                            }
                            if !found {
                                throw_error(pc, &v[i], 4);
                            }
                        }
                    } else if dir == 2 {
                        if c == 'x' {
                            continue;
                        } else if !(c >= '0' && c <= '9') {
                            throw_error(pc, &v[i], 5);
                        } else {
                            let dig = c.to_digit(10);
                            let mut val = 0;
                            if dig == None {
                                throw_error(pc, &v[i], 6);
                            } else {
                                val = dig.unwrap();
                            }
                            pc = pc * 16 + val;
                        }
                    }
                    k = k + 1;
                    if k == j + 1 && dir == 0 {
                        if c == ',' {
                            lexed.push(v[i][0..v[i].len() - 1].to_string());
                        } else {
                            lexed.push(v[i][0..v[i].len()].to_string());
                        }
                    }
                }
                i = i + 1;
            }
            i = 0;
            let mut wrote = false;
            for l in lexed {
                if i != 0 {
                    out.push_str(" ");
                }
                out.push_str(&l);
                wrote = true;
                i = i + 1;
            }
            if wrote {
                out.push_str("\n");
            }
            v.clear();
            pc = pc + 1;
            eol = false;
        }
        
        
    }
    fs::write("output.txt", out)
        .expect("Critical Error: Cannot produce machine code output file.");
    
}
