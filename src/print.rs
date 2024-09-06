pub fn print_help() {
    println!(
        "   Usage: asciit <view> [options]\n
    <view>:
        v: print the vertical table (32 x 4) (default)
        h: print the horizontal table (16 x 8)\n
    [options]:
        --no-color: print the table without colored output
        --no-explain: do not display explanations for special characters"
    );
}

pub fn print_table_header(wide_col_num: usize, narrow_col_num: usize) {
    // header top
    print!("┌");

    print!(
        "{}",
        "───────────────────────────────────────┬".repeat(wide_col_num)
    );
    print!("{}", "───────────┬".repeat(narrow_col_num - 1));
    println!("───────────┐");

    // header middle
    let tmp = format!("│{:<3} {:<3} {:<31}", "Dec", "Hex", "Chr").repeat(wide_col_num);
    print!("{}", tmp);

    let header = format!("│{:<3} {:<3} {:<3}", "Dec", "Hex", "Chr").repeat(narrow_col_num); // Adjust header for shorter "Chr" column
    print!("{}", header);
    println!("│");

    // header bottom
    print!("├");
    print!(
        "{}",
        "───────────────────────────────────────┼".repeat(wide_col_num)
    );
    print!("{}", "───────────┼".repeat(narrow_col_num - 1));
    println!("───────────┤");
}


pub fn print_table_footer(wide_col_num: usize, narrow_col_num: usize) {
    print!("└");
    print!(
        "{}",
        "───────────────────────────────────────┴".repeat(wide_col_num)
    );
    print!("{}", "───────────┴".repeat(narrow_col_num - 1));
    println!("───────────┘");
}


pub const SPECIALS_EXPLAINED: [&str; 33] = [
    "NUL ('\\0', null character)",
    "SOH (start of heading)",
    "STX (start of text)",
    "ETX (end of text)",
    "EOT (end of transmission)",
    "ENQ (enquiry)",
    "ACK (acknowledge)",
    "BEL ('\\a', bell)",
    "BS  ('\\b', backspace)",
    "HT  ('\\t', horizontal tab)",
    "LF  ('\\n', line feed)",
    "VT  ('\\v', vertical tab)",
    "FF  ('\\f', form feed)",
    "CR  ('\\r', carriage return)",
    "SO  (shift out)",
    "SI  (shift in)",
    "DLE (data link escape)",
    "DC1 (device control 1)",
    "DC2 (device control 2)",
    "DC3 (device control 3)",
    "DC4 (device control 4)",
    "NAK (negative acknowledge)",
    "SYN (synchronous idle)",
    "ETB (end of transmission block)",
    "CAN (cancel)",
    "EM  (end of medium)",
    "SUB (substitute)",
    "ESC (escape)",
    "FS  (file separator)",
    "GS  (group separator)",
    "RS  (record separator)",
    "US  (unit separator)",
    "DEL",
];

pub const SPECIALS: [&str; 33] = [
    "NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL", "BS", "HT", "LF", "VT", "FF", "CR",
    "SO", "SI", "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB", "CAN", "EM", "SUB",
    "ESC", "FS", "GS", "RS", "US", "DEL",
];
