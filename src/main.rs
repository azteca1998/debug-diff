#![feature(generators, generator_trait, iter_from_generator)]

use crate::diff::DiffPayload;
use chumsky::Parser;

mod ast;
mod diff;
mod parse;
mod util;

fn main() {
    let data_left = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let data_right = std::fs::read_to_string(std::env::args().nth(2).unwrap()).unwrap();

    let (ast_left, errors_left) = parse::parser().parse(&data_left).into_output_errors();
    let (ast_right, errors_right) = parse::parser().parse(&data_right).into_output_errors();

    match (&ast_left, errors_left.as_slice()) {
        (Some(_), &[]) => println!(" left: ok"),
        (None, x) if !x.is_empty() => println!(" left: err: {x:?}"),
        _ => todo!(),
    }
    match (&ast_right, errors_right.as_slice()) {
        (Some(_), &[]) => println!("right: ok"),
        (None, x) if !x.is_empty() => println!("right: err: {x:?}"),
        _ => todo!(),
    }

    let (ast_left, ast_right) = match (ast_left, ast_right) {
        (Some(x), Some(y)) => (x, y),
        _ => return,
    };

    let ast_diff = diff::find_diff(&ast_left, &ast_right);
    println!("Found {} differences.", ast_diff.len());
    println!();

    // println!(" left: {ast_left}");
    // println!();
    // println!("right: {ast_right}");
    // panic!();

    for diff in ast_diff {
        match diff.diff {
            DiffPayload::AstItem(l, r) => {
                // TODO: Improve type name.
                println!(
                    "  - Type mismatch: expected {r:?}, but got {l:?} at {:?}",
                    diff.path
                );
            }
            DiffPayload::ItemStruct(l, r) => {
                println!(
                    "  - Struct type mismatch: expected {}, but got {} at {:?}",
                    r.0, l.0, diff.path
                );
                if l.0 == r.0 {
                    println!("      Note: The internal structure differs.");
                }
            }
            DiffPayload::ItemTuple(l, r) => {
                println!(
                    "  - Tuple type mismatch: expected {} elements, but got {} at {:?}",
                    r.0.len(),
                    l.0.len(),
                    diff.path
                );
            }
            DiffPayload::ItemTupleStruct(l, r) => {
                println!(
                    "  - Tuple struct type mismatch: expected {}, but got {} at {:?}",
                    r.0, l.0, diff.path
                );
                if l.0 == r.0 {
                    println!("      Note: The internal structure differs.");
                }
            }
            DiffPayload::ItemUnitStruct(l, r) => {
                println!(
                    "  - Enum or type mismatch: expected {}, but got {} at {:?}",
                    r.0, l.0, diff.path
                );
            }
            DiffPayload::ValueBool(l, r) => {
                println!(
                    "  - Boolean mismatch: expected {}, but got {} at {:?}.",
                    r.0, l.0, diff.path
                );
            }
            DiffPayload::ValueNum(l, r) => {
                println!(
                    "  - Number mismatch: expected {}, but got {} at {:?}.",
                    r.0, l.0, diff.path
                );
            }
            DiffPayload::ValueStr(l, r) => {
                println!(
                    "  - String mismatch: expected \"{}\", but got \"{}\" at {:?}.",
                    r.0, l.0, diff.path
                );
            }
            DiffPayload::InsertedAt(i, r) => {
                println!(
                    "  - Source set (or option) is missing an item at index {i}: {r:?} at {:?}",
                    diff.path
                );
            }
            DiffPayload::RemovedAt(i, l) => {
                println!(
                    "  - Source set (or option) has an extra item at index {i}: {l:?} at {:?}",
                    diff.path
                );
            }
            DiffPayload::InsertedPair(k, r) => {
                println!("  - Source mapping is missing an entry at {:?}:", diff.path);
                println!("      Key  : {k}");
                println!("      Value: {r}");
            }
            DiffPayload::RemovedPair(k, l) => {
                println!("  - Source mapping has an extra entry at {:?}", diff.path);
                println!("      Key  : {k}");
                println!("      Value: {l}");
            }
        }
    }
}
