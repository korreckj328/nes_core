// addressing_modes.rs
// Jeremiah Korreck

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}