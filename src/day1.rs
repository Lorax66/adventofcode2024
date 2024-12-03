#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .split_terminator('\n')
        .map(|s| -> (i32, i32) {
            let mut sw = s.split_whitespace();
            (
                sw.next().unwrap().parse::<i32>().unwrap(),
                sw.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    // dbg!(&input);
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .split_terminator('\n')
        .map(|s| -> (i32, i32) {
            let mut sw = s.split_whitespace();
            (
                sw.next().unwrap().parse::<i32>().unwrap(),
                sw.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();
    left.sort();
    right.sort();
    // dbg!(&left);
    // dbg!(&right);
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    let mut result = 0i32;
    let mut last_found = -1i32;
    let mut next_l = left_iter.next().unwrap();
    let mut next_r = right_iter.next().unwrap();
    let mut found_value_left = 0i32;
    let mut found_value_right = 0i32;
    while *next_r != i32::MAX || *next_l != i32::MAX {
        if next_l < next_r {
            if last_found == *next_l {
                found_value_left += 1;
            }
            match left_iter.next() {
                Some(val) => next_l = val,
                None => next_l = &i32::MAX,
            };
        } else if next_r < next_l {
            if last_found == *next_r {
                found_value_right += 1;
            }
            match right_iter.next() {
                Some(val) => next_r = val,
                None => next_r = &i32::MAX,
            };
        } else {
            assert!(next_l == next_r);
            if *next_l != last_found {
                // dbg!(last_found * found_value_right * found_value_left);
                result += last_found * found_value_right * found_value_left;
                found_value_right = 0;
                found_value_left = 0;
            }
            found_value_right += 1;
            found_value_left += 1;
            last_found = *next_l;
            match right_iter.next() {
                Some(val) => next_r = val,
                None => next_r = &i32::MAX,
            };
            match left_iter.next() {
                Some(val) => next_l = val,
                None => next_l = &i32::MAX,
            };
        }
    }
    // dbg!(last_found * found_value_right * found_value_left);
    result += last_found * found_value_right * found_value_left;
    return result;
}
