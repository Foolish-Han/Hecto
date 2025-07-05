use super::{Annotation, AnnotationType, Line, SyntaxHighlighter};
use crate::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct RustSyntaxHighlighter {
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

impl RustSyntaxHighlighter {
    fn highlight_digits(line: &Line, result: &mut Vec<Annotation>) {
        for fragment in &line.fragments {
            if fragment.grapheme.len() == 1
                && fragment.grapheme.chars().any(|ch| ch.is_ascii_digit())
            {
                result.push(Annotation {
                    annotation_type: AnnotationType::Digit,
                    start: fragment.start,
                    end: fragment.start.saturating_add(1),
                });
            }
        }
    }
}

impl SyntaxHighlighter for RustSyntaxHighlighter {
    fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&idx)
    }

    fn highlight(&mut self, idx: LineIdx, line: &Line) {
        let mut result = Vec::new();
        Self::highlight_digits(line, &mut result);
        self.highlights.insert(idx, result);
    }
}
