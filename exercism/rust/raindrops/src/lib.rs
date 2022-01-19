pub fn raindrops(n: u32) -> String {
    let mut output = String::new();

    if n % 3 == 0 {
        output.push_str("Pling");
    }
    if n % 5 == 0 {
        output.push_str("Plang");
    }
    if n % 7 == 0 {
        output.push_str("Plong");
    }
    else {
        if n % 3 != 0 && n % 5 != 0 {
            output = n.to_string();
        }
    }

    output
}
