use super::Cmd;

pub struct Echo<'a>(Vec<&'a str>);
impl<'a> Echo<'a> {
    pub fn new(args: Vec<&'a str>) -> Option<Self> {
        Some(Self(args))
    }
}
impl<'a> Cmd for Echo<'a> {
    fn execute(&self) -> () {
        println!("{}", self.0.join(" "))
    }
}
