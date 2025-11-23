// -- TYPES

#[derive( Debug, Clone, Copy, PartialEq )]
pub enum ArcBackwardVectorType
{
    // -- CONSTANTS
    
    CUSTOM = 0,
    SYMMETRICAL = 1,
    RESIDUAL = 2,
    COUNT = 3,
    NONE = -1
}

