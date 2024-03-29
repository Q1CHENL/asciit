use colored::*;
use std::{env, process::exit};
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let cnt = args.len();
    if cnt > 1 {
        eprintln!("{} {}", "Wrong number of arguments: ", cnt);
        exit(1);
    }
    let mut row_number = 32;
    let mut col_number = 4;
    for argument in args {
        match argument.as_str() {
            "h" => {
                row_number = 16;
                col_number = 8
            }
            "v" => {
                row_number = 32;
                col_number = 4
            }
            "--help" => {
                print_help();
                exit(0);
            }
            &_ => {
                eprintln!("{} {}", "Unknown arguments: ", argument);
                exit(1)
            }
        }
    }
    let specials: [&str; 33] = [
        "NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL", "BS", "HT", "LF", "VT", "FF", "CR",
        "SO", "SI", "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB", "CAN", "EM", "SUB",
        "ESC", "FS", "GS", "RS", "US", "DEL",
        ];
        // Header
        print!("┌");
        print!("{}", "───────────┬".repeat(col_number - 1));
        print!("───────────");
        println!("┐");
        let header = format!("│{:<3} {:<3} {:<3}", "Dec", "Hex", "Chr").repeat(col_number);
        
        print!("{}", header);
        println!("│");
        print!("├");
        print!("{}", "───────────┼".repeat(col_number - 1));
        println!("───────────┤");
        
        for line in 0..row_number {
            for col in 0..col_number {
                // Explicitly cast to u8 to avoid type ambiguity
                let i = (line + col * row_number) as u8;
                
                let ch = if i.is_ascii_graphic() || i == 32 {
                    let char_i = i as char;
                    if char_i.is_lowercase() {
                        format!("{}  ", char_i.to_string().bright_blue())
                    } else if char_i.is_uppercase() {
                        format!("{}  ", char_i.to_string().blue())
                    } else if char_i.is_ascii_digit() {
                        format!("{}  ", char_i.to_string().green())
                    } else {
                        char_i.to_string()
                    }
                } else if i == 127 {
                    specials[32].to_string()
                } else {
                    specials[line].to_string()
            };
            print!("│{:<3} {:<3X} {:<3}", i, i, ch);
        }
        println!("│");
    }
    print!("└");
    print!("{}", "───────────┴".repeat(col_number - 1));
    print!("───────────");
    println!("┘");
}


fn print_help() {
    println!(
        "Usage:\n
    ascii h: print the horizontal table (16 x 8) (default)\n
    ascii v: print the vertical table (32 x 4)\n
    ascii --help: print this help message"
    );
}