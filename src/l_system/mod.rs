pub mod bush;
pub mod corn;
pub mod cursor;
pub mod fern;
pub mod hilbert;
pub mod peano;
pub mod peano_gosper;
pub mod peano_variety;
pub mod tree;

use std::{collections::HashMap, time::Instant};

use nannou::{glam::Vec2, prelude::Update, App};

use crate::segment::Segment;

use self::cursor::Cursor;

/// Actions when reading the L-System
#[derive(Debug, Copy, Clone)]
pub enum Action {
    /// Do nothing
    None,
    /// Do nothing, but report that symbol isn't recognized
    Unknown,
    /// Custom action
    Custom(&'static str),
    /// Move the Cursor forward the specified distance
    MoveForward(f32),
    /// Move the Cursor forward and save a Segment representing a line between the positions to self.segments
    DrawForward(f32),
    /// Move the Cursor of the specificed location
    MoveTo(Vec2),
    /// Move the Cursor of the specificed location and save a Segment representing a line between the positions to self.segments
    DrawTo(Vec2),
    /// Rotate the Cursor by an angle given in radians
    RotateRad(f32),
    /// Rotate the Cursor by an angle given in degrees
    RotateDeg(f32),
    /// Set the Cursor angle to the given value, which is normalized automatically
    SetAngle(Vec2),
    /// Push a copy of the Cursor to self.cursors
    PushCursor,
    /// Pop the top item of self.cursors and replace the Cursor with it
    PopCursor,
    /// Save the position of the Cursor to self.positions
    PushPosition,
    /// Pop the top item of self.cursors and replace the Cursor's position with it
    PopPosition,
    /// Save the angle of the Cursor to self.angles
    PushAngle,
    /// Pop the top item of self.angles and replace the Cursor's angle with it
    PopAngle,
}

/// Interpret a sequence of symbols as actions in 2D space.
#[derive(Debug, Clone)]
pub struct SymbolReader<I: Iterator<Item = char>> {
    expression: I,
    actions: HashMap<char, Action>,
    pub segments: Vec<Segment>,
    pub cursors: Vec<Cursor>,
    pub positions: Vec<Vec2>,
    pub angles: Vec<Vec2>,
    pub cursor: Cursor,
}

impl<I: Iterator<Item = char>> SymbolReader<I> {
    pub fn new(expression: I, actions: HashMap<char, Action>, cursor: Cursor) -> Self {
        SymbolReader {
            expression,
            actions,
            segments: Vec::new(),
            cursors: Vec::new(),
            positions: Vec::new(),
            angles: Vec::new(),
            cursor,
        }
    }

    /// Read the next character of the expression, perform the corresponding action, and then report the action
    /// Returns None if the expression has been read completely
    pub fn step(&mut self) -> Option<Action> {
        if let Some(c) = self.expression.next() {
            if let Some(a) = self.actions.get(&c) {
                match a {
                    Action::DrawForward(dist) => {
                        let old_pos = self.cursor.position();
                        self.cursor.forward(*dist);
                        self.segments
                            .push(Segment::from((old_pos, self.cursor.position())));
                    }
                    Action::MoveForward(dist) => self.cursor.forward(*dist),
                    Action::DrawTo(pos) => {
                        let old_pos = self.cursor.position();
                        self.cursor.set_position(*pos);
                        self.segments.push(Segment::from((old_pos, *pos)));
                    }
                    Action::MoveTo(pos) => self.cursor.set_position(*pos),
                    Action::RotateRad(radians) => self.cursor.rotate(*radians),
                    Action::RotateDeg(degrees) => self.cursor.rotate_degrees(*degrees),
                    Action::SetAngle(angle) => self.cursor.set_angle(*angle),
                    Action::PushCursor => self.cursors.push(self.cursor),
                    Action::PopCursor => {
                        self.cursor = self
                            .cursors
                            .pop()
                            .expect("tried to pop from self.cursors when it was empty")
                    }
                    Action::PushPosition => self.positions.push(self.cursor.position()),
                    Action::PopPosition => self.cursor.set_position(
                        self.positions
                            .pop()
                            .expect("tried to pop from self.positions when it was empty"),
                    ),
                    Action::PushAngle => self.angles.push(self.cursor.angle()),
                    Action::PopAngle => self.cursor.set_angle(
                        self.angles
                            .pop()
                            .expect("tried to pop from self.angles when it was empty"),
                    ),
                    Action::None | Action::Unknown | Action::Custom(_) => (),
                }
                Some(*a)
            } else {
                Some(Action::Unknown)
            }
        } else {
            None
        }
    }
}

fn print_center<I: Iterator<Item = char>>(model: &mut SymbolReader<I>) {
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

pub fn steps<I: Iterator<Item = char>>(app: &App, model: &mut SymbolReader<I>, _update: Update) {
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

pub fn steps_then_quit<I: Iterator<Item = char>>(
    app: &App,
    model: &mut SymbolReader<I>,
    _update: Update,
) {
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

pub fn draw<I: Iterator<Item = char>>(app: &App, model: &mut SymbolReader<I>, _update: Update) {
    if app.keys.down.contains(&nannou::prelude::Key::C) {
        print_center(model)
    }
    loop {
        if model.step().is_none() {
            break;
        }
    }
}

pub fn timed<I: Iterator<Item = char>>(_app: &App, model: &mut SymbolReader<I>, _update: Update) {
    let t0 = Instant::now();
    loop {
        if model.step().is_none() {
            break;
        }
    }
    println!("{:?}", Instant::now() - t0);
}
