use std::fmt::{Display, Formatter, Result};

use crate::distance::Approximation;

/**
 * An Answer is one possible answer to a problem. It retains various relevant information on the input too, to decide
 * which answer is best.
 */
#[derive(Clone, Debug)]
pub struct Answer {
    pub input: Approximation,
    pub output: Approximation,
}

impl Answer {
    pub fn new(input: &Approximation, output: &Approximation) -> Self {
        Self {
            input: input.to_owned(),
            output: output.to_owned(),
        }
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Answer(input: {}, output: {})", self.input, self.output)
    }
}
