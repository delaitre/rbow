use crate::stylize::Stylize;
use colored::*;
use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub struct RuleStyle {
    pub foreground_color: Option<Color>,
    pub background_color: Option<Color>,
}

#[derive(Clone)]
pub struct Rule {
    regex: Regex,
    styles: Vec<RuleStyle>,
}

impl Rule {
    pub fn new(regex: &str, styles: Vec<RuleStyle>) -> Result<Self, regex::Error> {
        Ok(Rule {
            regex: Regex::new(regex)?,
            styles,
        })
    }

    fn style(&self, index: usize) -> Option<&RuleStyle> {
        self.styles.get(index)
    }
}

impl Stylize for Rule {
    fn stylize(&self, s: &str) -> Option<String> {
        // Index 0 is the whole regex match which we ignore. Index 1 is the first group we care about and should use colors(0).
        let captures = self.regex.captures(s)?;
        let group_count = captures.len() - 1;

        // If nothing got captured in a group this rule has nothing to stylize.
        if group_count == 0 {
            return None;
        }

        // Allocate space for the stylized string using some heuristics.
        // We don't know exactly how big each color delimiter will take so try our best by using the biggest delimiter (true color).
        let mut colored_str = String::default();
        let delimiter_max_size = Color::TrueColor {
            r: 127,
            g: 127,
            b: 127,
        }
        .to_fg_str()
        .len();
        colored_str.reserve(s.len() + group_count * delimiter_max_size * 2); // x2 since we can have fg and gb colors...

        // Build the stylized string by iterating over each group and appending colored matches or uncolored unmatched content (between each group).
        let mut last_string_index = 0;
        for index in 1..captures.len() {
            if let Some(m) = captures.get(index) {
                // If the new match in nested, skip it as we don't support styling nested matches yet.
                // It will thus be stylised like the outer match.
                if m.start() < last_string_index {
                    continue;
                }

                // First, add the chunk between the previous match and the new one that need to *not* be stylized.
                colored_str.push_str(&s[last_string_index..m.start()]);

                // Then, stylized the current match.
                let mut stylized = m.as_str().clear();
                if let Some(style) = self.style(index - 1) {
                    if let Some(foreground_color) = style.foreground_color {
                        stylized = stylized.color(foreground_color);
                    }
                    if let Some(background_color) = style.background_color {
                        stylized = stylized.on_color(background_color);
                    }
                }
                colored_str.push_str(stylized.to_string().as_str());

                // Update the start of the next unstylized chunk.
                last_string_index = m.end();
            }
        }

        // Finally add the last chunk of unstylized content after the last match.
        colored_str.push_str(&s[last_string_index..]);

        Some(colored_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static STYLE_0: RuleStyle = RuleStyle {
        foreground_color: Some(Color::Red),
        background_color: None,
    };

    static STYLE_1: RuleStyle = RuleStyle {
        foreground_color: Some(Color::Blue),
        background_color: Some(Color::Yellow),
    };

    #[test]
    fn test_style_should_fail_if_no_style_available() {
        let rule = Rule::new(r"", vec![]).unwrap();

        assert_eq!(rule.style(0), None);
        assert_eq!(rule.style(1), None);
    }

    #[test]
    fn test_style_should_succeed_if_style_available() {
        let rule = Rule::new(r"", vec![STYLE_0.clone(), STYLE_1.clone()]).unwrap();

        assert_eq!(rule.style(0), Some(&STYLE_0));
        assert_eq!(rule.style(1), Some(&STYLE_1));
        assert_eq!(rule.style(2), None);
    }

    #[test]
    fn test_stylize_should_fail_if_no_match() {
        let rule = Rule::new(r"pattern", vec![STYLE_0.clone()]).unwrap();
        assert_eq!(rule.stylize("this won't match"), None);

        let rule = Rule::new(r"(pattern)", vec![STYLE_0.clone()]).unwrap();
        assert_eq!(rule.stylize("this won't match"), None);
    }

    #[test]
    fn test_stylize_should_return_stylized_string_on_match_if_style_available() {
        let rule = Rule::new(r"(a) (b) (c)", vec![STYLE_0.clone(), STYLE_1.clone()]).unwrap();
        let styled_a = "a".color(STYLE_0.foreground_color.unwrap());
        let styled_b = "b"
            .color(STYLE_1.foreground_color.unwrap())
            .on_color(STYLE_1.background_color.unwrap());
        let styled_c = "c".clear();

        assert_eq!(
            rule.stylize("a b c"),
            Some(format!("{} {} {}", styled_a, styled_b, styled_c))
        );

        let rule = Rule::new(r">(a) (b) (c)", vec![STYLE_0.clone(), STYLE_1.clone()]).unwrap();
        assert_eq!(
            rule.stylize(">a b c"),
            Some(format!(">{} {} {}", styled_a, styled_b, styled_c))
        );

        let rule = Rule::new(r"(a) (b) (c)<", vec![STYLE_0.clone(), STYLE_1.clone()]).unwrap();
        assert_eq!(
            rule.stylize("a b c<"),
            Some(format!("{} {} {}<", styled_a, styled_b, styled_c))
        );

        let rule = Rule::new(r">(a) (b) (c)<", vec![STYLE_0.clone(), STYLE_1.clone()]).unwrap();
        assert_eq!(
            rule.stylize(">a b c<"),
            Some(format!(">{} {} {}<", styled_a, styled_b, styled_c))
        );
    }

    #[test]
    fn test_stylize_should_return_unstylized_string_on_match_if_no_style_available() {
        let rule = Rule::new(r"(a) (b) (c)", vec![]).unwrap();
        assert_eq!(rule.stylize("a b c"), Some("a b c".to_string()));
    }

    #[test]
    fn test_stylize_should_return_stylized_string_even_with_nested_matches() {
        // Currently nested matches are skipped and will thus be stylised as part of the outer match.
        let rule = Rule::new(r">(a (b))", vec![STYLE_0.clone(), STYLE_1.clone()]).unwrap();
        assert_eq!(
            rule.stylize(">a b"),
            Some(format!(
                ">{}",
                "a b".color(STYLE_0.foreground_color.unwrap())
            ))
        );

        let rule = Rule::new(r">(a (b) c) d", vec![STYLE_0.clone(), STYLE_1.clone()]).unwrap();
        assert_eq!(
            rule.stylize(">a b c d"),
            Some(format!(
                ">{} d",
                "a b c".color(STYLE_0.foreground_color.unwrap())
            ))
        );
    }
}
