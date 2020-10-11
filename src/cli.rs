use std::fs;
use std::path::Path;

use clap::{App, AppSettings, Arg, ArgMatches, Shell, SubCommand};
use serde_json::{Number, Value};

use crate::config::Config;
use crate::error::*;
use crate::index;
use crate::library::Library;

const QEDA_YML: &str = ".qeda.yml";

const QEDA_EXAMPLES: &str = r"EXAMPLES:
    qeda reset
    qeda add capacitor/c0603
    qeda power +5V
    qeda ground GND
    qeda config generator.type kicad
    qeda generate mylib";

pub async fn run() -> Result<()> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        ("add", Some(m)) => add_component(m).await?,
        ("load", Some(m)) => load_component(m).await?,
        ("test", Some(m)) => test_component(m)?,
        ("power", Some(m)) => add_power(m)?,
        ("ground", Some(m)) => add_ground(m)?,
        ("config", Some(m)) => configure(m)?,
        ("generate", Some(m)) => generate(m).await?,
        ("reset", Some(_)) => reset()?,
        ("index", Some(m)) => index(m)?,
        ("update", Some(m)) => update(m).await?,
        ("completion", Some(m)) => get_completion(m)?,
        (_, _) => unreachable!(),
    }
    Ok(())
}

fn cli() -> App<'static, 'static> {
    App::new("qeda")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Tool for creating electronic component libraries")
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
                .about("Add a component definition to the config (with preloading if necessary)")
                .arg(
                    Arg::with_name("component")
                        .required(true)
                        .help("Component name"),
                ),
        )
        .subcommand(
            SubCommand::with_name("load")
                .about("Load a component definition from the global repository")
                .arg(
                    Arg::with_name("component")
                        .required(true)
                        .help("Component name"),
                ),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("Generate a test library with the only one component")
                .arg(
                    Arg::with_name("component")
                        .required(true)
                        .help("Component name"),
                ),
        )
        .subcommand(
            SubCommand::with_name("power")
                .about("Add power supply symbol to the config")
                .arg(Arg::with_name("net").required(true).help("Power net name")),
        )
        .subcommand(
            SubCommand::with_name("ground")
                .about("Add ground symbol to the config")
                .setting(AppSettings::DeriveDisplayOrder)
                .arg(Arg::with_name("net").required(true).help("Ground net name"))
                .arg(
                    Arg::with_name("signal")
                        .help("Signal ground (triangle symbol)")
                        .short("s")
                        .long("signal"),
                )
                .arg(
                    Arg::with_name("chassis")
                        .help("Chassis ground (fork symbol)")
                        .short("c")
                        .long("chassis"),
                )
                .arg(
                    Arg::with_name("earth")
                        .help("Earth ground (3-lines symbol)")
                        .short("e")
                        .long("earth"),
                ),
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Set/get config parameter")
                .arg(
                    Arg::with_name("param")
                        .required(true)
                        .help("Parameter name")
                        .takes_value(true),
                )
                .arg(Arg::with_name("value").help("New value to be set")),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate a library according to the config")
                .arg(
                    Arg::with_name("library")
                        .required(true)
                        .help("Library name"),
                ),
        )
        .subcommand(
            SubCommand::with_name("reset").about("Delete current config (use with attention!)"),
        )
        .subcommand(
            SubCommand::with_name("index")
                .about("Generate index for component descriptions")
                .arg(
                    Arg::with_name("directory")
                        .help("Directory with component descriptions to be indexed"),
                ),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Update library index from the global repository")
                .arg(
                    Arg::with_name("force")
                        .help("Force to update the index even if it has up-to-date parts")
                        .short("f")
                        .long("force"),
                ),
        )
        .subcommand(
            SubCommand::with_name("completion")
                .about("Generate completion scripts for your shell")
                .arg(
                    Arg::with_name("shell")
                        .required(true)
                        .help("Shell name")
                        .possible_values(&Shell::variants()),
                ),
        )
}

async fn add_component(m: &ArgMatches<'_>) -> Result<()> {
    let mut lib = Library::new();
    let component_id = m.value_of("component").unwrap();
    lib.add_component(component_id).await?;

    Config::create_if_missing(QEDA_YML)?;
    let mut config = Config::from_yaml_file(QEDA_YML)?;
    config.insert_object("components", component_id)?;
    config.save(QEDA_YML)
}

async fn load_component(m: &ArgMatches<'_>) -> Result<()> {
    let lib = Library::new();
    lib.load_component(m.value_of("component").unwrap()).await?;
    Ok(())
}

fn test_component(m: &ArgMatches) -> Result<()> {
    println!("test_component -> {}", m.value_of("component").unwrap());
    Ok(())
}

fn add_power(m: &ArgMatches) -> Result<()> {
    println!("add_power -> {}", m.value_of("net").unwrap());
    Ok(())
}

fn add_ground(m: &ArgMatches) -> Result<()> {
    println!("add_ground -> {}", m.value_of("net").unwrap());
    Ok(())
}

fn configure(m: &ArgMatches) -> Result<()> {
    let param = m.value_of("param").unwrap();
    let value = m.value_of("value");

    let mut config = if !Path::new(QEDA_YML).exists() {
        Config::new()
    } else {
        Config::from_yaml_file(QEDA_YML)?
    };

    let lib_cfg = load_config!("qeda.yml").merge(&config);
    let element = lib_cfg
        .get_element(param)
        .map_err(|_| QedaError::UnknownConfigParameter(param.to_string()))?;

    if let Some(value) = value {
        let new_value = match element {
            Value::String(_) => Value::String(value.to_string()),
            Value::Number(_) => Value::Number(Number::from_f64(value.parse::<f64>()?).unwrap()),
            Value::Bool(_) => Value::Bool(value.parse::<bool>()?),
            _ => bail!(QedaError::UnsupportedConfigParameterType(param.to_string())),
        };
        config.insert(param, new_value);
        config.save(QEDA_YML)?;
    } else {
        println!("{}", element);
    }

    Ok(())
}

async fn generate(m: &ArgMatches<'_>) -> Result<()> {
    ensure!(
        Path::new(QEDA_YML).exists(),
        QedaError::MissingConfigFile(QEDA_YML.to_string())
    );

    let config = Config::from_yaml_file(QEDA_YML)?;
    let lib = Library::from_config(&config).await?;
    lib.generate(m.value_of("library").unwrap())
}

fn reset() -> Result<()> {
    info!("removing '{}'", QEDA_YML);
    if !Path::new(QEDA_YML).exists() {
        warn!("nothing to remove");
    } else {
        fs::remove_file(QEDA_YML).with_context(|| "unable to remove")?;
    }
    Ok(())
}

fn index(m: &ArgMatches) -> Result<()> {
    info!(
        "indexing '{}'",
        m.value_of("directory").unwrap_or("qedalib")
    );
    index::generate(m.value_of("directory").unwrap_or("qedalib"))
}

async fn update(m: &ArgMatches<'_>) -> Result<()> {
    info!("updating index");
    let config = if !Path::new(QEDA_YML).exists() {
        Config::new()
    } else {
        Config::from_yaml_file(QEDA_YML)?
    };
    let lib_cfg = load_config!("qeda.yml").merge(&config);
    index::update(m.is_present("force"), &lib_cfg).await
}

fn get_completion(m: &ArgMatches) -> Result<()> {
    println!("get_completion -> {}", m.value_of("shell").unwrap());
    Ok(())
}
