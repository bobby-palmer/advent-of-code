use std::str;

fn split_items<'a>(list: &'a str) -> impl Iterator<Item = &'a str> {
    let mut level = 0;
    list.split(move |letter| match letter {
        ',' if level == 0 => true,
        letter => {
            match letter {
                '[' => level += 1,
                ']' => level -= 1,
                _ => (),
            }
            false
        }
    })
    .filter(|item| !item.is_empty())
}
fn ensure_unpack_list(list: &str) -> &str {
    if list.chars().next() == Some('[') {
        let mut new_list = list.chars();
        new_list.next();
        new_list.next_back();
        new_list.as_str()
    } else {
        list
    }
}
fn is_list(item: &str) -> bool {
    item.chars().next() == Some('[')
}
fn identical(list1: &str, list2: &str) -> bool {
    ensure_unpack_list(list1) == ensure_unpack_list(list2)
}
// passed in like "4,5,2,1,[342],3"
pub fn in_correct_order(list1: &str, list2: &str) -> bool {
    // unwrap list
    let mut values1 = split_items(ensure_unpack_list(list1));
    let mut values2 = split_items(ensure_unpack_list(list2));

    // while there are terms left in the first list
    while let Some(item1) = values1.next() {
        if let Some(item2) = values2.next() {
            if is_list(item1) || is_list(item2) {
                if !in_correct_order(item1, item2) {
                    return false;
                } else if !identical(item1, item2) {
                    return true;
                }
            } else {
                let num1 = item1.parse::<u32>().expect("Attempted to parse non-number");
                let num2 = item2.parse::<u32>().expect("Attempted to parse non-number");
                if num1 < num2 {
                    return true;
                } else if num1 > num2 {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    // if there are no terms in the first list it cannot be greater
    true
}
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_spliter() {
        let list = "[[],2,[],1,[4,1]],[3,[[8,4,0,7,8],4,2]],[[9,[2,1,8,2],[6,0,3,1,1]],4],[10,2,2]";
        let collection: Vec<&str> = split_items(list).collect();
        let first = "[[],2,[],1,[4,1]]";
        assert_eq!(collection.get(0), Some(&first));
    }
    #[test]
    fn test_compare() {
        let left = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let right = "[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert!(!in_correct_order(left, right));
    }
    #[test]
    fn test_compare2() {
        let left = "[[[]]]";
        let right = "[[]]";
        assert!(!in_correct_order(left, right));
    }
    #[test]
    fn test_empty_vs_non() {
        let left = "[]";
        let right = "[3]";
        assert!(in_correct_order(left, right));
    }
    #[test]
    fn test_extended() {
        let left = "[7,7,7]";
        let right = "[7,7,7,7]";
        assert!(in_correct_order(left, right));
    }
    #[test]
    fn test_contained() {
        let left = "[9]";
        let right = "[[8,7,6]]";
        assert!(!in_correct_order(left, right));
    }
    #[test]
    fn test_compare3() {
        let left = "[[1],[2,3,4]]";
        let right = "[[1],4]";
        assert!(in_correct_order(left, right));
    }
    #[test]
    fn test_compare4() {
        let left = "[[4,4],4,4]";
        let right = "[[4,4],4,4,4]";
        assert!(in_correct_order(left, right));
    }
    #[test]
    fn test_compare5() {
        let left = "[1,1,3,1,1]";
        let right = "[1,1,5,1,1]";
        assert!(in_correct_order(left, right));
    }
    #[test]
    fn test_contradiction() {
        let left = "[]";
        let right = "[]";
        assert!(in_correct_order(left, right));
    }
    #[test]
    fn test_same() {
        let left = "[1,1,[1]]";
        let right = "[1,1,1]";
        assert!(in_correct_order(left, right));
    }
}
