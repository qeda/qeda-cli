use std::fs;
use std::path::Path;

use clap::{App, AppSettings, ArgGroup, ArgMatches};
use serde_json::{Number, Value};

use crate::completion;
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

const GROUND_DETAILS: &str = r"DETAILS:
    Signal Ground:
        __|__
        \   /
         \ /

    Chassis Ground:
        __|__
        \ \ \

    Earth Ground:
        __|__
         ___
          _";

const MAX_INDEX_COUNT: usize = 512;
const MAX_LIST_COUNT: usize = 4096;

pub async fn run() -> Result<()> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("add", m)) => add_component(m).await?,
        Some(("load", m)) => load_component(m).await?,
        Some(("test", m)) => test_component(m)?,
        Some(("power", m)) => add_power(m)?,
        Some(("ground", m)) => add_ground(m)?,
        Some(("config", m)) => configure(m)?,
        Some(("generate", m)) => generate(m).await?,
        Some(("reset", _)) => reset()?,
        Some(("index", m)) => index(m)?,
        Some(("update", m)) => update(m).await?,
        Some(("list", m)) => list(m)?,
        Some(("completion", m)) => get_completion(m)?,
        _ => unreachable!(),
    }
    Ok(())
}

fn cli() -> App<'static> {
    App::new("qeda")
        .version(crate_version!())
        .about("Tool for creating electronic component libraries")
        .after_help(QEDA_EXAMPLES)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColorAuto)
        .subcommand(
            App::new("add")
                .about("Add a component definition to the config (with preloading if necessary)")
                .arg("<COMPONENT> 'Component name'")
        )
        .subcommand(
            App::new("load")
                .about("Load a component definition from the global repository")
                .arg("<COMPONENT> 'Component name'"),
        )
        .subcommand(
            App::new("test")
                .about("Generate a test library with the only one component")
                .arg("<COMPONENT> 'Component name'"),
        )
        .subcommand(
            App::new("power")
                .about("Add power supply symbol to the config")
                .arg("<NET> 'Power net name'"),
        )
        .subcommand(
            App::new("ground")
                .about("Add ground symbol to the config")
                .setting(AppSettings::DeriveDisplayOrder)
                .arg("<NET> 'Ground net name'")
                .arg("-s, --signal  'Signal ground (triangle symbol)'")
                .arg("-c, --chassis 'Chassis ground (fork symbol)'")
                .arg("-e, --earth   'Earth ground (3-lines symbol)'")
                .group(ArgGroup::new("ground-type").args(&["signal", "chassis", "earth"]))
                .after_help(GROUND_DETAILS),
        )
        .subcommand(
            App::new("config")
                .about("Set/get/list config parameter(s)")
                .arg("-l, --list 'List available config parameters'")
                .arg("[PARAM] 'Parameter name'")
                .group(ArgGroup::new("config-subcommand").args(&["PARAM", "list"]).required(true))
                .arg("[VALUE] 'New value to be set'"),
        )
        .subcommand(
            App::new("generate")
                .about("Generate a library according to the config")
                .arg("<LIBRARY> 'Library name'"),
        )
        .subcommand(App::new("reset").about("Delete current config (use with attention!)"))
        .subcommand(
            App::new("index")
                .about("Generate index for component descriptions")
                .arg("[DIR] 'Directory with component descriptions to be indexed (\'qedalib\' by default)'")
                .arg("-m, --max=[COUNT] 'Maximum component count in one index file (512 by default)'"), // TODO: Use MAX_INDEX_COUNT
        )
        .subcommand(
            App::new("update")
                .about("Update library index from the global repository")
                .arg("-f, --force 'Force to update the index even if it has up-to-date parts'"),
        )
        .subcommand(
            App::new("list")
                .about("List available components from index (run 'qeda update' first)")
                .arg("[PREFIX] 'Component name prefix'")
                .arg("-m, --max=[COUNT] 'Maximum listed component count (4096 by default)'"), // TODO: Use MAX_LIST_COUNT
        )
        .subcommand(
            App::new("completion")
                .about("Generate or install completion scripts for your shell")
                .arg("-i, --install 'Install completion scripts to standard paths'")
                .arg("-b, --bash 'Show completion script for Bash'")
                .arg("-w, --words=[WORDS] 'Command words'")
                .arg("-c, --current=[WORD] 'Current word number'")
                .arg("-m, --max=[COUNT] 'Maximum listed component count (4096 by default)'") // TODO: Use MAX_LIST_COUNT
                .group(ArgGroup::new("completion-flag").args(&["install", "bash", "words"]).required(true)),
        )
}

