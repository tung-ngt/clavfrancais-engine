pub trait InputSimulator {
    fn text(content: &str);
    fn character(c: char);
}