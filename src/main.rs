mod flower;
mod genetics;
use flower::*;
use genetics::{Genome, Genome3};
mod visualization;
use visualization::PrintCoverage;

fn cross_breed<F: Flower + std::fmt::Debug>(left: F, right: F) {
    let kids = left.distinguishable_offspring(right).collect::<Vec<_>>();
    println!(
        "Expected off-spring from {:?} × {:?}: {:?}",
        left, right, kids
    );
}

fn main() {
    seeds();
    plant_gens(Rose::all_seeds().to_vec());
    let mut all_roses = Rose::all_seeds().to_vec();
    all_roses.extend(Rose::all_wild());
    plant_gens(all_roses);
    plant_gens(Tulip::all_seeds().to_vec());
    plant_gens(Lily::all_seeds().to_vec());
    plant_gens(Hyacinth::all_seeds().to_vec());
    plant_gens(Mum::all_seeds().to_vec());
    plant_gens(Cosmo::all_seeds().to_vec());
    plant_gens(Windflower::all_seeds().to_vec());
    plant_gens(Pansy::all_seeds().to_vec());
    
}

fn run_safe_generation<F: Flower + Ord>(gen: &[F]) -> Vec<F> {
    let mut next_gen: Vec<_> = gen
        .iter()
        .copied()
        .flat_map(|l| gen.iter().copied().map(move |r| (l, r)))
        .flat_map(|(l, r)| l.distinguishable_offspring(r))
        .collect();
    next_gen.extend(gen);
    next_gen.sort_unstable();
    next_gen.dedup();
    next_gen
}

fn run_yolo_generation<F: Flower + Ord>(gen: &[F]) -> Vec<F> {
    let mut next_gen: Vec<_> = gen
        .iter()
        .copied()
        .flat_map(|l| gen.iter().copied().map(move |r| (l, r)))
        .flat_map(|(l, r)| l.offspring(r))
        .collect();
    next_gen.extend(gen);
    next_gen.sort_unstable();
    next_gen.dedup();
    next_gen
}

fn plant_gens<F>(mut gen: Vec<F>) where
  F: Flower + Ord + Eq,
  F::GenomeType: PrintCoverage
  {
    println!("Running generations for {}", gen[0].name());
    let mut prev_gen = vec![];
    while gen != prev_gen {
        F::GenomeType::coverage(gen.iter().map(|c| c.genome()));
        prev_gen = run_safe_generation(&gen);
        std::mem::swap(&mut gen, &mut prev_gen);
    }
    let colours: std::collections::HashSet<_> = gen.iter().map(|f| f.colour()).collect();
    println!("Colours reached: {:?}", colours);
    let all_colours: std::collections::HashSet<_> = F::GenomeType::all_genomes().map(|g| F::from_genome(g).colour()).collect();
    let difference: std::collections::HashSet<_> = all_colours.difference(&colours).collect();
    println!("Colours missing: {:?}", difference);
    if !difference.is_empty() {
        println!("Missing colours, so running a YOLO generation");
        let final_gen = run_yolo_generation(&gen);
        let colours: std::collections::HashSet<_> = final_gen.iter().map(|f| f.colour()).collect();
        let difference: std::collections::HashSet<_> = all_colours.difference(&colours).collect();
        println!("Colours still missing: {:?}", difference);
    }
}

fn seeds() {
    for left in &[
        Tulip::red_from_seed(),
        Tulip::white_from_seed(),
        Tulip::yellow_from_seed(),
    ] {
        for right in &[
            Tulip::red_from_seed(),
            Tulip::white_from_seed(),
            Tulip::yellow_from_seed(),
        ] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[
        Rose::red_from_seed(),
        Rose::white_from_seed(),
        Rose::yellow_from_seed(),
    ] {
        for right in &[
            Rose::red_from_seed(),
            Rose::white_from_seed(),
            Rose::yellow_from_seed(),
        ] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[
        Pansy::red_from_seed(),
        Pansy::white_from_seed(),
        Pansy::yellow_from_seed(),
    ] {
        for right in &[
            Pansy::red_from_seed(),
            Pansy::white_from_seed(),
            Pansy::yellow_from_seed(),
        ] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[
        Windflower::red_from_seed(),
        Windflower::white_from_seed(),
        Windflower::orange_from_seed(),
    ] {
        for right in &[
            Windflower::red_from_seed(),
            Windflower::white_from_seed(),
            Windflower::orange_from_seed(),
        ] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[
        Lily::red_from_seed(),
        Lily::white_from_seed(),
        Lily::yellow_from_seed(),
    ] {
        for right in &[
            Lily::red_from_seed(),
            Lily::white_from_seed(),
            Lily::yellow_from_seed(),
        ] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[
        Cosmo::red_from_seed(),
        Cosmo::white_from_seed(),
        Cosmo::yellow_from_seed(),
    ] {
        for right in &[
            Cosmo::red_from_seed(),
            Cosmo::white_from_seed(),
            Cosmo::yellow_from_seed(),
        ] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[
        Mum::red_from_seed(),
        Mum::white_from_seed(),
        Mum::yellow_from_seed(),
    ] {
        for right in &[
            Mum::red_from_seed(),
            Mum::white_from_seed(),
            Mum::yellow_from_seed(),
        ] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[
        Hyacinth::red_from_seed(),
        Hyacinth::white_from_seed(),
        Hyacinth::yellow_from_seed(),
    ] {
        for right in &[
            Hyacinth::red_from_seed(),
            Hyacinth::white_from_seed(),
            Hyacinth::yellow_from_seed(),
        ] {
            cross_breed(*left, *right);
        }
    }
}
