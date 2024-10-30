mod schema;
use std::fs::File;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn read_from_file(path_to_file: &str) {
    let f = File::open(path_to_file).expect("The file provided in the path does not exist");

    // let mut reader = BufReader::new(f);
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn reads() {
    }
}
