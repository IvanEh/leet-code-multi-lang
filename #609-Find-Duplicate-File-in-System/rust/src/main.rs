use std::collections::HashMap;
use std::ops::Add;
use std::cmp;

const FILE_DELIM: &str = "/";
const INFO_DELIM: &str = " ";
const LEFT_PAREN: &str = "(";
const RIGHT_PAREN: &str = ")";
const KEY_LEN_THRESHOLD: usize = 16;

pub fn find_duplicate(arr: &[&str]) -> Vec<Vec<String>> {
    let fileEntries = extract_file_entries(arr);
    let mut files_map = HashMap::new();
    let mut duplicates = Vec::new();

    if let Some(fileEntries) = fileEntries {
        let file_entries = extract_file_entries(arr).unwrap_or(Vec::new());
        store_entries(&mut files_map, file_entries);

        for files in files_map.into_iter() {
            find_duplicates_for(files.1);
        }
    }

    duplicates
}

fn find_duplicates_for(mut files_bucket: Vec<FileEntry>) -> Vec<Vec<FileEntry>>{
    let mut duplicates = Vec::new();
    let mut i = 0;

    while i < files_bucket.len() {
        let mut group = Vec::new();
        let pivot = files_bucket[i];
        group.push(pivot);

        for j in i + 1..files_bucket.len() {
            let file_entry = files_bucket[j];
            if file_entry.content == pivot.content {
                group.push(file_entry);
                files_bucket.remove(j);
                j -= 1;
            }
        }

        if group.len() > 1 {
            duplicates.push(group);
        }

        i+=1;
    }

    duplicates
}

fn store_entries(map: &mut HashMap<String, FileEntry>, file_entries: Vec<FileEntry>) {
    for file_entry in file_entries {
        map.insert(get_key(&file_entry.content), file_entry);
    }
}

fn get_key(content: &String) -> String {
    let size = cmp::min(content.len(), KEY_LEN_THRESHOLD);
    content[..size].to_string()
}

fn extract_file_entries(arr: &[&str]) -> Option<Vec<FileEntry>> {
    let mut file_entries_iter = arr.iter().map(|s| parse_single(s));

    if file_entries_iter.all(|file_entry| file_entry.is_some()) {
        Some(file_entries_iter
            .map(|vec| vec.unwrap())
            .flat_map(|vec| vec.into_iter())
            .collect())
    } else {
        None
    }
}

fn parse_single(s: &str) -> Option<Vec<FileEntry>> {
    partition_info(s).and_then(|(dir, files)| {
        let mut entries = files
            .split(INFO_DELIM)
            .map(|file| FileEntry::parse(file, dir));

        if entries.any(|entry| entry.is_none()) {
            None
        } else {
            Some(entries.map(|entry| entry.unwrap()).collect::<Vec<_>>())
        }
    })
}

fn partition_info(s: &str) -> Option<(&str, &str)> {
    let dir_end_pos = s.find(INFO_DELIM);
    dir_end_pos.map(|dir_end_pos| {
        (&s[..dir_end_pos], &s[dir_end_pos + 1..])
    })
}

struct FileEntry {
    path: String,
    content: String
}

impl FileEntry {
    pub fn parse(s: &str, dir: &str) -> Option<FileEntry> {
        let leftParen = s.find(LEFT_PAREN);
        let rightParen = s.find(RIGHT_PAREN);

        match (leftParen, rightParen) {
            (Some(leftParen), Some(rightParen)) => Some(FileEntry::parse_given_indices(s, dir, leftParen, rightParen)),
            _ => None
        }
    }

    /// Assumes that leftParen and rightParen are valid byte indices
    fn parse_given_indices(s: &str, dir: &str, leftParen: usize, rightParen: usize) -> FileEntry {
        FileEntry {
            path: dir.to_string() + FILE_DELIM,
            content: s[leftParen..rightParen].to_string()
        }
    }
}

fn main() {
    find_duplicate(&["1", "2", "3"]);

}
