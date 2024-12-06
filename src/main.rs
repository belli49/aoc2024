use anyhow::Result;

mod ans;

fn main() -> Result<()> {
    println!("Ans: {}", ans::day2::problem1()?);
    println!("Ans: {}", ans::day2::problem2()?);

    Ok(())
}
