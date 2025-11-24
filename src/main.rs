fn main() {
    // Print classic greeting
    println!("Hello, World! 👋");

    // Bonus: Interactive greeting
    println!("What is your name?");

    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    // Trim whitespace and newline
    let name = name.trim();

    // Use a fallback
    let display_name = if name.is_empty() { "Stranger" } else { name };

    println!("Hello, {display_name}! Welcome to Rust! 🚀");
}
