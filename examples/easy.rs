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
// Error::new("expected type, found `x`)
//     .error(260, 0, 18, msg, "and here's an error")
//     .help("try using a foobs intead".to_string());
//
// Waring::new("expected type, found `x`)
//     .warning(260, 0, 18, msg, "and here's a warning")
//     .help("try using a foobs intead".to_string());
// ```

/// An error formatter.
pub struct Error {
    snippet: Snippet,
}

impl Error {
    /// Create a new `Error` formatter.
    pub fn new(label: String) -> Self {
        Self {
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

    /// Pass a new error to the formatter.
    pub fn error(
        mut self,
        start: usize,
        end: usize,
        line_start: usize,
        source: String,
        label: String,
    ) -> Self {
        self.snippet.slices.push(Slice {
            source,
            line_start,
            origin: None,
            fold: true,
            annotations: vec![SourceAnnotation {
                label,
                annotation_type: AnnotationType::Error,
                range: (start, end),
            }],
        });
        self
    }

    pub fn to_string(self)  -> String {
        let dl = DisplayList::from(self.snippet);
        let dlf = DisplayListFormatter::new(true, false);
        format!("{}", dlf.format(&dl))
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
