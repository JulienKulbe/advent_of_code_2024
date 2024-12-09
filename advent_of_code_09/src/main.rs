use anyhow::Result;
use core::fmt;
use std::{fmt::Formatter, fs::read_to_string, iter};

fn main() -> Result<()> {
    let checksum = get_file_checksum("input.txt")?;
    println!("Checksum: {checksum}");

    let checksum = get_file_checksum_2("input.txt")?;
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

fn get_file_checksum_2(filename: &str) -> Result<u128> {
    let data = read_to_string(filename)?;
    //println!("Data: {data}");

    let filesystem = create_filesystem(&data);
    //println!("Filesystem: {:?}", filesystem);

    let defragmented = defragment_filesystem(filesystem);
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
        let result = get_file_checksum_2("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(2858, result.unwrap())
    }

    #[test]
    #[ignore = "reason"]
    fn test_input_b() {
        let result = get_file_checksum_2("input.txt");
        assert!(result.is_ok());
        assert_eq!(20351745, result.unwrap())
    }
}
