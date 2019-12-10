use playground_annotate_snippets::Error;
use std::io::Cursor;

fn main() {
    let cursor = Cursor::new(r#"This is an example
content of the slice
which will be annotated
with the list of annotations below.
"#);

    let msg = Error::new("expected type, found `x`".to_string())
        .error(260, 0, 12, cursor.into_inner().to_string(), "found `x`".to_string())
        .help("try using a foobs intead".to_string())
        .to_string();

    println!("{}", msg);
}

