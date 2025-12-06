fn helper(data: &Vec<Vec<bool>>, i: usize, j: usize) -> bool {
    let mut count: u32 = 0;
    for di in -1..=1 {
        for dj in -1..=1 {
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if !((di == 0 && dj == 0)
                || ((ni >= 0 && nj >= 0)
                    && (ni < data.len() as i32 && nj < data[i].len() as i32)
                    && data[ni as usize][nj as usize]))
            {
                count += 1;
            }
        }
    }

    count > 4
}

pub fn day4(path: &String) -> [u64; 2] {
    let mut result: [u64; 2] = [0, 0];

    let mut data: Vec<Vec<bool>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] && helper(&data, i, j) {
                result[0] += 1;
            }
        }
    }

    let mut ans = true;
    while ans {
        ans = false;
        for i in 0..data.len() {
            for j in 0..data[i].len() {
                if data[i][j] && helper(&data, i, j) {
                    (data[i][j], ans, result[1]) = (false, true, result[1] + 1);
                }
            }
        }
    }

    result
}
