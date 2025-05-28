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
            "Expected an enum formed with the token tree `enum Ids {{}}`.",
            span,
        );
    }

    let mut ids: Vec<String> = Vec::new();
    for token in attr {
        if let TokenTree::Literal(literal) = token {
            let maybe_value = value_from_literal(&literal);
            if let Err(err) = maybe_value {
                let span = literal.span();
                return error(err, span);
            }
            let value = maybe_value.unwrap();

            if value.is_empty() {
                let span = literal.span();
                return error(
                    "String literals in the attribute cannot be empty.",
                    span,
                );
            }

            if ids.contains(&value) {
                let span = literal.span();
                return error("Duplicated string literal found.", span);
            }

            ids.push(value);
        } else if let TokenTree::Punct(punct) = token {
            if punct.as_char() != ',' {
                let span = punct.span();
                return error(
                    "Expected a comma between string literals in the attribute.",
                    span,
                );
            }
        } else {
            let span = token.span();
            return error(
                "Expected only string literals and commas in the attribute.",
                span,
            );
        }
    }

    let call_site_span = Span::call_site();

    // remove the last token and add the implementation
    let mut tokens: Vec<TokenTree> = item.into_iter().collect();
    tokens.pop();

    // enum declaration
    let group = Group::new(Delimiter::Brace, {
        let mut inner = TokenStream::new();
        let ids_iter = ids.clone().into_iter();
        for id in ids_iter {
            inner.extend([
                TokenTree::Ident(to_pascal_case_ident(&id, &call_site_span)),
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
            TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                let mut inner = TokenStream::new();
                inner.extend([
                    TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                    TokenTree::Ident(Ident::new("self", call_site_span)),
                ]);
                inner
            })),
            TokenTree::Punct(Punct::new('-', Spacing::Joint)),
            TokenTree::Punct(Punct::new('>', Spacing::Alone)),
            TokenTree::Punct(Punct::new('&', Spacing::Joint)),
            TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
            TokenTree::Ident(Ident::new("static", call_site_span)),
            TokenTree::Ident(Ident::new("str", call_site_span)),
        ]);

        let group = Group::new(Delimiter::Brace, {
            let mut inner = TokenStream::new();
            inner.extend([
                TokenTree::Ident(Ident::new("match", call_site_span)),
                TokenTree::Ident(Ident::new("self", call_site_span)),
                TokenTree::Group(Group::new(Delimiter::Brace, {
                    let mut inner = TokenStream::new();
                    for id in &ids {
                        inner.extend([
                            TokenTree::Ident(to_pascal_case_ident(
                                id,
                                &call_site_span,
                            )),
                            TokenTree::Punct(Punct::new('=', Spacing::Joint)),
                            TokenTree::Punct(Punct::new('>', Spacing::Alone)),
                            TokenTree::Literal(Literal::string(id)),
                            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        ]);
                    }
                    inner.extend([
                        TokenTree::Ident(Ident::new("_", call_site_span)),
                        TokenTree::Punct(Punct::new('=', Spacing::Joint)),
                        TokenTree::Punct(Punct::new('>', Spacing::Alone)),
                        TokenTree::Ident(Ident::new(
                            "unreachable",
                            call_site_span,
                        )),
                        TokenTree::Punct(Punct::new('!', Spacing::Joint)),
                        TokenTree::Group(Group::new(
                            Delimiter::Parenthesis,
                            TokenStream::new(),
                        )),
                    ]);
                    inner
                })),
            ]);
            inner
        });
        inner.extend([TokenTree::Group(group)]);

        inner
    });

    tokens.push(TokenTree::Group(impl_group));

    tokens.into_iter().collect()
}

fn error(message: &str, span: Span) -> TokenStream {
    let mut error_message = Literal::string(&format!("Error: {message}"));

    // Asignamos el span del token original
    error_message.set_span(span);

    let tokens = vec![
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
    ];

    tokens.into_iter().collect()
}

/// Convert a literal to a string, removing the quotes and the string type characters
fn value_from_literal(
    literal: &proc_macro::Literal,
) -> Result<String, &'static str> {
    let literal_str = literal.to_string();
    if literal_str.starts_with("r#") {
        Ok(literal_str
            .strip_prefix("r#\"")
            .unwrap()
            .strip_suffix("\"#")
            .unwrap()
            .into())
    } else if literal_str.starts_with("c\"") {
        Ok(literal_str
            .strip_prefix("c\"")
            .unwrap()
            .strip_suffix('"')
            .unwrap()
            .into())
    } else if literal_str.starts_with("cr#") {
        Ok(literal_str
            .strip_prefix("cr#\"")
            .unwrap()
            .strip_suffix("\"#")
            .unwrap()
            .into())
    } else if literal_str.starts_with('"') {
        Ok(literal_str
            .strip_prefix('"')
            .unwrap()
            .strip_suffix('"')
            .unwrap()
            .into())
    } else {
        Err("Literal must be a string literal")
    }
}

fn to_pascal_case_ident(input: &str, span: &Span) -> Ident {
    let pascal = input.to_case(Case::Pascal);
    Ident::new(&pascal, *span)
}
