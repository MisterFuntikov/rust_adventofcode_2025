pub fn day1(path: &String) -> [u64; 2] {
    let mut pos: i32 = 50;
    let mut result: [u64; 2] = [0, 0];

    for line in std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
    {
        if line.starts_with('L') {
            let mut has_zero = pos == 0;
            pos -= line[1..].trim().parse::<i32>().unwrap();
            while pos < 0 {
                pos += 100;
                result[1] = if !has_zero { result[1] + 1 } else { result[1] };
                has_zero = false;
            }
            result[1] = if pos == 0 { result[1] + 1 } else { result[1] };
        } else {
            pos += line[1..].trim().parse::<i32>().unwrap();
            let nm = pos / 100;
            (pos, result[1]) = (pos - nm * 100, result[1] + nm as u64)
        }
        result[0] = if pos == 0 { result[0] + 1 } else { result[0] };
    }

    result
}
