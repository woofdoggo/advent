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

fn num_to_vec(n: i64, xs: &mut Vec<i64>) {
    if n >= 10 {
        num_to_vec(n / 10, xs);
    }
    if n % 10 == 0 { return; }
    xs.push(n % 10);
}

fn sim_stack(block: BlkConf, inp: i64, stack: &mut Vec<i64>) {
    fn pop(stack: &mut Vec<i64>) -> i64 {
        if stack.len() == 0 {
            0
        } else {
            *stack.last().unwrap()
        }
    }

    let top = pop(stack) + block.1;
    if block.0 {
        stack.pop();
    }

    if top != inp {
        stack.push(inp + block.2);
    }
}

fn part1() -> EmptyResult {
    // each block will actually add a new number onto the stack,
    // but POP operations also remove one and so there ends up
    // being one less number on the stack.
    //
    // PUSH (#) POP (v) : # # # # v # v # v v # v v v
    // block #          : 0 1 2 3 4 5 6 7 8 9 a b c d

    Ok(())
}

fn part2() -> EmptyResult {

    Ok(())
}
