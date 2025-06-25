use hecto::editor::AnnotationType;
use hecto::editor::uicomponents::view::{Highlighter, Line};

fn main() {
    // 测试数字高亮
    let line = Line::from("Hello 123 world 456");
    let mut highlighter = Highlighter::new(None, None);
    highlighter.highlight(0, &line);

    if let Some(annotations) = highlighter.get_annotations(0) {
        println!("数字高亮测试:");
        for annotation in annotations {
            println!(
                "  类型: {:?}, 范围: {}..{}",
                annotation.annotation_type, annotation.start, annotation.end
            );
        }
    }

    // 测试搜索匹配高亮
    let line = Line::from("needle in haystack needle");
    let mut highlighter = Highlighter::new(Some("needle"), None);
    highlighter.highlight(0, &line);

    if let Some(annotations) = highlighter.get_annotations(0) {
        println!("\n搜索匹配高亮测试:");
        for annotation in annotations {
            println!(
                "  类型: {:?}, 范围: {}..{}",
                annotation.annotation_type, annotation.start, annotation.end
            );
        }
    }

    // 测试选中匹配高亮
    use hecto::prelude::Location;
    let line = Line::from("needle in haystack");
    let mut highlighter = Highlighter::new(
        Some("needle"),
        Some(Location {
            line_idx: 0,
            grapheme_idx: 0,
        }),
    );
    highlighter.highlight(0, &line);

    if let Some(annotations) = highlighter.get_annotations(0) {
        println!("\n选中匹配高亮测试:");
        for annotation in annotations {
            println!(
                "  类型: {:?}, 范围: {}..{}",
                annotation.annotation_type, annotation.start, annotation.end
            );
        }
    }
}
