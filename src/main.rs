use clap::Command;
use std::ffi::OsString;
use std::io::ErrorKind;
use std::process::{self, Command as Cmd};

fn main() {
    let mut cli = Command::new("specmd")
        .about("Markdown for testing CLI commands")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true);

    let matches = cli.get_matches_mut();
    let (subcmd, sub_matches) = matches
        .subcommand()
        .expect("subcommand_required guarantees a subcommand");

    // External subcommand args live under the empty-string id
    let args: Vec<OsString> = sub_matches
        .get_many::<OsString>("")
        .map(|vals| vals.cloned().collect())
        .unwrap_or_default();

    let (bin, args) = match (subcmd, args.first()) {
        ("help", None) => {
            cli.print_help().unwrap();
            return;
        }
        ("help", Some(target)) => (
            format!("specmd-{}", target.to_string_lossy()),
            vec![OsString::from("--help")],
        ),
        _ => (format!("specmd-{subcmd}"), args),
    };

    match Cmd::new(&bin).args(&args).status() {
        Ok(status) => process::exit(status.code().unwrap_or(1)),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            eprintln!("specmd: '{subcmd}' is not a specmd command. See 'specmd --help'.");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("specmd: failed to run '{bin}': {e}");
            process::exit(1);
        }
    }
}
