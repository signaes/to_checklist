use clipboard::{ClipboardContext, ClipboardProvider};

extern crate clipboard;

fn main() {
    match clipboard::ClipboardProvider::new()
        .and_then(|mut clipboard: ClipboardContext| clipboard.get_contents())
    {
        Ok(contents) => {
            println!("\n{}", checklist_by_line_breaks(contents))
        }
        Err(err) => eprintln!("Error trying to access the clipboard: {:?}", err),
    }
}

fn checklist_by_line_breaks(content: String) -> String {
    content
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            if line.starts_with("[ ]") {
                return format!("{}\n", line);
            }

            format!("[ ] {}\n", line)
        })
        .collect()
}
