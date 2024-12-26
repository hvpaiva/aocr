use aoc_runner_derive::aoc;

/// A trait for counting the "in-memory" length of a string
/// according to the Advent of Code Day 8 rules.
pub trait MemoryCount {
    /// Returns the length of this string after interpreting
    /// the escape sequences. This assumes the string is
    /// surrounded by quotes (like `"abc"`).
    fn memory_len(&self) -> usize;
}

impl MemoryCount for &str {
    fn memory_len(&self) -> usize {
        // If the line is shorter than 2, there's nothing to parse.
        // Typically, AoC lines have at least two quotes.
        if self.len() < 2 {
            return 0;
        }

        // We'll skip the opening quote (index 0)
        // and the closing quote (index self.len() - 1).
        let mut i = 1;
        let end = self.len() - 1;
        let bytes = self.as_bytes();
        let mut count = 0;

        while i < end {
            match bytes[i] {
                b'\\' => {
                    // We found a backslash. Let's look at the next characters.
                    // If there's something like `\\`, `\"`, or `\x..`.
                    if i + 1 < end {
                        match bytes[i + 1] {
                            b'\\' | b'"' => {
                                // Examples: `\\` -> single backslash in memory
                                //           `\"` -> single quote in memory
                                count += 1;
                                i += 2;
                            }
                            b'x' => {
                                // Example: `\x27` means one character in memory,
                                // skipping four bytes total: `\`, `x`, and 2 hex digits.
                                count += 1;
                                i += 4;
                            }
                            _ => {
                                // Some unrecognized escape: we'll treat it as one character,
                                // and skip two bytes (`\?`).
                                count += 1;
                                i += 2;
                            }
                        }
                    } else {
                        // There's a backslash at the end, but no next char.
                        // We'll treat it as a single character to be safe.
                        count += 1;
                        i += 1;
                    }
                }
                _ => {
                    // A normal character
                    count += 1;
                    i += 1;
                }
            }
        }

        count
    }
}

/// A trait for encoding a string according to the Advent of Code Day 8 (part 2) rules.
///
/// The main points are:
/// 1. Wrap the entire string in double quotes.
/// 2. Escape any existing backslash `\` as `\\`.
/// 3. Escape any existing quote `"` as `\"`.
///
/// # Examples
///
/// ```rust
/// use aoc_2015::Encode;
/// use pretty_assertions::assert_eq;
///
/// assert_eq!("".encode(), "\"\"");
/// assert_eq!("abc".encode(), "\"abc\"");
/// assert_eq!("\"".encode(), "\"\\\"\"");
/// assert_eq!("\\".encode(), "\"\\\\\"");
/// assert_eq!("\"\\\"".encode(), "\"\\\"\\\\\\\"\"");
/// ```
pub trait Encode {
    /// Returns the encoded version of this string, following AoC Day 8 part 2 rules.
    fn encode(&self) -> String;
}

impl<T> Encode for T
where
    T: AsRef<str>,
{
    fn encode(&self) -> String {
        let s = self.as_ref();
        let mut encoded = String::from("\"");

        for c in s.chars() {
            match c {
                '\\' => {
                    encoded.push_str("\\\\");
                }
                '"' => encoded.push_str("\\\""),
                _ => encoded.push(c),
            }
        }
        encoded.push('"');
        encoded
    }
}

#[aoc(day8, part1)]
fn solve_one(input: &str) -> usize {
    let mut raw_len = 0;
    let mut mem_len = 0;

    for line in input.lines() {
        raw_len += line.len();
        mem_len += line.memory_len();
    }

    raw_len - mem_len
}

