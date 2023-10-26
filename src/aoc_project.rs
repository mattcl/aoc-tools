use std::{
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::solution::Solution;

/// A representation of a particpating AOC project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AocProject {
    username: String,
    location: PathBuf,
    input_cmd: String,
    entrypoint: String,
    #[serde(default)]
    is_solver: bool,
}

impl AocProject {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn is_solver(&self) -> bool {
        self.is_solver
    }

    pub fn input_path(&self, day: usize) -> Result<Option<PathBuf>> {
        let output = self
            .input_command(day)?
            .output()
            .context("Failed to execute command")?;

        if !output.status.success() {
            // we assume that any nonzero is indicates that day does not exist
            return Ok(None);
        }

        let raw = String::from_utf8(output.stdout).with_context(|| {
            format!(
                "Failed to parse output as utf-8 for project: {}",
                &self.username()
            )
        })?;
        let raw_path = PathBuf::from_str(raw.trim()).with_context(|| {
            format!(
                "Failed to parse output as a path for project: {}",
                &self.username()
            )
        })?;

        Ok(Some(self.join_with_location(raw_path)?))
    }

    /// Construct a [Command] to get the path to the input for a given day.
    ///
    /// This command is set up with the `current_dir` as the project's location.
    pub fn input_command(&self, day: usize) -> Result<Command> {
        let parts = shell_words::split(&self.input_cmd).with_context(|| {
            format!(
                "Failed to parse input command for project: {}",
                &self.username()
            )
        })?;
        let (prog, args) = parts.split_first().ok_or_else(|| {
            anyhow!(
                "Could not extract program for project: {}",
                &self.username()
            )
        })?;

        let mut cmd = Command::new(prog);

        cmd.env("AOC_DAY", day.to_string());
        cmd.current_dir(&self.location);

        if !args.is_empty() {
            cmd.args(args);
        }

        Ok(cmd)
    }

    /// Join the given path with the locatio of the project.
    pub fn join_with_location<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
        let path = path.as_ref();
        // We don't want to let the project come up with a path that's outside
        // of its directory.
        if path.is_absolute() {
            bail!("Path must be relative: '{}'", path.to_string_lossy());
        }

        Ok(self.location.join(path))
    }

    pub fn solve(&self, day: usize, input: &Path) -> Result<Option<Solution>> {
        let output = self
            .solver_command(day, input)?
            .output()
            .context("Failed to execute command")?;

        if !output.status.success() {
            bail!("Failed to solve: {:?}", output);
        }

        let raw_solution: Value = serde_json::from_slice(&output.stdout)?;

        if let Value::String(ref msg) = raw_solution {
            if msg == "not implemented" {
                return Ok(None);
            }
        }

        let solution: Solution =
            serde_json::from_value(raw_solution).context("Solution is in invalid format.")?;

        Ok(Some(solution))
    }

    /// Get a command to produce the solution for a given day and absolute path
    /// to an input.
    pub fn solver_command(&self, day: usize, input: &Path) -> Result<Command> {
        if input.is_relative() {
            bail!("Inputs provided to the solver must be absolute");
        }

        let parts = shell_words::split(&self.entrypoint).with_context(|| {
            format!(
                "Failed to parse entrypoint command for project: {}",
                self.username()
            )
        })?;
        let (prog, args) = parts
            .split_first()
            .ok_or_else(|| anyhow!("Could not extract program"))?;

        let mut cmd = Command::new(prog);

        cmd.env("AOC_DAY", day.to_string());
        cmd.env("AOC_INPUT", input.to_string_lossy().to_string());
        cmd.env("AOC_JSON", "true");
        cmd.current_dir(&self.location);

        if !args.is_empty() {
            cmd.args(args);
        }

        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_with_location() {
        let project = AocProject {
            username: "mattcl".into(),
            location: "/foo/bar".into(),
            input_cmd: "echo 'not implemented'".into(),
            entrypoint: "echo 'not implemented'".into(),
            is_solver: false,
        };

        let expected = PathBuf::from_str("/foo/bar/baz.txt").unwrap();
        let joined = project.join_with_location("./baz.txt").unwrap();
        assert_eq!(joined, expected);

        let expected = PathBuf::from_str("/foo/bar/baz.txt").unwrap();
        let joined = project.join_with_location("baz.txt").unwrap();
        assert_eq!(joined, expected);
    }
}
