use clap::Parser;
use console::{StyledObject, style};
use similar::{Algorithm, ChangeTag, TextDiff};
use specmd::prelude::{SpecMdResult, parse};
use std::process::Command;

#[derive(Parser)]
#[command(version, about, long_about = Some("Run a command and output a specmd file"))]
struct Cli {
    #[arg(short, long)]
    shell: Option<String>,

    path: Vec<String>,
}

fn main() -> SpecMdResult<()> {
    let diff_algorithm = Algorithm::Myers;

    let cli = Cli::parse();
    let shell = cli.shell.unwrap_or(String::from("/bin/sh"));

    for path in cli.path.iter() {
        let content = std::fs::read_to_string(path).expect("should open file path passed");

        let specmd_file = parse(&content)?;

        let cmd = &content[specmd_file.shell];
        let expected_stdout = &content[specmd_file.stdout];
        let expected_stderr = &content[specmd_file.stderr];
        let expected_exit = &content[specmd_file.exit];
        let expected_exit_code =
            i32::from_str_radix(expected_exit, 10).expect("should parse as i32");

        eprintln!("{}\n\n{}\n", style(path).underlined(), style(cmd).dim());

        let mut vars: Vec<(String, String)> = vec![];

        if let Some(env) = specmd_file.env {
            let env_text = &content[env];

            vars = dotenvy::from_read_iter(env_text.as_bytes())
                .collect::<Result<_, _>>()
                .unwrap();
        }

        match Command::new(&shell)
            .args(vec!["-c", &format!("{cmd}")])
            .envs(vars)
            .output()
        {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let code = output.status.code().unwrap_or(1);

                if expected_stdout != stdout {
                    println!("{}\n", style("Mismatch: stdout").bold());

                    let diff = TextDiff::configure()
                        .algorithm(diff_algorithm)
                        .diff_lines(expected_stdout, stdout);

                    for change in diff.iter_all_changes() {
                        let (sign, style_fn): (&str, Box<dyn Fn(&str) -> StyledObject<&str>>) =
                            match change.tag() {
                                ChangeTag::Delete => ("- |", Box::new(|s: &str| style(s).red())),
                                ChangeTag::Insert => ("+ |", Box::new(|s: &str| style(s).green())),
                                ChangeTag::Equal => (
                                    &style("  |").dim().to_string(),
                                    Box::new(|s: &str| style(s)),
                                ),
                            };

                        print!("{}{}", style_fn(sign), style_fn(change.value_ref()));
                    }

                    println!("");
                }

                if expected_stderr != stderr {
                    println!("{}\n", style("Mismatch: stderr").bold());

                    let diff = TextDiff::configure()
                        .algorithm(diff_algorithm)
                        .diff_lines(expected_stderr, stderr);

                    for change in diff.iter_all_changes() {
                        let (sign, style_fn): (&str, Box<dyn Fn(&str) -> StyledObject<&str>>) =
                            match change.tag() {
                                ChangeTag::Delete => ("- |", Box::new(|s: &str| style(s).red())),
                                ChangeTag::Insert => ("+ |", Box::new(|s: &str| style(s).green())),
                                ChangeTag::Equal => (
                                    &style("  |").dim().to_string(),
                                    Box::new(|s: &str| style(s)),
                                ),
                            };

                        print!("{}{}", style_fn(sign), style_fn(change.value_ref()));
                    }

                    println!("");
                }

                assert!(
                    expected_exit_code == code,
                    "exit code: Expected: {expected_exit_code} but got: {code}"
                );
            }
            Err(e) => {
                eprintln!("Error running '{cmd}': {e}");
            }
        }
    }

    Ok(())
}
