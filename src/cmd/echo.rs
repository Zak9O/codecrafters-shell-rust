use super::{Cmd, StdOutput};

pub struct Echo<'a>(&'a Vec<String>);
impl<'a> Echo<'a> {
    pub fn new(args: &'a Vec<String>) -> Option<Self> {
        Some(Self(args))
    }
}
impl<'a> Cmd for Echo<'a> {
    fn execute(&self, StdOutput(stdout,_): &mut StdOutput) -> () {
        stdout.write(self.0.join(" ").as_bytes()).unwrap();
        stdout.write(b"\n").unwrap();
    }
}