#[aoc(day8, part2)]
fn solve_two(input: &str) -> usize {
    let mut raw_len = 0;
    let mut enc_len = 0;

    for line in input.lines() {
        raw_len += line.len();
        enc_len += line.encode().len();
    }

    enc_len - raw_len
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::input;

    use super::*;

    #[test]
    fn test_aoc_example_part_one() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

        assert_eq!(solve_one(input), 12);
    }

    #[test]
    fn test_aoc_example_part_two() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

        assert_eq!(solve_two(input), 19);
    }

    #[test]
    fn test_aoc_example_part_two_each() {
        assert_eq!(r#""""#.encode().len(), 6);
        assert_eq!(r#""abc""#.encode().len(), 9);
        assert_eq!(r#""aaa\"aaa""#.encode().len(), 16);
        assert_eq!(r#""\x27""#.encode().len(), 11);
    }

    #[test]
    fn empty_quotes() {
        // The string is just `""`. That means length is 2 in raw,
        // but in memory it's 0 characters.
        assert_eq!("\"\"".memory_len(), 0);
    }

    #[test]
    fn normal_chars() {
        // The string is `"abc"` in raw form.
        // Memory should be 3.
        assert_eq!("\"abc\"".memory_len(), 3);
    }

    #[test]
    fn escaped_quote() {
        // The string is `"aaa\"aaa"` (raw length is 10).
        // After skipping outer quotes, we parse `aaa\"aaa`.
        // The `\"` becomes just one character `"`.
        // So the memory is 7 characters total.
        assert_eq!("\"aaa\\\"aaa\"".memory_len(), 7);
    }

    #[test]
    fn hex_escape() {
        // The string is `"\x27"` in raw form, which is 4 chars + 2 quotes = 6 total.
        // In memory, `\x27` is a single character (ASCII 0x27, i.e. `'`).
        // So the memory length is 1.
        assert_eq!("\"\\x27\"".memory_len(), 1);
    }

    #[test]
    fn backslashes() {
        // The string is `"\\\\\\"`, meaning raw: quote, backslash, backslash, backslash,
        // backslash, backslash, quote. That's 7 in total.
        // Let's break it down inside (skipping outer quotes): `\\\\\`
        // We have 5 backslashes in a row inside.
        //
        // The first two `\\` -> 1 char in memory
        // Next two `\\` -> another 1 char in memory
        // The last one is just `\` alone (unrecognized escape?),
        // but let's see how we handle it:
        // Actually, since we always look ahead, we might treat them in pairs:
        // - `\\` => 1
        // - `\\` => 1
        // - leftover `\` => 1
        // total: 3
        assert_eq!("\"\\\\\\\\\\\"".memory_len(), 3);
    }

    #[test]
    fn random_example() {
        // Some random example: `"ab\\x20cd"`.
        // Outer quotes: skip them => ab\\x20cd
        // parse: 'a','b','\\','x','2','0','c','d'
        // inside:
        //   'a' => normal => count=1
        //   'b' => normal => count=2
        //   '\\' => check next => 'x' => that is a \x pattern
        //   so we skip \x plus 2 digits => total skip=4
        //   that means 1 char in memory => count=3
        //   next is 'c' => count=4
        //   next is 'd' => count=5
        let line = "\"ab\\x20cd\"";
        assert_eq!(line.memory_len(), 5);
    }

    #[test]
    fn test_encode_empty_string() {
        // Original: "" (2 characters)
        // Encoded: "\"\"\"\"" (6 characters)
        assert_eq!(r#""""#.encode(), r#""\"\"""#);
    }

    #[test]
    fn test_encode_simple_string() {
        // Original: "abc" (5 characters)
        // Encoded: "\"abc\"" (7 characters)
        assert_eq!(r#""abc""#.encode(), r#""\"abc\"""#);
    }

    #[test]
    fn test_encode_string_with_quotes() {
        // Original: "aaa\"aaa" (10 characters)
        // Encoded: "\"aaa\\\"aaa\""
        assert_eq!(r#""aaa\"aaa""#.encode(), r#""\"aaa\\\"aaa\"""#);
    }

    #[test]
    fn test_encode_string_with_quotes_2() {
        // Original: "aaa\\"aaa" (10 characters)
        // Encoded: "\"aaa\\\\\"aaa\""
        assert_eq!(r#""aaa\\"aaa""#.encode(), r#""\"aaa\\\\\"aaa\"""#);
    }

    #[test]
    fn test_encode_string_with_backslashes() {
        // Original: "\\" (2 characters)
        // Encoded: "\"\\\\\""
        assert_eq!(r#""\\""#.encode(), r#""\"\\\\\"""#);
    }

    #[test]
    fn test_encode_string_with_quotes_and_backslashes() {
        // Original: "\\\"" (3 characters)
        // Encoded: "\"\\\\\\\"\""
        assert_eq!(r#""\\\"""#.encode(), r#""\"\\\\\\\"\"""#);
    }

    #[test]
    fn test_encode_string_with_hex_escape() {
        // Original: "\x27" (4 characters)
        // Encoded: "\"\\x27\""
        assert_eq!(r#""\x27""#.encode(), r#""\"\\x27\"""#);
    }

    #[test]
    fn test_encode_complex_string() {
        // Original: "ab\\x27cd\"ef\\" (12 characters)
        // Encoded: "\"ab\\\\x27cd\\\"ef\\\\\""
        assert_eq!(
            r#""ab\\x27cd\"ef\\""#.encode(),
            r#""\"ab\\\\x27cd\\\"ef\\\\\"""#
        );
    }

    #[test]
    fn test_encode_multiple_escapes() {
        // Original: "\\\"\\x27\\" (6 characters)
        // Encoded: "\"\\\\\\\"\\x27\\\\\""
        assert_eq!(r#""\\\"\x27\\""#.encode(), r#""\"\\\\\\\"\\x27\\\\\"""#);
    }

    #[test]
    fn test_encode_unicode_characters() {
        // Original: "😊" (4 bytes in UTF-8)
        // Encoded: "\"😊\""
        // Note: Since Encode works with chars, it correctly handles multi-byte Unicode
        assert_eq!(r#""😊""#.encode(), r#""\"😊\"""#);
    }

    #[test]
    fn test_encode_string_with_mixed_escapes() {
        // Original: "a\\b\"c\\x27d" (10 characters)
        // Encoded: `\"a\\\\b\\\"c\\\\x27d\"`
        assert_eq!(
            r#""a\\b\"c\\x27d""#.encode(),
            r#""\"a\\\\b\\\"c\\\\x27d\"""#
        );
    }

    const INPUT: &str = input!("day8");

    #[test]
    fn integration_part_one() {
        assert_eq!(solve_one(INPUT), 1342);
    }

    #[test]
    fn integration_part_two() {
        assert_eq!(solve_two(INPUT), 2074);
    }
}
