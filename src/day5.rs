fn helper(data: &mut Vec<[u64; 2]>, checki: usize) -> bool {
    let (check_left, check_right) = (data[checki][0], data[checki][1]);
    for i in 0..data.len() {
        if i != checki && data[i][0] <= check_right && check_left <= data[i][1] {
            (data[i][0], data[i][1]) = (data[i][0].min(check_left), data[i][1].max(check_right));
            return true;
        }
    }
    false
}

pub fn day5(path: &String) -> [u64; 2] {
    let mut result: [u64; 2] = [0, 0];
    let mut data: Vec<[u64; 2]> = vec![];
    let mut fill_mode = true;

    for line in std::fs::read_to_string(path).unwrap().lines() {
        if line.is_empty() {
            fill_mode = false;
            'l: loop {
                for i in 0..data.len() {
                    if data[i][0] != 0 && data[i][1] != 0 && helper(&mut data, i) {
                        (data[i][0], data[i][1]) = (0, 0);
                        continue 'l;
                    }
                }
                break;
            }
        } else if fill_mode {
            let (left, right) = line.trim().split_once('-').unwrap();
            data.push([left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()]);
        } else {
            let value = line.parse::<u64>().unwrap();
            if let Some(_r) = data.iter().find(|d| value >= d[0] && value <= d[1]) {
                result[0] += 1;
            }
        }
    }

    for dt in data.iter().filter(|x| x[0] != 0 && x[1] != 0) {
        result[1] += dt[1] + 1 - dt[0];
    }

    result
}
