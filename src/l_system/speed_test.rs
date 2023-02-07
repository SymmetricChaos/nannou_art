use crate::l_system::expression::write_expression;

#[test]
pub fn string_model() {
    use super::{cursor::Cursor, expression::LSystemString, LSystem};
    use crate::l_system::{expression::LSystemExpr, Action};
    use std::{collections::HashMap, time::Instant};

    let expression_string = LSystemString::new(
        String::from("X"),
        HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]),
        12,
    );

    let bytes_used = write_expression(
        String::from("X"),
        HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]),
        12,
    )
    .chars()
    .count()
        * 4;

    let expression_struct = LSystemExpr::new(
        String::from("X"),
        HashMap::from([('X', "F[X][+DX]-DX"), ('D', "F")]),
        12,
    );

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
    let mut system = LSystem::new(expression_string, actions.clone(), cursor);
    let t0 = Instant::now();
    loop {
        if system.step().is_none() {
            break;
        }
    }
    println!("Vec<char>: {:?}", Instant::now() - t0);
    println!("using {} bytes", bytes_used);

    let cursor = Cursor::new((0.0, 0.0), (0.0, 1.0));
    let mut system = LSystem::new(expression_struct, actions, cursor);
    let t0 = Instant::now();
    loop {
        if system.step().is_none() {
            break;
        }
    }
    println!("Memory Efficient Struct: {:?}", Instant::now() - t0);
}
