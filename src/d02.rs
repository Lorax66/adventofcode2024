fn is_safe(mut report: Vec<i32>) -> bool {
    if report.len() <= 1 {
        return true;
    }
    if report[0] > report[1] {
        if report[report.len() - 1] > report[report.len() - 2] {
            return false;
        }
        report.reverse();
        return is_safe(report);
    }
    return report.windows(2).all(|w| {
        // dbg!(w);
        w[1] - w[0] <= 3 && w[1] - w[0] >= 1
    });
}

fn is_safe_with_dampener(mut report: Vec<i32>) -> bool {
    if is_safe(report.clone()) {
        return true;
    }
    for i in 0..report.len() {
        let dampened_level = report[i];
        report.remove(i);
        if is_safe(report.clone()) {
            return true;
        }
        report.insert(i, dampened_level);
    }
    return false;
}

#[aoc(day2, part1)]
pub fn task1(input: &str) -> usize {
    // dbg!(&input);
    input
        .split_terminator("\n")
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report: &Vec<i32>| is_safe(report.clone()))
        .count()
}

#[aoc(day2, part2)]
pub fn task2(input: &str) -> usize {
    // dbg!(&input);
    input
        .split_terminator("\n")
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report: &Vec<i32>| is_safe_with_dampener(report.clone()))
        .count()
}
