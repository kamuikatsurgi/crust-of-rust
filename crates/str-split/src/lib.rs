#[derive(Debug)]
pub struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D> StrSplit<'a, D> {
    pub fn new(haystack: &'a str, delimiter: D) -> Self {
        StrSplit { remainder: Some(haystack), delimiter }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices().find(|(_, c)| c == self).map(|(start, _)| (start, start + self.len_utf8()))
    }
}

impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimiter,
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delimiter_start, delimiter_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delimiter_start];
            *remainder = &remainder[delimiter_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c).next().expect("StrSpilt always gives atleast one result")
}

#[cfg(test)]
mod tests {
    use crate::{until_char, StrSplit};

    #[test]
    fn test_simple_str_split() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_delimiter_at_the_end() {
        let haystack = "a b c d e ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e", ""]);
    }

    #[test]
    fn test_until_char_fn() {
        assert_eq!(until_char("Hello World!", 'o'), "Hell");
    }
}
