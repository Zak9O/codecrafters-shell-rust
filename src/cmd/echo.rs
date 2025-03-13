use super::Cmd;

pub struct Echo<'a>(&'a Vec<String>);
impl<'a> Echo<'a> {
    pub fn new(args: &'a Vec<String>) -> Option<Self> {
        Some(Self(args))
    }
}
impl<'a> Cmd for Echo<'a> {
    fn execute(&self) -> Option<String> {
        Some(self.0.join(" "))
    }
}
