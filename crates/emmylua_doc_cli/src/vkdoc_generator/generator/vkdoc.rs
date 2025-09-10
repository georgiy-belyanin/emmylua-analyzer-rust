use std::path::Path;

use vkdocs_rs::{Page, Vkdoc};

pub struct VkdocEntry {
    pub name: String,
    pub description: String,
    pub content: String,
}

pub fn generate_vkdoc_entry(output: &Path, entry: VkdocEntry, vkdoc: &Vkdoc) -> Option<()> {
    let page = Page::new()
        .with_title(entry.name)
        .with_short_description(entry.description)
        .with_content(entry.content);

    if vkdoc.upsert(output, page).unwrap() {
        println!("Updated module file: {}", output.display());
    }

    Some(())
}
