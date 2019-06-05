#[derive(Debug, Clone)]
pub struct Line {
    line: String,
    width: usize,
    index: i32,
    wait: usize,
    wait_timer: usize,
    increment: i32,
}

impl Line {
    pub fn new(s: &str) -> Line {
        Line {
            line: s.to_string(),
            width: 16,
            index: 0,
            wait: 1,
            wait_timer: 2,
            increment: 1,
        }
    }

    pub fn get(&self) -> &str {
        if self.line.len() <= self.width {
            &self.line
        } else {
            &self.line[self.index as usize..(self.index + self.width as i32) as usize]
        }
    }

    pub fn tick(&mut self) {
        if self.line.len() <= self.width {
            return;
        }

        if self.wait_timer > 0 {
            self.wait_timer -= 1;
            return;
        }

        self.index += self.increment;

        if self.index > (self.line.len() - self.width) as i32 {
            self.index = (self.line.len() - self.width) as i32;
            self.increment = -1;
            self.wait_timer = self.wait;
        } else if self.index < 0 {
            self.index = 0;
            self.increment = 1;
            self.wait_timer = self.wait;
        }
    }
}

unsafe impl Send for Line {}
unsafe impl Sync for Line {}