use crate::stylize::Stylize;
use colored::*;
use regex::Regex;

pub struct Rule {
    regex: Regex,
    colors: Vec<Color>,
}

impl Rule {
    pub fn new(regex: &str, colors: Vec<Color>) -> Result<Self, regex::Error> {
        Ok(Rule {
            regex: Regex::new(regex)?,
            colors,
        })
    }

    fn color(&self, index: usize) -> Option<Color> {
        if self.colors.is_empty() {
            return None;
        }

        Some(self.colors.get(index % self.colors.len()).copied().unwrap())
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
                // First, add the chunk between the previous match and the new one that need to *not* be stylized.
                colored_str.push_str(&s[last_string_index..m.start()]);

                // Then, stylized the current match if a color is available, other add the match un-styled.
                if let Some(color) = self.color(index - 1) {
                    let stylized = m.as_str().color(color);
                    colored_str.push_str(stylized.to_string().as_str());
                } else {
                    colored_str.push_str(m.as_str());
                }

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

    #[test]
    fn test_color_should_fail_if_no_color() {
        let rule = Rule::new(r"", vec![]).unwrap();

        assert_eq!(rule.color(0), None);
        assert_eq!(rule.color(1), None);
    }

    #[test]
    fn test_color_should_wrap_when_out_of_bound_with_colors_available() {
        let rule = Rule::new(r"", vec![Color::Red, Color::Blue]).unwrap();

        assert_eq!(rule.color(0), Some(Color::Red));
        assert_eq!(rule.color(1), Some(Color::Blue));
        assert_eq!(rule.color(2), Some(Color::Red));
        assert_eq!(rule.color(3), Some(Color::Blue));
    }

    #[test]
    fn test_stylise_should_fail_if_no_match() {
        let rule = Rule::new(r"pattern", vec![Color::Red, Color::Blue]).unwrap();
        assert_eq!(rule.stylize("this won't match"), None);

        let rule = Rule::new(r"(pattern)", vec![Color::Red, Color::Blue]).unwrap();
        assert_eq!(rule.stylize("this won't match"), None);
    }

    #[test]
    fn test_stylise_should_return_stylised_string_on_match_when_colors_available() {
        let rule = Rule::new(r"(a) (b) (c)", vec![Color::Red, Color::Blue]).unwrap();
        assert_eq!(
            rule.stylize("a b c"),
            Some(format!("{} {} {}", "a".red(), "b".blue(), "c".red()))
        );

        let rule = Rule::new(r">(a) (b) (c)", vec![Color::Red, Color::Blue]).unwrap();
        assert_eq!(
            rule.stylize(">a b c"),
            Some(format!(">{} {} {}", "a".red(), "b".blue(), "c".red()))
        );

        let rule = Rule::new(r"(a) (b) (c)<", vec![Color::Red, Color::Blue]).unwrap();
        assert_eq!(
            rule.stylize("a b c<"),
            Some(format!("{} {} {}<", "a".red(), "b".blue(), "c".red()))
        );

        let rule = Rule::new(r">(a) (b) (c)<", vec![Color::Red, Color::Blue]).unwrap();
        assert_eq!(
            rule.stylize(">a b c<"),
            Some(format!(">{} {} {}<", "a".red(), "b".blue(), "c".red()))
        );
    }

    #[test]
    fn test_stylise_should_return_unstylised_string_on_match_when_no_color_available() {
        let rule = Rule::new(r"(a) (b) (c)", vec![]).unwrap();
        assert_eq!(rule.stylize("a b c"), Some("a b c".to_string()));
    }
}
