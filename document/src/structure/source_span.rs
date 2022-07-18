/// Span of something in the parsed source file.
#[derive(Debug, Copy, Clone)]
pub struct SourceSpan {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

impl SourceSpan {
    pub fn new(start: SourcePosition, end: SourcePosition) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}

impl SourcePosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}
