use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::Render;

use super::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    // pub file_name: String,
    pub html: String,
    // Relative path of page file.
    pub file_path: PathBuf,
}

impl Page {
    pub fn slug(&self) -> String {
        self.file_path.to_str().unwrap().replace(".md", "")
    }
}

impl Entity for Page {
    fn render(&self, mut context: Context, dest: &Path) -> Result<()> {
        context.insert("content", &self.html);
        Render::render("page.jinja", &context, dest.join(self.slug()))?;
        Ok(())
    }
}
