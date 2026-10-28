use clap::Parser;
use std::process::{Command, exit};

#[derive(Parser)]
#[command(version, about, long_about = Some("Run a command and output a specmd file"))]
struct Cli {
    #[arg(short, long)]
    shell: Option<String>,

    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    cmd: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let cmd = cli.cmd.join(" ");
    let shell = cli.shell.unwrap_or(String::from("/bin/sh"));

    match Command::new(&shell)
        .args(vec!["-c", &format!("{cmd}")])
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let code = output.status.code().unwrap_or(1);

            println!("```shell {shell}\n{cmd}\n```\n");
            println!("```stdout\n{stdout}```\n");
            println!("```stderr\n{stderr}```\n");
            println!("```exit\n{code}\n```\n");

            exit(code);
        }
        Err(e) => {
            eprintln!("Error running '{cmd}': {e}");
            exit(1);
        }
    }
}
