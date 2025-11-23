// -- TYPES

enum ArcBackwardVectorType
{
    // -- CONSTANTS

    custom( 0 ),
    symmetrical( 1 ),
    residual( 2 ),
    count( 3 ),
    none( -1 );

    // -- ATTRIBUTES
    
    final int 
        value;

    // -- CONSTRUCTORS

    const ArcBackwardVectorType(
        this.value
        );
}
