// -- TYPES

enum ArcInterpolationMethod
{
    // -- CONSTANTS

    quadratic( 0 ),
    spherical( 1 ),
    count( 2 ),
    none( -1 );

    // -- ATTRIBUTES

    final int 
        value;

    // -- CONSTRUCTORS

    const ArcInterpolationMethod(
        this.value
        );
}
