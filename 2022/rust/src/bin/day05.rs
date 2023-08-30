use advtools::input;
use advtools::prelude::{Itertools, VecDeque};

const RX: &str = r"move (\d+) from (\d+) to (\d+)|(.*)";

fn run(instrs: &[(usize, usize, usize)], mut stacks: Vec<VecDeque<char>>, multi: bool) -> String {
    for &(n, from, to) in instrs {
        let m = stacks[from-1].len();
        let transfer = stacks[from-1].drain(m-n..).collect_vec();
        if multi {
            stacks[to-1].extend(transfer);
        } else {
            stacks[to-1].extend(transfer.iter().rev());
        }
    }

    stacks.iter().map(|s| s[s.len() - 1]).collect()
}


fn main() {
    let mut instrs: Vec<(usize, usize, usize)> = vec![];
    let mut stacks: Vec<VecDeque<char>> = (0..9).map(|_| VecDeque::new()).collect_vec();

    for (instr, line) in input::rx_lines::<(Option<(usize, usize, usize)>, &str)>(RX) {
        if let Some(instr) = instr {
            instrs.push(instr);
        } else if line.contains('[') {
            line.chars().skip(1).step_by(4).enumerate().for_each(|(i, c)| {
                if c != ' ' {
                    stacks[i].push_front(c);
                }
            });
        }
    }

    advtools::verify("part1", run(&instrs, stacks.clone(), false), "VRWBSFZWM");
    advtools::verify("part1", run(&instrs, stacks.clone(), true), "RBTWJWMCF");
}
