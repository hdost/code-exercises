pub fn reverse(input: &str) -> String {
    let mut stack = String::new();
    let iter = input.chars().rev();
    for l in iter {
        stack.push(l.to_owned());
    }
    return stack;
}
