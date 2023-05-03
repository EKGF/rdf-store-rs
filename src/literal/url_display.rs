// Copyright (c) 2018-2023, agnos.ai UK Ltd, all rights reserved.
//---------------------------------------------------------------

use {
    crate::literal::this::Literal,
    std::fmt::{Display, Formatter},
};

pub struct LiteralUrlDisplay<'a> {
    pub(crate) literal: &'a Literal,
}

impl<'a> Display for LiteralUrlDisplay<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.literal.data_type.is_string() {
            write!(
                f,
                "{}",
                urlencoding::encode(self.literal.as_str().unwrap_or(""))
            )
        } else if self.literal.data_type.is_boolean() {
            write!(
                f,
                "{:}",
                self.literal.as_boolean().unwrap_or(false)
            )
        } else {
            write!(f, "{:}", self.literal.to_string().as_str())
        }
    }
}
