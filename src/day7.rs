fn helper(
    lines: &Vec<&str>,
    linenum: usize,
    beam: usize,
    split: &mut std::collections::HashMap<usize, u64>,
) -> u64 {
    for i in linenum..lines.len() {
        for (pos, char) in lines[i].chars().enumerate() {
            if char == '^' && beam == pos {
                if !split.contains_key(&(i * 1000 + pos)) {
                    let value =
                        helper(lines, i + 1, pos - 1, split) + helper(lines, i + 1, pos + 1, split);
                    split.insert(i * 1000 + pos, value + 1);
                }
                return *split.get(&(i * 1000 + pos)).unwrap();
            }
        }
    }
    0
}

pub fn day7(path: &String) -> [u64; 2] {
    let content = std::fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = content.lines().filter(|l| !l.is_empty()).collect();
    let mut split: std::collections::HashMap<usize, u64> = std::collections::HashMap::new();
    let r = helper(&lines, 1, lines[0].find('S').unwrap(), &mut split);
    [split.len() as u64, r + 1]
}
