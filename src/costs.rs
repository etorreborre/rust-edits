use Cost::*;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Cost {
    Insertion(usize),
    Deletion(usize),
    Substitution(usize),
    NoAction(usize),
}

// Don't prefix Cost variants

impl Cost {
    pub fn cost(self) -> usize {
        match self {
            Insertion(c) => c,
            Deletion(c) => c,
            Substitution(c) => c,
            NoAction(c) => c,
        }
    }
}

pub fn show_cost(c: &Cost) -> String {
    match c {
        Insertion(c) => format!("{}{}", "+ ", c),
        Deletion(c) => format!("{}{}", "- ", c),
        Substitution(c) => format!("{}{}", "~ ", c),
        NoAction(c) => format!("{}{}", "o ", c),
    }
}

// This component contains functions to evaluate the cost of
// substituting, inserting, deleting an element
pub trait Costs<T>: Copy {
    fn insertion_cost(self, t: &T) -> usize;
    fn deletion_cost(self, t: &T) -> usize;
    fn substitution_cost(self, t1: &T, t2: &T) -> usize;
    fn lower_cost(self, t1: &T, t2: &T, del: usize, sub: usize, ins: usize) -> Cost;
}

// Implementation of the Costs trait for the Levenshtein distance

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct LevenshteinCosts {}

impl Costs<char> for LevenshteinCosts {
    fn insertion_cost(self, _t: &char) -> usize {
        1
    }
    fn deletion_cost(self, _t: &char) -> usize {
        1
    }
    fn substitution_cost(self, t1: &char, t2: &char) -> usize {
        if t1 == t2 {
            0
        } else {
            1
        }
    }
    fn lower_cost(self, t1: &char, t2: &char, del: usize, sub: usize, ins: usize) -> Cost {
        let (op_del, op_sub, op_ins) = (Deletion(del), Substitution(sub), Insertion(ins));
        if ins < del {
            if (ins < sub) || (ins == sub && t1 == t2) {
                op_ins
            } else {
                op_sub
            }
        } else if (del < sub) || (del == sub && t1 == t2) {
            op_del
        } else {
            op_sub
        }
    }
}

pub fn levenshtein_costs() -> LevenshteinCosts {
    LevenshteinCosts {}
}
