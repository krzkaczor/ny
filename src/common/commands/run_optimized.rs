use crate::agent::Agent;

pub fn try_optimize_script_execution<'a>(
    agent: &'a Agent,
    script: &'a str,
) -> Option<Vec<&'a str>> {
    let run_prefix = agent_to_run_command(agent);
    let chunks: Vec<_> = script.split("&&").map(str::trim).collect();

    let optimizable = chunks.iter().all(|chunk| chunk.starts_with(run_prefix));

    if !optimizable {
        return None;
    }

    let stripped = chunks.iter().map(|chunk| chunk[run_prefix.len()..].trim());

    Some(stripped.collect())
}

// @todo: this could return also common shortcuts like npm test
fn agent_to_run_command(agent: &Agent) -> &str {
    match agent {
        Agent::Npm => "npm run ",
        Agent::Pnpm => "pnpm run ",
        Agent::Yarn => "yarn run ",
        Agent::Bun => "bun run ",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stripping_pnpm_runs() {
        let result = try_optimize_script_execution(&Agent::Pnpm, "pnpm run a && pnpm run b");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), vec!["a", "b"]);
    }

    #[test]
    fn test_does_not_optimize_other_calls() {
        let result = try_optimize_script_execution(&Agent::Pnpm, "mocha --timeout=0");
        assert!(result.is_none());
    }
}
