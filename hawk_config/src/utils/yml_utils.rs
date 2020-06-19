//
//use std::{
//    error, fmt, fs,
//    path::{Path, PathBuf},
//    thread,
//    time::{Duration, SystemTime},
//};
//
//fn read(path: String) {
//    let path = path.as_ref().to_path_buf();
//    let format = Format::from_path(&path)?;
//    let source = read_config(&path)?;
//// An Err here could come because mtime isn't available, so don't bail
//    let modified = fs::metadata(&path).and_then(|m| m.modified()).ok();
//    let config = format.parse(&source)?;
//
//    let refresh_rate = config.refresh_rate();
//    let config = deserialize(&config, &deserializers);
//}
//
//
//enum Format {
//    #[cfg(feature = "yaml_format")]
//    Yaml,
//    #[cfg(feature = "json_format")]
//    Json,
//    #[cfg(feature = "toml_format")]
//    Toml,
//    #[cfg(feature = "xml_format")]
//    #[deprecated(since = "0.11.0")]
//    Xml,
//}
//
//impl Format {
//    fn from_path(path: &Path) -> Result<Format, Box<dyn error::Error + Sync + Send>> {
//        match path.extension().and_then(|s| s.to_str()) {
//            #[cfg(feature = "yaml_format")]
//            Some("yaml") | Some("yml") => Ok(Format::Yaml),
//            #[cfg(not(feature = "yaml_format"))]
//            Some("yaml") | Some("yml") => {
//                Err("the `yaml_format` feature is required for YAML support".into())
//            }
//            #[cfg(feature = "json_format")]
//            Some("json") => Ok(Format::Json),
//            #[cfg(not(feature = "json_format"))]
//            Some("json") => Err("the `json_format` feature is required for JSON support".into()),
//
//            #[cfg(feature = "toml_format")]
//            Some("toml") => Ok(Format::Toml),
//            #[cfg(not(feature = "toml_format"))]
//            Some("toml") => Err("the `toml_format` feature is required for TOML support".into()),
//
//            #[cfg(feature = "xml_format")]
//            Some("xml") => Ok(Format::Xml),
//            #[cfg(not(feature = "xml_format"))]
//            Some("xml") => Err("the `xml_format` feature is required for XML support".into()),
//
//            Some(f) => Err(format!("unsupported file format `{}`", f).into()),
//            None => Err("unable to determine the file format".into()),
//        }
//    }
//
//    #[allow(unused_variables)]
//    fn parse(&self, source: &str) -> Result<RawConfig, Box<dyn error::Error + Send + Sync>> {
//        match *self {
//            #[cfg(feature = "yaml_format")]
//            Format::Yaml => ::serde_yaml::from_str(source).map_err(Into::into),
//            #[cfg(feature = "json_format")]
//            Format::Json => ::serde_json::from_str(source).map_err(Into::into),
//            #[cfg(feature = "toml_format")]
//            Format::Toml => ::toml::from_str(source).map_err(Into::into),
//            #[cfg(feature = "xml_format")]
//            Format::Xml => ::serde_xml_rs::from_reader::<_, RawConfigXml>(source.as_bytes())
//                .map(Into::into)
//                .map_err(|e| e.to_string().into()),
//        }
//    }
//}
