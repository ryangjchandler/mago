use serde::Deserialize;
use serde::Serialize;

use mago_span::Span;

use crate::identifier::Name;
use crate::r#type::TypeReflection;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ConstantReflection {
    pub name: Name,
    pub type_reflection: TypeReflection,
    pub item_span: Span,
    pub definition_span: Span,
    pub is_populated: bool,
}
