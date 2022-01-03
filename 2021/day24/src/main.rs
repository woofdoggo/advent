use std::collections::HashMap;

use radix_fmt::radix_26;

type EmptyResult = Result<(), Box<dyn std::error::Error>>;
type BlkConf = (bool, i64, i64);

const DIFFS: [BlkConf; 14] = [
    (false, 14, 16),
    (false, 11, 3),
    (false, 12, 2),
    (false, 11, 7),
    (true, -10, 13),
    (false, 15, 6),
    (true, -14, 10),
    (false, 10, 11),
    (true, -4, 6),
    (true, -3, 5),
    (false, 13, 11),
    (true, -3, 4),
    (true, -9, 4),
    (true, -12, 6)
];

fn main() -> EmptyResult {
    part1()?;
    part2()?;
    Ok(())
}

fn solve(max: bool) -> i64 {
    // each block will actually add a new number onto the stack,
    // but POP operations also remove one and so there ends up
    // being one less number on the stack.
    //
    // PUSH (#) POP (v) : # # # # v # v # v v # v v v
    // block #          : 0 1 2 3 4 5 6 7 8 9 a b c d

    // push/pop pairs:
    // 3 -> 4
    // 5 -> 6
    // 7 -> 8
    // 2 -> 9
    // a -> b
    // 1 -> c
    // 0 -> d
    // hardcoded might fix for all inputs later
    const DIFF_PAIRS: [(usize, usize); 7] = [
        (3, 4),
        (5, 6),
        (7, 8),
        (2, 9),
        (10, 11),
        (1, 12),
        (0, 13)
    ];

    // calculate diffs between #'s
    let mut diffs: Vec<(usize, usize, i64)> = Vec::new();
    for pair in DIFF_PAIRS {
        let a = DIFFS[pair.0];
        let b = DIFFS[pair.1];

        // a.2 is always positive, b.1 is always negative
        diffs.push((pair.0, pair.1, a.2 + b.1));
    }

    // try all valid models and find greatest
    // you could certainly make this smarter
    // instead of bruteforce
    let mut res = if max { i64::MIN } else { i64::MAX };
    for a in 1 ..= 9 {
        for b in 1 ..= 9 {
            for c in 1 ..= 9 {
                for d in 1 ..= 9 {
                    for e in 1 ..= 9 {
                        for f in 1 ..= 9 {
                            'g: for g in 1 ..= 9 {
                                let mut model = vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0];
                                model[diffs[0].0] = a;
                                model[diffs[0].1] = a + diffs[0].2;

                                model[diffs[1].0] = b;
                                model[diffs[1].1] = b + diffs[1].2;

                                model[diffs[2].0] = c;
                                model[diffs[2].1] = c + diffs[2].2;

                                model[diffs[3].0] = d;
                                model[diffs[3].1] = d + diffs[3].2;

                                model[diffs[4].0] = e;
                                model[diffs[4].1] = e + diffs[4].2;

                                model[diffs[5].0] = f;
                                model[diffs[5].1] = f + diffs[5].2;

                                model[diffs[6].0] = g;
                                model[diffs[6].1] = g + diffs[6].2;

                                for n in &model {
                                    if *n > 9 || *n < 1 { continue 'g; }
                                }

                                let mut key: i64 = 0;
                                for (i, n) in model.iter().rev().enumerate() {
                                    key += *n * 10_i64.pow(i as u32);
                                }

                                if max {
                                    res = std::cmp::max(res, key);
                                } else {
                                    res = std::cmp::min(res, key);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    res
}

fn part1() -> EmptyResult {
    println!("part 1: {}", solve(true));
    Ok(())
}

fn part2() -> EmptyResult {
    println!("part 2: {}", solve(false));
    Ok(())
}
