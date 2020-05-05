use crate::genetics::{Genome3, Genome4};
pub trait PrintCoverage: Sized {
    fn coverage(genomes: impl Iterator<Item=Self>);
}

impl PrintCoverage for Genome3 {
    fn coverage(genomes: impl Iterator<Item=Self>) {
        let mut states = [false; 27];

        for genome in genomes {
            let offset = genome.red().value() * 9
                + genome.yellow().value() * 3
                + genome.white().value();
            states[offset as usize] = true;
        }

        for w in 0..=2usize {
            for r in 0..=2 {
                for y in 0..=2 {
                    let offset = r * 9 + y * 3 + w;
                    if states[offset] {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                print!(" ");
            }
            println!();
        }
        println!();
    }
}
impl PrintCoverage for Genome4 {
    fn coverage(genomes: impl Iterator<Item=Self>) {
        let mut states = [false; 81];

        for genome in genomes {
            let offset = genome.red().value() * 27
                + genome.yellow().value() * 9
                + genome.white().value() * 3
                + genome.blue().value();
            states[offset as usize] = true;
        }

        for b in 0..=2 {
            for w in 0..=2usize {
                for r in 0..=2 {
                    for y in 0..=2 {
                        let offset = r * 27 + y * 9 + w * 3 + b;
                        if states[offset] {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    print!(" ");
                }
                println!();
            }
            println!();
        }
        println!();
    
    }
}
