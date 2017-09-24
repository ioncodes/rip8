#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    JP,
    LdI,
    LdV,
    LdXK,
    DRW,
    AddI,
    AddX,
    SeX,    // swiggity swooty,
    SeXY,   // i'm coming for that booty.
    CLS,
    RET,
    Unknown
}