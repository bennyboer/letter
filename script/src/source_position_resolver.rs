use document::structure::SourcePosition;

struct LineRange {
    start_offset: usize,
    end_offset: usize,
}

pub(crate) struct SourcePositionResolver {
    line_ranges: Vec<LineRange>,
    cache_invalid: bool,
    cached_line_start_offset: usize,
    cached_line_end_offset: usize,
    cached_line_number: usize,
}

/// Utility for looking up source positions (line number + column) from a source offset.
impl SourcePositionResolver {
    pub(crate) fn from_str(src: &str) -> Self {
        Self {
            line_ranges: Self::find_line_ranges(src),
            cache_invalid: true,
            cached_line_start_offset: 0,
            cached_line_end_offset: 0,
            cached_line_number: 1,
        }
    }

    pub(crate) fn lookup(&mut self, offset: usize) -> Option<SourcePosition> {
        let result = self.validate_cache_for_offset(offset);
        if result.is_err() {
            return None;
        }

        let line = self.cached_line_number;
        let column = self.calculate_column_for_offset(offset);

        Some(SourcePosition { line, column })
    }

    fn calculate_column_for_offset(&self, offset: usize) -> usize {
        offset - self.cached_line_start_offset + 1
    }

    fn validate_cache_for_offset(&mut self, offset: usize) -> Result<(), ()> {
        let is_invalid = self.is_cache_invalid_for_offset(offset);
        if is_invalid {
            self.update_cache_for_offset(offset)?;
        }

        Ok(())
    }

    fn is_cache_invalid_for_offset(&self, offset: usize) -> bool {
        self.cache_invalid
            || offset < self.cached_line_start_offset
            || offset > self.cached_line_end_offset
    }

    fn update_cache_for_offset(&mut self, offset: usize) -> Result<(), ()> {
        let is_line_number_cached = !self.cache_invalid;

        let mut start_line_index = if self.cache_invalid {
            0
        } else {
            self.cached_line_number - 1
        };

        self.cache_invalid = true;

        let mut line_ranges = self.line_ranges.iter().collect::<Vec<&LineRange>>();
        let max_line_index = line_ranges.len() - 1;
        let should_reverse_search = offset < self.cached_line_start_offset;
        if should_reverse_search {
            line_ranges.reverse();
            start_line_index = max_line_index - start_line_index;
        }

        let start_line_index = if is_line_number_cached {
            start_line_index + 1 // Optimizing search by starting at next line
        } else {
            start_line_index
        };

        let line_ranges = &line_ranges[start_line_index..];
        for (idx, line_range) in line_ranges.iter().enumerate() {
            let mut line_index = start_line_index + idx;

            let is_in_bounds = offset >= line_range.start_offset && offset <= line_range.end_offset;
            if is_in_bounds {
                if should_reverse_search {
                    line_index = max_line_index - line_index;
                }
                self.cached_line_number = line_index + 1;
                self.cached_line_start_offset = line_range.start_offset;
                self.cached_line_end_offset = line_range.end_offset;
                self.cache_invalid = false;

                return Ok(());
            }
        }

        return Err(()); // Could not find line number for offset
    }

    fn find_line_ranges(src: &str) -> Vec<LineRange> {
        let mut result = Vec::new();

        let mut start_offset = 0;
        for (offset, c) in src.as_bytes().iter().enumerate() {
            if *c == b'\n' {
                let end_offset = offset;
                let line_range = LineRange {
                    start_offset,
                    end_offset,
                };
                result.push(line_range);

                start_offset = offset + 1;
            }
        }

        let final_end_offset = src.len() - 1;
        let final_line_range = LineRange {
            start_offset,
            end_offset: final_end_offset,
        };
        result.push(final_line_range);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_source_position_for_zero_offset() {
        // Given: A source text with multiple lines
        let src = "Lorem ipsum dolor sit amet,
consectetur adipiscing elit.";

        // And: A source position resolver for the source text
        let mut resolver = SourcePositionResolver::from_str(src);

        // When: Looking up the source position for an offset of 0
        let pos = resolver.lookup(0);

        // Then: The source position should be Some
        assert!(pos.is_some());

        // And: The source position should be line 1, column 1
        let pos = pos.unwrap();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 1);
    }

    #[test]
    fn should_find_source_position_for_offset_at_end_of_range() {
        // Given: A source text with multiple lines
        let src = "Lorem ipsum dolor sit amet, 
consectetur adipiscing elit.";

        // And: A source position resolver for the source text
        let mut resolver = SourcePositionResolver::from_str(src);

        // When: Looking up the source position for the offset at the end of the source text
        let pos = resolver.lookup(56);

        // Then: The source position should be Some
        assert!(pos.is_some());

        // And: The source position should be line 2, column 28
        let pos = pos.unwrap();
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 28);
    }

    #[test]
    fn should_return_none_for_offset_out_of_range() {
        // Given: A source text with multiple lines
        let src = "Lorem ipsum dolor sit amet,
consectetur adipiscing elit.";

        // And: A source position resolver for the source text
        let mut resolver = SourcePositionResolver::from_str(src);

        // When: Looking up the source position for an offset of 0
        let pos = resolver.lookup(238432);

        // Then: The source position should be None
        assert!(pos.is_none());
    }

    #[test]
    fn should_find_source_position_for_offset() {
        // Given: A source text with multiple lines
        let src = "Lorem ipsum dolor 
sit amet, consectetur 
adipiscing elit.";

        // And: A source position resolver for the source text
        let mut resolver = SourcePositionResolver::from_str(src);

        // When: Looking up the source position for the offset 44
        let pos = resolver.lookup(44);

        // Then: The source position should be Some
        assert!(pos.is_some());

        // And: The source position should be line 3, column 3
        let pos = pos.unwrap();
        assert_eq!(pos.line, 3);
        assert_eq!(pos.column, 3);
    }

    #[test]
    fn should_find_source_position_for_multiple_offsets() {
        // Given: A source text with multiple lines
        let src = "Lorem ipsum dolor 
sit amet, consectetur 
adipiscing elit.";

        // And: A source position resolver for the source text
        let mut resolver = SourcePositionResolver::from_str(src);

        // When: Looking up the source position for the offset 10
        let pos = resolver.lookup(10);

        // Then: The source position should be line 1, column 11
        let pos = pos.unwrap();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 11);

        // When: Looking up the source position for the offset 25
        let pos = resolver.lookup(25);

        // Then: The source position should be line 2, column 6
        let pos = pos.unwrap();
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 7);

        // When: Looking up the source position for the offset 3
        let pos = resolver.lookup(3);

        // Then: The source position should be line 1, column 4
        let pos = pos.unwrap();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 4);

        // When: Looking up the source position for the offset 44
        let pos = resolver.lookup(44);

        // Then: The source position should be line 3, column 3
        let pos = pos.unwrap();
        assert_eq!(pos.line, 3);
        assert_eq!(pos.column, 3);
    }
}
