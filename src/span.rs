#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Position {
    line: u32,
    col: u32,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Span {
    start: Position,
    end: Position,
}

impl core::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{} - {}:{}",
            self.start.line + 1,
            self.start.col,
            self.end.line + 1,
            self.end.col - 1
        )
    }
}

impl Span {
    pub fn new(line: u32, start: u32, end: u32) -> Span {
        Span {
            start: Position { line, col: start },
            end: Position { line, col: end },
        }
    }

    pub fn compose(a: &Span, b: &Span) -> Span {
        Span {
            start: a.start,
            end: b.end,
        }
    }
}
