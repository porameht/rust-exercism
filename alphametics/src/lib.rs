use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let equation = parse(input);
    // Validate equation, "ABC + DEF == GH" is invalid, sum isn't wide enough
    let n = equation[0..equation.len() - 1]
        .iter()
        .map(|r| r.len())
        .reduce(max)
        .unwrap();
    if n > equation[equation.len() - 1].len() {
        return None;
    }
    // println!("{:?}", equation);
    let solution = &mut HashMap::new();
    if is_solvable(&equation, 0, 0, 0, solution) {
        Some(solution.clone())
    } else {
        None
    }
}

/* SEND + MORE = MONEY is parsed into:
 * [
 *   ['D', 'N', 'E', 'S'],
 *   ['E', 'R', 'O', 'M'],
 *   ['Y', 'E', 'N', 'O', 'M']
 * ]
 */
fn parse(s: &str) -> Vec<Vec<char>> {
    s.split_whitespace()
        .map(|x| x.trim().to_ascii_uppercase())
        .filter(|x| x.chars().all(|y| y.is_ascii_uppercase()))
        .map(|x| x.chars().rev().collect())
        .collect()
}

/*
 * Exit conditions:
 *   If we are beyond the leftmost digit of the sum:
 *     Return true if no carry, false otherwise.
 *     Also check that there is no leading zero in the sum.
 *   Else if addend and current column index is beyond the current row:
 *     Recur on row beneath this one.
 *
 * If we are currently trying to assign a char in one of the addends:
 *   If char already assigned, recur on row beneath this one.
 *   If not assigned, then:
 *     For every possible choice among the digits not in use:
 *       Make that choice and recur on row beneath this one.
 *         If successful, return true.
 *         Else, unmake assignment and try another digit.
 *     Return false if no assignment worked to trigger backtracking.
 *
 * Else if trying to assign a char in the sum:
 *   If char already assigned:
 *     If matches the sum digit, recur on next column to the left with carry.
 *     Else, return false to trigger backtracking.
 *   If char unassigned:
 *     If correct digit already used, return false.
 *     Else:
 *       Assign it and recur on next column to the left with carry:
 *         If successful return true.
 *         Else, unmake assignment, and return false to trigger backtracking.
 */
fn is_solvable(
    equation: &[Vec<char>],
    row: usize,
    col: usize,
    carry: u32,
    solution: &mut HashMap<char, u8>,
) -> bool {
    let is_addend = row < (equation.len() - 1);
    // println!(
    //     "row={}, col={}, carry={}, is_addend={}",
    //     row, col, carry, is_addend
    // );
    if col >= equation[row].len() && is_addend {
        return is_solvable(equation, row + 1, col, carry, solution);
    }
    if col == equation[row].len() && !is_addend {
        return carry == 0 && solution[&equation[row][col - 1]] > 0;
    }

    let digit: char = equation[row][col];
    let assigned = solution.contains_key(&digit);

    // let mut sol: Vec<_> = solution.iter().collect();
    // sol.sort_unstable_by_key(|e| e.0);
    // println!(
    //     "digit={}, assigned={}, solution={:?}",
    //     digit, assigned, sol
    // );

    if is_addend {
        if assigned {
            is_solvable(equation, row + 1, col, carry, solution)
        } else {
            let used: HashSet<&u8> = HashSet::from_iter(solution.values());
            let unused: Vec<u8> = (0..=9).filter(|x| !used.contains(x)).collect();
            for i in unused {
                solution.insert(digit, i);
                if is_solvable(equation, row + 1, col, carry, solution) {
                    return true;
                }
                solution.remove(&digit);
            }
            false
        }
    } else {
        let s: u32 = equation[0..equation.len() - 1]
            .iter()
            .filter(|r| col < r.len())
            .map(|r| solution[&r[col]] as u32)
            .sum::<u32>()
            + carry;
        let carry = s / 10;
        let sum_digit = (s % 10) as u8;
        // println!("s={}, sum_digit={}, carry = {}", s, sum_digit, carry);
        if assigned {
            (solution[&digit] == sum_digit) && is_solvable(equation, 0, col + 1, carry, solution)
        } else {
            let used = solution.values().any(|&x| x == sum_digit);
            if used {
                return false;
            }
            solution.insert(digit, sum_digit);
            if is_solvable(equation, 0, col + 1, carry, solution) {
                return true;
            }
            solution.remove(&digit);
            false
        }
    }
}