async fn add_component(m: &ArgMatches) -> Result<()> {
    let mut lib = Library::new();
    let component_id = m.value_of("COMPONENT").unwrap();
    lib.add_component(component_id).await?;

    Config::create_if_missing(QEDA_YML)?;
    let mut config = Config::from_yaml_file(QEDA_YML)?;
    config.insert_object("components", component_id)?;
    config.save(QEDA_YML)
}

async fn load_component(m: &ArgMatches) -> Result<()> {
    let lib = Library::new();
    lib.load_component(m.value_of("COMPONENT").unwrap()).await?;
    Ok(())
}

fn test_component(m: &ArgMatches) -> Result<()> {
    println!("test_component -> {}", m.value_of("COMPONENT").unwrap());
    Ok(())
}

fn add_power(m: &ArgMatches) -> Result<()> {
    println!("add_power -> {}", m.value_of("NET").unwrap());
    Ok(())
}

fn add_ground(m: &ArgMatches) -> Result<()> {
    println!("add_ground -> {}", m.value_of("NET").unwrap());
    Ok(())
}

fn configure(m: &ArgMatches) -> Result<()> {
    let mut config = if !Path::new(QEDA_YML).exists() {
        Config::new()
    } else {
        Config::from_yaml_file(QEDA_YML)?
    };

    let lib_cfg = load_config!("qeda.yml").merge(&config);
    if m.is_present("list") {
        let params = lib_cfg.keys();
        let _: Vec<_> = params.into_iter().map(|s| println!("{}", s)).collect();
    } else {
        let param = m.value_of("PARAM").unwrap();
        let value = m.value_of("VALUE");
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
    }

    Ok(())
}

async fn generate(m: &ArgMatches) -> Result<()> {
    ensure!(
        Path::new(QEDA_YML).exists(),
        QedaError::MissingConfigFile(QEDA_YML.to_string())
    );

    let config = Config::from_yaml_file(QEDA_YML)?;
    let lib = Library::from_config(&config).await?;
    lib.generate(m.value_of("LIBRARY").unwrap())
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
    let max_count = m
        .value_of("max")
        .unwrap_or(&MAX_INDEX_COUNT.to_string())
        .parse::<usize>()?;
    let dir = m.value_of("DIR").unwrap_or("qedalib");
    info!("indexing '{}' (max {} records per index)", dir, max_count);
    index::generate(dir, max_count)
}

async fn update(m: &ArgMatches) -> Result<()> {
    info!("updating index");
    let config = if !Path::new(QEDA_YML).exists() {
        Config::new()
    } else {
        Config::from_yaml_file(QEDA_YML)?
    };
    let lib_cfg = load_config!("qeda.yml").merge(&config);
    index::update(m.is_present("force"), &lib_cfg).await
}

fn list(m: &ArgMatches) -> Result<()> {
    let max_count = m
        .value_of("max")
        .unwrap_or(&MAX_LIST_COUNT.to_string())
        .parse::<usize>()?;
    let components = index::list(m.value_of("PREFIX").unwrap_or(""), max_count);
    let _: Vec<_> = components.into_iter().map(|s| println!("{}", s)).collect();
    Ok(())
}

fn get_completion(m: &ArgMatches) -> Result<()> {
    if m.is_present("bash") {
        print!("{}", completion::bash_script());
    } else if m.is_present("install") {
        completion::install()?;
    } else if let Some(words) = m.value_of("words") {
        let current = m.value_of("current").unwrap_or("0").parse::<usize>()?;
        let max_count = m
            .value_of("max")
            .unwrap_or(&MAX_LIST_COUNT.to_string())
            .parse::<usize>()?;
        let words = completion::words(words, current, max_count);
        let _: Vec<_> = words.into_iter().map(|s| println!("{}", s)).collect();
    }
    Ok(())
}
