use super::*;
use core::iter::FromIterator;

#[derive(Clone, PartialEq, Copy)]
/// The type of the insert mode
pub enum InsertMode {
    /// Append text (after the cursor)
    Append,
    /// Insert text (before the cursor)
    Insert,
    /// Replace text (on the cursor)
    Replace,
}

#[derive(Clone, PartialEq, Copy)]
/// The insert options
pub struct InsertOptions {
    /// The mode type
    pub mode: InsertMode,
}

impl Editor {
    /// Insert text
    pub fn insert(&mut self, k: Key, InsertOptions { mode: mode }: InsertOptions) {
        let mut x = self.x();
        let mut y = self.y();
        match mode {
            InsertMode::Insert => match k {
                Key::Char('\n') => {
                    let ln = self.text[y].clone();
                    let (slice, _) = ln.as_slices();

                    let first_part = (&slice[..x]).clone();
                    let second_part = (&slice[x..]).clone();

                    self.text[y] = VecDeque::from_iter(first_part.iter().map(|x| *x));
                    self.text.insert(y + 1, VecDeque::from_iter(second_part.iter().map(|x| *x)));

                    self.goto_next();
                },
                Key::Escape => { // Escape key
                    self.cursor_mut().mode = Mode::Command(CommandMode::Normal);
                },
                Key::Backspace => { // Backspace
                    if self.x() != 0 || self.y() != 0 {
                        self.goto_previous();
                        self.delete();
                    }
                },
                Key::Char(c) => {
                    self.text[y].insert(x, c);
                    self.goto_next();
                }
                _ => {},
            },
            InsertMode::Replace => match k {
                Key::Char(c) => {
                    if x == self.text[y].len() {
                        self.goto_next();
                        x = self.x();
                        y = self.y();
                    }

                    if self.text.len() != y {
                        if self.text[y].len() == x {
                            self.goto_next();
                        } else {
                            self.text[y][x] = c;
                        }
                    }
                    self.goto_next();
                },
                _ => {},
            },
            _ => {},
        }
    }
}
