pub fn day2(path: &String) -> [u64; 2] {
    let mut result: [u64; 2] = [0, 0];

    for line in std::fs::read_to_string(path).unwrap().split(',') {
        let (left, right) = line.trim().split_once('-').unwrap();
        let (left, right) = (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap());

        for i in left..=right {
            let stri = i.to_string();
            let byti = stri.as_bytes();
            let ln = stri.len();

            if ln % 2 == 0 && stri[..ln / 2] == stri[ln / 2..] {
                result[0] += i;
            }

            for j in 1..=ln / 2 {
                if byti.chunks(j).all(|x| x == &byti[0..j]) {
                    result[1] += i;
                    break;
                }
            }
        }
    }

    result
}
