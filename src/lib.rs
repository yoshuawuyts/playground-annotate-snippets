//! Easily format errors for parsers.
//!
//! # Examples
//! ```
//! fn main() {
//!     let src = r#"This is an example
//! content of the slice
//! which will be annotated
//! with the list of annotations below.
//! "#;
//!
//!     let msg = Error::new("expected type, found `x`".to_string())
//!         .error(260, 0, 12, src.to_string(), "found `x`".to_string())
//!         .help("try using a foobs intead".to_string())
//!         .to_string();
//!
//!     println!("{}", msg);
//! }
//! ```

use annotate_snippets::display_list::DisplayList;
use annotate_snippets::formatter::DisplayListFormatter;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

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
        line_start: usize,
        start: usize,
        end: usize,
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

    /// Create a new footer.
    pub fn help(mut self, label: String) -> Self {
        self.snippet.footer.push(Annotation {
            label: Some(label),
            id: None,
            annotation_type: AnnotationType::Help,
        });
        self
    }

    pub fn to_string(self) -> String {
        let dl = DisplayList::from(self.snippet);
        let dlf = DisplayListFormatter::new(true, false);
        format!("{}", dlf.format(&dl))
    }
}
