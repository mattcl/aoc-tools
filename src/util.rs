use serde_json::Value;

/// Given a day, returns a zero-padded directory name for that day.
pub fn day_directory_name(day: usize) -> String {
    format!("day_{:03}", day)
}

// Prepare a value for displaying in a markdown table on github.
//
// We display solutions in <pre> tags in markdown tables.
//
// If the value is a string, this will:
// - replace periods in strings with whitespace
// - replace #'s in strings with a block character
// - replace newlines with <br> tags
//
// Otherwise the Value is converted to a String via `.to_string()`
pub fn sanitize_value_for_display(value: &Value) -> String {
    match value {
        Value::String(s) => s
            .replace('.', " ")
            .replace('#', "&#9608;")
            .replace('\n', "<br>"),
        x => x.to_string(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_directory_name() {
        let out = super::day_directory_name(1);
        assert_eq!(out, String::from("day_001"));

        let out = super::day_directory_name(3);
        assert_eq!(out, String::from("day_003"));

        let out = super::day_directory_name(11);
        assert_eq!(out, String::from("day_011"));

        let out = super::day_directory_name(25);
        assert_eq!(out, String::from("day_025"));
    }
}
