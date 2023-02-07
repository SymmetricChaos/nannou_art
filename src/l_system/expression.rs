use std::collections::HashMap;

use itertools::Itertools;
use nannou::rand::seq::SliceRandom;
use nannou::rand::thread_rng;

pub fn write_expression(axiom: String, rules: HashMap<char, &str>, depth: usize) -> String {
    let mut expression = axiom;
    for _ in 0..depth {
        let mut new = String::new();
        for c in expression.chars() {
            if let Some(s) = rules.get(&c) {
                new.push_str(s)
            } else {
                new.push(c)
            }
        }
        expression = new;
    }
    expression
}

pub fn write_expression_stochastic(
    axiom: String,
    rules: HashMap<char, Vec<(&str, f32)>>,
    depth: usize,
) -> String {
    let mut expression = axiom;
    let mut rng = thread_rng();
    for _ in 0..depth {
        let mut new = String::new();
        for c in expression.chars() {
            if let Some(s) = rules.get(&c) {
                match s.choose_weighted(&mut rng, |item| item.1) {
                    Ok(s) => new.push_str(s.0),
                    Err(e) => panic!("{}", e.to_string()),
                }
            } else {
                new.push(c)
            }
        }
        expression = new;
    }
    expression
}

pub struct LSystemString {
    chars: Vec<char>,
}

impl LSystemString {
    pub fn new(axiom: String, rules: HashMap<char, &'static str>, depth: usize) -> Self {
        LSystemString {
            chars: write_expression(axiom, rules, depth)
                .chars()
                .rev()
                .collect_vec(),
        }
    }
}

impl Iterator for LSystemString {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.pop()
    }
}

pub struct LSystemStringStochastic {
    chars: Vec<char>,
}

impl LSystemStringStochastic {
    pub fn new(axiom: String, rules: HashMap<char, Vec<(&str, f32)>>, depth: usize) -> Self {
        LSystemStringStochastic {
            chars: write_expression_stochastic(axiom, rules, depth)
                .chars()
                .rev()
                .collect_vec(),
        }
    }
}

impl Iterator for LSystemStringStochastic {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.pop()
    }
}

pub struct LSystemBuilder {
    rules: HashMap<char, &'static str>,
    depth: usize,
    layers: Vec<Vec<char>>,
    ended: bool,
}

impl LSystemBuilder {
    pub fn new(axiom: String, rules: HashMap<char, &'static str>, depth: usize) -> Self {
        let mut layers = vec![Vec::<char>::new(); depth + 1];
        layers[depth] = axiom.chars().rev().collect_vec();

        Self {
            rules,
            depth,
            layers,
            ended: false,
        }
    }

    fn vec_from_rules(&self, c: &char) -> Vec<char> {
        if let Some(s) = self.rules.get(&c) {
            s.chars().rev().collect_vec()
        } else {
            vec![*c]
        }
    }
}

impl Iterator for LSystemBuilder {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ptr = 0_usize;

        loop {
            if ptr > self.depth {
                self.ended = true;
            }
            if self.ended {
                return None;
            }
            if ptr == 0 && self.layers[0].len() > 0 {
                return self.layers[0].pop();
            }
            if let Some(c) = self.layers[ptr].pop() {
                self.layers[ptr - 1] = self.vec_from_rules(&c);
                ptr -= 1
            } else {
                ptr += 1
            }
        }
    }
}

pub struct LSystemBuilderStochastic {
    rules: HashMap<char, Vec<(&'static str, f32)>>,
    depth: usize,
    layers: Vec<Vec<char>>,
    ended: bool,
}

impl LSystemBuilderStochastic {
    pub fn new(
        axiom: String,
        rules: HashMap<char, Vec<(&'static str, f32)>>,
        depth: usize,
    ) -> Self {
        let mut layers = vec![Vec::<char>::new(); depth + 1];
        layers[depth] = axiom.chars().rev().collect_vec();

        Self {
            rules,
            depth,
            layers,
            ended: false,
        }
    }

    fn vec_from_rules(&self, c: &char) -> Vec<char> {
        let mut rng = thread_rng();
        if let Some(s) = self.rules.get(&c) {
            match s.choose_weighted(&mut rng, |item| item.1) {
                Ok(s) => s.0.chars().rev().collect_vec(),
                Err(e) => panic!("{}", e.to_string()),
            }
        } else {
            vec![*c]
        }
    }
}

impl Iterator for LSystemBuilderStochastic {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ptr = 0_usize;

        loop {
            if ptr > self.depth {
                self.ended = true;
            }
            if self.ended {
                return None;
            }
            if ptr == 0 && self.layers[0].len() > 0 {
                return self.layers[0].pop();
            }
            if let Some(c) = self.layers[ptr].pop() {
                self.layers[ptr - 1] = self.vec_from_rules(&c);
                ptr -= 1
            } else {
                ptr += 1
            }
        }
    }
}

#[test]
fn expr_test() {
    let mut e = LSystemBuilder::new(
        String::from("X"),
        HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]),
        4,
    );

    let s = write_expression(
        String::from("X"),
        HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]),
        4,
    );

    println!("{s}\n");
    loop {
        if let Some(c) = e.next() {
            print!("{c}")
        } else {
            println!("");
            break;
        }
    }
}
