use std::collections::{HashSet, VecDeque};
use std::error::Error;

struct SlidingWindow {
    max_size: usize,
    buf: VecDeque<char>,
}

impl SlidingWindow {
    fn new(size: usize) -> Self {
        Self {
            max_size: size,
            buf: VecDeque::new(),
        }
    }

    fn is_full(&self) -> bool {
        self.max_size == self.buf.len()
    }

    fn push(&mut self, c: char) {
        if self.buf.len() == self.max_size {
            self.buf.pop_back();
        }
        self.buf.push_front(c);
    }

    // this impl takes like 15ms.. can be improved
    fn all_unique_chars(&self) -> bool {
        let unique_chars = HashSet::<&char>::from_iter(self.buf.iter());
        unique_chars.len() == self.buf.len()
    }
}

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let f = input.get(0).unwrap().chars();

    let mut first_packet_window = SlidingWindow::new(4);
    let mut start_of_message_window = SlidingWindow::new(14);
    let mut first_packet_index = 0;
    let mut start_of_message_index = 0;

    for (i, c) in f.enumerate() {
        first_packet_window.push(c);
        start_of_message_window.push(c);

        if first_packet_index == 0
            && first_packet_window.is_full()
            && first_packet_window.all_unique_chars()
        {
            first_packet_index = i + 1;
        }

        if start_of_message_index == 0
            && start_of_message_window.is_full()
            && start_of_message_window.all_unique_chars()
        {
            start_of_message_index = i + 1;
        }
    }

    Ok((
        first_packet_index.to_string(),
        start_of_message_index.to_string(),
    ))
}
