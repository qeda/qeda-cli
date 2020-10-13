use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use std::{env, fmt, fs};

use async_recursion::async_recursion;
use crypto_hash::{Algorithm, Hasher};
use directories::ProjectDirs;
use futures::future;
use tokio::prelude::*;
use tokio::task;

use crate::config::Config;
use crate::error::*;
use crate::log::{info_async, warn_async};

const INDEX_DIR: &str = ".index";
const INDEX_FILE: &str = "index";
const HASH_FILE: &str = "hash";
const EXT: &str = ".yml";
const MAX_ITEM_COUNT: usize = 1000;

#[derive(Debug, Default)]
struct Index {
    name: String,
    entries: Vec<Entry>,
    hash: String,
}

#[derive(Debug)]
enum Entry {
    Item(String),
    Group(Index),
}

impl Entry {
    pub fn is_group(&self) -> bool {
        matches!(self, Entry::Group(_))
    }

    pub fn is_empty_group(&self) -> bool {
        match self {
            Entry::Group(i) => i.entries.is_empty(),
            _ => false,
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Entry::Item(s) => write!(f, "{}", s),
            Entry::Group(i) => write!(f, "{}/", i.name),
        }
    }
}

impl Index {
    /// Creates an empty `Index`.
    pub fn new(name: String, entries: Vec<Entry>) -> Self {
        let hash = Self::calc_hash(&entries);
        Index {
            name,
            entries,
            hash,
        }
    }

    /// Returns an `Index` with items packed into groups.
    pub fn packed(name: String, items: Vec<String>) -> Self {
        let items = Self::pack(items, 0);
        Self::new(name, items)
    }

    /// Renders an `Entry` to a file structure.
    pub fn render(entry: &Entry) -> Result<()> {
        if let Entry::Group(i) = entry {
            if !i.entries.is_empty() {
                fs::create_dir(&i.name)?;
                let prev_dir = env::current_dir()?;
                env::set_current_dir(&i.name)?;

                let mut f = File::create(HASH_FILE)?;
                writeln!(f, "{}", i.hash)?;

                let mut lines: Vec<_> = i
                    .entries
                    .iter()
                    .filter(|e| !e.is_empty_group())
                    .map(|e| format!("{}", e))
                    .collect();
                lines.sort();
                let mut f = File::create(INDEX_FILE)?;
                let _ = lines
                    .iter()
                    .map(|s| writeln!(f, "{}", s))
                    .collect::<Vec<_>>();

                let groups: Vec<_> = i.entries.iter().filter(|e| e.is_group()).collect();
                for group in groups {
                    Self::render(group)?;
                }
                env::set_current_dir(prev_dir)?;
            }
        }
        Ok(())
    }

    fn pack(items: Vec<String>, level: usize) -> Vec<Entry> {
        let mut len = items.len();
        let mut result = Vec::new();

        let mut groups: HashMap<char, Vec<String>> = HashMap::new();
        for item in items {
            if item.len() > level + 1 {
                let key = item.chars().nth(level).unwrap();
                groups.entry(key).or_insert_with(Vec::new).push(item);
            } else {
                result.push(Entry::Item(item));
            }
        }

        for (key, group) in groups {
            let group_len = group.len();
            if len <= MAX_ITEM_COUNT || group_len == 1 {
                result.append(&mut group.into_iter().map(Entry::Item).collect::<Vec<_>>());
            } else {
                result.push(Entry::Group(Index::new(
                    key.to_string(),
                    Self::pack(group, level + 1),
                )));
            }
            len -= group_len - 1;
        }
        result
    }

    fn calc_hash(entries: &[Entry]) -> String {
        let mut hasher = Hasher::new(Algorithm::SHA256);
        for entry in entries {
            match entry {
                Entry::Item(s) => hasher.write_all(s.as_bytes()).unwrap(),
                Entry::Group(i) => hasher
                    .write_all(Self::calc_hash(&i.entries).as_bytes())
                    .unwrap(),
            }
        }
        hex::encode(hasher.finish())
    }
}

pub fn generate(dir: &str) -> Result<()> {
    env::set_current_dir(dir)?;
    let subdirs: Vec<_> = fs::read_dir("./")?
        .filter_map(|res| res.map(|ent| ent.path()).ok())
        .filter(|ent| ent.is_dir() && ent.file_name().is_some())
        .filter_map(|dir| dir.file_name().unwrap().to_str().map(|s| s.to_string()))
        .filter(|dir| !dir.starts_with('.'))
        .collect();

    let mut entries: Vec<Entry> = Vec::new();

    for subdir in subdirs {
        info!("  • directory: '{}'", &subdir);
        let files: Vec<_> = fs::read_dir(&subdir)?
            .filter_map(|res| res.map(|ent| ent.path()).ok())
            .filter(|ent| ent.is_file() && ent.file_name().is_some())
            .filter_map(|file| file.file_name().unwrap().to_str().map(|s| s.to_string()))
            .filter(|file| file.ends_with(EXT))
            .map(|mut file| {
                file.truncate(file.len() - EXT.len());
                file
            })
            .collect();
        entries.push(Entry::Group(Index::packed(subdir, files)));
    }

    let index = Entry::Group(Index::new(INDEX_DIR.to_string(), entries));
    if Path::new(INDEX_DIR).exists() {
        fs::remove_dir_all(INDEX_DIR)?;
    }
    Index::render(&index)
}

