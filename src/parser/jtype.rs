use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum JType<JO, JA, JS, JN, JB, JZ> {
    Object(JO),
    Array(JA),
    String(JS),
    Number(JN),
    Boolean(JB),
    Null(JZ),
}
