use std::collections::{HashSet, VecDeque};
use std::error::Error;

struct SlidingWindow {
    max_size: usize,
    buf: VecDeque<char>,
    done: bool,
    chars_processed: u32,
}

impl SlidingWindow {
    fn new(size: usize) -> Self {
        Self {
            max_size: size,
            buf: VecDeque::new(),
            done: false,
            chars_processed: 0,
        }
    }

    fn push(&mut self, c: char) {
        if !self.done {
            if self.buf.len() == self.max_size {
                self.buf.pop_back();
            }
            self.buf.push_front(c);
            self.chars_processed += 1;
            self.done = self.is_done();
        }
    }

    // this impl takes like 15ms.. can be improved
    fn is_done(&self) -> bool {
        if !self.done && self.buf.len() == self.max_size {
            let unique_chars = HashSet::<&char>::from_iter(self.buf.iter());
            unique_chars.len() == self.buf.len()
        } else {
            self.done
        }
    }
}

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let f = input.get(0).unwrap().chars();

    let mut first_packet_window = SlidingWindow::new(4);
    let mut start_of_message_window = SlidingWindow::new(14);

    for c in f {
        first_packet_window.push(c);
        start_of_message_window.push(c);
    }

    Ok((
        first_packet_window.chars_processed.to_string(),
        start_of_message_window.chars_processed.to_string(),
    ))
}
