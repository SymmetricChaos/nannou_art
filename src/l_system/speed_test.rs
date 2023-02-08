#[test]
pub fn string_model() {
    use super::{cursor::Cursor, expression::LSystemString, LSystem};
    use crate::l_system::{expression::LSystemBuilder, Action};
    use std::{collections::HashMap, time::Instant};

    let axiom = String::from("X");
    let rules = HashMap::from([
        ('X', "F[X][+DX]-DX"),
        ('D', "F"),
        ('F', "F"),
        ('+', "+"),
        ('-', "-"),
        ('[', "["),
        (']', "]"),
    ]);
    let depth = 13_usize;

    let expression_string = LSystemString::new(axiom.clone(), rules.clone(), depth);

    let bytes_used =
        crate::l_system::expression::write_expression(axiom.clone(), rules.clone(), depth)
            .chars()
            .count()
            * 4;

    let expression_struct = LSystemBuilder::new("X", rules.clone(), depth);

    let actions = HashMap::from([
        ('F', Action::DrawForward(15.0)),
        ('X', Action::None),
        ('D', Action::Dot),
        ('+', Action::RotateRad(-1.04)),
        ('-', Action::RotateRad(1.04)),
        ('[', Action::PushCursor),
        (']', Action::PopCursor),
    ]);

    let cursor = Cursor::new((0.0, 0.0), (0.0, 1.0));
    let mut system = LSystem::new(Box::new(expression_string), actions.clone(), cursor);
    let t0 = Instant::now();
    loop {
        if system.step().is_none() {
            break;
        }
    }
    println!("Vec<char>: {:?}", Instant::now() - t0);
    println!("using {} bytes", bytes_used);

    let cursor = Cursor::new((0.0, 0.0), (0.0, 1.0));
    let mut system = LSystem::new(Box::new(expression_struct), actions, cursor);
    let t0 = Instant::now();
    loop {
        if system.step().is_none() {
            break;
        }
    }
    println!("Memory Efficient Struct: {:?}", Instant::now() - t0);
    println!("using ~200 bytes (roughly k*n where k is the widest replacement and n in the recursion depth)");
}
