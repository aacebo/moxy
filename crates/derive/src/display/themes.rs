use colored::CustomColor;

pub trait Theme {
    fn name(&self) -> CustomColor;
    fn field(&self) -> CustomColor;
    fn value(&self) -> CustomColor;
    fn punct(&self) -> CustomColor;
}

pub struct Dracula;
pub struct AtomOneDark;
pub struct GitHubDark;

impl Theme for Dracula {
    fn name(&self) -> CustomColor {
        CustomColor::new(139, 233, 253)
    }
    fn field(&self) -> CustomColor {
        CustomColor::new(255, 121, 198)
    }
    fn value(&self) -> CustomColor {
        CustomColor::new(241, 250, 140)
    }
    fn punct(&self) -> CustomColor {
        CustomColor::new(248, 248, 242)
    }
}

impl Theme for AtomOneDark {
    fn name(&self) -> CustomColor {
        CustomColor::new(230, 192, 123)
    }
    fn field(&self) -> CustomColor {
        CustomColor::new(198, 120, 221)
    }
    fn value(&self) -> CustomColor {
        CustomColor::new(152, 195, 121)
    }
    fn punct(&self) -> CustomColor {
        CustomColor::new(171, 178, 191)
    }
}

impl Theme for GitHubDark {
    fn name(&self) -> CustomColor {
        CustomColor::new(121, 192, 255)
    }
    fn field(&self) -> CustomColor {
        CustomColor::new(255, 123, 114)
    }
    fn value(&self) -> CustomColor {
        CustomColor::new(165, 214, 255)
    }
    fn punct(&self) -> CustomColor {
        CustomColor::new(201, 209, 217)
    }
}

pub fn get(name: &str) -> Box<dyn Theme> {
    match name {
        "atom-one-dark" => Box::new(AtomOneDark),
        "github-dark" => Box::new(GitHubDark),
        _ => Box::new(Dracula),
    }
}
