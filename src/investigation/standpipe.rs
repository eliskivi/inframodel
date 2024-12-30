use crate::ParseResult;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Standpipe {
    // ZP token
    pub top_elevation: ParseResult<f32>,
    pub ground_elevation: ParseResult<f32>,
    pub protection_top_elevation: ParseResult<f32>,
    pub cover_elevation: ParseResult<f32>,
    pub sieve_bottom_elevation: ParseResult<f32>,
    // TP token
    pub upper_structure: ParseResult<String>,
    pub sieve_length: ParseResult<f32>,
    pub sieve_type: ParseResult<String>,
    pub diameter: ParseResult<f32>,
    pub material: ParseResult<String>,
    // LP token
    pub measure_point: ParseResult<String>,
    pub details: ParseResult<String>,
    pub locked: ParseResult<String>,
    pub lock_owner: ParseResult<String>,
    pub installer: ParseResult<String>,
}
