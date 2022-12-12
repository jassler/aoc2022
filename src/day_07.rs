use std::collections::{BTreeMap, HashMap};
use std::fmt::Error;
use std::str::Lines;
use std::iter::Peekable;
use regex::Regex;

#[derive(Debug)]
struct Folder {
    files: BTreeMap<String, usize>,
    folders: BTreeMap<String, Folder>,
    size: usize,
}

#[inline]
fn parse_cd(it: &mut Peekable<Lines>, line: &str, folder: &mut Folder) {
    // "$ cd abc"
    // abc starts on index 5
    folder.folders.insert(line[5..].to_string(), parse_folder(it));
}

fn parse_ls(it: &mut Peekable<Lines>, folder: &mut Folder) {
    while let Some(line) = it.peek() {
        if line.starts_with("$") {
            break;
        }

        if !line.starts_with("dir") {
            let mut parts = line.split(' ');
            let size = parts.next().unwrap();
            let name = parts.next().unwrap();
            folder.files.insert(name.to_string(), size.parse().unwrap());
        }
        it.next();
    }
}

fn parse_folder(it: &mut Peekable<Lines>) -> Folder {
    let mut f = Folder {
        files: BTreeMap::new(),
        folders: BTreeMap::new(),
        size: 0,
    };

    while let Some(line) = it.next() {
        match line {
            "$ cd .." => break,
            "$ ls" => parse_ls(it, &mut f),
            _ => parse_cd(it, line, &mut f),
        }
    }

    f.size = f.files.values().sum::<usize>()
            + f.folders.values().map(|f| f.size).sum::<usize>();

    f
}

fn sum_small_folders(f: &Folder) -> usize {
    f
        .folders.values()
        .map(|f| sum_small_folders(f))
        .sum::<usize>()
        + (if f.size <= 100_000 { f.size } else { 0 })
}

pub fn part_01(input: &str) -> String {
    let mut it = input.lines().peekable();
    // skip cd /
    it.next();
    let root = parse_folder(&mut it);

    sum_small_folders(&root).to_string()
}

fn find_smallest_folder(f: &Folder, space_needed: usize) -> Option<&Folder> {
    if f.size < space_needed {
        None
    } else {
        Some(f
            .folders.values()
            .filter_map(|f| find_smallest_folder(f, space_needed))
            .min_by_key(|f| f.size)
            .unwrap_or(f)
        )
    }
}

pub fn part_02(input: &str) -> String {
    let mut it = input.lines().peekable();
    // skip cd /
    it.next();
    let root = parse_folder(&mut it);

    // total space available: 70_000_000
    // needed space: 30_000_000
    // root: 48_008_081
    let space_needed = 30_000_000 - (70_000_000 - root.size);

    find_smallest_folder(&root, space_needed).unwrap().size.to_string()
}
