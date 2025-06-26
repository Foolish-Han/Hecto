use std::collections::HashMap;

use super::super::super::{Annotation, AnnotationType, Line};
use crate::prelude::*;

#[derive(Default)]
pub struct Highlighter<'a> {
    matched_word: Option<&'a str>,
    selected_match: Option<Location>,
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

impl<'a> Highlighter<'a> {
    pub fn new(matched_word: Option<&'a str>, selected_match: Option<Location>) -> Self {
        Self {
            matched_word,
            selected_match,
            highlights: HashMap::new(),
        }
    }
    pub fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&idx)
    }
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
    fn highlight_matched_words(&self, line: &Line, result: &mut Vec<Annotation>) {
        if let Some(matched_word) = self.matched_word {
            if matched_word.is_empty() {
                return;
            }
            line.find_all(matched_word, 0..line.len())
                .iter()
                .for_each(|(start, _)| {
                    result.push(Annotation {
                        annotation_type: AnnotationType::Match,
                        start: *start,
                        end: start.saturating_add(matched_word.len()),
                    });
                });
        }
    }
    fn highlight_selected_match(&self, line: &Line, result: &mut Vec<Annotation>) {
        if let Some(selected_match) = self.selected_match {
            if let Some(matched_word) = self.matched_word {
                if matched_word.is_empty() {
                    return;
                }
                let start = line.grapheme_idx_to_byte_idx(selected_match.grapheme_idx);
                let annotation = Annotation {
                    annotation_type: AnnotationType::SelectedMatch,
                    start,
                    end: start.saturating_add(matched_word.len()),
                };
                info!(
                    "add annotation {:?} from {:?} to {:?}",
                    annotation.annotation_type, annotation.start, annotation.end
                );
                result.push(annotation);
            }
        }
    }
    pub fn highlight(&mut self, idx: LineIdx, line: &Line) {
        let mut result = Vec::new();
        Self::highlight_digits(line, &mut result);
        self.highlight_matched_words(line, &mut result);
        if let Some(selected_match) = self.selected_match {
            if selected_match.line_idx == idx {
                self.highlight_selected_match(line, &mut result);
            }
        }
        self.highlights.insert(idx, result);
    }
}
