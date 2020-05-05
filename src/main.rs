mod flower;
mod genetics;
use flower::*;
use genetics::Genome3;

fn cross_breed<F: Flower + std::fmt::Debug>(left: F, right: F) {
    let kids = left.offspring(right).collect::<Vec<_>>();
    println!(
        "Expected off-spring from {:?} × {:?}: {:?}",
        left, right, kids
    );
}

fn coverage3<F>(flowers: &[F])
where
    F: Flower<GenomeType = Genome3>,
{
    let mut states = [false; 27];

    for flower in flowers {
        let offset = flower.genome().red().value() * 9
            + flower.genome().yellow().value() * 3
            + flower.genome().white().value();
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
}

fn main() {
    let mut gen = vec![
        Pansy::red_from_seed(),
        Pansy::yellow_from_seed(),
        Pansy::white_from_seed(),
    ];
    coverage3(&gen);
    gen = gen
        .iter()
        .copied()
        .flat_map(|l| gen.iter().copied().map(move |r| (l, r)))
        .flat_map(|(l, r)| l.offspring(r))
        .collect();
    coverage3(&gen);
    gen = gen
        .iter()
        .copied()
        .flat_map(|l| gen.iter().copied().map(move |r| (l, r)))
        .flat_map(|(l, r)| l.offspring(r))
        .collect();
    coverage3(&gen);
    gen.sort_unstable();
    gen.dedup();
    gen = gen
        .iter()
        .copied()
        .flat_map(|l| gen.iter().copied().map(move |r| (l, r)))
        .flat_map(|(l, r)| l.offspring(r))
        .collect();
    coverage3(&gen);
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
