use std::borrow::Cow;

use crate::ast::Charset;

pub(crate) fn decode<'a>(input: &'a [u8], charset: &Charset) -> Result<Cow<'a, str>, ()> {
    let decoder = match charset {
        Charset::ASCII => encoding_rs::UTF_8,
        Charset::ShiftJIS => encoding_rs::SHIFT_JIS,
        Charset::UTF8 => encoding_rs::UTF_8,
        Charset::Default => encoding_rs::UTF_8,
    };

    let (cow, encoding_used, had_errors) = decoder.decode(input);
    if had_errors || encoding_used != decoder {
        Err(())
    } else {
        Ok(cow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod decode {
        use encoding_rs::{SHIFT_JIS, UTF_8};

        use super::*;

        #[test]
        fn success_when_valid_str() {
            let case_raw = r#"charset,Shift_JIS
// あいうえおかきくけこ
"#;
            let (case, _, _) = SHIFT_JIS.encode(case_raw);
            let result = decode(&case, &Charset::ShiftJIS).unwrap();
            assert_eq!(result, case_raw);

            let case_raw = r#"charset,UTF-8
// あいうえおかきくけこ
"#;
            let (case, _, _) = UTF_8.encode(case_raw);
            let result = decode(&case, &Charset::UTF8).unwrap();
            assert_eq!(result, case_raw);
        }

        #[test]
        fn failed_when_invalid_str() {
            let case_raw = r#"charset,Shift_JIS
// あいうえおかきくけこ
"#;
            let (case, _, _) = UTF_8.encode(case_raw);
            assert!(decode(&case, &Charset::ShiftJIS).is_err());
        }
    }
}
