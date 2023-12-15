use axum::{Json, debug_handler, http::StatusCode};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Input {
    input: String,
}

/// All integers (sequences of consecutive digits) in the string must
/// add up to 2023.
fn rule4(s: &str) -> bool {
    let mut sum = 0;
    let mut chars = s.char_indices();
    while let Some((i, c)) = chars.next() {
        if ! c.is_digit(10) { continue }
        let mut j = i;
        while let Some((j1, c)) = chars.next() {
            j = j1;
            if ! c.is_digit(10) { break; }
        }
        let n: usize = s[i .. j].parse().unwrap();
        sum += n;
    }
    sum == 2023
}

/// Must contain the letters 'j', 'o', and 'y' in that order and in no
/// other order.
fn rule5(s: &str) -> bool {
    let mut seen_j = false;
    let mut seen_o = false;
    let mut seen_y = false;
    for c in s.chars() {
        if c == 'j' {
            if seen_o || seen_y { return false }
            seen_j = true;
        } else if c == 'o' {
            if seen_y { return false }
            seen_o = true;
        } else if c == 'y' {
            seen_y = true;
        }
    }
    seen_j && seen_o && seen_y
}

/// Must contain a letter that repeats with exactly one other letter
/// between them (like xyx).
fn rule6(s: &str) -> bool {
    if s.len() <= 2 { return false }
    let mut c0 = ' ';
    let mut c1 = ' ';
    for c in s.chars() {
        if ! c.is_alphabetic() {
            c0 = ' '; // reset to non alphanetic
            c1 = ' ';
            continue
        }
        if c != c1 && c == c0 {
            return true
        }
        c0 = c1;
        c1 = c;
    }
    false
}

/// Must contain at least one unicode character in the range [U+2980,
/// U+2BFF].
fn rule7(s: &str) -> bool {
    s.chars().find(|&c| '\u{2980}' <= c && c <= '\u{2BFF}').is_some()
}

/// Must contain at least one emoji.
fn rule8(s: &str) -> bool {
    // https://www.unicode.org/emoji/charts/full-emoji-list.html
    s.chars().find(|&c| emojis::get(&String::from(c)).is_some()).is_some()
}

/// The hexadecimal representation of the sha256 hash of the string
/// must end with an 'a'.
fn rule9(s: &str) -> bool {
    let s = sha256::digest(s);
    s.ends_with('a')
}


pub async fn game(Json(s): Json<Input>) -> (StatusCode, String) {
    let s = s.input;
    let (code, reason) = if s.len() < 8 {
        (StatusCode::BAD_REQUEST, "8 chars")
    } else if !(s.chars().any(|c| c.is_lowercase())
        && s.chars().any(|c| c.is_uppercase())
        && s.chars().any(|c| c.is_digit(10)))
    {
        (StatusCode::BAD_REQUEST, "more types of chars")
    } else if s.chars().filter(|c| c.is_digit(10)).count() < 5 {
        (StatusCode::BAD_REQUEST, "55555")
    } else if !rule4(&s) {
        (StatusCode::BAD_REQUEST, "math is hard")
    } else if !rule5(&s) {
        (StatusCode::NOT_ACCEPTABLE , "not joyful enough")
    } else if !rule6(&s) {
        (StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS , "illegal: no sandwich")
    } else if !rule7(&s) {
        (StatusCode::RANGE_NOT_SATISFIABLE , "outranged")
    } else if !rule8(&s) {
        (StatusCode::UPGRADE_REQUIRED , "😳")
    } else if !rule9(&s) {
        (StatusCode::IM_A_TEAPOT , "not a coffee brewer")
    } else {
        return (StatusCode::OK, "{\"result\":\"nice\",\
        \"reason\":\"that's a nice password\"}".to_string());
    };
    (code, format!("{{\"result\":\"naughty\",\"reason\":\"{reason}\"}}"))
}

fn is_nice(s: &str) -> bool {
    let vowels = "aeiouy";
    let mut n_vowels = 0;
    let mut prev = '\0';
    let mut twice_in_a_row = false;
    for c in s.chars() {
        if vowels.contains(c) {
            n_vowels += 1;
        }
        if c.is_alphabetic() && c == prev {
            twice_in_a_row = true;
        }
        prev = c;
    }
    n_vowels >= 3 && twice_in_a_row
        && ! s.contains("ab") && ! s.contains("cd")
        && ! s.contains("pq") && ! s.contains("xy")
}

#[debug_handler]
pub async fn nice(Json(s): Json<Input>) -> (StatusCode, String) {
     if is_nice(&s.input) {
        (StatusCode::OK, "{\"result\":\"nice\"}".to_string())
    } else {
        (StatusCode::BAD_REQUEST, "{\"result\":\"naughty\"}".to_string())
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_nice() {
        assert!(is_nice("hello there"));
        assert!(!is_nice("abcd"));
        assert!(!is_nice("he77o there"));
        assert!(!is_nice("hello"));
        assert!(!is_nice("hello xylophone"));
        assert!(!is_nice("password"));
    }

    #[test]
    fn test_rule4() {
        assert!(rule4("er2000defz23fre"));
        assert!(rule4("er1800defz220f3re"))
    }

    #[test]
    fn test_rule5() {
        assert!(rule5("jjazqooDESyay"));
        assert!(!rule5("jyj"));
        assert!(!rule5("jazeosd"));
    }

    #[test]
    fn test_rule6() {
        assert!(rule6("xyx"));
        assert!(!rule6("xxx"));
        assert!(rule6("xyabcb"));
        assert!(!rule6("xy7x"));
    }

    #[test]
    fn test_rule7() {
        assert!(rule7("a unicode ⦀ char 2980"));
        assert!(rule7("a unicode ⨀ char 2A00"));
        assert!(rule7("a unicode ⯿ char 2BFF"));
        assert!(!rule7("a unicode 😀 char"));
        assert!(!rule7("xxx"));
        assert!(!rule7("xy∫cb"));
    }

    #[test]
    fn test_rule8() {
        assert!(!rule8("a unicode ⯿ char 2BFF"));
        assert!(rule8("a unicode 😀 char"));
        assert!(!rule8("xxx"));
        assert!(!rule8("xy∫cb"));
    }
}
