#[derive(PartialEq, Eq, Debug)]
pub enum Element {
    Digit(i32),
    Array(Vec<Element>),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Element::Digit(l), Element::Digit(r)) => l.cmp(r),
            (Element::Digit(l), Element::Array(r)) => vec![Element::Digit(*l)].cmp(r),
            (Element::Array(l), Element::Digit(r)) => l.cmp(&vec![Element::Digit(*r)]),
            (Element::Array(l), Element::Array(r)) => l.cmp(r),
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
