mod syntax_highlighter;
use super::super::super::{Annotation, AnnotationType, FileType, Line};
use crate::prelude::*;
use syntax_highlighter::SyntaxHighlighter;

mod rust_syntax_highlighter;
use rust_syntax_highlighter::RustSyntaxHighlighter;
mod search_result_highlighter;
use search_result_highlighter::SearchResultHighlighter;

fn create_syntax_highlighter(file_type: FileType) -> Option<Box<dyn SyntaxHighlighter>> {
    match file_type {
        FileType::Rust => Some(Box::<RustSyntaxHighlighter>::default()),
        _ => None,
    }
}
#[derive(Default)]
pub struct Highlighter<'a> {
    syntax_highlighter: Option<Box<dyn SyntaxHighlighter>>,
    search_result_highlighter: Option<SearchResultHighlighter<'a>>,
}

impl<'a> Highlighter<'a> {
    pub fn new(
        matched_word: Option<&'a str>,
        selected_match: Option<Location>,
        file_type: FileType,
    ) -> Self {
        let search_result_highlighter = matched_word
            .map(|matched_word| SearchResultHighlighter::new(matched_word, selected_match));
        Self {
            syntax_highlighter: create_syntax_highlighter(file_type),
            search_result_highlighter,
        }
    }

    pub fn get_annotations(&self, idx: LineIdx) -> Vec<Annotation> {
        let mut result = Vec::new();
        if let Some(syntax_highlighter) = &self.syntax_highlighter {
            if let Some(annotations) = syntax_highlighter.get_annotations(idx) {
                result.extend(annotations.iter().cloned());
            }
        }
        if let Some(search_result_highlighter) = &self.search_result_highlighter {
            if let Some(annotations) = search_result_highlighter.get_annotations(idx) {
                result.extend(annotations.iter().cloned());
            }
        }
        result
    }
    pub fn highlight(&mut self, idx: LineIdx, line: &Line) {
        if let Some(syntax_highlighter) = &mut self.syntax_highlighter {
            syntax_highlighter.highlight(idx, line);
        }
        if let Some(search_result_highlighter) = &mut self.search_result_highlighter {
            search_result_highlighter.highlight(idx, line);
        }
    }
}
