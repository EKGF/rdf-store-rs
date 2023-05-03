// Copyright (c) 2018-2023, agnos.ai UK Ltd, all rights reserved.
//---------------------------------------------------------------

use {
    crate::literal::this::Literal,
    iref::IriBuf,
    std::fmt::{Display, Formatter},
};

pub struct LiteralIdUrlDisplay<'a> {
    pub(crate) literal:     &'a Literal,
    pub(crate) id_base_iri: &'a IriBuf,
}

impl<'a> Display for LiteralIdUrlDisplay<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.literal.is_id_iri(self.id_base_iri) {
            write!(f, "{}", self.literal.as_id(self.id_base_iri))
        } else {
            write!(f, "{:}", self.literal.to_string().as_str())
        }
    }
}
