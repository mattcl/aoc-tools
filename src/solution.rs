use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::{Deref, DerefMut},
    path::Path,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Solution {
    part_one: Value,
    part_two: Value,
}

impl Solution {
    /// Compare the solution components by conversting the values to strings.
    ///
    /// This is done to handle projects whose solvers do not produce typed
    /// solutions, or those whose types differ from the reference solutions.
    pub fn string_compare(&self, other: &Self) -> bool {
        compare_value(self.part_one(), other.part_one())
            && compare_value(self.part_two(), other.part_two())
    }

    /// Compare a solution for the last day of the event.
    ///
    /// This is clunky, but day 25 only has one part, so we need to disregard
    /// whatever people put as the part two solution.
    pub fn last_day_compare(&self, other: &Self) -> bool {
        self.part_one.to_string().trim() == other.part_one.to_string().trim()
    }

    pub fn part_one(&self) -> &Value {
        &self.part_one
    }

    pub fn part_two(&self) -> &Value {
        &self.part_two
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "part_one: {}\npart_two: {}",
            self.part_one(),
            self.part_two()
        )
    }
}

fn compare_value(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Bool(l), Value::String(r)) => l.to_string().trim() == r.trim(),
        (Value::Number(l), Value::String(r)) => l.to_string().trim() == r.trim(),
        (Value::String(l), Value::Bool(r)) => l.trim() == r.to_string().trim(),
        (Value::String(l), Value::Number(r)) => l.trim() == r.to_string().trim(),
        (Value::String(l), Value::String(r)) => l.trim() == r.trim(),
        _ => left.to_string().trim() == right.to_string().trim(),
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Solutions(BTreeMap<String, Solution>);

impl Deref for Solutions {
    type Target = BTreeMap<String, Solution>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Solutions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Solutions {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path).context("Failed to read solution file")?;
        serde_json::from_str(&contents).context("Failed to parse solution file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_compare() {
        let input1 = "{\"part_one\": 15, \"part_two\": 16}";
        let input2 = "{\"part_one\": \"15\", \"part_two\": \"16\"}";
        let input3 = "{\"part_one\": \"65\", \"part_two\": \"16\"}";

        let s1: Solution = serde_json::from_str(input1).unwrap();
        let s2: Solution = serde_json::from_str(input2).unwrap();
        let s3: Solution = serde_json::from_str(input3).unwrap();

        assert!(s1.string_compare(&s2));
        assert!(!s1.string_compare(&s3));
        assert!(!s2.string_compare(&s3));
    }
}
