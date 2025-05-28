use convert_case::{Case, Casing};
use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream,
    TokenTree,
};

#[proc_macro_attribute]
pub fn leptos_unique_ids(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_clone = item.clone();
    let output_item_iter = item.clone().into_iter();

    let mut vis = None;

    let mut enum_tokens_iter = output_item_iter.skip_while(|token| {
        if let TokenTree::Ident(ident) = token {
            let ident_str = ident.to_string();
            if ident_str == "enum" {
                return false;
            } else if ident_str == "pub" {
                vis = Some(TokenStream::from(TokenTree::Ident(ident.clone())));
                return true; // skip searchng for a possible full pub(...) visibility
            }
        } else if let TokenTree::Group(group) = token {
            let group_str = group.to_string();
            if group_str.starts_with("(") {
                let mut new_vis = vis.clone().unwrap_or_default();
                new_vis.extend([TokenTree::Group(group.clone())]);
                vis = Some(new_vis);
                return false;
            }
        }
        true
    });

    if !enum_tokens_iter.all(|token| {
        if let TokenTree::Ident(ident) = token {
            let ident_str = ident.to_string();
            ident_str == "Ids" || ident_str == "enum" || ident_str == "pub"
        } else if let TokenTree::Group(_) = token {
            let group_str = token.to_string();
            return group_str == "{}" || group_str.starts_with("(");
        } else {
            return false;
        }
    }) {
        let mut enum_tokens_iter = item_clone.into_iter().skip_while(|token| {
            !matches!(token, proc_macro::TokenTree::Ident(ident) if ident.to_string() == "enum")
        });
        let first_token = enum_tokens_iter
            .next()
            .expect("Expected at least one token in the enum declaration");
        let span = first_token.span();

        return error(
            b"Expected an enum formed with the token tree `enum Ids {{}}`.",
            span,
        );
    }

    let call_site_span = Span::call_site();

    let mut ids: Vec<String> = Vec::new();
    let mut ids_variants_idents = Vec::new();

    for token in attr {
        if let TokenTree::Literal(literal) = token {
            let literal_str = literal.to_string();
            let maybe_value = value_from_literal_str(&literal_str);
            if let Err(err) = maybe_value {
                let span = literal.span();
                return error(err, span);
            }
            let value = maybe_value.unwrap().to_string();

            if value.is_empty() {
                let span = literal.span();
                return error(
                    b"String literals in the attribute cannot be empty.",
                    span,
                );
            }

            if ids.contains(&value) {
                let span = literal.span();
                return error(b"Duplicated string literal found.", span);
            }

            ids_variants_idents
                .push(to_pascal_case_ident(&value, &call_site_span));
            ids.push(value);
        } else if let TokenTree::Punct(punct) = token {
            if punct.as_char() != ',' {
                let span = punct.span();
                return error(
                    b"Expected a comma between string literals in the attribute.",
                    span,
                );
            }
        } else {
            let span = token.span();
            return error(
                b"Expected only string literals and commas in the attribute.",
                span,
            );
        }
    }

    let ids_length = ids.len();

    // remove the last token and add the implementation
    let mut tokens: Vec<TokenTree> = item.into_iter().collect();
    tokens.pop();

    // enum declaration
    let group = Group::new(Delimiter::Brace, {
        let mut inner = TokenStream::new();
        #[expect(clippy::needless_range_loop)]
        for i in 0..ids_length {
            let ident = &ids_variants_idents[i];
            inner.extend([
                TokenTree::Ident(ident.clone()),
                TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            ]);
        }
        inner
    });
    tokens.push(TokenTree::Group(group));

    // as_str impl
    tokens.extend([
        TokenTree::Ident(Ident::new("impl", call_site_span)),
        TokenTree::Ident(Ident::new("Ids", call_site_span)),
    ]);

    let impl_group = Group::new(Delimiter::Brace, {
        let mut inner = TokenStream::new();

        if let Some(vis) = vis {
            inner.extend(vis);
        }

        inner.extend([
            TokenTree::Ident(Ident::new("fn", call_site_span)),
            TokenTree::Ident(Ident::new("as_str", call_site_span)),
            TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                [
                    TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                    TokenTree::Ident(Ident::new("self", call_site_span)),
                ]
                .into_iter()
                .collect(),
            )),
            TokenTree::Punct(Punct::new('-', Spacing::Joint)),
            TokenTree::Punct(Punct::new('>', Spacing::Alone)),
            TokenTree::Punct(Punct::new('&', Spacing::Joint)),
            TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
            TokenTree::Ident(Ident::new("static", call_site_span)),
            TokenTree::Ident(Ident::new("str", call_site_span)),
        ]);

        let group = Group::new(
            Delimiter::Brace,
            [
                TokenTree::Ident(Ident::new("match", call_site_span)),
                TokenTree::Ident(Ident::new("self", call_site_span)),
                TokenTree::Group(Group::new(Delimiter::Brace, {
                    let mut inner = TokenStream::new();
                    for i in 0..ids_length {
                        let id = &ids[i];
                        let ident = &ids_variants_idents[i];
                        inner.extend([
                            TokenTree::Ident(Ident::new(
                                "Self",
                                call_site_span,
                            )),
                            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                            TokenTree::Ident(ident.to_owned()),
                            TokenTree::Punct(Punct::new('=', Spacing::Joint)),
                            TokenTree::Punct(Punct::new('>', Spacing::Alone)),
                            TokenTree::Literal(Literal::string(id)),
                            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        ]);
                    }
                    inner
                })),
            ]
            .into_iter()
            .collect(),
        );
        inner.extend([TokenTree::Group(group)]);

        inner
    });
    tokens.push(TokenTree::Group(impl_group));

    // Into<&'static str> impl
    #[cfg(feature = "into-str")]
    tokens.extend([
        TokenTree::Ident(Ident::new("impl", call_site_span)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Ident(Ident::new("std", call_site_span)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Ident(Ident::new("convert", call_site_span)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Ident(Ident::new("Into", call_site_span)),
        TokenTree::Punct(Punct::new('<', Spacing::Joint)),
        TokenTree::Punct(Punct::new('&', Spacing::Joint)),
        TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
        TokenTree::Ident(Ident::new("static", call_site_span)),
        TokenTree::Ident(Ident::new("str", call_site_span)),
        TokenTree::Punct(Punct::new('>', Spacing::Alone)),
        TokenTree::Ident(Ident::new("for", call_site_span)),
        TokenTree::Ident(Ident::new("Ids", call_site_span)),
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            [
                TokenTree::Ident(Ident::new("fn", call_site_span)),
                TokenTree::Ident(Ident::new("into", call_site_span)),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from(TokenTree::Ident(Ident::new(
                        "self",
                        call_site_span,
                    ))),
                )),
                TokenTree::Punct(Punct::new('-', Spacing::Joint)),
                TokenTree::Punct(Punct::new('>', Spacing::Alone)),
                TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
                TokenTree::Ident(Ident::new("static", call_site_span)),
                TokenTree::Ident(Ident::new("str", call_site_span)),
                TokenTree::Group(Group::new(
                    Delimiter::Brace,
                    [
                        TokenTree::Ident(Ident::new("self", call_site_span)),
                        TokenTree::Punct(Punct::new('.', Spacing::Joint)),
                        TokenTree::Ident(Ident::new("as_str", call_site_span)),
                        TokenTree::Group(Group::new(
                            Delimiter::Parenthesis,
                            TokenStream::new(),
                        )),
                    ]
                    .into_iter()
                    .collect(),
                )),
            ]
            .into_iter()
            .collect(),
        )),
    ]);

    // leptos::prelude::IntoAttributeValue impl
    #[cfg(feature = "into-attribute-value")]
    tokens.extend([
        TokenTree::Ident(Ident::new("impl", call_site_span)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Ident(Ident::new("leptos", call_site_span)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Ident(Ident::new("prelude", call_site_span)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Ident(Ident::new("IntoAttributeValue", call_site_span)),
        TokenTree::Ident(Ident::new("for", call_site_span)),
        TokenTree::Ident(Ident::new("Ids", call_site_span)),
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            [
                TokenTree::Ident(Ident::new("type", call_site_span)),
                TokenTree::Ident(Ident::new("Output", call_site_span)),
                TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
                TokenTree::Ident(Ident::new("static", call_site_span)),
                TokenTree::Ident(Ident::new("str", call_site_span)),
                TokenTree::Punct(Punct::new(';', Spacing::Joint)),
                TokenTree::Ident(Ident::new("fn", call_site_span)),
                TokenTree::Ident(Ident::new(
                    "into_attribute_value",
                    call_site_span,
                )),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from(TokenTree::Ident(Ident::new(
                        "self",
                        call_site_span,
                    ))),
                )),
                TokenTree::Punct(Punct::new('-', Spacing::Joint)),
                TokenTree::Punct(Punct::new('>', Spacing::Alone)),
                TokenTree::Ident(Ident::new("Self", call_site_span)),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Ident(Ident::new("Output", call_site_span)),
                TokenTree::Group(Group::new(
                    Delimiter::Brace,
                    [
                        TokenTree::Ident(Ident::new("self", call_site_span)),
                        TokenTree::Punct(Punct::new('.', Spacing::Joint)),
                        TokenTree::Ident(Ident::new("as_str", call_site_span)),
                        TokenTree::Group(Group::new(
                            Delimiter::Parenthesis,
                            TokenStream::new(),
                        )),
                    ]
                    .into_iter()
                    .collect(),
                )),
            ]
            .into_iter()
            .collect(),
        )),
    ]);

    tokens.into_iter().collect()
}

fn error(message: &[u8], span: Span) -> TokenStream {
    let mut error_message = Literal::string(&String::from_utf8_lossy(message));
    error_message.set_span(span);

    let mut stream = TokenStream::new();
    stream.extend([
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct({
            let mut punct = Punct::new('!', Spacing::Alone);
            punct.set_span(span);
            punct
        }),
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            TokenStream::from(TokenTree::Literal(error_message)),
        )),
    ]);

    stream
}

fn value_from_literal_str(literal_str: &str) -> Result<&str, &'static [u8]> {
    if literal_str.starts_with("r#") {
        Ok(&literal_str[2..literal_str.len() - 2])
    } else if literal_str.starts_with("c\"") {
        Ok(&literal_str[2..literal_str.len() - 1])
    } else if literal_str.starts_with("cr#") {
        Ok(&literal_str[3..literal_str.len() - 2])
    } else if literal_str.starts_with('"') {
        Ok(&literal_str[1..literal_str.len() - 1])
    } else {
        Err(b"Literal must be a string literal")
    }
}

fn to_pascal_case_ident(input: &str, span: &Span) -> Ident {
    let pascal = input.to_case(Case::Pascal);
    Ident::new(&pascal, *span)
}
