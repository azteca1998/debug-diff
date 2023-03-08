use crate::ast::*;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DiffItem<'a, 'b> {
    pub path: DiffPath<'a, 'b>,
    pub diff: DiffPayload<'a, 'b>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DiffPathSegment<'a, 'b> {
    Field(&'a str),
    Index(usize, usize),
    Key(&'b AstItem<'a>),
}

pub type DiffPath<'a, 'b> = Vec<DiffPathSegment<'a, 'b>>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DiffPayload<'a, 'b> {
    AstItem(&'b AstItem<'a>, &'b AstItem<'a>),
    // ...
    ItemStruct(&'b ItemStruct<'a>, &'b ItemStruct<'a>),
    ItemTuple(&'b ItemTuple<'a>, &'b ItemTuple<'a>),
    ItemTupleStruct(&'b ItemTupleStruct<'a>, &'b ItemTupleStruct<'a>),
    ItemUnitStruct(&'b ItemUnitStruct<'a>, &'b ItemUnitStruct<'a>),
    ValueBool(&'b ValueBool, &'b ValueBool),
    ValueNum(&'b ValueNum<'a>, &'b ValueNum<'a>),
    ValueStr(&'b ValueStr<'a>, &'b ValueStr<'a>),

    /// The value was removed (not present on the right).
    InsertedAt(usize, &'b AstItem<'a>),
    /// The value was inserted (not present on the left).
    RemovedAt(usize, &'b AstItem<'a>),
    /// The value was removed (not present on the right).
    InsertedPair(&'b AstItem<'a>, &'b AstItem<'a>),
    /// The value was inserted (not present on the left).
    RemovedPair(&'b AstItem<'a>, &'b AstItem<'a>),
}

pub fn find_diff<'a, 'b>(lhs: &'b AstItem<'a>, rhs: &'b AstItem<'a>) -> Vec<DiffItem<'a, 'b>> {
    let mut target = Vec::new();
    let mut stack = Vec::new();

    diff_ast(&mut target, &mut stack, lhs, rhs);

    target
}

fn diff_ast<'a, 'b>(
    target: &mut Vec<DiffItem<'a, 'b>>,
    stack: &mut DiffPath<'a, 'b>,
    lhs: &'b AstItem<'a>,
    rhs: &'b AstItem<'a>,
) {
    match (lhs, rhs) {
        (AstItem::ItemList(lhs), AstItem::ItemList(rhs)) => diff_item_list(target, stack, lhs, rhs),
        (AstItem::ItemMap(lhs), AstItem::ItemMap(rhs)) => diff_item_map(target, stack, lhs, rhs),
        (AstItem::ItemOption(lhs), AstItem::ItemOption(rhs)) => {
            diff_item_option(target, stack, lhs, rhs)
        }
        (AstItem::ItemSet(lhs), AstItem::ItemSet(rhs)) => diff_item_set(target, stack, lhs, rhs),
        (AstItem::ItemStruct(lhs), AstItem::ItemStruct(rhs)) => {
            diff_item_struct(target, stack, lhs, rhs)
        }
        (AstItem::ItemTuple(lhs), AstItem::ItemTuple(rhs)) => {
            diff_item_tuple(target, stack, lhs, rhs)
        }
        (AstItem::ItemTupleStruct(lhs), AstItem::ItemTupleStruct(rhs)) => {
            diff_item_tuple_struct(target, stack, lhs, rhs)
        }
        (AstItem::ItemUnitStruct(lhs), AstItem::ItemUnitStruct(rhs)) => {
            diff_item_unit_struct(target, stack, lhs, rhs)
        }
        (AstItem::ValueBool(lhs), AstItem::ValueBool(rhs)) => {
            diff_value_bool(target, stack, lhs, rhs)
        }
        (AstItem::ValueNum(lhs), AstItem::ValueNum(rhs)) => diff_value_num(target, stack, lhs, rhs),
        (AstItem::ValueStr(lhs), AstItem::ValueStr(rhs)) => diff_value_str(target, stack, lhs, rhs),
        _ => target.push(DiffItem {
            path: stack.clone(),
            diff: DiffPayload::AstItem(lhs, rhs),
        }),
    }
}

fn diff_item_list<'a, 'b>(
    target: &mut Vec<DiffItem<'a, 'b>>,
    stack: &mut DiffPath<'a, 'b>,
    lhs: &'b ItemList<'a>,
    rhs: &'b ItemList<'a>,
) {
    let mut lhs_iter = lhs.0.iter().peekable();
    let mut rhs_iter = rhs.0.iter().peekable();

    let mut count = 0;
    loop {
        match (lhs_iter.peek(), rhs_iter.peek()) {
            (Some(lhs), Some(rhs)) => match lhs.cmp(rhs) {
                Ordering::Equal => {
                    lhs_iter.next();
                    rhs_iter.next();
                }
                Ordering::Less => {
                    target.push(DiffItem {
                        path: stack.clone(),
                        diff: DiffPayload::RemovedAt(count, lhs),
                    });
                    lhs_iter.next();
                }
                Ordering::Greater => {
                    target.push(DiffItem {
                        path: stack.clone(),
                        diff: DiffPayload::RemovedAt(count, rhs),
                    });
                    rhs_iter.next();
                }
            },
            (Some(lhs), None) => {
                target.push(DiffItem {
                    path: stack.clone(),
                    diff: DiffPayload::RemovedAt(count, lhs),
                });
                lhs_iter.next();
            }
            (None, Some(rhs)) => {
                target.push(DiffItem {
                    path: stack.clone(),
                    diff: DiffPayload::RemovedAt(count, rhs),
                });
                rhs_iter.next();
            }
            (None, None) => break,
        }

        count += 1;
    }
}

fn diff_item_map<'a, 'b>(
    target: &mut Vec<DiffItem<'a, 'b>>,
    stack: &mut DiffPath<'a, 'b>,
    lhs: &'b ItemMap<'a>,
    rhs: &'b ItemMap<'a>,
) {
    let mut lhs_iter = lhs.0.iter().peekable();
    let mut rhs_iter = rhs.0.iter().peekable();

    loop {
        match (lhs_iter.peek(), rhs_iter.peek()) {
            (Some((lhs_key, lhs_val)), Some((rhs_key, rhs_val))) => {
                match lhs_key.cmp(rhs_key) {
                    Ordering::Equal => {
                        stack.push(DiffPathSegment::Key(lhs_key));
                        diff_ast(target, stack, lhs_val, rhs_val);
                        stack.pop();

                        lhs_iter.next();
                        rhs_iter.next();
                    }
                    Ordering::Less => {
                        target.push(DiffItem {
                            path: stack.clone(),
                            diff: DiffPayload::RemovedPair(lhs_key, lhs_val),
                        });
                        lhs_iter.next();
                    }
                    Ordering::Greater => {
                        target.push(DiffItem {
                            path: stack.clone(),
                            diff: DiffPayload::InsertedPair(rhs_key, rhs_val),
                        });
                        rhs_iter.next();
                    }
                }
            },
            (Some((lhs_key, lhs_val)), None) => {
                target.push(DiffItem {
                    path: stack.clone(),
                    diff: DiffPayload::RemovedPair(lhs_key, lhs_val),
                });
                lhs_iter.next();
            }
            (None, Some((rhs_key, rhs_val))) => {
                target.push(DiffItem {
                    path: stack.clone(),
                    diff: DiffPayload::InsertedPair(rhs_key, rhs_val),
                });
                rhs_iter.next();
            }
            (None, None) => break,
        }
    }
}

fn diff_item_option<'a, 'b>(
    target: &mut Vec<DiffItem<'a, 'b>>,
    stack: &mut DiffPath<'a, 'b>,
    lhs: &'b ItemOption<'a>,
    rhs: &'b ItemOption<'a>,
) {
    match (lhs.0.as_deref(), rhs.0.as_deref()) {
        (Some(lhs), Some(rhs)) => diff_ast(target, stack, lhs, rhs),
        (Some(lhs), None) => target.push(DiffItem {
            path: stack.clone(),
            diff: DiffPayload::RemovedAt(0, lhs),
        }),
        (None, Some(rhs)) => target.push(DiffItem {
            path: stack.clone(),
            diff: DiffPayload::InsertedAt(0, rhs),
        }),
        (None, None) => {}
    }
}

fn diff_item_set<'a, 'b>(
    target: &mut Vec<DiffItem<'a, 'b>>,
    stack: &mut DiffPath<'a, 'b>,
    lhs: &'b ItemSet<'a>,
    rhs: &'b ItemSet<'a>,
) {
    let mut lhs_iter = lhs.0.iter().peekable();
    let mut rhs_iter = rhs.0.iter().peekable();

    let mut count = 0;
    loop {
        match (lhs_iter.peek(), rhs_iter.peek()) {
            (Some(lhs), Some(rhs)) => match lhs.cmp(rhs) {
                Ordering::Equal => {
                    lhs_iter.next();
                    rhs_iter.next();
                }
                Ordering::Less => {
                    target.push(DiffItem {
                        path: stack.clone(),
                        diff: DiffPayload::RemovedAt(count, lhs),
                    });
                    lhs_iter.next();
                }
                Ordering::Greater => {
                    target.push(DiffItem {
                        path: stack.clone(),
                        diff: DiffPayload::InsertedAt(count, rhs),
                    });
                    rhs_iter.next();
                }
            },
            (Some(lhs), None) => {
                target.push(DiffItem {
                    path: stack.clone(),
                    diff: DiffPayload::RemovedAt(count, lhs),
                });
                lhs_iter.next();
            }
            (None, Some(rhs)) => {
                target.push(DiffItem {
                    path: stack.clone(),
                    diff: DiffPayload::InsertedAt(count, rhs),
                });
                rhs_iter.next();
            }
            (None, None) => break,
        }

        count += 1;
    }
}

fn diff_item_struct<'a, 'b>(
    target: &mut Vec<DiffItem<'a, 'b>>,
    stack: &mut DiffPath<'a, 'b>,
    lhs: &'b ItemStruct<'a>,
    rhs: &'b ItemStruct<'a>,
) {
    if lhs.0 != rhs.0 || lhs.1.keys().zip(rhs.1.keys()).any(|(a, b)| a != b) {
        target.push(DiffItem {
            path: stack.clone(),
            diff: DiffPayload::ItemStruct(lhs, rhs),
        });
    } else {
        for ((k, lhs), (_, rhs)) in lhs.1.iter().zip(rhs.1.iter()) {
            stack.push(DiffPathSegment::Field(k));
            diff_ast(target, stack, lhs, rhs);
            stack.pop();
        }
    }
}

fn diff_item_tuple<'a, 'b>(
    target: &mut Vec<DiffItem<'a, 'b>>,
    stack: &mut DiffPath<'a, 'b>,
    lhs: &'b ItemTuple<'a>,
    rhs: &'b ItemTuple<'a>,
) {
    if lhs.0.len() != rhs.0.len() {
        target.push(DiffItem {
            path: stack.clone(),
            diff: DiffPayload::ItemTuple(lhs, rhs),
        });
    } else {
        for (idx, (lhs, rhs)) in lhs.0.iter().zip(rhs.0.iter()).enumerate() {
            stack.push(DiffPathSegment::Index(idx, idx));
            diff_ast(target, stack, lhs, rhs);
            stack.pop();
        }
    }
}

fn diff_item_tuple_struct<'a, 'b>(
    target: &mut Vec<DiffItem<'a, 'b>>,
    stack: &mut DiffPath<'a, 'b>,
    lhs: &'b ItemTupleStruct<'a>,
    rhs: &'b ItemTupleStruct<'a>,
) {
    if lhs.0 != rhs.0 || lhs.1.len() != rhs.1.len() {
        target.push(DiffItem {
            path: stack.clone(),
            diff: DiffPayload::ItemTupleStruct(lhs, rhs),
        });
    } else {
        for (idx, (lhs, rhs)) in lhs.1.iter().zip(rhs.1.iter()).enumerate() {
            stack.push(DiffPathSegment::Index(idx, idx));
            diff_ast(target, stack, lhs, rhs);
            stack.pop();
        }
    }
}

fn diff_item_unit_struct<'a, 'b>(
    target: &mut Vec<DiffItem<'a, 'b>>,
    stack: &mut DiffPath<'a, 'b>,
    lhs: &'b ItemUnitStruct<'a>,
    rhs: &'b ItemUnitStruct<'a>,
) {
    if lhs.0 != rhs.0 {
        target.push(DiffItem {
            path: stack.clone(),
            diff: DiffPayload::ItemUnitStruct(lhs, rhs),
        });
    }
}

fn diff_value_bool<'a, 'b>(
    target: &mut Vec<DiffItem<'a, 'b>>,
    stack: &mut DiffPath<'a, 'b>,
    lhs: &'b ValueBool,
    rhs: &'b ValueBool,
) {
    if lhs.0 != rhs.0 {
        target.push(DiffItem {
            path: stack.clone(),
            diff: DiffPayload::ValueBool(lhs, rhs),
        });
    }
}

fn diff_value_num<'a, 'b>(
    target: &mut Vec<DiffItem<'a, 'b>>,
    stack: &mut DiffPath<'a, 'b>,
    lhs: &'b ValueNum<'a>,
    rhs: &'b ValueNum<'a>,
) {
    if lhs.0 != rhs.0 {
        target.push(DiffItem {
            path: stack.clone(),
            diff: DiffPayload::ValueNum(lhs, rhs),
        });
    }
}

fn diff_value_str<'a, 'b>(
    target: &mut Vec<DiffItem<'a, 'b>>,
    stack: &mut DiffPath<'a, 'b>,
    lhs: &'b ValueStr<'a>,
    rhs: &'b ValueStr<'a>,
) {
    if lhs.0 != rhs.0 {
        target.push(DiffItem {
            path: stack.clone(),
            diff: DiffPayload::ValueStr(lhs, rhs),
        });
    }
}
