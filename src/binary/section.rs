use num_derive::FromPrimitive;

// https://www.w3.org/TR/wasm-core-1/#sections%E2%91%A0
#[derive(Debug, PartialEq, FromPrimitive)]
pub enum SectionCode {
    Type = 0x01,
    Import = 0x02,
    Function = 0x03,
    Memory = 0x05,
    Export = 0x07,
    Code = 0x0a,
    Data = 0x0b,
}
