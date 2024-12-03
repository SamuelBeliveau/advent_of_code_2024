use std::fs;

pub fn solve_a() {
    let pairs = read_pairs();
    let (list1, list2) = split_lists(pairs);

    let mut distances = 0u32;

    for i in 0..list1.len() {
        distances += list1[i].abs_diff(list2[i]);
    }

    println!("{:?}", distances);
}

pub fn solve_b() {
    let pairs = read_pairs();
    let (list1, list2) = split_lists(pairs);

    let mut score = 0usize;

    for entry1 in list1.iter() {
        let frequency = list2.iter().filter(|entry2| *entry1 == **entry2).count();
        score += *entry1 as usize * frequency;
    }

    println!("score is {}", score);
}

fn read_pairs() -> Vec<(u32, u32)> {
    let contents = fs::read_to_string("inputs/day1.txt").unwrap();
    contents
        .lines()
        .map(|line| {
            let entry: Vec<_> = line
                .split("   ")
                .map(|location_str| location_str.parse::<u32>().unwrap())
                .collect();
            (entry[0], entry[1])
        })
        .collect()
}

fn split_lists(pairs: Vec<(u32, u32)>) -> (Vec<u32>, Vec<u32>) {
    let mut list1 = Vec::with_capacity(pairs.len());
    let mut list2 = Vec::with_capacity(pairs.len());

    for (entry1, entry2) in pairs.into_iter() {
        list1.push(entry1);
        list2.push(entry2);
    }

    list1.sort();
    list2.sort();

    (list1, list2)
}
