use std::{env, fmt, fs};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crypto_hash::{Algorithm, Hasher};

use crate::error::Result;

const INDEX_DIR: &str = ".index";
const EXT: &str = ".yml";
const MAX_ITEM_COUNT: usize = 200;

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

                let mut f = File::create("hash")?;
                writeln!(f, "{}", i.hash)?;

                let mut lines: Vec<_> = i
                    .entries
                    .iter()
                    .filter(|e| !e.is_empty_group())
                    .map(|e| format!("{}", e))
                    .collect();
                lines.sort();
                let mut f = File::create("index")?;
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
