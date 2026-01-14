use std::collections::HashMap;
use std::fs;
use std::path::Path;

const DISK_SPACE: usize = 70_000_000;
const FREE_NEEDED: usize = 30_000_000;
const PATH_SEP: &str = "/";

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        File {
            name: name.to_string(),
            size,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Directory {
    name: String,
    contents: Vec<Content>,
}

impl Directory {
    fn new(name: &str, contents: Vec<Content>) -> Self {
        Self {
            name: name.to_string(),
            contents,
        }
    }

    fn size(&self) -> usize {
        self.contents.iter().map(|c| c.get_size()).sum()
    }
}

#[derive(Debug, PartialEq)]
enum Content {
    File(File),
    Directory(Directory),
}

impl Content {
    fn get_size(&self) -> usize {
        match self {
            Content::File(f) => f.size,
            Content::Directory(dir) => dir.size(),
        }
    }
}

fn main() {
    let input = Path::new("2022/day07/src/input.txt");
    let root = parse_filesystem(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of directories at most 100 000: {:?}",
        size_of_dirs_below(&root, 100_000)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Smallest directory to delete to have at least 30 000 000 free space: {:?}",
        smallest_directory_to_free_up(&root, FREE_NEEDED)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_filesystem(input: &str) -> Directory {
    let mut lines = input.lines().chain(["$ ls"]).peekable();

    let mut dir_path_to_contents: HashMap<String, Vec<Content>> = HashMap::new();
    let mut dirs_in_path: Vec<&str> = Vec::new();
    while let Some(line) = lines.next() {
        if line.starts_with("$ cd") {
            match line {
                "$ cd .." => {
                    dirs_in_path.pop();
                }
                line => {
                    let dir = line.trim_start_matches("$ cd ");
                    let mut dir_name = dirs_in_path.to_vec().join(PATH_SEP);
                    dir_name.push('/');
                    if dir != "/" {
                        dir_name.push_str(dir);
                    }
                    dir_path_to_contents.entry(dir_name).or_insert(vec![]);
                    dirs_in_path.push(dir);
                }
            }
        } else if line.starts_with("$ ls") {
            let parents: Vec<_> = dirs_in_path.to_vec();
            while let Some(peeked) = lines.peek() {
                if peeked.starts_with("$") {
                    break;
                }

                let item = lines.next().unwrap();
                let content = if item.starts_with("dir") {
                    let dir = item.trim_start_matches("dir ");
                    let mut dir_name = parents.join(PATH_SEP);
                    dir_name.push('/');
                    dir_name.push_str(dir);
                    dir_path_to_contents
                        .entry(dir_name.clone())
                        .or_insert(vec![]);

                    Content::Directory(Directory::new(dir_name.as_str(), vec![]))
                } else {
                    let (size, name) = item.split_once(" ").unwrap();
                    Content::File(File::new(name, size.parse().unwrap()))
                };

                dir_path_to_contents
                    .entry(parents.join(PATH_SEP))
                    .and_modify(|parent| parent.push(content));
            }
        }
    }

    let (name, root_contents) = dir_path_to_contents.remove_entry("/").unwrap();
    let mut root_dir = Directory::new(name.as_str(), root_contents);
    for item in root_dir.contents.iter_mut() {
        if let &mut Content::Directory(ref mut dir) = item {
            let dir_contents = dir_path_to_contents.remove(dir.name.as_str()).unwrap();
            dir.contents = dir_contents;
            fill_dir(dir, &mut dir_path_to_contents)
        }
    }
    root_dir
}

fn fill_dir(dir: &mut Directory, dir_id_to_contents: &mut HashMap<String, Vec<Content>>) {
    for item in dir.contents.iter_mut() {
        if let Content::Directory(next_dir) = item {
            let next_dir_contents = dir_id_to_contents.remove(&next_dir.name).unwrap();
            next_dir.contents = next_dir_contents;
            fill_dir(next_dir, dir_id_to_contents);
        }
    }
}

fn size_of_dirs_below(dir: &Directory, max_size: usize) -> usize {
    let dir_size = dir.size();
    (dir.contents.iter().filter_map(|item| match item {
        Content::Directory(next_dir) => Some(size_of_dirs_below(next_dir, max_size)),
        _ => None,
    }))
    .sum::<usize>()
        + if dir_size > max_size { 0 } else { dir_size }
}

fn smallest_directory_to_free_up(dir: &Directory, free_needed: usize) -> usize {
    let total_taken = dir.size();
    let free_space = DISK_SPACE - total_taken;
    let to_delete = free_needed - free_space;

    let dirs = dirs_above(dir, to_delete);
    *dirs.iter().min().unwrap()
}

fn dirs_above(dir: &Directory, limit: usize) -> Vec<usize> {
    let mut dirs = Vec::new();
    let dir_size = dir.size();
    if dir_size > limit {
        dirs.push(dir_size);
    }
    dir.contents.iter().for_each(|item| {
        if let Content::Directory(next_dir) = item {
            let dir_sizes = dirs_above(next_dir, limit);
            if !dir_sizes.is_empty() {
                dirs.extend(dir_sizes);
            }
        }
    });
    dirs
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_get_size() {
        assert_eq!(
            584,
            Directory {
                name: "e".to_string(),
                contents: Vec::from([Content::File(File {
                    name: "i".to_string(),
                    size: 584,
                })]),
            }
            .size()
        );

        assert_eq!(
            94853,
            Directory::new(
                "a",
                Vec::from([
                    Content::Directory(Directory::new(
                        "e",
                        Vec::from([Content::File(File::new("i", 584))])
                    )),
                    Content::File(File::new("f", 29116)),
                    Content::File(File::new("g", 2557)),
                    Content::File(File::new("h.lst", 62596)),
                ])
            )
            .size()
        );
    }

    #[test]
    fn test_part_1_sample() {
        let filesystem = parse_filesystem(SAMPLE);

        assert_eq!(size_of_dirs_below(&filesystem, 100_000), 95437);
    }

    #[test]
    fn test_part_2_sample() {
        let filesystem = parse_filesystem(SAMPLE);

        assert_eq!(
            smallest_directory_to_free_up(&filesystem, FREE_NEEDED),
            24933642
        );
    }
}
