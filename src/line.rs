use core::fmt;
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Clone)]
enum GraphemeWidth {
    Half,
    Full,
}

#[derive(Clone)]
pub struct Fragment {
    pub grapheme: String,
    render_width: GraphemeWidth,
    pub replacement: Option<char>,
}

#[derive(Clone, Default)]
pub struct Line {
    fragments: Vec<Fragment>,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let result: String = self
            .fragments
            .iter()
            .map(|fragment| fragment.grapheme.clone())
            .collect();
        write!(f, "{result}")
    }
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Self {
            fragments
        }
    }

    pub fn str_to_fragments(line_str: &str) -> Vec<Fragment> {
        line_str
            .graphemes(true)
            .map(|grapheme| {
                let (render_width, replacement) = Self::replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let render_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (render_width, None)
                        },
                        |replacement| ( GraphemeWidth::Half, Some(replacement)),
                    );

                Fragment {
                    grapheme: grapheme.to_string(),
                    render_width,
                    replacement
                }
            })
            .collect()
    }

    pub fn replacement_character(grapheme: &str) -> Option<char> {
        let width = grapheme.width();
        match grapheme {
            "\t" => Some(' '),
            " " => None,
            _ if grapheme.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = grapheme.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next() == None {
                        return Some('▯');
                    }
                }
                Some('·')
            },
            _ => None

        }
    }

    pub fn get_substr(&self, range: Range<usize>) -> String {
        if range.start >= range.end {
            return String::new();
        }

        let mut substr = String::new();
        let mut pos: usize = 0;
        for fragment in &self.fragments {
            if pos >= range.end {
                break;
            }

            let fragment_end = match fragment.render_width {
                GraphemeWidth::Half => pos + 1,
                GraphemeWidth::Full => pos + 2,
            };

            if fragment_end > range.start {
                if fragment_end > range.end || pos < range.start {
                    substr.push('⋯');
                    break;
                } else if let Some(repl) = fragment.replacement {
                    substr.push(repl);
                } else {
                    substr.push_str(&fragment.grapheme);
                }
            }

            pos = fragment_end;
        }
        substr
    }

    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    pub fn sum_width_until(&self, index: usize) -> usize {
        self.fragments
            .iter()
            .take(index)
            .map(|fragment| {
                match fragment.render_width {
                    GraphemeWidth::Half => 1,
                    GraphemeWidth::Full => 2,
                }
            })
            .sum()
    }

    pub fn get_fragments(&self) -> &Vec<Fragment> {
        &self.fragments
    }

    pub fn insert_char(&mut self, character: char, grapheme_index: usize) {
        let mut line_builder = String::new();

        for (index, fragment) in self.fragments.iter().enumerate() {
            if grapheme_index == index {
                line_builder.push(character);
            }
            line_builder.push_str(&fragment.grapheme);
        }

        if grapheme_index >= self.fragments.len() {
            line_builder.push(character);
        }

        self.fragments = Self::str_to_fragments(&line_builder);
    }

    pub fn delete(&mut self, grapheme_index: usize) {
        let mut new_line = String::new();

        for (index, fragment) in self.fragments.iter().enumerate() {
            if index != grapheme_index {
                new_line.push_str(&fragment.grapheme);
            }
        }

        self.fragments = Self::str_to_fragments(&new_line);
    }

    pub fn append(&mut self, other: &Self) {
        let mut concat = self.to_string();
        concat.push_str(&other.to_string());
        self.fragments = Self::str_to_fragments(&concat);

    }

    pub fn split(&mut self, at: usize) -> Self {
        if at > self.fragments.len() {
            return Self::default();
        }
        let next = self.fragments.split_off(at);
        Self {
            fragments: next,
        }
    }
}