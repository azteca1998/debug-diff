use super::AstItem;
use crate::util::DebugToDisplay;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ItemList<'a>(pub Vec<AstItem<'a>>);

impl<'a> fmt::Display for ItemList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list()
            .entries(self.0.iter().map(DebugToDisplay))
            .finish()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ItemMap<'a>(pub BTreeMap<AstItem<'a>, AstItem<'a>>);

impl<'a> fmt::Display for ItemMap<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(
                self.0
                    .iter()
                    .map(|(k, v)| (DebugToDisplay(k), DebugToDisplay(v))),
            )
            .finish()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ItemOption<'a>(pub Option<Box<AstItem<'a>>>);

impl<'a> fmt::Display for ItemOption<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.as_deref() {
            Some(x) => f.debug_tuple("Some").field(&DebugToDisplay(x)).finish(),
            None => f.debug_struct("None").finish(),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ItemSet<'a>(pub BTreeSet<AstItem<'a>>);

impl<'a> fmt::Display for ItemSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set()
            .entries(self.0.iter().map(DebugToDisplay))
            .finish()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ItemStruct<'a>(pub &'a str, pub BTreeMap<&'a str, AstItem<'a>>);

impl<'a> fmt::Display for ItemStruct<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fmt = f.debug_struct(self.0);
        for (k, v) in &self.1 {
            fmt.field(k, &DebugToDisplay(v));
        }
        fmt.finish()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ItemTuple<'a>(pub Vec<AstItem<'a>>);

impl<'a> fmt::Display for ItemTuple<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            f.debug_tuple("()").finish()
        } else {
            let mut fmt = f.debug_tuple("");
            for field in &self.0 {
                fmt.field(&DebugToDisplay(field));
            }
            fmt.finish()
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ItemTupleStruct<'a>(pub &'a str, pub Vec<AstItem<'a>>);

impl<'a> fmt::Display for ItemTupleStruct<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fmt = f.debug_tuple(self.0);
        for field in &self.1 {
            fmt.field(&DebugToDisplay(field));
        }
        fmt.finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ItemUnitStruct<'a>(pub &'a str);

impl<'a> fmt::Display for ItemUnitStruct<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(self.0).finish()
    }
}
