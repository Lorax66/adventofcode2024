fn is_safe(report: &mut Vec<i32>) -> bool {
    if report.len() <= 1 {
        return true;
    }
    if report[0] > report[1] {
        if report[report.len() - 1] > report[report.len() - 2] {
            return false;
        }
        report.reverse();
        let result = is_safe(report);
        report.reverse();
        return result;
    }
    return report.windows(2).all(|w| {
        // dbg!(w);
        w[1] - w[0] <= 3 && w[1] - w[0] >= 1
    });
}

fn is_safe_with_dampener(report: &mut Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let dampened_level = report[i];
        report.remove(i);
        if is_safe(report) {
            return true;
        }
        report.insert(i, dampened_level);
    }
    return false;
}

pub fn task1(input: Vec<String>) {
    // dbg!(&input);
    let mut reports = input
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    println!(
        "result: {}",
        reports
            .iter_mut()
            .filter(|report| is_safe_with_dampener(*(mut report)))
            .collect::<Vec<&mut Vec<i32>>>()
            .len()
    );
}
