use annotate_snippets::display_list::DisplayList;
use annotate_snippets::formatter::DisplayListFormatter;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

// components:
// - error description
// - error kind (error, warn)
// - range
// - optional help text
//
// ```rust
// let msg = thingy::error("expected type, found `22`")
//     .error(0, 18, "and here's an error".to_string())
//     .help("try using a foobs intead".to_string());
//
// Error::new("expected type, found `x`)
//     .error(260, 0, 18, msg, "and here's an error")
//     .help("try using a foobs intead".to_string());
//
// Waring::new("expected type, found `x`)
//     .warning(260, 0, 18, msg, "and here's a warning")
//     .help("try using a foobs intead".to_string());
//
// let msg = Message::new("expected type, found `22`")
//     .push(Slice::new(80
//     .error(0, 18, "and here's an error".to_string())
//     .help("try using a foobs intead".to_string());
// ```

struct Message {
    line_start: usize,
    snippet: Snippet,
}

impl Message {
    pub fn new(line_start: usize, label: String) -> Self {
        Self {
            line_start,
            snippet: Snippet {
                title: Some(Annotation {
                    label: Some(label),
                    id: None,
                    annotation_type: AnnotationType::Error,
                }),
                slices: vec![],
                footer: vec![],
            },
        }
    }

    pub fn error(self, start: usize, end: usize, line_start: usize, source: String) -> Self {
        self.snippet.slices.push(Slice {
            source: r#"This is an example
content of the slice
which will be annotated
with the list of annotations below.
                "#
            .to_string(),
            line_start: 260,
            origin: None,
            fold: true,
            annotations: vec![SourceAnnotation {
                label: "and here's a warning".to_string(),
                annotation_type: AnnotationType::Error,
                range: (3, 30),
            }],
        })
    }
}

fn main() {
    let snippet = Snippet {
        title: Some(Annotation {
            label: Some("expected type, found `22`".to_string()),
            id: None,
            annotation_type: AnnotationType::Error,
        }),
        slices: vec![Slice {
            source: r#"This is an example
content of the slice
which will be annotated
with the list of annotations below.
                "#
            .to_string(),
            line_start: 260,
            origin: None,
            fold: true,
            annotations: vec![SourceAnnotation {
                label: "and here's a warning".to_string(),
                annotation_type: AnnotationType::Error,
                range: (3, 30),
            }],
        }],
        footer: vec![Annotation {
            label: Some("try using a foobs instead".to_string()),
            id: None,
            annotation_type: AnnotationType::Help,
        }],
    };

    let dl = DisplayList::from(snippet);
    let dlf = DisplayListFormatter::new(true, false);
    println!("{}", dlf.format(&dl));
}
// error: expected type, found `22`
//     |
// 260 |   This is an example
//     |  ____^
// 261 | | content of the slice
//     | |___________^ and here's a warning
// 262 |   which will be annotated
// 263 |   with the list of annotations below.
// 264 |
//     |
//     = help: try using a foobs instead
