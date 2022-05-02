use crate::meta::{Id, WithMeta};

#[derive(Clone, Debug, PartialEq)]
pub struct Handler {
    pub input: WithMeta<Type>,
    pub output: WithMeta<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Effect {
    pub input: WithMeta<Type>,
    pub output: WithMeta<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Number,
    String,
    Trait(Vec<WithMeta<Self>>),
    Effectful {
        ty: Box<WithMeta<Self>>,
        effects: WithMeta<EffectExpr>,
    },
    Infer,
    This,
    Product(Vec<WithMeta<Self>>),
    Sum(Vec<WithMeta<Self>>),
    Function {
        parameter: Box<WithMeta<Self>>,
        body: Box<WithMeta<Self>>,
    },
    Array(Box<WithMeta<Self>>),
    Set(Box<WithMeta<Self>>),
    Let {
        variable: Id,
        // definition: Box<WithMeta<Self>>,
        body: Box<WithMeta<Self>>,
    },
    Variable(Id),
    BoundedVariable {
        bound: Box<WithMeta<Self>>,
        identifier: String,
    },
    Brand {
        brand: String,
        item: Box<WithMeta<Self>>,
    },
    // just label brand
    Label {
        label: String,
        item: Box<WithMeta<Self>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum EffectExpr {
    Effects(Vec<WithMeta<Effect>>),
    Add(Vec<WithMeta<EffectExpr>>),
    Sub {
        minuend: Box<WithMeta<EffectExpr>>,
        subtrahend: Box<WithMeta<EffectExpr>>,
    },
    Apply {
        function: Box<WithMeta<Type>>,
        arguments: Vec<WithMeta<Type>>,
    },
}
