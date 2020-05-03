#![allow(dead_code)]

pub fn compare_tables(left: &str, right: &str) {
    let mut buf = String::new();
    let mut lbuf = String::new();
    let mut rbuf = String::new();
    let mut cbuf = String::new();
    let mut mistakes = false;

    for (l, r) in left.chars().zip(right.chars()) {
        if l == '\n' {
            assert!(r == '\n', "Tables are different widths?");
            buf.push_str(&lbuf);
            buf.push_str("  ");
            buf.push_str(&rbuf);
            buf.push_str("  ");
            buf.push_str(&cbuf);
            buf.push('\n');
            lbuf.clear();
            rbuf.clear();
            cbuf.clear();
        } else {
            lbuf.push(l);
            rbuf.push(r);
            cbuf.push(if l == r {
                '.'
            } else {
                mistakes = true;
                'X'
            });
        }
    }
    if mistakes {
        println!("{}", buf);
        panic!("Tables don't match");
    }
}

pub fn build_matcher3(table: &str) {
    let mut chars = table.chars().filter(|&c| c != '\n');
    let mut codes: [Vec<_>; 9] = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    for r in 0..=2 {
        for y in 0..=2 {
            for w in 0..=2 {
                let c = chars.next().expect("Too few entries in table");
                let colour = letter_to_colour(c);
                codes[colour as usize].push(format!("R{}Y{}W{}", r, y, w));
            }
        }
    }
    for colour in &COLOURS {
        let codess = &codes[*colour as usize];
        if codess.is_empty() {
            continue;
        } else {
            println!("{} => \"{}\",", codess.join(" | "), colour.name());
        }
    }
}

fn letter_to_colour(c: char) -> Colour {
    match c {
        'R' => Colour::Red,
        'W' => Colour::White,
        'P' => Colour::Pink,
        'B' => Colour::Blue,
        'K' => Colour::Black,
        'Y' => Colour::Yellow,
        'O' => Colour::Orange,
        'U' => Colour::Purple,
        'G' => Colour::Green,
        _ => unreachable!("Invalid colour {}", c),
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Colour {
    White = 0,
    Yellow = 1,
    Red = 2,
    Orange = 3,
    Pink = 4,
    Purple = 5,
    Blue = 6,
    Black = 7,
    Green = 8,
}

static COLOURS: [Colour; 9] = [
    Colour::White,
    Colour::Yellow,
    Colour::Red,
    Colour::Orange,
    Colour::Pink,
    Colour::Purple,
    Colour::Blue,
    Colour::Black,
    Colour::Green,
];

impl Colour {
    fn name(self) -> &'static str {
        match self {
            Colour::White => "White",
            Colour::Yellow => "Yellow",
            Colour::Red => "Red",
            Colour::Orange => "Orange",
            Colour::Pink => "Pink",
            Colour::Purple => "Purple",
            Colour::Blue => "Blue",
            Colour::Black => "Black",
            Colour::Green => "Green",
        }
    }
}