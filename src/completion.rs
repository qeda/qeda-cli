use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::error::{Context, Result};
use crate::index;

const BASH_SCRIPT: &str = r#"_qeda() {
    local words current
    words="${COMP_WORDS[@]}"
    current="${COMP_CWORD}"
    COMPREPLY=( $(qeda completion -w "${words}" -c "${current}" -m 4096 2> /dev/null) )
}
complete -F _qeda qeda
"#;

const SUBCOMMANDS: [&str; 13] = [
    "add",
    "completion",
    "config",
    "generate",
    "ground",
    "help",
    "index",
    "list",
    "load",
    "power",
    "reset",
    "test",
    "update",
];

pub fn bash_script() -> &'static str {
    BASH_SCRIPT
}

pub fn install() -> Result<()> {
    if cfg!(target_os = "linux") {
        let bash_dirs = vec![
            "/usr/share/bash-completion/completions",
            "/etc/bash_completion.d",
        ];
        for dir in bash_dirs {
            if Path::new(dir).is_dir() {
                return install_completion_linux("Bash", dir, BASH_SCRIPT);
            }
        }
    }

    if cfg!(target_os = "macos") {
        // Find bash completion scripts directory
        warn!("not implemented yet");
    }

    if cfg!(target_os = "windows") {
        // TODO: Install PowerShell completion script
        warn!("not implemented yet");
    }
    Ok(())
}

pub fn words(words: &str, current: usize, max_count: usize) -> Vec<String> {
    let mut result = Vec::new();
    let words: Vec<_> = words.split(' ').collect();
    let prev_words: HashSet<_> = words.iter().rev().skip(1).cloned().collect();

    if current == 1 {
        // Show subcommands
        let prefix = if words.len() > 1 { words[1] } else { "" };
        add_words(&mut result, &SUBCOMMANDS, prefix);
        let options = ["-h", "--help", "-V", "--version"];
        add_words(&mut result, &options, prefix);
    } else if words.len() >= 2 && current >= 2 {
        // Subcommands parameters
        let subcommand = words[1];
        let prefix = if words.len() > current {
            words[current]
        } else {
            ""
        };
        match subcommand {
            "add" | "load" | "test" => {
                if current == 2 {
                    if prefix.starts_with('-') {
                        let options = ["-h", "--help"];
                        add_words(&mut result, &options, prefix);
                    } else {
                        result.append(&mut index::list(prefix, max_count));
                    }
                }
            }
            "completion" => {
                let options = ["-b", "--bash", "-i", "--install", "-w", "--words"];
                let options_set: HashSet<_> = options.iter().cloned().collect();
                if prev_words.intersection(&options_set).next().is_none() {
                    add_words(&mut result, &options, prefix);
                }
            }
            "config" => {
                if current == 2 {
                    if prefix.starts_with('-') {
                        let options = ["-h", "--help", "-l", "--list"];
                        add_words(&mut result, &options, prefix);
                    } else {
                        let params = load_config!("qeda.yml").keys();
                        result.append(
                            &mut params
                                .into_iter()
                                .filter(|s| s.starts_with(prefix))
                                .collect(),
                        );
                    }
                }
            }
            "generate" | "power" | "reset" => {
                let options = ["-h", "--help"];
                let options_set: HashSet<_> = options.iter().cloned().collect();
                if prev_words.intersection(&options_set).next().is_none() {
                    add_words(&mut result, &options, prefix);
                }
            }
            "ground" => {
                let options = [
                    "-h",
                    "--help",
                    "-s",
                    "--signal",
                    "-c",
                    "--chassis",
                    "-e",
                    "--earth",
                ];
                let options_set: HashSet<_> = options.iter().cloned().collect();
                if prev_words.intersection(&options_set).next().is_none() {
                    add_words(&mut result, &options, prefix);
                }
            }
            "help" => {
                if current == 2 {
                    add_words(&mut result, &SUBCOMMANDS, prefix);
                }
            }
            "index" | "list" => {
                let options = ["-h", "--help", "-m", "--max"];
                let options_set: HashSet<_> = options.iter().cloned().collect();
                if prev_words.intersection(&options_set).next().is_none() {
                    add_words(&mut result, &options, prefix);
                }
            }
            "update" => {
                let options = ["-h", "--help", "-f", "--force"];
                let options_set: HashSet<_> = options.iter().cloned().collect();
                if prev_words.intersection(&options_set).next().is_none() {
                    add_words(&mut result, &options, prefix);
                }
            }

            _ => (),
        }
    }
    result
}

fn add_words(words: &mut Vec<String>, new_words: &[&str], prefix: &str) {
    words.append(
        &mut new_words
            .iter()
            .filter(|s| s.starts_with(prefix))
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
    );
}

fn install_completion_linux(sh: &str, dir: &str, script: &str) -> Result<()> {
    info!("{} completion directory found: '{}'", sh, dir);
    let script_path = format!("{}/qeda", dir);
    let mut f = File::create(&script_path)
        .with_context(|| "cannot create completion script, consider using 'sudo'")?;
    writeln!(f, "{}", script)
        .with_context(|| "cannot write completion script, consider using 'sudo'")?;
    info!("{} completion script installed", sh);
    info!("run for this {} session:", sh);
    infoln!("  source {} && complete -p qeda", &script_path);
    Ok(())
}
