use std::collections::HashSet;
use rand::{Rng, prelude::ThreadRng};
use std::io::{self, Write};
use anyhow::Result;

#[derive(Debug)]
pub struct Choice (HashSet::<String>, ThreadRng);

impl Choice {
    fn push(&mut self, choice: String) -> bool {
        self.0.insert(choice)
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn pull(&mut self, index: usize) -> String {
        let mut count = 0;
        for e in &self.0 {
            if count == index {
                return e.clone();
            }
            count += 1;
        }
        panic!("index is over length");
    }
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        rng.gen::<i32>();
        Choice(HashSet::new(), rng)
    }
    pub fn collect() -> Result<Self> {
        let mut set = Self::new();
        loop {
            print!("Input choice or the same: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.pop();
            if !set.push(input) {
                return Ok(set)
            }

        }
    }
    pub fn roulette(&mut self) -> Result<()>{
        print!("Roulette > (Enter)");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let index = self.1.gen::<usize>() % self.len();
        println!("\nYour choise is .... {}!\n", self.pull(index));
        Ok(())
    }
}
