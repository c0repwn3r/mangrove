use colored::Colorize;

// info, warn, err

pub fn info(text: String) {
    println!("{} {}", "info:".bold(), text.bold());
}

pub fn warn(text: String) {
    println!("{} {}", "warn:".bold().yellow(), text.bold());
}

pub fn err(text: String) {
    println!("{} {}", "err:".bold().red(), text.bold());
}