#[derive(Debug, Clone)]
pub struct Line {
    line: String,
    width: usize,
    index: usize,
    wait: usize,
}

impl Line {
    pub fn new(s: &str) -> Line {
        Line {
            line: s.to_string(),
            width: 16,
            index: 0,
            wait: 1,
        }
    }

    pub fn get(&self) -> &String {
        &self.line
    }
}

unsafe impl Send for Line {}
unsafe impl Sync for Line {}