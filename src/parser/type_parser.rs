use crate::parser::jtype::JType;

pub trait JTypeParser {
    type JO<JS, JT>;
    type JA<JT>;
    type JS;
    type JN;
    type JB;
    type JZ;
    type JT;

    fn get_object(&self, obj: Vec<(Self::JS, Self::JT)>) -> Self::JO<Self::JS, Self::JT>;
    fn get_array(&self, arr: Vec<Self::JT>) -> Self::JA<Self::JT>;
    fn get_string(&self, str: &str) -> Self::JS;
    fn get_number(&self, num: &str) -> Self::JN;
    fn get_boolean(&self, boo: bool) -> Self::JB;
    fn get_null(&self) -> Self::JZ;
    fn get_jtype(
        &self,
        jt: JType<
            Self::JO<Self::JS, Self::JT>,
            Self::JA<Self::JT>,
            Self::JS,
            Self::JN,
            Self::JB,
            Self::JZ,
        >,
    ) -> Self::JT;
}
