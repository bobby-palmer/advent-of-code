use std::fs;
use std::io::Read;

// Returns a String that contains the contents of the given file path
// 
fn get_contents(file_path: &str) -> String {
    let mut file = fs::File::open(file_path)
        .expect("unable to find file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("cannot read file");
    contents
}
// Returns a vector with each element being a string slice with one elfs total inventory
//
fn get_max(list: &str) -> Option<u32> {
    list.split("\n\n").map(|inventory| {
        inventory.split_terminator("\n").map(|item| {
            item.parse::<u32>().unwrap_or_else(|err| {
                println!("got error :{}", err);
                0
            })
        }).sum()
    }).max()
}
// Overarching function that calls the other functions
//
pub fn solve(file_path: &str) -> u32 {
    get_max(&get_contents(file_path))
        .unwrap_or_else(|| {
            println!("failure in get_max function!");
            0
        })
}


