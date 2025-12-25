pub trait Color {
    fn bright_cyan(&self) -> String;
}
impl Color for str {
    fn bright_cyan(&self) -> String {
        format!("\x1b[96m{}\x1b[0m", self)
    }
} 
