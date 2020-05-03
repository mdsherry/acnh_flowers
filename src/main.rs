mod flower;
mod genetics;
use flower::*;

fn main() {
    Pansy::red_from_seed().colour();
    Tulip::red_from_seed().colour();
    Rose::red_from_seed().colour();
}
