fn main() {
    let mut res = 42;
    let option = Some(12);
    // Використовуйте `if-let` замість ітерації.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
