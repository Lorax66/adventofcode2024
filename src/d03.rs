// const LEGAL_CHARS: [char; 16] = [
//     'm', 'u', 'l', '(', ',', ')', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
// ];
const RADIX: u32 = 10;

fn solve_line(line: &str, ignore_active: bool) -> u32 {
    let mut active: bool = true;
    let mut result: u32 = 0;
    #[derive(Debug, PartialEq, Eq)]
    enum State {
        FoundNothing,
        FoundD,
        FoundDo,
        FoundDoPar,
        FoundDon,
        FoundDonApostroph,
        FoundDont,
        FoundDontPar,
        FoundM,
        FoundMu,
        FoundMul,
        FoundMulPar { first_num: u32 },
        FoundMulParComma { first_num: u32, second_num: u32 },
    }
    let mut state: State = State::FoundNothing;
    line.chars().into_iter().for_each(|c| {
        state = match state {
            State::FoundNothing if c == 'm' => State::FoundM,
            State::FoundNothing if c == 'd' => State::FoundD,
            State::FoundM if c == 'u' => State::FoundMu,
            State::FoundMu if c == 'l' => State::FoundMul,
            State::FoundD if c == 'o' => State::FoundDo,
            State::FoundDo if c == 'n' => State::FoundDon,
            State::FoundDon if c == '\'' => State::FoundDonApostroph,
            State::FoundDonApostroph if c == 't' => State::FoundDont,

            State::FoundMul if c == '(' => State::FoundMulPar { first_num: 0 },
            State::FoundDo if c == '(' => State::FoundDoPar,
            State::FoundDont if c == '(' => State::FoundDontPar,
            State::FoundMulPar { first_num } if c == ',' => State::FoundMulParComma {
                first_num,
                second_num: 0,
            },
            State::FoundMulParComma {
                first_num,
                second_num,
            } if c == ')' => {
                // dbg!(active, first_num, second_num);
                if active || ignore_active {
                    result += first_num * second_num;
                };
                State::FoundNothing
            }
            State::FoundDoPar if c == ')' => {
                active = true;
                State::FoundNothing
            }
            State::FoundDontPar if c == ')' => {
                active = false;
                State::FoundNothing
            }
            State::FoundMulPar { first_num } if c.is_digit(RADIX) => State::FoundMulPar {
                first_num: first_num * RADIX + c.to_digit(RADIX).unwrap(),
            },
            State::FoundMulParComma {
                first_num,
                second_num,
            } if c.is_digit(RADIX) => State::FoundMulParComma {
                first_num,
                second_num: second_num * RADIX + c.to_digit(RADIX).unwrap(),
            },
            _ => State::FoundNothing,
        }
    });
    // dbg!(active);
    return result;
}

#[aoc(day3, part1)]
pub fn task1(input: &str) -> u32 {
    solve_line(input, true)
}
#[aoc(day3, part2)]
pub fn task2(input: &str) -> u32 {
    solve_line(input, false)
}
