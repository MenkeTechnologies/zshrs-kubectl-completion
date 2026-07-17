//! **kubectl completion** as a native zshrs plugin.
//!
//! This is the native equivalent of a zsh `_kubectl` completion (e.g.
//! nnao45/zsh-kubectl-completion), but instead of a large static, version-
//! pinned `_arguments` tree it delegates to **cobra's built-in completion
//! protocol** — `kubectl __complete <args…> <partial>` — which every modern
//! kubectl exposes. That returns candidates (subcommands, flags, and live
//! resources) for *the installed kubectl version*, one `name\tdescription`
//! per line, terminated by a `:<directive>` line.
//!
//! So the whole completion is: forward the current words to
//! `kubectl __complete` and emit the candidate column. It tracks whatever
//! kubectl is on `PATH`, needs no per-version maintenance, and covers
//! resources dynamically (subject to cluster access, exactly like the zsh
//! completion's `kubectl get` calls).
//!
//! Load after `compinit`; then `kubectl <TAB>`, `kubectl get po<TAB>`, etc.

use std::os::raw::c_int;
use std::process::Command;
use znative::{declare_plugin, Args, Host};

/// Build the `kubectl __complete …` argv from `$CURRENT` and the command
/// words (`words[0]` is "kubectl"). cobra wants the args after the command
/// with the word being completed as the final arg. `.max(1)` guards the
/// `words[1..end]` slice so a stray `$CURRENT` of 0 cannot panic.
fn complete_args(current: usize, words: &[String]) -> Vec<String> {
    let end = current.min(words.len()).max(1);
    let mut cargs: Vec<String> = vec!["__complete".to_string()];
    cargs.extend(words[1..end].iter().cloned());
    // Completing a fresh word past the last typed one → append the empty
    // partial cobra expects (e.g. `kubectl get <TAB>` → all resource types).
    if current > words.len() {
        cargs.push(String::new());
    }
    cargs
}

/// Parse cobra `__complete` stdout into candidates: the pre-tab column of
/// each `name\tdescription` line, stopping at the trailing `:<directive>`
/// line and skipping blanks.
fn parse_completions(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    for line in text.lines() {
        // A `:<int>` line is cobra's trailing ShellCompDirective — stop there.
        if line.starts_with(':') {
            break;
        }
        if line.is_empty() {
            continue;
        }
        // "candidate\tdescription" → candidate.
        let cand = line.split('\t').next().unwrap_or(line);
        if !cand.is_empty() {
            out.push(cand);
        }
    }
    out
}

/// compsys generator for `kubectl`. `args.rest()` is `[$CURRENT, word0,
/// word1, …]` where `word0 == "kubectl"` and `$CURRENT` is the 1-based
/// index of the word being completed.
fn kubectl_complete(host: &Host, args: &Args) -> c_int {
    let a = args.rest();
    let Some(current) = a.first().and_then(|s| s.parse::<usize>().ok()) else {
        return 1;
    };
    let words = &a[1..]; // ["kubectl", "get", "po", …]
    if words.is_empty() {
        return 0;
    }

    let cargs = complete_args(current, words);
    let Ok(out) = Command::new("kubectl").args(&cargs).output() else {
        return 0; // kubectl not on PATH — no completions, no error noise.
    };
    let text = String::from_utf8_lossy(&out.stdout);
    for cand in parse_completions(&text) {
        host.add_match(cand);
    }
    0
}

declare_plugin! {
    name: "kubectl-completion",
    version: "0.1.0",
    completions: {
        "kubectl" => kubectl_complete,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn complete_args_mid_word() {
        // `kubectl get po<TAB>` : CURRENT=3, completing the 3rd word "po"
        assert_eq!(
            complete_args(3, &words(&["kubectl", "get", "po"])),
            words(&["__complete", "get", "po"])
        );
    }

    #[test]
    fn complete_args_fresh_word_appends_empty() {
        // `kubectl get <TAB>` : CURRENT=3 past the 2 typed words -> empty partial
        assert_eq!(
            complete_args(3, &words(&["kubectl", "get"])),
            words(&["__complete", "get", ""])
        );
    }

    #[test]
    fn complete_args_guards_zero_current() {
        // a stray CURRENT=0 must not panic on words[1..0]
        assert_eq!(
            complete_args(0, &words(&["kubectl"])),
            words(&["__complete"])
        );
    }

    #[test]
    fn parse_completions_extracts_candidate_column() {
        let out = "get\tDisplay resources\ndelete\tDelete resources\n\napply\tApply config\n:4\nafter-directive\n";
        assert_eq!(parse_completions(out), vec!["get", "delete", "apply"]);
    }

    #[test]
    fn parse_completions_plain_and_empty() {
        assert_eq!(parse_completions("pod\nservice\n"), vec!["pod", "service"]);
        assert_eq!(parse_completions(""), Vec::<&str>::new());
        assert_eq!(parse_completions(":0\n"), Vec::<&str>::new());
    }
}
