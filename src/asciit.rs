use colored::*;

fn main() {
    let specials: [&str; 33] = [
        "NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL", "BS", "HT", "LF", "VT", "FF", "CR",
        "SO", "SI", "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB", "CAN", "EM", "SUB",
        "ESC", "FS", "GS", "RS", "US", "DEL",
    ];
    // Header
    print!("┌");
    print!("{}", "───────────┬".repeat(3));
    print!("───────────");
    println!("┐");
    let header = format!("│{:<3} {:<3} {:<3}", "Dec", "Hex", "Chr").repeat(4);

    print!("{}", header);
    println!("│");
    print!("├");
    print!("{}", "───────────┼".repeat(3));
    println!("───────────┤");

    for line in 0..32 {
        for col in 0..4 {
            // Explicitly cast to u8 to avoid type ambiguity
            let i = (line + col * 32) as u8;

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
                specials[line as usize].to_string()
            };
            print!("│{:<3} {:<3X} {:<3}", i, i, ch);
        }
        println!("│");
    }
    print!("└");
    print!("{}", "───────────┴".repeat(3));
    print!("───────────");
    println!("┘");
}
