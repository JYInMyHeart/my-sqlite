use std::io::{stdin,stdout,Write};
fn main() {
    let mut input = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("input");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    print!("{}", input);

}