pub async fn update(force: bool, lib_cfg: &Config) -> Result<()> {
    let proj_dirs =
        ProjectDirs::from("org", "qeda", "qeda-cli").ok_or(QedaError::UnableToGetProjectDir)?;
    let cache_dir = proj_dirs.cache_dir();

    info!("cache directory: '{}'", cache_dir.display());

    if force && Path::new(cache_dir).exists() {
        tokio::fs::remove_dir_all(cache_dir).await?;
    }
    if !Path::new(cache_dir).exists() {
        tokio::fs::create_dir(cache_dir).await?;
    }
    env::set_current_dir(cache_dir)?;
    let mut url = lib_cfg.get_string("base-url")?;
    if !url.ends_with('/') {
        url += "/";
    }
    let timeout = lib_cfg.get_u64("timeout").unwrap();
    download(format!("{}/", INDEX_DIR), url, timeout).await
}

pub fn list(pat: &str) -> Vec<String> {
    let mut result = Vec::new();
    if let Some(proj_dirs) = ProjectDirs::from("org", "qeda", "qeda-cli") {
        let index_dir = format!("{}/{}", proj_dirs.cache_dir().display(), INDEX_DIR);
        if Path::new(&index_dir).exists() {
            let mut index = fs::read_to_string(format!("{}/{}", &index_dir, INDEX_FILE))
                .unwrap_or_else(|_| "".to_string());
            while index.ends_with('\n') {
                index.truncate(index.len() - 1);
            }
            let prefixes: Vec<_> = index.split('\n').collect();
            let mut count = MAX_ITEM_COUNT;
            for prefix in prefixes {
                if intersects(prefix, pat) {
                    let mut components =
                        get_from_index(&format!("{}/{}", &index_dir, prefix), prefix, pat, count);
                    count -= components.len();
                    result.append(&mut components);
                    if count == 0 {
                        break;
                    }
                }
            }
        }
    }
    result
}

#[async_recursion]
async fn download(dir: String, url: String, timeout: u64) -> Result<()> {
    if !Path::new(&dir).exists() {
        tokio::fs::create_dir_all(&dir).await?;
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout))
        .build()?;

    let response = client
        .get(&format!("{}{}{}", url, &dir, HASH_FILE))
        .send()
        .await?
        .error_for_status()?;
    let hash = response.text().await?;

    let hash_file = format!("{}{}", dir, HASH_FILE);
    if Path::new(&hash_file).exists() {
        let local_hash = fs::read_to_string(&hash_file)?;
        if local_hash == hash {
            info_async(&format!("  • skip: '{}'", &dir)).await;
            return Ok(());
        }
    }

    info_async(&format!("  • download: '{}'", &dir)).await;
    let response = client
        .get(&format!("{}{}{}", url, &dir, INDEX_FILE))
        .send()
        .await?
        .error_for_status()?;
    let index = response.text().await?;

    let mut f = tokio::fs::File::create(format!("{}{}", dir, INDEX_FILE)).await?;
    f.write_all(index.as_bytes()).await?;

    let tasks: Vec<_> = index
        .split('\n')
        .filter(|s| s.ends_with('/'))
        .map(|subdir| download(format!("{}{}", &dir, subdir), url.clone(), timeout))
        .map(task::spawn)
        .collect();

    let is_err = future::join_all(tasks)
        .await
        .into_iter()
        .any(|r| r.is_err() || r.unwrap().is_err());
    if !is_err {
        let mut f = tokio::fs::File::create(format!("{}{}", dir, HASH_FILE)).await?;
        f.write_all(hash.as_bytes()).await?;
    } else {
        warn_async(&format!(
            "some of index elements hasn't been downloaded: '{}'",
            &dir
        ))
        .await;
    }

    Ok(())
}

fn intersects(s1: &str, s2: &str) -> bool {
    s1.starts_with(s2) || s2.starts_with(s1)
}

fn get_from_index(path: &str, prefix: &str, pat: &str, max_count: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut index =
        fs::read_to_string(format!("{}{}", path, INDEX_FILE)).unwrap_or_else(|_| "".to_string());
    while index.ends_with('\n') {
        index.truncate(index.len() - 1);
    }
    let mut count = max_count;
    let components: Vec<_> = index.split('\n').collect();
    for component in components {
        if component.ends_with('/') {
            // Group
            let mut prefix = format!("{}{}", prefix, &component[0..component.len()]);
            prefix.truncate(prefix.len() - 1);
            if intersects(&prefix, pat) && count > 0 {
                let mut childs =
                    get_from_index(&format!("{}{}", path, component), &prefix, pat, count);
                count -= childs.len();
                result.append(&mut childs);
            }
        } else {
            // Component
            let name = format!("{}{}", prefix, component);
            if intersects(&name, pat) && count > 0 {
                result.push(name);
                count -= 1;
            }
        }
        if count == 0 {
            break;
        }
    }

    result
}
