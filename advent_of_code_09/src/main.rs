use anyhow::Result;
use core::fmt;
use std::{fmt::Formatter, fs::read_to_string, iter};

fn main() -> Result<()> {
    let checksum = get_file_checksum("input.txt")?;
    println!("Checksum: {checksum}");

    let checksum = get_file_checksum_ext("input.txt")?;
    println!("Checksum: {checksum}");

    Ok(())
}

fn get_file_checksum(filename: &str) -> Result<u128> {
    let data = read_to_string(filename)?;
    //println!("Data: {data}");

    let filesystem = create_filesystem(&data);
    //println!("Filesystem: {:?}", filesystem);

    let defragmented = defragment_filesystem(filesystem);
    //println!("Defragmented Filesystem: {:?}", defragmented);

    let checksum = calculate_checksum(&defragmented);

    Ok(checksum)
}

fn get_file_checksum_ext(filename: &str) -> Result<u128> {
    let data = read_to_string(filename)?;
    //println!("Data: {data}");

    let filesystem = create_filesystem(&data);
    //println!("Filesystem: {:?}", filesystem);

    let defragmented = defragment_filesystem_ext(filesystem);
    //println!("Defragmented Filesystem: {:?}", defragmented);

    let checksum = calculate_checksum(&defragmented);

    Ok(checksum)
}

fn create_filesystem(file_string: &str) -> Filesystem {
    let mut data = Vec::new();
    let mut space = Filespace::Free;
    let mut id = 0;
    for n in file_string.chars() {
        space = space.next(id);
        let n = n.to_digit(10).unwrap() as usize;
        data.extend(iter::repeat(space).take(n));

        if let Filespace::File(_) = space {
            id += 1;
        }
    }
    Filesystem { data }
}

fn defragment_filesystem(mut filesystem: Filesystem) -> Filesystem {
    let mut free_id = 0;
    let mut last_id = filesystem.data.len() - 1;

    loop {
        // iterate free pointer until there is no file
        while let Filespace::File(_) = filesystem.data[free_id] {
            free_id += 1;
        }

        // iterate last pointer to the left until file was found
        while let Filespace::Free = filesystem.data[last_id] {
            last_id -= 1;
        }

        if free_id > last_id {
            break;
        }

        // swap memory to free space
        filesystem.data[free_id] = filesystem.data[last_id];
        filesystem.data[last_id] = Filespace::Free;
    }

    filesystem
}

fn defragment_filesystem_ext(mut filesystem: Filesystem) -> Filesystem {
    let mut index = filesystem.data.len() - 1;
    loop {
        let file = get_next_file(&filesystem, index);
        if let Some(file) = file {
            // find space that is big enough for the file
            let space = get_free_space(&filesystem, file.length);
            if let Some(space) = space {
                if space.start < file.start {
                    move_file(&mut filesystem, &file, &space);
                }
            }

            index = file.start - 1;
            if index == 0 {
                break;
            }
        } else {
            break;
        }
    }

    filesystem
}

fn get_next_file(filesystem: &Filesystem, index: usize) -> Option<File> {
    let mut end_index = index;
    // iterate from the end until next file was found
    while let Filespace::Free = filesystem.data[end_index] {
        if end_index == 0 {
            return None;
        }

        end_index -= 1;
    }

    // get file id
    let file_id = if let Filespace::File(id) = filesystem.data[end_index] {
        id
    } else {
        panic!("Invalid file")
    };

    // iterate to find the start of the file
    let mut start_index = end_index - 1;
    while let Filespace::File(id) = filesystem.data[start_index] {
        if id == file_id && start_index > 0 {
            start_index -= 1;
        } else {
            break;
        }
    }

    Some(File {
        file_space: Filespace::File(file_id),
        start: start_index + 1,
        length: end_index - start_index,
    })
}

fn get_free_space(filesystem: &Filesystem, length: usize) -> Option<File> {
    let mut index = 0;
    loop {
        let space = get_next_space(filesystem, index);
        if space.length >= length {
            return Some(space);
        } else {
            index = space.start + space.length + 1;
            if index >= filesystem.data.len() {
                break;
            }
        }
    }
    None
}

fn get_next_space(filesystem: &Filesystem, mut start: usize) -> File {
    // iterate free pointer until there is no file
    while let Filespace::File(_) = filesystem.data[start] {
        start += 1;
    }

    let mut end = start;
    while let Filespace::Free = filesystem.data[end] {
        if end < filesystem.data.len() - 1 {
            end += 1;
        } else {
            break;
        }
    }

    File {
        file_space: Filespace::Free,
        start,
        length: end - start,
    }
}

fn move_file(filesystem: &mut Filesystem, file: &File, space: &File) {
    for i in space.start..space.start + file.length {
        filesystem.data[i] = file.file_space;
    }

    for i in file.start..file.start + file.length {
        filesystem.data[i] = Filespace::Free;
    }
}

fn calculate_checksum(filesystem: &Filesystem) -> u128 {
    let mut checksum: u128 = 0;
    for (i, f) in filesystem.data.iter().enumerate() {
        if let Filespace::File(f) = f {
            checksum += i as u128 * *f as u128;
        }
    }

    checksum
}

#[derive(Clone, Copy)]
enum Filespace {
    File(u32),
    Free,
}

impl Filespace {
    fn next(&self, id: u32) -> Self {
        match self {
            Self::File(_) => Self::Free,
            Self::Free => Self::File(id),
        }
    }
}

impl fmt::Debug for Filespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = match self {
            Filespace::File(c) => c.to_string(),
            Filespace::Free => ".".to_string(),
        };
        write!(f, "{data}")
    }
}

#[derive(Debug)]
struct File {
    file_space: Filespace,
    start: usize,
    length: usize,
}

#[derive(Debug)]
struct Filesystem {
    data: Vec<Filespace>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = get_file_checksum("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(1928, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = get_file_checksum("input.txt");
        assert!(result.is_ok());
        assert_eq!(6415184586041, result.unwrap())
    }

    #[test]
    fn test_small_b() {
        let result = get_file_checksum_ext("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(2858, result.unwrap())
    }

    #[test]
    fn test_input_b() {
        let result = get_file_checksum_ext("input.txt");
        assert!(result.is_ok());
        assert_eq!(6436819084274, result.unwrap())
    }
}
