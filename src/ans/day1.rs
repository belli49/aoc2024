use std::fs::read_to_string;
use std::collections::HashMap;
use anyhow::{Result, bail};

pub fn problem1() -> Result<u32> {
    let mut ans = 0;
    let mut a_list: Vec<u32> = vec![];
    let mut b_list: Vec<u32> = vec![];

    for input_line in read_to_string("inputs/day1.txt")?.lines() {
        let elements: Vec<&str> = input_line.split_whitespace().collect();

        match elements[..] {
            [a, b] => {
                a_list.push(a.parse::<u32>()?);
                b_list.push(b.parse::<u32>()?);
            }

            _ => bail!("Unexpected format!"),
        }
    }

    a_list.sort();
    b_list.sort();

    for (a, b) in a_list.iter().zip(b_list.iter()) {
        ans += a.abs_diff(*b);
    }

    Ok(ans)
}

pub fn problem2() -> Result<u32> {
    let mut ans = 0;
    let mut a_list: Vec<u32> = vec![];
    let mut b_map = HashMap::<u32, u32>::new();

    for input_line in read_to_string("inputs/day1.txt")?.lines() {
        let elements: Vec<&str> = input_line.split_whitespace().collect();

        match elements[..] {
            [a, b] => {
                a_list.push(a.parse::<u32>()?);

                let b_val = b.parse::<u32>()?;
                match b_map.get_mut(&b_val) {
                    Some(val) => *val += 1,
                    None => { b_map.insert(b_val, 1); }
                }
            }

            _ => bail!("Unexpected format!"),
        }
    }


    for a in a_list {
        ans += match b_map.get(&a) {
            Some(val) => a * val,
            None => 0,
        }
    }

    Ok(ans)
}

