use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineItem<T>
where
    T: AsRef<str>,
{
    line_number: usize,
    line_contents: T,
}

impl<T> LineItem<T>
where
    T: AsRef<str>,
{
    pub fn new(line_number: usize, line_contents: T) -> Self {
        LineItem {
            line_number,
            line_contents,
        }
    }

    pub fn line_number(&self) -> usize {
        self.line_number
    }

    pub fn line_contents(&self) -> &T {
        &self.line_contents
    }
}

pub type Line = LineItem<String>;

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.line_number, self.line_contents)
    }
}
