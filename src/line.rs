use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

enum GraphemeWidth {
    Half,
    Full,
}

pub struct Fragment {
    grapheme: String,
    render_width: GraphemeWidth,
    replacement: Option<char>,
}

pub struct Line {
    fragments: Vec<Fragment>,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        let fragments = line_str
            .graphemes(true)
            .map(|grapheme| {
                let width = grapheme.width();

                let render_width = match width {
                    0 | 1 => GraphemeWidth::Half,
                    _ => GraphemeWidth::Full,
                };

                let replacement = match width {
                    0 => Some('·'),
                    _ => None,
                };

                Fragment {
                    grapheme: grapheme.to_string(),
                    render_width,
                    replacement
                }
            })
            .collect();
        Self {
            fragments,
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {
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
}