struct MachineLine {
    lights: u64,
    buttons: Vec<u64>,
    joltage: Vec<u16>,
}

fn part1(ml: &MachineLine, mut best: u64, pos: usize, mut state: u64, cur: u64) -> u64 {
    if cur < best {
        state = state ^ ml.buttons[pos];
        if state == ml.lights {
            best = cur;
        } else {
            for i in 0..ml.buttons.len() {
                if i != pos {
                    best = part1(&ml, best, i, state, cur + 1);
                }
                if cur + 1 >= best {
                    break;
                }
            }
        }
    }
    best
}

fn part2(ml: &MachineLine) -> usize {
    let n_counters = ml.joltage.len();
    let mut best = usize::MAX;

    fn dfs(
        idx: usize,
        state: &mut Vec<u16>,
        cost: usize,
        best: &mut usize,
        buttons: &Vec<u64>,
        target: &Vec<u16>,
    ) {
        if cost >= *best {
            return;
        }

        if idx == buttons.len() {
            if state.iter().zip(target).all(|(&s, &t)| s == t) {
                *best = cost.min(*best);
            }
            return;
        }

        let mut max_press = 0;
        for i in 0..state.len() {
            if (buttons[idx] >> i) & 1 == 1 {
                let remaining = target[i].saturating_sub(state[i]);
                max_press = max_press.max(remaining);
            }
        }

        for presses in 0..=max_press {
            for i in 0..state.len() {
                if (buttons[idx] >> i) & 1 == 1 {
                    state[i] += presses;
                }
            }

            dfs(
                idx + 1,
                state,
                cost + presses as usize,
                best,
                buttons,
                target,
            );

            for i in 0..state.len() {
                if (buttons[idx] >> i) & 1 == 1 {
                    state[i] -= presses;
                }
            }
        }
    }

    let mut state = vec![0u16; n_counters];
    dfs(0, &mut state, 0, &mut best, &ml.buttons, &ml.joltage);
    best
}

pub fn day10(path: &String) -> [u64; 2] {
    let content = std::fs::read_to_string(path).unwrap();
    let mut result = [0, 0];

    for line in content.lines().filter(|l| !l.is_empty()) {
        let blocks: Vec<&str> = line.split(' ').collect();
        let mut ml = MachineLine {
            lights: 0u64,
            buttons: vec![],
            joltage: vec![],
        };

        for (i, char) in (blocks[0][1..blocks[0].len() - 1]).chars().enumerate() {
            if char == '#' {
                ml.lights ^= 1u64 << i;
            }
        }

        for el in (blocks[blocks.len() - 1][1..blocks[blocks.len() - 1].len() - 1]).split(',') {
            ml.joltage.push(el.parse::<u16>().unwrap());
        }

        for i in 1..blocks.len() - 1 {
            let mut bt = 0u64;
            for el in (blocks[i][1..blocks[i].len() - 1]).split(',') {
                bt ^= 1u64 << el.parse::<u16>().unwrap();
            }
            ml.buttons.push(bt);
        }

        let mut m = 7;
        for i in 0..ml.buttons.len() {
            m = part1(&ml, m, i, 0u64, 1);
        }
        result[0] += m;
        result[1] += part2(&ml) as u64;
    }

    result
}
