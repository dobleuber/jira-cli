pub fn get_column_string(text: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    let truncated_text = if text.len() > width {
        if width <= 3 {
            ".".repeat(width)
        } else {
            format!("{}...", &text[..width - 3])
        }
    } else {
        text.to_string()
    };

    format!("{:width$}", truncated_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned(), "1");

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned(), "2");

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned(), "3");

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned(), "4");

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    }
}
