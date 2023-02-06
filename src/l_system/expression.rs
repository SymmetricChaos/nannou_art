use std::collections::HashMap;

use itertools::Itertools;

pub struct LSystemExpr {
    rules: HashMap<char, &'static str>,
    depth: usize,
    layers: Vec<Vec<char>>,
    ended: bool,
}

pub fn build_epression(axiom: String, rules: HashMap<char, &str>, depth: usize) -> Vec<char> {
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
    expression.chars().rev().collect_vec()
}

impl LSystemExpr {
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

    pub fn next(&mut self) -> Option<char> {
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
    let mut e = LSystemExpr::new(
        String::from("X"),
        HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]),
        4,
    );

    let s = super::write_expression(
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
