use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Clone, Debug)]
pub struct TextLine {
    pub line: String,
}

impl Display for TextLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        std::fmt::Display::fmt(&self.line, f)
    }
}
