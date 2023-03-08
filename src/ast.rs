pub use self::{language::*, primitives::*};
use std::fmt;

mod language;
mod primitives;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AstItem<'a> {
    ItemList(ItemList<'a>),
    ItemMap(ItemMap<'a>),
    ItemOption(ItemOption<'a>),
    ItemSet(ItemSet<'a>),
    ItemStruct(ItemStruct<'a>),
    ItemTuple(ItemTuple<'a>),
    ItemTupleStruct(ItemTupleStruct<'a>),
    ItemUnitStruct(ItemUnitStruct<'a>),
    ValueBool(ValueBool),
    ValueNum(ValueNum<'a>),
    ValueStr(ValueStr<'a>),
}

impl<'a> fmt::Display for AstItem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstItem::ItemList(x) => x.fmt(f),
            AstItem::ItemMap(x) => x.fmt(f),
            AstItem::ItemOption(x) => x.fmt(f),
            AstItem::ItemSet(x) => x.fmt(f),
            AstItem::ItemStruct(x) => x.fmt(f),
            AstItem::ItemTuple(x) => x.fmt(f),
            AstItem::ItemTupleStruct(x) => x.fmt(f),
            AstItem::ItemUnitStruct(x) => x.fmt(f),
            AstItem::ValueBool(x) => x.fmt(f),
            AstItem::ValueNum(x) => x.fmt(f),
            AstItem::ValueStr(x) => x.fmt(f),
        }
    }
}

impl<'a> From<ItemList<'a>> for AstItem<'a> {
    fn from(value: ItemList<'a>) -> Self {
        Self::ItemList(value)
    }
}

impl<'a> From<ItemMap<'a>> for AstItem<'a> {
    fn from(value: ItemMap<'a>) -> Self {
        Self::ItemMap(value)
    }
}

impl<'a> From<ItemOption<'a>> for AstItem<'a> {
    fn from(value: ItemOption<'a>) -> Self {
        Self::ItemOption(value)
    }
}

impl<'a> From<ItemSet<'a>> for AstItem<'a> {
    fn from(value: ItemSet<'a>) -> Self {
        Self::ItemSet(value)
    }
}

impl<'a> From<ItemStruct<'a>> for AstItem<'a> {
    fn from(value: ItemStruct<'a>) -> Self {
        Self::ItemStruct(value)
    }
}

impl<'a> From<ItemTuple<'a>> for AstItem<'a> {
    fn from(value: ItemTuple<'a>) -> Self {
        Self::ItemTuple(value)
    }
}

impl<'a> From<ItemTupleStruct<'a>> for AstItem<'a> {
    fn from(value: ItemTupleStruct<'a>) -> Self {
        Self::ItemTupleStruct(value)
    }
}

impl<'a> From<ItemUnitStruct<'a>> for AstItem<'a> {
    fn from(value: ItemUnitStruct<'a>) -> Self {
        Self::ItemUnitStruct(value)
    }
}

impl<'a> From<ValueBool> for AstItem<'a> {
    fn from(value: ValueBool) -> Self {
        Self::ValueBool(value)
    }
}

impl<'a> From<ValueNum<'a>> for AstItem<'a> {
    fn from(value: ValueNum<'a>) -> Self {
        Self::ValueNum(value)
    }
}

impl<'a> From<ValueStr<'a>> for AstItem<'a> {
    fn from(value: ValueStr<'a>) -> Self {
        Self::ValueStr(value)
    }
}
