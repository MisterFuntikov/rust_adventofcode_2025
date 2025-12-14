mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

const TEST_ANSWERS: [[u64; 2]; 12] = [
    [3, 6],
    [1227775554, 4174379265],
    [357, 3121910778619],
    [13, 43],
    [3, 14],
    [4277556, 3263827],
    [21, 40],
    [40, 25272],
    [50, 24],
    [7, 33],
    [8, 2],
    [0, 0],
];

const ANSWRES: [[u64; 2]; 12] = [
    [0x45E, 0x1891],
    [0x95CF6B113, 0xBD38CDE0E],
    [0x4395, 0x9C94AEAE298D],
    [0x644, 0x24B5],
    [0x259, 0x14E9A63E9F40C],
    [0x5EA2C7FD9AD, 0x8C4A308DA41],
    [0x638, 0x104CC88BFF71],
    [0x136C8, 0x1DBCE54],
    [0x11AD16D09, 0x5BCD78F4],
    [0x23B, 0],
    [0x2F1, 0x19A0CB2B236BC],
    [0, 0],
];

const DAYS_FUNC: [for<'a> fn(&'a String) -> [u64; 2]; 11] = [
    day1::day1,
    day2::day2,
    day3::day3,
    day4::day4,
    day5::day5,
    day6::day6,
    day7::day7,
    day8::day8,
    day9::day9,
    day10::day10,
    day11::day11,
];

fn day(num: usize) {
    let nn = num.to_string();
    let ex = format!("inputs/day{nn}_example.txt");
    let path = format!("inputs/day{nn}.txt");
    assert_eq!(DAYS_FUNC[num - 1](&ex), TEST_ANSWERS[num - 1]);
    let result = DAYS_FUNC[num - 1](&path);
    assert_eq!(result, ANSWRES[num - 1]);
    println!("Day {num} = {:?}", result);
}

fn main() {
    day(1);
    day(2);
    day(3);
    day(4);
    day(5);
    day(6);
    day(7);
    day(8);
    day(9);
    // day(10);
    day(11);
}
