pub fn print_help() {
    println!(
"    Usage:
        asciit <view> [options]    (print ascii table)
        asciit \"<str>\"             (string-to-hex conversion)

    <view>:
        -v: print the vertical table (32 x 4) (default)
        -h: print the horizontal table (16 x 8)

    [options]:
        --no-color: print the table without colored output
        --no-explain: do not display explanations for special characters
        -O, --octal: print the table with decimal and octal values instead of hexadecimal
        --help: show this help message
        --version: show asciit version
"
    );
}

pub fn print_table_header(octal: bool, wide_col_num: usize, narrow_col_num: usize) {
    // header top
    print!("┌");

    print!(
        "{}",
        "───────────────────────────────────────┬".repeat(wide_col_num)
    );
    print!("{}", "───────────┬".repeat(narrow_col_num - 1));
    println!("───────────┐");

    let base = if octal { "Oct" } else { "Hex" };
    // header middle
    let tmp = format!("│{:<3} {:<3} {:<31}", "Dec", base, "Chr").repeat(wide_col_num);
    print!("{}", tmp);

    let header = format!("│{:<3} {:<3} {:<3}", "Dec", base, "Chr").repeat(narrow_col_num); // Adjust header for shorter "Chr" column
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

pub fn print_string_to_hex(input: &str) {
    let chars: Vec<char> = input.chars().collect();
    let n = chars.len();
    // Table symbols
    let h = "───"; // horizontal
    let v = "│";   // vertical
    let tl = "┌";  // top left
    let tr = "┐";  // top right
    let bl = "└";  // bottom left
    let br = "┘";  // bottom right
    let t = "┬";   // top junction
    let b = "┴";   // bottom junction

    // Top border
    print!("{}", tl);
    // header column border
    print!("{}", h);
    print!("{}", t);
    for i in 0..n {
        print!("{}", h);
        if i != n - 1 { print!("{}", t); }
    }
    println!("{}", tr);

    // Chars row
    print!("{}", v);
    // header label
    print!("{:^3}", "Str");
    for (_, c) in chars.iter().enumerate() {
        print!("{}", v);
        print!("{:>2} ", c);
    }
    println!("{}", v);

    // Separator
    let sep_left = "├";
    let sep_mid = "┼";
    let sep_h = "─";
    let sep_right = "┤";
    print!("{}", sep_left);
    // header separator
    print!("{}{}{}", sep_h, sep_h, sep_h);
    print!("{}", sep_mid);
    for i in 0..n {
        print!("{}{}{}", sep_h, sep_h, sep_h);
        if i != n - 1 { print!("{}", sep_mid); }
    }
    println!("{}", sep_right);

    // Hex row
    print!("{}", v);
    // header label
    print!("{:^3}", "Hex");
    for (_, c) in chars.iter().enumerate() {
        print!("{}", v);
        print!("{:>2X} ", *c as u8);
    }
    println!("{}", v);

    // Bottom border
    print!("{}", bl);
    // header column border
    print!("{}", h);
    print!("{}", b);
    for i in 0..n {
        print!("{}", h);
        if i != n - 1 { print!("{}", b); }
    }
    println!("{}", br);
    return;
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
    "SO", "SI", "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB", "CAN", "EM", "SUB", "ESC",
    "FS", "GS", "RS", "US", "DEL",
];
