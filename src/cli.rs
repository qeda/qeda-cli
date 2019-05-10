use clap::{App, AppSettings, Arg, ArgMatches, Shell, SubCommand};
use std::{fs, path::Path};

use crate::errors::*;
use crate::library::Library;

const QEDA_EXAMPLES: &'static str = r"EXAMPLES:
    qeda reset
    qeda add ti:iso721
    qeda power +5V_DC
    qeda ground GND_DC
    qeda config output kicad
    qeda generate mylib";

const QEDA_YML: &'static str = ".qeda.yml";

pub fn run() -> Result<()> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        ("add", Some(m)) => add_component(m)?,
        ("load", Some(m)) => load_component(m)?,
        ("test", Some(m)) => test_component(m)?,
        ("power", Some(m)) => add_power(m)?,
        ("ground", Some(m)) => add_ground(m)?,
        ("config", Some(m)) => configure(m)?,
        ("generate", Some(m)) => generate(m)?,
        ("reset", Some(_)) => reset()?,
        ("sort", Some(_)) => sort()?,
        ("completion", Some(m)) => get_completion(m)?,
        (_, _) => unreachable!(),
    }
    Ok(())
}

fn cli() -> App<'static, 'static> {
    App::new("qeda")
        .version(crate_version!())
        .about("A tool for creating libraries of electronic components")
        .after_help(QEDA_EXAMPLES)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColorAuto)
        .arg(
            Arg::with_name("verbose")
                .help("Enable verbose output")
                .short("v")
                .long("verbose")
                .multiple(true),
            )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add component definition to config (with preloading if necessary)")
                .arg(Arg::with_name("component").required(true).help("Component name"))
        )
        .subcommand(
            SubCommand::with_name("load")
                .about("Load component definition from global repository")
                .arg(Arg::with_name("component").required(true).help("Component name"))
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("Generate test library with only one component")
                .arg(Arg::with_name("component").required(true).help("Component name"))
        )
        .subcommand(
            SubCommand::with_name("power")
                .about("Add power supply symbol to config")
                .arg(Arg::with_name("net").required(true).help("Power net name"))
        )
        .subcommand(
            SubCommand::with_name("ground")
                .about("Add ground symbol to config")
                .setting(AppSettings::DeriveDisplayOrder)
                .arg(Arg::with_name("net").required(true).help("Ground net name"))
                .arg(Arg::with_name("signal").help("Signal ground (triangle symbol)").short("s").long("signal"))
                .arg(Arg::with_name("chassis").help("Chassis ground (fork symbol)").short("c").long("chassis"))
                .arg(Arg::with_name("earth").help("Earth ground (3-lines symbol)").short("e").long("earth"))
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Set/get config parameter")
                .arg(Arg::with_name("param").required(true).help("Parameter name").takes_value(true))
                .arg(Arg::with_name("value").help("New value to be set"))
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate library according to config")
                .arg(Arg::with_name("library").required(true).help("Library name"))
        )
        .subcommand(
            SubCommand::with_name("reset")
                .about("Delete current config (use with attention!)")
        )
        .subcommand(
            SubCommand::with_name("sort")
                .about("Sort components and nets in config alphabetically (use with caution due to possible annotation issues)")
        )
        .subcommand(
            SubCommand::with_name("completion")
                .about("Generate completion scripts for your shell")
                .arg(Arg::with_name("shell").required(true).help("Shell name").possible_values(&Shell::variants()))
        )
}

fn add_component(m: &ArgMatches) -> Result<()> {
    println!("add_component -> {}", m.value_of("component").expect(""));
    debug!("add_component");
    let lib = Library::new();
    lib.add_component(m.value_of("component").expect(""))?;
    Ok(())
}

fn load_component(m: &ArgMatches) -> Result<()> {
    let lib = Library::new();
    lib.load_component(m.value_of("component").expect(""))?;
    Ok(())
}

fn test_component(m: &ArgMatches) -> Result<()> {
    println!("test_component -> {}", m.value_of("component").expect(""));
    Ok(())
}

fn add_power(m: &ArgMatches) -> Result<()> {
    println!("add_power -> {}", m.value_of("net").expect(""));
    Ok(())
}

fn add_ground(m: &ArgMatches) -> Result<()> {
    println!("add_ground -> {}", m.value_of("net").expect(""));
    Ok(())
}

fn configure(m: &ArgMatches) -> Result<()> {
    println!("configure -> {}", m.value_of("param").expect(""));
    Ok(())
}

fn generate(m: &ArgMatches) -> Result<()> {
    println!("generate -> {}", m.value_of("library").expect(""));
    Ok(())
}

fn reset() -> Result<()> {
    info!("removing \"{}\"", QEDA_YML);
    if !Path::new(QEDA_YML).exists() {
        warn!("nothing to remove");
    } else {
        fs::remove_file(QEDA_YML)
            .chain_err(|| "unable to remove")?;
    }
    Ok(())
}

fn sort() -> Result<()> {
    println!("sort");
    Ok(())
}

fn get_completion(m: &ArgMatches) -> Result<()> {
    println!("get_completion -> {}", m.value_of("shell").expect(""));
    Ok(())
}