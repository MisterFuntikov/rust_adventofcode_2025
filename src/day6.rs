fn helper(data: &Vec<Vec<&str>>, i: usize, ans: u64, value: &str) -> u64 {
    if data[data.len() - 1][i].trim() == "*" {
        return ans.max(1) * value.trim().parse::<u64>().unwrap();
    }
    ans + value.trim().parse::<u64>().unwrap()
}

pub fn day6(path: &String) -> [u64; 2] {
    let mut result: [u64; 2] = [0, 0];
    let content = std::fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = content.lines().filter(|l| !l.is_empty()).collect();
    let mut data: Vec<Vec<&str>> = vec![vec![]; lines.len()];
    let mut prev: usize = 0;

    for i in 0..lines[0].len() {
        let ltg = &lines[lines.len() - 1][i..(i + 2).min(lines[0].len())];
        if ltg.starts_with(' ') && !ltg.ends_with(' ') {
            for j in 0..lines.len() {
                data[j].push(&lines[j][prev..i]);
            }
            prev = i + 1;
        }
    }
    for j in 0..lines.len() {
        data[j].push(&lines[j][prev..]);
    }

    for i in 0..data[0].len() {
        let mut ans: (u64, u64) = (0, 0);

        for q in 0..data[0][i].len() {
            let mut st: String = String::new();
            ans.0 = 0;
            for j in 0..data.len() - 1 {
                ans.0 = helper(&data, i, ans.0, data[j][i]);
                st.push_str(&data[j][i][q..q + 1]);
            }
            ans.1 = helper(&data, i, ans.1, &st);
        }

        (result[0], result[1]) = (result[0] + ans.0, result[1] + ans.1);
    }

    result
}
