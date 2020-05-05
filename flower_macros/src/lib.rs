#[macro_export]
macro_rules! make_flower {
    (ident: $ty_name:ident
     genome: $genome:ident
     name: $str_name:literal
     colours: { $($c:ident)* }
     seeds: [
         $($colour:ident : $genes:ident ),*
     ]
     wild: [
        $($wild_colour:ident : $wild_genes:ident ),*
     ]
    ) => {
use crate::genetics::constants::*;
use crate::genetics::*;
use super::{Flower, Colour};
use $crate::flower_match;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct $ty_name {
    genome: $genome,
}

impl Flower for $ty_name {
    type GenomeType = $genome;

    fn colour(self) -> Colour {
        $crate::flower_match! {
            $($c)*
            : self.genome
        }
    }

    fn name(self) -> &'static str {
        $str_name
    }

    fn genome(self) -> Self::GenomeType {
        self.genome
    }

    fn from_genome(genome: Self::GenomeType) -> Self {
        Self { genome }
    }

    fn all_seeds() -> &'static [$ty_name] {
        ALL_SEEDS
    }
    
    fn all_wild() -> &'static [$ty_name] {
        ALL_WILD
    }
}

impl $ty_name {
    $(
        paste::item!{
            pub fn [<$colour:snake _from_seed>]() -> Self {
                $ty_name { genome: $genes }
            }
        }
    )*
    $(
        paste::item!{
            pub fn [<$wild_colour:snake _wild>]() -> Self {
                $ty_name { genome: $wild_genes }
            }
        }
    )*
}

static ALL_SEEDS: &'static [$ty_name] = &[
    $(
        $ty_name { genome: $genes }
    ),*
];

static ALL_WILD: &'static [$ty_name] = &[
    $(
        $ty_name { genome: $wild_genes }
    ),*
];

impl std::ops::Mul<Self> for $ty_name {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            genome: self.genome * other.genome,
        }
    }
}
impl std::fmt::Debug for $ty_name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.debug(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::flower::Colour;
    #[test]
    fn test_seed_colours() {
            $(
            paste::expr! {
                let val = $ty_name::[<$colour:snake _from_seed>]().colour();
                assert_eq!(Colour::$colour, val);
            }
        )*
    }
}

    };
}

#[macro_export]
macro_rules! flower_match {
    (yy $rr1:tt yy $rr2:tt yy $rr3:tt : $var:expr) => {
        flower_match!(r [0 $rr1] [1 $rr2] [2 $rr3] : $var)
    };
    ($(ww $ww1:tt ww $ww2:tt ww $ww3:tt)* : $var:expr) => {flower_match!($(yy [$ww1, $ww2, $ww3])* : $var)};
    
    ($($a:ident $b:ident $c:ident)* : $var:expr) => {flower_match!($(ww [$a, $b, $c])* : $var)};

    (r $([$r:literal [$y1:tt, $y2:tt, $y3: tt]])*  : $var:expr ) => {
        flower_match!(y $([$r 0 $y1] [$r 1 $y2] [$r 2 $y3])*  : $var)
    };
    (y $([$r:literal $y:literal [$w1:ident, $w2:ident, $w3: ident]])* : $var:expr) => {
        flower_match!(w $(
            [$r $y 0 $w1]
            [$r $y 1 $w2]
            [$r $y 2 $w3]
        )* : $var)
    };
    (w $([$r:literal $y:literal $w:literal $c:ident])* : $var:expr) => {
        match $var {
            $(_ if $var == paste::expr!(crate::genetics::constants::[<R $r Y $y W $w>]) => flower_match!($c),)*
            _ => unreachable!("No colour for {:?}", $var),
        }
    };
    ($colour:ident) => {crate::flower::Colour::$colour}
}

#[macro_export]
macro_rules! flower_match4 {
    (rr $rr1:tt rr $rr2:tt rr $rr3:tt : $var:expr) => {
        flower_match4!(r [0 $rr1] [1 $rr2] [2 $rr3] : $var)
    };
    ($(yy $yy1:tt yy $yy2:tt yy $yy3:tt)* : $var:expr) => {flower_match4!($(rr [$yy1, $yy2, $yy3])* : $var)};
    ($(ww $ww1:tt ww $ww2:tt ww $ww3:tt)* : $var:expr) => {flower_match4!($(yy [$ww1, $ww2, $ww3])* : $var)};
    
    ($($a:ident $b:ident $c:ident)* : $var:expr) => {flower_match4!($(ww [$a, $b, $c])* : $var)};

    (r $([$r:literal [$y1:tt, $y2:tt, $y3: tt]])*  : $var:expr ) => {
        flower_match4!(y $([$r 0 $y1] [$r 1 $y2] [$r 2 $y3])*  : $var)
    };
    (y $([$r:literal $y:literal [$w1:tt, $w2:tt, $w3: tt]])* : $var:expr) => {
        flower_match4!(w $(
            [$r $y 0 $w1]
            [$r $y 1 $w2]
            [$r $y 2 $w3]
        )* : $var)
    };
    (w $([$r:literal $y:literal $w:literal [$b1:ident, $b2:ident, $b3: ident]])* : $var:expr) => {
        flower_match4!(b $(
            [$r $y $w 0 $b1]
            [$r $y $w 1 $b2]
            [$r $y $w 2 $b3]
        )* : $var)
    };
    (b $([$r:literal $y:literal $w:literal $b:literal $c:ident])* : $var:expr) => {
        match $var {
            $(_ if $var == paste::expr!(crate::genetics::constants::[<R $r Y $y W $w B $b>]) => $crate::flower_match!($c),)*
            _ => unreachable!("No colour for {:?}", $var),
        }
    };
}

#[macro_export]
macro_rules! make_constants3 {
    () => {make_constants3!(r [0 1 2]);};
    (r [$($r:literal)*]) => { make_constants3!(y [$($r 0 $r 1 $r 2)*]);};
    (y [$($r:literal $y:literal)*]) => { make_constants3!(w [$($r $y 0 $r $y 1 $r $y 2)*]);};
    (w [$($r:literal $y:literal $w:literal)*]) => {
        $(
            paste::item!{
                pub const [< R $r Y $y W $w>]: Genome3 = paste::expr! {
                    gen3([<R $r>], [<Y $y>], [<W $w>])
                };
            }
        )*
    }
}

#[macro_export]
macro_rules! make_constants4 {
    () => {make_constants4!(r [0 1 2]);};
    (r [$($r:literal)*]) => { make_constants4!(y [$($r 0 $r 1 $r 2)*]);};
    (y [$($r:literal $y:literal)*]) => { make_constants4!(w [$($r $y 0 $r $y 1 $r $y 2)*]);};
    (w [$($r:literal $y:literal $w:literal)*]) => { make_constants4!(b [$($r $y $w 0 $r $y $w 1 $r $y $w 2)*]);};
    (b [$($r:literal $y:literal $w:literal $b:literal)*]) => {
        $(
            paste::item!{
                pub const [< R $r Y $y W $w B $b>]: Genome4 = paste::expr! {
                    gen4([<R $r>], [<Y $y>], [<W $w>], [<B $b>])
                };
            }
        )*
    }
}