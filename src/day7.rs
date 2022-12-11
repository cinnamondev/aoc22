use std::collections::HashMap;
use crate::day7::Line::Folder;

// Don't need to consider root too much, cd / can be ignored and assume starting point is /
#[derive(Debug)]
pub enum Line<'l> {
    InFolder(&'l str),       // cd dir
    OutFolder,                 // cd ..
    Listing,                 // ls
    File(u32, &'l str),  // File - Size and name.
    Folder(&'l str)      // Folder - name
}

//#[aoc_generator(day7)]
// lifetimes in cargo-aoc is bugged. gdi :(.
pub fn parse_input(input: &str) -> Vec<Line> {
    // /
    // dir
    input.lines()
        .skip(1)
        .map(|l| {
            // $ cd dir
            // $ ls
            // dir X
            match &l[0..=3] {
                "$ cd" => { // Into/Out
                    if (&l).ends_with("..") {
                        //dbg!("outfolder");
                        Line::OutFolder
                    } else {
                        //dbg!(&l[4..]);
                        Line::InFolder(&l[5..])
                    }
                },
                "$ ls" => { // Listing
                    Line::Listing
                },
                "dir " => {
                    Line::Folder(&l[4..])
                },
                _ => {
                    let file = l.split_once(' ').expect("uh oh");
                    Line::File(file.0.parse().expect("uh oghhhhh"), file.1)

                }
            }
        })
        .collect()
}

#[derive(Default,Debug)]
pub struct Directory<'l> {
    sub: HashMap<&'l str, Directory<'l>>,   // link to folder inside structure
    size: u32,                              // we don't need to keep file info, just add to this.
}
/// Build recursive "tree" structure. cws should be passed as your "root", line counter is for tracking when passing through fn. start at 0
/// build_structure(input, &root, 0)
/// return value updates the index when returning, the calling function needs to skip "n" items.
pub fn build_strucutre<'l>(input: &[Line<'l>], cwd: &mut Directory<'l>, list_index: usize) -> usize {
    let mut explored = false;
    let mut i = list_index;
    //dbg!(format!("{i} OUTER"));
    while i < input.len() {
        let l = &input[i];
        //dbg!(i);
        match l {
            Line::InFolder(name) => {
                // folder is new root. cd [folder] is always followed by $ ls so we can ignore it!
                let folder = cwd.sub.entry(name).or_insert_with(|| {Directory::default()});
                // build new tree with folder as root, after $ ls. all subsequent non commands will be directory/file info.
                i = build_strucutre(&input, folder,  (i + 1)) -1;
                //dbg!(format!("entered folder {name}"));
                // update to new index.
                cwd.size += folder.size; // root size contains size of all files and directories.
            },
            Line::OutFolder => {    // go out a level
                return i+1;  // Tier below should continue from next index.
            },
            Line::Folder(folder) => {   // new folder found
                if explored {i+=1;continue;}
                //dbg!(&folder);
                cwd.sub.entry(folder).or_insert_with(|| {Directory::default()});
            },
            Line::File(size, name) => {    // grow file size
                if explored {i+=1;continue;} // prevent dupes
                //dbg!(name);
                cwd.size += size;
            },
            //_ => continue,
            Line::Listing => {
                if cwd.sub.len() != 0 {
                    explored = true;    // Path has been explored, skip (prevents duplicates)
                }
            }

        }
        i+=1
    }
    i+1
}

pub fn flatten_folders<'l>(cwd: &Directory<'l>, init_key: &'l str) -> Vec<(&'l str, u32)> {
    let mut folders: Vec<(&'l str, u32)> = Vec::new();
    let pair = (init_key, cwd.size);
    folders.push(pair); // push current folder (starting with root)
    for (key, directory) in &cwd.sub {
        let mut new_keys = flatten_folders(&directory,key); // go into folder and evaluate all sizes. all directories within x go here.
        folders.append(&mut new_keys); // gross structure but ok
    }
    folders
}
#[aoc(day7,part1)]
pub fn solve_part1<'l>(input: &str) -> u32 {
    // sum of all directory sizes that are below 10000
    // CWD at start: /
    let parsed = parse_input(input);
    let mut root = Directory::default();
    build_strucutre(&parsed, &mut root, 0);
    let flattened_structure = flatten_folders(&root, &"/");
    flattened_structure.into_iter()
        .filter(|&(_,v)| v < 100000)
        .fold(0, |acc,(_,v)| acc+v)
}
// part 2: find the value where |delta x| is closest to 0!! lowest |delta x| wins.
#[aoc(day7,part2)]
pub fn solve_part2<'l>(input: &str) -> u32 {
    let parsed = parse_input(input);
    let mut root = Directory::default();
    build_strucutre(&parsed, &mut root, 0);
    let flattened_structure = flatten_folders(&root, &"/");
    let to_free = (30000000 -(70000000 - root.size as i32)); // free space (<300000)
    dbg!(&to_free);
    let smallest_delta = flattened_structure.iter()
        .filter_map(|&(dir,v)| {
            let delta = (to_free-v as i32); // if v is larger than or eq, will be negative.
            if delta > 0 {
                dbg!("refuse",&delta,&v);
                None
            } else {
                dbg!(&delta,&v);
                Some((delta,v))
            }
        })
        .max_by(|&(d1,_),&(d2,_)| d1.cmp(&d2))
        .expect("veuorhferiuh");
    dbg!(&smallest_delta.0,&smallest_delta.1);
    smallest_delta.1


}