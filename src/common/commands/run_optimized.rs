use crate::agent::Agent;

pub fn try_optimize_script_execution<'a>(
    agent: &'a Agent,
    script: &'a str,
) -> Option<Vec<&'a str>> {
    // let run_prefix = agent_to_run_command(agent);
    let chunks: Vec<_> = script.split("&&").map(str::trim).collect();

    let optimized: Vec<_> = chunks
        .iter()
        .map(|chunk| optimize_script(agent, chunk))
        .collect();

    if optimized.contains(&None) {
        return None;
    }

    let unwrapped: Vec<&str> = optimized.into_iter().map(|opt| opt.unwrap()).collect();
    Some(unwrapped)
}

fn optimize_script<'a>(agent: &Agent, script: &'a str) -> Option<&'a str> {
    match agent {
        Agent::Pnpm => {
            let prefix = "pnpm run ";
            if !script.starts_with(prefix) {
                return None;
            }
            if script.contains("--filter") {
                return Some(script);
            }
            Some(script[prefix.len()..].trim())
        }
        Agent::Npm => {
            let prefix = "npm run ";
            if !script.starts_with(prefix) {
                return None;
            }
            if script.contains("--workspaces") {
                return Some(script);
            }
            Some(script[prefix.len()..].trim())
        }
        Agent::Yarn => {
            let prefix = "yarn run ";
            Some(script[prefix.len()..].trim())
        }
        Agent::Bun => None,
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
    fn test_pnpm_does_not_optimize_calls_with_filtering() {
        let result = try_optimize_script_execution(
            &Agent::Pnpm,
            "pnpm run biome check . && pnpm run --parallel --filter './packages/**' verify",
        );
        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            vec![
                "biome check .",
                "pnpm run --parallel --filter './packages/**' verify"
            ]
        );
    }

    #[test]
    fn test_npm_does_not_optimize_calls_with_filtering() {
        let result = try_optimize_script_execution(
            &Agent::Npm,
            "npm run biome check . && npm run --workspaces verify",
        );
        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            vec!["biome check .", "npm run --workspaces verify"]
        );
    }

    #[test]
    fn test_does_not_optimize_other_calls() {
        let result = try_optimize_script_execution(&Agent::Pnpm, "mocha --timeout=0");
        assert!(result.is_none());
    }
}
