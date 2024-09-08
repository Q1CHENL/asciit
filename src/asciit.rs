use colored::*;
use std::{env, process::exit};
mod print;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut row_number = 32;
    let mut col_number = 4;
    let mut use_color = true;
    let mut print_special_explained = true;
    let mut horizontal = false;

    // Parse arguments
    for argument in &args {
        match argument.as_str() {
            "h" => {
                row_number = 16;
                col_number = 8;
                horizontal = true;
            }
            "v" => {
                row_number = 32;
                col_number = 4;
            }
            "--no-color" => {
                use_color = false;
            }
            "--no-explain" => {
                print_special_explained = false;
            }
            "--help" => {
                print::print_help();
                exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {}", argument);
                exit(1);
            }
        }
    }

    // Number of columns with explainations for special chars
    let wide_col_num = if horizontal && print_special_explained {
        2
    } else if print_special_explained {
        1
    } else {
        0
    };
    let narrow_col_num = col_number - wide_col_num;

    // Header
    print::print_table_header(wide_col_num, narrow_col_num);

    // Print table body row by row
    for row in 0..row_number {
        for col in 0..col_number {
            let i = (row + col * row_number) as u8;

            let ch = format_character(
                i,
                use_color,
                if print_special_explained {
                    &print::SPECIALS_EXPLAINED
                } else {
                    &print::SPECIALS
                },
            );
            let padded_str = if i < 32 && print_special_explained {
                // For special characters with descriptions
                format!("│{:<3} {:<3X} {:<31}", i, i, ch)
            } else {
                // For normal characters, no extra padding
                format!("│{:<3} {:<3X} {:<3}", i, i, ch)
            };

            print!("{}", padded_str);
        }
        println!("│");
    }

    // Footer
    print::print_table_footer(wide_col_num, narrow_col_num);
}

// return the final formatted (colored) character
fn format_character(i: u8, use_color: bool, specials_explained: &[&str; 33]) -> String {
    if i.is_ascii_graphic() || i == 32 {
        let char_i = i as char;
        if use_color {
            if char_i.is_lowercase() {
                return format!("{}  ", char_i.to_string().bright_blue());
            } else if char_i.is_uppercase() {
                return format!("{}  ", char_i.to_string().blue());
            } else if char_i.is_ascii_digit() {
                return format!("{}  ", char_i.to_string().green());
            }
        }
        return char_i.to_string();
    } else if i == 127 {
        return specials_explained[32].to_string();
    } else {
        return specials_explained[i as usize].to_string();
    }
}
