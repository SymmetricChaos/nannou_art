pub mod bush;
pub mod corn;
pub mod cursor;
pub mod expression;
pub mod fern;
pub mod hilbert;
pub mod peano;
pub mod peano_gosper;
pub mod peano_variety;
pub mod tree;

use std::collections::HashMap;

use itertools::Itertools;
use nannou::{
    prelude::{Update, Vec2},
    rand::{seq::SliceRandom, thread_rng},
    App,
};

use crate::segment::Segment;

use self::{cursor::Cursor, expression::LSystemExpr};

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

pub fn build_epression_stochastic(
    axiom: String,
    rules: HashMap<char, Vec<(&str, f32)>>,
    depth: usize,
) -> Vec<char> {
    let mut rng = thread_rng();
    let mut expression = axiom;
    for _ in 0..depth {
        let mut new = String::new();
        for c in expression.chars() {
            if let Some(replacements) = rules.get(&c) {
                match replacements.choose_weighted(&mut rng, |item| item.1) {
                    Ok(s) => new.push_str(s.0),
                    Err(e) => panic!("{e}"),
                };
            } else {
                new.push(c)
            }
        }
        expression = new;
    }
    expression.chars().rev().collect_vec()
}

/// Actions when reading the L-System
#[derive(Debug, Copy, Clone)]
pub enum Action {
    /// Do nothing
    None,
    /// Move the Cursor forward
    MoveForward(f32),
    /// Move the Cursor forward and save a Segment representing a line between the positions to self.segments
    DrawForward(f32),
    /// Rotate the Cursor by an angle given in radians
    RotateRad(f32),
    /// Rotate the Cursor by an angle given in degrees
    RotateDeg(f32),
    /// Push a copy of the Cursor to the cursor stack
    PushCursor,
    /// Pop the top item of the cursor stack and replace the Cursor with it
    PopCursor,
    /// Save the position of the Cursor to self.dots
    Dot,
}

pub struct LSystem {
    expression: LSystemExpr,
    actions: HashMap<char, Action>,
    cursor_stack: Vec<Cursor>,
    pub segments: Vec<Segment>,
    pub dots: Vec<Vec2>,
    cursor: Cursor,
}

impl LSystem {
    pub fn new(expression: LSystemExpr, actions: HashMap<char, Action>, cursor: Cursor) -> Self {
        LSystem {
            expression,
            actions,
            cursor_stack: Vec::new(),
            segments: Vec::new(),
            dots: Vec::new(),
            cursor,
        }
    }

    /// Read the next character of th expression, performs the corresponding action, and then report the action
    /// Returns None if the expression has been read completely
    pub fn step(&mut self) -> Option<Action> {
        if let Some(c) = self.expression.next() {
            if let Some(a) = self.actions.get(&c) {
                match a {
                    Action::None => (),
                    Action::DrawForward(dist) => {
                        let mut new_cursor = self.cursor;
                        new_cursor.forward(*dist);
                        self.segments.push(Segment::from((
                            self.cursor.position(),
                            new_cursor.position(),
                        )));
                        self.cursor = new_cursor;
                    }
                    Action::MoveForward(dist) => self.cursor.forward(*dist),
                    Action::RotateRad(radians) => self.cursor.rotate(*radians),
                    Action::RotateDeg(degrees) => self.cursor.rotate_degrees(*degrees),
                    Action::PushCursor => self.cursor_stack.push(self.cursor),
                    Action::PopCursor => {
                        self.cursor = self.cursor_stack.pop().expect("pop from empty stack")
                    }
                    Action::Dot => self.dots.push(self.cursor.position()),
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
    loop {
        if let Some(a) = model.step() {
            match a {
                // To save drawing time we break only when reaching an Action that changes the image
                Action::DrawForward(_) => break,
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
                Action::DrawForward(_) => break,
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
