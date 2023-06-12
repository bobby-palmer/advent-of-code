use std::collections::BTreeSet;

pub fn separate_inventory(list: &str) -> BTreeSet<u32> {
    list.split("\n\n").map(|inventory| {
        inventory.lines().map(|line| {
            line.parse::<u32>().unwrap_or_else(|err| {
                println!("unable to parse: {}", err);
                0
            })
        }).sum()
    }).collect()

}

pub fn n_max(mut collection: BTreeSet<u32>, n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for _ in 0..n {
        result.push(collection.pop_last()
            .unwrap_or_else(|| {
                println!("Binary tree is empty");
                0
            }));
    }
    result
}
