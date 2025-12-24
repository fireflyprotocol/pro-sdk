//! This is how Bluefin generates Rust code from the accompanying OpenAPI specifications.
//!
//! ```sh
//! cargo run --bin apigen
//! ```
//!
//! If you don't have `cargo`, start [here](https://rustup.rs/).
//!
//! # TODO
//!
//! - [ ] Automate publication to package repositories: PyPI, crates.io, and NPM

use std::path::PathBuf;
use std::process::{Command, exit};
use std::{env, fmt, io};

use tools::Lang;

// Import version bump functionality from library (only when feature is enabled)
#[cfg(feature = "version-bump")]
use tools::semver;

/// Name of the directory where OpenAPI YAML specs live.
const INPUT_DIR: &str = "resources";

const USAGE: &str = "

    apigen [ {-l | --lang} { rust | python | typescript | rs | py | ts } ]...
           [ -B | --no-bump ]

Options:
    -l, --lang <LANG>   Target language(s) for code generation
    -B, --no-bump       Skip version bumping (state file still updated)
    -h, --help          Show this help message

Please ensure that npm and openapi-generator-cli are installed following the instructions at:
https://openapi-generator.tech/docs/installation
";

#[derive(Debug)]
pub enum Error {
    /// A command line flag was not recognized.
    Flag(String),
    /// System interaction failed.
    Io(io::Error),
    /// The supplied value is not a supported target language.
    Lang(String),
    /// The input directory could not be found.
    Path,
    /// A subcommand returned bad status.
    Status { command: &'static str },
    /// Version bump error.
    #[cfg(feature = "version-bump")]
    Bump(semver::Error),
}

impl Error {
    fn status(command: &'static str) -> Self {
        Error::Status { command }
    }

    /// Returns true if this error was definitely the user's mistake.
    fn is_user(&self) -> bool {
        matches!(self, Error::Flag(_))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Flag(flag) => write!(f, "{flag} is not recognized"),
            Error::Io(err) => write!(f, "{err}; are npm and openapi-generator-cli in your PATH?"),
            Error::Lang(s) => write!(f, "{s} is not a supported language"),
            Error::Path => write!(f, "{INPUT_DIR}: directory not found"),
            Error::Status { command } => write!(f, "{command} returned bad status"),
            #[cfg(feature = "version-bump")]
            Error::Bump(err) => write!(f, "{err}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<tools::ParseLangError> for Error {
    fn from(value: tools::ParseLangError) -> Self {
        Error::Lang(value.0)
    }
}

#[cfg(feature = "version-bump")]
impl From<semver::Error> for Error {
    fn from(value: semver::Error) -> Self {
        Error::Bump(value)
    }
}

type Result<T> = std::result::Result<T, Error>;

/// Runs a command and returns an error if it fails.
fn run_command(cmd: &mut Command, name: &'static str) -> Result<()> {
    cmd.status()?
        .success()
        .then_some(())
        .ok_or(Error::status(name))
}

/// Verifies that required tools are installed and sets the generator version.
fn setup_generator() -> Result<()> {
    run_command(Command::new("npm").arg("--version"), "npm")?;
    run_command(
        Command::new("openapi-generator-cli").arg("version"),
        "openapi-generator-cli",
    )?;
    run_command(
        Command::new("openapi-generator-cli").args(["version-manager", "set", "7.13.0"]),
        "openapi-generator-cli",
    )
}

/// Invokes the OpenAPI code generator command for the named specification.
fn generate(lang: Lang) -> Result<()> {
    setup_generator()?;

    run_command(
        Command::new("openapi-generator-cli")
            .arg("generate")
            .args(["--input-spec", &format!("{INPUT_DIR}/bluefin-api.yaml")])
            .args(["--config", lang.config()])
            .args(["--generator-name", lang.generator()])
            .args(["--output", lang.output()]),
        "openapi-generator-cli",
    )
}

/// Returns the nearest ancestor of the current working directory containing a "resources" folder.
fn input_parent() -> Result<PathBuf> {
    let dir = env::current_dir()?;
    let mut dir = dir.as_path();
    while !dir.join(INPUT_DIR).is_dir() {
        dir = dir.parent().ok_or(Error::Path)?;
    }
    Ok(dir.into())
}

/// Parsed command-line arguments.
struct Args {
    langs: Vec<Lang>,
    #[cfg(feature = "version-bump")]
    no_bump: bool,
}

/// Prints help and warns about any remaining arguments.
fn print_help(remaining_args: impl Iterator<Item = String>) {
    println!("usage: {USAGE}");
    let remaining: Vec<_> = remaining_args.collect();
    if !remaining.is_empty() {
        eprintln!("warning: ignoring args: {remaining:?}");
    }
}

/// Parses command-line arguments and returns None if help was requested.
fn parse_args() -> Result<Option<Args>> {
    let mut langs = Vec::new();
    #[cfg(feature = "version-bump")]
    let mut no_bump = false;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help(args);
                return Ok(None);
            }
            "-l" | "--lang" => langs.push(args.next().ok_or(Error::Flag(arg))?.parse()?),
            #[cfg(feature = "version-bump")]
            "-B" | "--no-bump" => no_bump = true,
            #[cfg(not(feature = "version-bump"))]
            "-B" | "--no-bump" => {
                eprintln!("warning: --no-bump ignored (version-bump feature disabled)")
            }
            _ => return Err(Error::Flag(arg)),
        }
    }

    if langs.is_empty() {
        langs.extend([Lang::Python, Lang::Rust, Lang::TypeScript]);
    }

    Ok(Some(Args {
        langs,
        #[cfg(feature = "version-bump")]
        no_bump,
    }))
}

fn main_imp() -> Result<()> {
    // Change to repo root so we can use relative paths in the generate function.
    env::set_current_dir(input_parent()?)?;

    let Some(args) = parse_args()? else {
        return Ok(());
    };

    #[cfg(feature = "version-bump")]
    semver::run(args.no_bump, &args.langs)?;

    for lang in args.langs {
        generate(lang)?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = main_imp() {
        eprintln!("error: {err}");
        exit(if err.is_user() { 2 } else { 1 });
    }
}
