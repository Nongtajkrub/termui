#[repr(u8)]
#[derive(Debug)]
pub enum FtuiError {
    TextFlagNoneWithOther,
    TextFlagMultipleColor,
    HeaderLabelEmpty,
    MissingHeader,
}
