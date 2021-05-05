use anyhow::Result;

mod choice;

fn main() -> Result<()> {
    let mut set = crate::choice::Choice::collect()?;
    loop {
        set.roulette()?;
    }
}
