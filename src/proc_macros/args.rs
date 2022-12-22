use super::*;

/// `$(::)? $($macro_name:ident)::+ $(!)?`
pub
struct Args {
    pub macro_name: Path,
}

impl Parse for Args {
    fn parse(
        input: ParseStream<'_>,
    ) -> Result<Args>
    {
        if input.is_empty() {
            return Ok(Self {
                macro_name: parse_quote_spanned!(Span::mixed_site()=>
                    r#try
                ),
            });
        }
        Ok((
            Self {
                macro_name: input.call(Path::parse_mod_style)?,
            },
            input.parse::<Option<Token![ ! ]>>()?,
            input.parse::<Option<Token![ , ]>>()?,
        ).0)
    }
}
