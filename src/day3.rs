use std::{fs, path::Path};

pub fn main() {
    let path = Path::new("input/day3.txt");
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut total = 0;
    for line in content.lines() {
        let duplicate = find_duplicate(line);
        total += get_item_priority(&duplicate);
    }

    println!("Total: {}", total);
}

fn split_two_part(line: &str) -> (Vec<char>, Vec<char>) {
    let chars = line.chars().collect::<Vec<char>>();

    if chars.len() % 2 != 0 {
        panic!("odd number of chars");
    }

    let mid = chars.len() / 2;

    (chars[..mid].to_vec(), chars[mid..].to_vec())
}

#[test]
#[should_panic]
fn test_split_string_panic() {
    let (a, b) = split_two_part("abcde");
}

#[test]
fn test_split_string() {
    let (a, b) = split_two_part("abcdef");
    assert_eq!(a, vec!['a', 'b', 'c']);
    assert_eq!(b, vec!['d', 'e', 'f']);
}

fn find_duplicate(line: &str) -> char {
    let (a, b) = split_two_part(line);

    let duplicate = a
        .iter()
        .find(|&c| b.contains(c))
        .expect("no duplicate found");

    duplicate.to_owned()
}

#[test]
#[should_panic]
fn test_find_duplicate_panic() {
    find_duplicate("abcde");
}

#[test]
fn test_find_duplicate() {
    let duplicate = find_duplicate("abcabc");
    assert_eq!(duplicate, 'a');

    let duplicate = find_duplicate("vJrwpWtwJgWrhcsFMMfFFhFp");
    assert_eq!(duplicate, 'p');

    let duplicate = find_duplicate("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
    assert_eq!(duplicate, 'L');
}

fn get_item_priority(item: &char) -> i32 {
    let item_number = item.to_owned() as i32;
    if item.is_uppercase() {
        return item_number - 38;
    }

    item_number - 96
}

#[test]
fn test_get_item_priority() {
    let priority = get_item_priority(&'a');
    assert_eq!(priority, 1);

    let priority = get_item_priority(&'b');
    assert_eq!(priority, 2);

    let priority = get_item_priority(&'p');
    assert_eq!(priority, 16);

    let priority = get_item_priority(&'L');
    assert_eq!(priority, 38);

    let priority = get_item_priority(&'P');
    assert_eq!(priority, 42);
}
