use mago_interner::ThreadedInterner;
use mago_semantics::Semantics;

#[derive(Debug)]
pub struct Context<'a> {
    pub interner: &'a ThreadedInterner,
    pub semantics: &'a Semantics,
}

impl<'a> Context<'a> {
    pub fn new(interner: &'a ThreadedInterner, semantics: &'a Semantics) -> Self {
        Self { interner, semantics }
    }
}
