use eyre::{eyre, Context, Result};
use hubcaps::labels::{Label, LabelOptions};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonLabel {
    pub color: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub name: String,
}

impl JsonLabel {
    pub fn from(color: String, description: String, name: String) -> Self {
        Self {
            color,
            description,
            name,
        }
    }
}

impl From<JsonLabel> for LabelOptions {
    fn from(lbl: JsonLabel) -> Self {
        LabelOptions::new(lbl.name, lbl.color, lbl.description)
    }
}

impl From<Label> for JsonLabel {
    fn from(lbl: Label) -> Self {
        Self {
            color: lbl.color,
            name: lbl.name,
            description: lbl.description.unwrap_or_default(),
        }
    }
}

impl PartialEq<hubcaps::labels::Label> for JsonLabel {
    fn eq(&self, other: &hubcaps::labels::Label) -> bool {
        other.color == self.color
            && other.description.as_deref() == Some(&self.description)
            && other.name == self.name
    }
}

pub type JsonLabels = Vec<JsonLabel>;

fn read_from_file(path: impl AsRef<Path>) -> Result<JsonFile> {
    let file = File::open(path.as_ref()).wrap_err_with(|| "Cannot find label definition file")?;
    serde_json::from_reader::<_, JsonFile>(file)
        .wrap_err_with(|| "Misformatted label definition file. Make sure the file is valid json!")
}

pub fn read_from_cli_arg_or_fallback_to_config_dir(
    cli_path: Option<impl AsRef<Path>>,
) -> Result<JsonFile> {
    cli_path.map(|p| read_from_file(p.as_ref())).unwrap_or_else(|| crate::config::config_file().map(read_from_file).unwrap_or_else(|| Err(eyre!("Either create a global configuration file or pass a label definition file to the CLI"))))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonTemplate {
    pub color: String,
    pub name: String,
}

pub type JsonTemplates = Vec<JsonTemplate>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JsonFile {
    pub labels: JsonLabels,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub templates: JsonTemplates,
}
