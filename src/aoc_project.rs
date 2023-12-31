use std::{
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::solution::Solution;

/// A representation of a particpating AOC project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AocProject {
    username: String,
    repo: Url,
    location: PathBuf,
    input_cmd: String,
    entrypoint: String,
    language: String,
    #[serde(default)]
    bench_entrypoint: Option<String>,
    #[serde(default)]
    is_solver: bool,
    #[serde(default)]
    skip_inputs: bool,
}

impl AocProject {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn repo(&self) -> &Url {
        &self.repo
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn entrypoint(&self) -> &str {
        &self.entrypoint
    }

    pub fn bench_entrypoint(&self) -> &str {
        self.bench_entrypoint
            .as_deref()
            .unwrap_or(self.entrypoint())
    }

    pub fn is_solver(&self) -> bool {
        self.is_solver
    }

    pub fn skip_inputs(&self) -> bool {
        self.skip_inputs
    }

    pub fn input_path(&self, year: usize, day: usize) -> Result<Option<PathBuf>> {
        let output = self
            .input_command(year, day)?
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
    pub fn input_command(&self, year: usize, day: usize) -> Result<Command> {
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

        cmd.env("AOC_YEAR", year.to_string());
        cmd.env("AOC_DAY", day.to_string());
        cmd.env("AOC_CI", "true");
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

    pub fn solve(
        &self,
        year: usize,
        day: usize,
        input: &Path,
        timeout: Option<usize>,
    ) -> Result<Option<Solution>> {
        let output = self
            .solver_command(year, day, input, timeout)?
            .output()
            .context("Failed to execute command")?;

        if !output.status.success() {
            bail!("Failed to solve: {:?}", output);
        }

        let out = String::from_utf8_lossy(&output.stdout);

        if out.trim() == "not implemented" {
            return Ok(None);
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
    pub fn solver_command(
        &self,
        year: usize,
        day: usize,
        input: &Path,
        timeout: Option<usize>,
    ) -> Result<Command> {
        if input.is_relative() {
            bail!("Inputs provided to the solver must be absolute");
        }

        let parts = shell_words::split(self.entrypoint()).with_context(|| {
            format!(
                "Failed to parse entrypoint command for project: {}",
                self.username()
            )
        })?;

        let mut cmd = if let Some(time) = timeout {
            let mut cmd = Command::new("timeout");
            cmd.args(["-v", &time.to_string()]);
            cmd.args(parts);

            cmd
        } else {
            let (prog, args) = parts
                .split_first()
                .ok_or_else(|| anyhow!("Could not extract program"))?;

            let mut cmd = Command::new(prog);

            if !args.is_empty() {
                cmd.args(args);
            }

            cmd
        };

        cmd.env("AOC_YEAR", year.to_string());
        cmd.env("AOC_DAY", day.to_string());
        cmd.env("AOC_INPUT", input.to_string_lossy().to_string());
        cmd.env("AOC_JSON", "true");
        cmd.env("AOC_CI", "true");

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
            repo: Url::from_str("https://ancalagon.black/foo").unwrap(),
            location: "/foo/bar".into(),
            input_cmd: "echo 'not implemented'".into(),
            entrypoint: "echo 'not implemented'".into(),
            language: "cobol".into(),
            bench_entrypoint: None,
            is_solver: false,
            skip_inputs: false,
        };

        let expected = PathBuf::from_str("/foo/bar/baz.txt").unwrap();
        let joined = project.join_with_location("./baz.txt").unwrap();
        assert_eq!(joined, expected);

        let expected = PathBuf::from_str("/foo/bar/baz.txt").unwrap();
        let joined = project.join_with_location("baz.txt").unwrap();
        assert_eq!(joined, expected);
    }
}
