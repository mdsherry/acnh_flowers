mod flower;
mod genetics;
use flower::*;

fn cross_breed<F: Flower + std::fmt::Debug>(left: F, right: F) {
    let kids = left
        .offspring(right)
        .collect::<Vec<_>>();
    println!("Expected off-spring from {:?} × {:?}: {:?}", left, right, kids);
}

fn main() {
    for left in &[Tulip::red_from_seed(), Tulip::white_from_seed(), Tulip::yellow_from_seed()] {
        for right in &[Tulip::red_from_seed(), Tulip::white_from_seed(), Tulip::yellow_from_seed()] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[Rose::red_from_seed(), Rose::white_from_seed(), Rose::yellow_from_seed()] {
        for right in &[Rose::red_from_seed(), Rose::white_from_seed(), Rose::yellow_from_seed()] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[Pansy::red_from_seed(), Pansy::white_from_seed(), Pansy::yellow_from_seed()] {
        for right in &[Pansy::red_from_seed(), Pansy::white_from_seed(), Pansy::yellow_from_seed()] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[Windflower::red_from_seed(), Windflower::white_from_seed(), Windflower::orange_from_seed()] {
        for right in &[Windflower::red_from_seed(), Windflower::white_from_seed(), Windflower::orange_from_seed()] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[Lily::red_from_seed(), Lily::white_from_seed(), Lily::yellow_from_seed()] {
        for right in &[Lily::red_from_seed(), Lily::white_from_seed(), Lily::yellow_from_seed()] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[Cosmo::red_from_seed(), Cosmo::white_from_seed(), Cosmo::yellow_from_seed()] {
        for right in &[Cosmo::red_from_seed(), Cosmo::white_from_seed(), Cosmo::yellow_from_seed()] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[Mum::red_from_seed(), Mum::white_from_seed(), Mum::yellow_from_seed()] {
        for right in &[Mum::red_from_seed(), Mum::white_from_seed(), Mum::yellow_from_seed()] {
            cross_breed(*left, *right);
        }
    }
    println!();
    for left in &[Hyacinth::red_from_seed(), Hyacinth::white_from_seed(), Hyacinth::yellow_from_seed()] {
        for right in &[Hyacinth::red_from_seed(), Hyacinth::white_from_seed(), Hyacinth::yellow_from_seed()] {
            cross_breed(*left, *right);
        }
    }
}
