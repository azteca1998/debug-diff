use crate::ast::*;
use chumsky::{extra::Err, prelude::*, text::keyword};

pub fn parser<'a>() -> impl Parser<'a, &'a str, AstItem<'a>, Err<Rich<'a, char>>> {
    ast_parser().padded().then_ignore(end())
}

fn ast_parser<'a>() -> impl Parser<'a, &'a str, AstItem<'a>, Err<Rich<'a, char>>> {
    recursive(|value| {
        let item_list = value
            .clone()
            .separated_by(just(',').padded())
            .allow_trailing()
            .collect()
            .padded()
            .delimited_by(just('['), just(']'))
            .map(ItemList);
        let item_map = value
            .clone()
            .then_ignore(just(':').padded())
            .then(value.clone())
            .separated_by(just(',').padded())
            .allow_trailing()
            .collect()
            .padded()
            .delimited_by(just('{'), just('}'))
            .map(ItemMap);
        let item_option = keyword("None")
            .to(None)
            .or(keyword("Some")
                .ignore_then(value.clone().padded().delimited_by(just('('), just(')')))
                .map(Box::new)
                .map(Some))
            .map(ItemOption);
        let item_set = value
            .clone()
            .separated_by(just(',').padded())
            .allow_trailing()
            .collect()
            .padded()
            .delimited_by(just('{'), just('}'))
            .map(ItemSet);
        let item_struct = text::ident()
            .then_ignore(text::whitespace())
            .then(
                text::ident()
                    .then_ignore(just(':').padded())
                    .then(value.clone())
                    .separated_by(just(',').padded())
                    .allow_trailing()
                    .collect()
                    .padded()
                    .delimited_by(just('{'), just('}')),
            )
            .map(|(name, items)| ItemStruct(name, items));
        let item_tuple = value
            .clone()
            .separated_by(just(',').padded())
            .allow_trailing()
            .collect()
            .padded()
            .delimited_by(just('('), just(')'))
            .map(ItemTuple);
        let item_tuple_struct = text::ident()
            .then(
                value
                    .clone()
                    .separated_by(just(',').padded())
                    .allow_trailing()
                    .collect()
                    .padded()
                    .delimited_by(just('('), just(')')),
            )
            .map(|(name, items)| ItemTupleStruct(name, items));
        let item_unit_struct = text::ident().map(ItemUnitStruct);

        let value_bool = text::keyword("true")
            .to(true)
            .or(text::keyword("false").to(false))
            .map(ValueBool);
        let value_num = just('-')
            .or_not()
            .then(
                text::digits(10)
                    .repeated()
                    .at_least(1)
                    .then(just('.').then(text::digits(10).repeated()).or_not()),
            )
            .map_slice(ValueNum);
        let value_str = none_of("\\\"")
            .ignored()
            .or(just('\\').ignore_then(any()).ignored())
            .repeated()
            .map_slice(ValueStr)
            .delimited_by(just('"'), just('"'));

        choice((
            item_list.map(AstItem::from),
            item_map.map(AstItem::from),
            item_option.map(AstItem::from),
            item_set.map(AstItem::from),
            item_struct.map(AstItem::from),
            item_tuple.map(AstItem::from),
            item_tuple_struct.map(AstItem::from),
            value_bool.map(AstItem::from),
            item_unit_struct.map(AstItem::from),
            value_num.map(AstItem::from),
            value_str.map(AstItem::from),
        ))
    })
}
