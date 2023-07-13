use std::error::Error;

use toml::Table;

use document::meta_data::{
    Author, DocumentEncoding, DocumentLanguage, DocumentMetaData, DocumentVariables,
    DocumentVersion,
};

pub fn read_meta_data(src: &str) -> Result<DocumentMetaData, Box<dyn Error>> {
    let table = src.parse::<Table>()?;

    let mut result = DocumentMetaData::default();

    for (key, value) in table.into_iter() {
        match key.as_str() {
            "encoding" => {
                result.encoding =
                    DocumentEncoding::new(value.as_str().expect("expected value for encoding"))
            }
            "language" => {
                let lang_str = value.as_str().expect("expected value for language");

                if lang_str.contains("-") {
                    let mut parts = lang_str.split("-");

                    let lang = parts.next().expect("expected language");
                    let region = parts.next().expect("expected region");

                    result.language = DocumentLanguage::new(lang, Some(region));
                } else {
                    result.language = DocumentLanguage::new(lang_str, None);
                }
            }
            "authors" => {
                result.authors = value
                    .as_array()
                    .expect("expected array for authors")
                    .into_iter()
                    .map(|author| {
                        author
                            .as_str()
                            .expect("expected string for author")
                            .to_string()
                    })
                    .map(|author| {
                        let parts: Vec<&str> = author.split(" ").collect();

                        let mut name = String::new();
                        let mut mail = None;

                        let last_part = parts.last().expect("expected last part");
                        if last_part.contains("<") {
                            mail = Some(
                                last_part
                                    .replace("<", "")
                                    .replace(">", "")
                                    .trim()
                                    .to_string(),
                            );
                            name.push_str(
                                &parts
                                    .iter()
                                    .take(parts.len() - 1)
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>()
                                    .join(" "),
                            );
                        } else {
                            name.push_str(
                                &parts
                                    .iter()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>()
                                    .join(" "),
                            );
                        }

                        return Author::new(name, mail);
                    })
                    .collect();
            }
            "version" => {
                result.version =
                    DocumentVersion::new(value.as_str().expect("expected value for version"))
            }
            "variables" => {
                let mut variables = DocumentVariables::new();

                value
                    .as_table()
                    .expect("expected table for variables")
                    .into_iter()
                    .map(|(key, value)| (key.to_string(), value.to_string()))
                    .for_each(|(key, value)| variables.set(&key, &value));

                result.variables = variables;
            }
            _ => return Err(format!("Unknown meta data key: {}", key).into()),
        }
    }

    Ok(result)
}
