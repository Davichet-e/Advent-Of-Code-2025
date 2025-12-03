use std::ops::RangeInclusive;

pub fn day2() {
    let input = // Hola
    
    include_str!("../inputs/day2");

    let response: (usize, usize) = input.trim_end().split(",").fold((0, 0), |acc, range| {
        let split = range.split_once("-").unwrap();
        let range: RangeInclusive<usize> = split.0.parse().unwrap()..=split.1.parse().unwrap();

        (
            acc.0 + range.clone().filter(is_not_valid_id).sum::<usize>(),
            acc.1 + range.filter(is_not_valid_id_v2).sum::<usize>(),
        )
    });
    println!("{response:?}");
}

fn is_not_valid_id(num: &usize) -> bool {
    let s = num.to_string();
    if s.len() % 2 != 0 {
        return false;
    }
    let middle_point = s.len() / 2;
    s[..middle_point] == s[middle_point..]
}

fn is_not_valid_id_v2(num: &usize) -> bool {
    let s = num.to_string();
    for c in 1..(s.len() / 2 + 1) {
        let string_to_check = &s[..c];
        let v: Vec<char> = s.chars().skip(c).collect();

        if v.chunks(c)
            .all(|s| s.iter().collect::<String>() == string_to_check)
        {
            return true;
        }
    }
    return false;
}
