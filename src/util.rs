/// Given a day, returns a zero-padded directory name for that day.
pub fn day_directory_name(day: usize) -> String {
    format!("day_{:03}", day)
}

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
