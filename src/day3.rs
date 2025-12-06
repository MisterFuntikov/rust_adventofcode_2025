fn helper(line: &str, digits: usize) -> u64 {
    let mut nums: Vec<&str> = Vec::with_capacity(digits);
    (0..digits).for_each(|i| nums.push(&line[i..i + 1]));

    for i in 1..line.len() {
        for j in 0..digits {
            if j > 0 && std::ptr::eq(&line[i..i + 1], nums[j]) {
                break;
            }
            if &line[i..i + 1] > nums[j] && digits - 1 - j + i < line.len() {
                for u in 0..digits - j {
                    nums[j + u] = &line[i + u..i + u + 1];
                }
                break;
            }
        }
    }

    nums.concat().parse::<u64>().unwrap()
}

pub fn day3(path: &String) -> [u64; 2] {
    let mut result: [u64; 2] = [0, 0];

    for line in std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
    {
        result[0] += helper(line, 2);
        result[1] += helper(line, 12);
    }

    result
}
