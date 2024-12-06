use std::fs::read_to_string;
use std::collections::HashMap;
use anyhow::{Result, bail};

pub fn problem1() -> Result<u32> {
    let mut ans = 0;

    'outer: for input_line in read_to_string("inputs/day2.txt")?.lines() {
        let input_vec: Vec<&str> = input_line.split_whitespace().collect();
        let mut report: Vec<i32> = vec![];

        for el in input_vec {
            report.push(el.parse()?);
        }

        if report.len() < 2 {
            ans += 1;
            continue;
        }

        let is_increasing = if report[0] < report[1] { true } else { false };
        for it in report.windows(2) {
            let delta = it[1] - it[0];
            if is_increasing {
                if delta < 1 || delta > 3 {
                    continue 'outer;
                }
            } else {
                if delta < -3 || delta > -1 {
                    continue 'outer;
                }
            }
        }

        ans += 1;
    }


    Ok(ans)
}

fn is_ok(is_increasing: bool, a: i32, b: i32) -> bool {
    let delta = b - a;
    if is_increasing {
        if delta < 1 || delta > 3 {
            return false;
        }
    } else {
        if delta < -3 || delta > -1 {
            return false;
        }
    }

    true
}

pub fn problem2() -> Result<u32> {
    let mut ans = 0;

    'outer: for input_line in read_to_string("inputs/day2.txt")?.lines() {
        let input_vec: Vec<&str> = input_line.split_whitespace().collect();
        let mut report: Vec<i32> = vec![];

        for el in input_vec {
            report.push(el.parse()?);
        }

        if report.len() < 2 {
            ans += 1;
            continue;
        }

        let is_increasing = if report[0] < report[1] { true } else { false };
        let mut did_remove = false;

        let mut it = report.iter_mut().peekable();
        while let Some(&mut val) = it.next() {
            let next_val = match it.peek_mut() {
                Some(val) => val.clone(),
                None => {
                    ans += 1;
                    break;
                }
            };

            if !is_ok(is_increasing, val, next_val) {
                if did_remove { continue 'outer; } else { did_remove = true; }

                it.next();
                let next_next_val = match it.peek_mut() {
                    Some(val) => val.clone(),
                    None => {
                        ans += 1;
                        break;
                    }
                };

                if !is_ok(is_increasing, val, next_next_val)
                    && !is_ok(is_increasing, next_val, next_next_val) {
                        continue 'outer;
                }
            }
        }
    }


    Ok(ans)

}

