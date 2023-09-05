use crate::rule::{Rule, RuleStyle};
use colored::Color;
use hex_color::HexColor;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{fs::File, io::BufReader, path::PathBuf, str::FromStr};

pub struct ConfigPath {
    starting_directory: PathBuf,
    filename: String,
}

impl ConfigPath {
    pub fn new(starting_directory: &str, filename: &str) -> ConfigPath {
        let canonical = PathBuf::from_str(starting_directory)
            .unwrap()
            .canonicalize()
            .unwrap();
        ConfigPath {
            starting_directory: canonical,
            filename: filename.into(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.starting_directory.ancestors().filter_map(|path| {
            let mut path = path.to_path_buf();
            path.push(&self.filename);
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct ConfigStyle {
    foreground_color: Option<String>,
    background_color: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigRule {
    name: String,
    pattern: String,
    styles: Vec<ConfigStyle>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigRuleGroup {
    group_name: String,
    rules: Vec<ConfigRule>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    rule_groups: Vec<ConfigRuleGroup>,
}

impl Config {
    pub fn load(paths: ConfigPath) -> Config {
        let mut config = Config {
            rule_groups: vec![],
        };

        for path in paths.iter() {
            println!("Loading configuration from: \"{}\"", path.to_str().unwrap());

            // Open the file in read-only mode with buffer.
            let file = File::open(path).unwrap(); // FIXME: unwrap
            let reader = BufReader::new(file);

            // Read the JSON contents of the file.
            // FIXME: ok -> error handling
            let c: Option<Config> = serde_json::from_reader(reader).ok();
            if let Some(mut c2) = c {
                // Add the new rules to the global config.
                config.rule_groups.append(c2.rule_groups.as_mut());
            }
        }

        config
    }

    pub fn serialized(&self) -> serde_json::error::Result<String> {
        serde_json::to_string_pretty(self)
    }

    fn color_from_str(s: &str) -> Color {
        let hex = HexColor::parse(s).unwrap_or_default();
        Color::TrueColor {
            r: hex.r,
            g: hex.g,
            b: hex.b,
        }
    }

    pub fn as_rules(&self) -> Vec<Rule> {
        self.rule_groups.iter().fold(vec![], |rules, rule_group| {
            rule_group.rules.iter().fold(rules, |mut rules, rule| {
                let styles = rule.styles.iter().fold(vec![], |mut styles, style| {
                    let s = RuleStyle {
                        foreground_color: style
                            .foreground_color
                            .as_ref()
                            .map(|color_string| Config::color_from_str(color_string)),
                        background_color: style
                            .background_color
                            .as_ref()
                            .map(|color_string| Config::color_from_str(color_string)),
                    };
                    styles.push(s);
                    styles
                });
                rules.push(Rule::new(&rule.pattern, styles).unwrap());
                rules
            })
        })
    }

    pub fn example() -> Config {
        Config {
            rule_groups: vec![
                ConfigRuleGroup {
                    group_name: "errors_and_warnings".to_string(),
                    rules: vec![
                        ConfigRule {
                            name: "error".to_string(),
                            pattern: r"(.*)([eE][rR][rR][oO][rR])(.*)".to_string(),
                            styles: vec![
                                ConfigStyle {
                                    foreground_color: Some("#e73c3e".to_string()),
                                    background_color: None,
                                },
                                ConfigStyle {
                                    foreground_color: None,
                                    background_color: Some("#e73c3e".to_string()),
                                },
                                ConfigStyle {
                                    foreground_color: Some("#e73c3e".to_string()),
                                    background_color: None,
                                },
                            ],
                        },
                        ConfigRule {
                            name: "warning".to_string(),
                            pattern: r"(.*)([wW][aA][rR][nN][iI][nN][gG])(.*)".to_string(),
                            styles: vec![
                                ConfigStyle {
                                    foreground_color: Some("#f4f454".to_string()),
                                    background_color: None,
                                },
                                ConfigStyle {
                                    foreground_color: None,
                                    background_color: Some("#f4f454".to_string()),
                                },
                                ConfigStyle {
                                    foreground_color: Some("#f4f454".to_string()),
                                    background_color: None,
                                },
                            ],
                        },
                    ],
                },
                ConfigRuleGroup {
                    group_name: "timestamp".to_string(),
                    rules: vec![ConfigRule {
                        name: "ISO8601".to_string(),
                        pattern: r"(\d{4}-[01]\d-[0-3]\d)?T([0-2]\d:[0-5]\d:[0-5]\d(\.\d+)?)"
                            .to_string(),
                        styles: vec![
                            ConfigStyle {
                                foreground_color: Some("#000000".to_string()),
                                background_color: Some("#6fb9f5".to_string()),
                            },
                            ConfigStyle {
                                foreground_color: Some("#000000".to_string()),
                                background_color: Some("#abdafd".to_string()),
                            },
                        ],
                    }],
                },
            ],
        }
    }
}
