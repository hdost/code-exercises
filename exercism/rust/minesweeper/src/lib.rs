pub fn annotate(minefield: &[&str]) -> Vec<String> {
    for x in minefield {
        for y in x {
            y.into()
        }

    }
    unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
}
