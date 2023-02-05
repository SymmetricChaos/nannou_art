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
    //println!("{expression}");
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

    pub fn forward(&mut self, dist: f32) {
        let mut new_cursor = self.cursor;
        new_cursor.forward(dist);
        self.segments
            .push(Segment::from((self.cursor.position, new_cursor.position)));
        self.cursor = new_cursor;
    }

    pub fn push_cursor(&mut self) {
        self.stack.push(self.cursor)
    }

    pub fn pop_cursor(&mut self) {
        self.cursor = self.stack.pop().expect("pop from empty stack")
    }

    pub fn rotate(&mut self, angle: f32) {
        self.cursor.rotate(angle)
    }
}

fn get_center(model: &mut LSystem) {
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

pub fn update(app: &App, model: &mut LSystem, _update: Update) {
    if app.keys.down.contains(&nannou::prelude::Key::C) {
        get_center(model)
    }
    // To save drawing time we will loop through action until reaching an Action::Forward
    loop {
        if let Some(c) = model.expression.pop() {
            if let Some(action) = model.actions.get(&c) {
                match action {
                    Action::None => (),
                    Action::Forward(dist) => {
                        model.forward(*dist);
                        break;
                    }
                    Action::Rotate(angle) => model.rotate(*angle),
                    Action::Push => model.push_cursor(),
                    Action::Pop => model.pop_cursor(),
                }
            } else {
                println!("uknown character encountered in expression: {c}");
                app.quit();
                break;
            }
        } else {
            //app.quit();
            break;
        }
    }
}

pub fn update_fast(app: &App, model: &mut LSystem, _update: Update) {
    if app.keys.down.contains(&nannou::prelude::Key::C) {
        get_center(model)
    }
    // To save drawing time we will loop through action until reaching an Action::Forward
    loop {
        if let Some(c) = model.expression.pop() {
            if let Some(action) = model.actions.get(&c) {
                match action {
                    Action::None => (),
                    Action::Forward(dist) => {
                        model.forward(*dist);
                        //break;
                    }
                    Action::Rotate(angle) => model.rotate(*angle),
                    Action::Push => model.push_cursor(),
                    Action::Pop => model.pop_cursor(),
                }
            } else {
                println!("uknown character encountered in expression: {c}");
                app.quit();
                break;
            }
        } else {
            //app.quit();
            break;
        }
    }
}
