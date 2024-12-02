use anyhow::Result;

mod ans;

fn main() -> Result<()> {
    println!("Ans: {}", ans::day1::problem1()?);
    println!("Ans: {}", ans::day1::problem2()?);

    Ok(())
}
