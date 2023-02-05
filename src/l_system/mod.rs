pub mod bush;
pub mod corn;
pub mod fern;
pub mod hilbert;
pub mod peano;
pub mod peano_gosper;
pub mod peano_variety;

use std::collections::HashMap;

use itertools::Itertools;
use nannou::{
    math::Vec2Rotate,
    prelude::{Update, Vec2},
    App,
};

use crate::segment::Segment;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cursor {
    position: Vec2,
    angle: Vec2,
}

impl Cursor {
    pub fn new(position: impl Into<Vec2>, angle: impl Into<Vec2>) -> Self {
        Cursor {
            position: Into::into(position),
            angle: Into::into(angle).normalize(),
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn angle(&self) -> Vec2 {
        self.angle
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position
    }

    pub fn set_angle(&mut self, angle: Vec2) {
        self.angle = angle
    }

    pub fn rotate(&mut self, radians: f32) {
        self.angle = self.angle.rotate(radians)
    }

    pub fn forward(&mut self, distance: f32) {
        self.position += self.angle * distance
    }
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

#[derive(Debug, Copy, Clone)]
pub enum Action {
    None,
    Forward(f32),
    Rotate(f32),
    Push,
    Pop,
}

pub struct LSystem {
    expression: Vec<char>,
    actions: HashMap<char, Action>,
    stack: Vec<Cursor>,
    segments: Vec<Segment>,
    cursor: Cursor,
}

impl LSystem {
    pub fn new(expression: Vec<char>, actions: HashMap<char, Action>, cursor: Cursor) -> Self {
        LSystem {
            expression,
            actions,
            stack: Vec::new(),
            segments: Vec::new(),
            cursor,
        }
    }

    pub fn step(&mut self) -> Option<Action> {
        if let Some(c) = self.expression.pop() {
            if let Some(a) = self.actions.get(&c) {
                match a {
                    Action::None => (),
                    Action::Forward(dist) => {
                        let mut new_cursor = self.cursor;
                        new_cursor.forward(*dist);
                        self.segments
                            .push(Segment::from((self.cursor.position, new_cursor.position)));
                        self.cursor = new_cursor;
                    }
                    Action::Rotate(angle) => self.cursor.rotate(*angle),
                    Action::Push => self.stack.push(self.cursor),
                    Action::Pop => self.cursor = self.stack.pop().expect("pop from empty stack"),
                }
                Some(*a)
            } else {
                panic!("unknown character encountered in expression: {c}")
            }
        } else {
            None
        }
    }
}

fn print_center(model: &mut LSystem) {
    let x = {
        let x_max = model
            .segments
            .iter()
            .map(|s| s.center().x)
            .reduce(f32::max)
            .unwrap();
        let x_min = model
            .segments
            .iter()
            .map(|s| s.center().x)
            .reduce(f32::min)
            .unwrap();
        (x_max + x_min) / 2.0
    };
    let y = {
        let y_max = model
            .segments
            .iter()
            .map(|s| s.center().y)
            .reduce(f32::max)
            .unwrap();
        let y_min = model
            .segments
            .iter()
            .map(|s| s.center().y)
            .reduce(f32::min)
            .unwrap();
        (y_max + y_min) / 2.0
    };
    println!("center: ({x},{y})");
}

pub fn steps(app: &App, model: &mut LSystem, _update: Update) {
    if app.keys.down.contains(&nannou::prelude::Key::C) {
        print_center(model)
    }
    // To save drawing time we will loop through action until reaching an Action::Forward
    loop {
        if let Some(a) = model.step() {
            match a {
                Action::Forward(_) => break,
                _ => (),
            }
        } else {
            break;
        }
    }
}

pub fn steps_then_quit(app: &App, model: &mut LSystem, _update: Update) {
    if app.keys.down.contains(&nannou::prelude::Key::C) {
        print_center(model)
    }
    loop {
        if let Some(a) = model.step() {
            match a {
                Action::Forward(_) => break,
                _ => (),
            }
        } else {
            app.quit();
            break;
        }
    }
}

pub fn draw(app: &App, model: &mut LSystem, _update: Update) {
    if app.keys.down.contains(&nannou::prelude::Key::C) {
        print_center(model)
    }
    loop {
        if model.step().is_none() {
            break;
        }
    }
}
