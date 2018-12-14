use regex::{Error, Regex, RegexBuilder};

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    UnknownCommand(char),
    UnknownFlag(char),
    TooManySegments,
    NotEnoughSegments,
    RegexError(Error),
}

#[derive(Debug, PartialEq)]
pub struct RegexData {
    pattern_str: String,
    pub replace_str: String,
    pub flag_global: bool,
    flag_case_insensitive: bool,
}

impl RegexData {
    pub fn build_regex(&self) -> Result<Regex, ErrorKind> {
        RegexBuilder::new(&self.pattern_str)
            .case_insensitive(self.flag_case_insensitive)
            .build()
            .map_err(ErrorKind::RegexError)
    }
}

pub fn split_regex(expr: &str) -> Result<RegexData, ErrorKind> {
    let expr = expr.chars().collect::<Vec<_>>();
    if expr[0] != 's' {
        return Err(ErrorKind::UnknownCommand(expr[0]));
    }
    let delimiter = expr[1];

    let mut segments = vec![];
    let mut segment = vec![];

    let mut i = 2;
    while i < expr.len() {
        let c = expr[i];
        if c == '\\' {
            segment.push(expr[i + 1]);
            i += 1;
        } else if c == delimiter {
            segments.push(segment.iter().collect::<String>());
            segment.clear();
        } else {
            segment.push(c);
        }
        i += 1;
    }
    if !segment.is_empty() {
        segments.push(segment.iter().collect::<String>());
    }

    if segments.len() < 2 {
        return Err(ErrorKind::NotEnoughSegments);
    } else if segments.len() > 3 {
        return Err(ErrorKind::TooManySegments);
    }

    let mut ret = RegexData {
        pattern_str: segments[0].to_owned(),
        replace_str: segments[1].to_owned(),
        flag_global: false,
        flag_case_insensitive: false,
    };

    if segments.len() == 3 {
        for c in segments[2].chars() {
            match c {
                'i' => ret.flag_case_insensitive = true,
                'g' => ret.flag_global = true,
                _ => return Err(ErrorKind::UnknownFlag(c)),
            }
        }
    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use crate::sedregex::*;

    #[test]
    fn test() {
        assert_eq!(
            Ok(RegexData {
                pattern_str: "123".to_string(),
                replace_str: "456".to_string(),
                flag_global: false,
                flag_case_insensitive: false,
            }),
            split_regex("s/123/456")
        );
    }
}
