# -- IMPORTS

from arc import ArcBackwardVectorType, ArcCurve, ArcVertex, ArcVector3, ArcInterpolationMethod;

# -- FUNCTIONS

def main(
    ) -> None:

    curve_array = [
        ArcCurve( 
            [
                ArcVertex( ArcVector3( 0.0, 1.0, 0.0 ), ArcVector3( 1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 1.0, 0.0, 0.0 ), ArcVector3( 0.0, -1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 0.0, -1.0, 0.0 ), ArcVector3( -1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( -1.0, 0.0, 0.0 ), ArcVector3( 0.0, 1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual )
            ], 
            4 
            ),
        ArcCurve( 
            [
                ArcVertex( ArcVector3( 0.0, 1.0, 0.0 ), ArcVector3( 1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 2.0, 1.0, 0.0 ), ArcVector3( 1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 3.0, 0.0, 0.0 ), ArcVector3( 0.0, -1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 2.0, -1.0, 0.0 ), ArcVector3( -1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 0.0, -1.0, 0.0 ), ArcVector3( -1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( -1.0, 0.0, 0.0 ), ArcVector3( 0.0, 1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual )
            ], 
            6 
            ),
        ArcCurve( 
            [
                ArcVertex( ArcVector3( 0.0, 1.0, 0.0 ), ArcVector3( 1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.symmetrical ),
                ArcVertex( ArcVector3( 2.0, 0.0, 0.0 ), ArcVector3( 0.0, -1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.symmetrical ),
                ArcVertex( ArcVector3( 0.0, -1.0, 0.0 ), ArcVector3( -1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.symmetrical ),
                ArcVertex( ArcVector3( -1.0, 0.0, 0.0 ), ArcVector3( 0.0, 1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.symmetrical )
            ], 
            4 
            ),
        ArcCurve( 
            [
                ArcVertex( ArcVector3( 0.0, 1.0, 0.0 ), ArcVector3( 2.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 2.0, 0.0, 0.0 ), ArcVector3( 0.0, -1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 0.0, -1.0, 0.0 ), ArcVector3( -1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( -1.0, 0.0, 0.0 ), ArcVector3( 0.0, 1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual )
            ], 
            4 
            ),
        ArcCurve( 
            [
                ArcVertex( ArcVector3( 0.0, 1.0, 0.0 ), ArcVector3( 1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.symmetrical ),
                ArcVertex( ArcVector3( 2.0, 0.0, 0.0 ), ArcVector3( 1.0, -1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.symmetrical ),
                ArcVertex( ArcVector3( 0.0, -1.0, 0.0 ), ArcVector3( -1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.symmetrical ),
                ArcVertex( ArcVector3( -1.0, 0.0, 0.0 ), ArcVector3( 0.0, 1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.symmetrical )
            ], 
            4 
            ),
        ArcCurve( 
            [
                ArcVertex( ArcVector3( 0.0, 1.0, 0.0 ), ArcVector3( 1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 2.0, 0.0, 0.0 ), ArcVector3( 1.0, -1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 0.0, -1.0, 0.0 ), ArcVector3( -1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( -1.0, 0.0, 0.0 ), ArcVector3( 0.0, 1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual )
            ], 
            4 
            ),
        ArcCurve( 
            [
                ArcVertex( ArcVector3( 0.0, 1.0, 0.0 ), ArcVector3( 1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 2.0, 0.0, 0.0 ), ArcVector3( 1.5, -1.5, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 0.0, -1.0, 0.0 ), ArcVector3( -1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( -1.0, 0.0, 0.0 ), ArcVector3( 0.0, 1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual )
            ], 
            4 
            ),
        ArcCurve( 
            [
                ArcVertex( ArcVector3( 0.0, 1.0, 0.0 ), ArcVector3( 1.0, 0.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual ),
                ArcVertex( ArcVector3( 2.0, 0.0, 0.0 ), ArcVector3( 1.5, -1.5, 0.0 ), backwardVector = ArcVector3( -1.5, 1.5, 0.0 ) ),
                ArcVertex( ArcVector3( 0.0, -1.0, 0.0 ), ArcVector3( -1.0, 0.0, 0.0 ), backwardVector = ArcVector3( 1.0, 0.0, 0.0 ) ),
                ArcVertex( ArcVector3( -1.0, 0.0, 0.0 ), ArcVector3( 0.0, 1.0, 0.0 ), backwardVectorType = ArcBackwardVectorType.residual )
            ], 
            4 
            ),
        ArcCurve( 
            [
                ArcVertex( ArcVector3( 1.0, 2.0, 0.0 ), ArcVector3( 1.5, 1.5, 0.0 ), backwardVector = ArcVector3( -1.5, -1.5, 0.0 ) ),
                ArcVertex( ArcVector3( 3.0, 5.0, 0.0 ), ArcVector3( 1.5, -1.5, 0.0 ), backwardVector = ArcVector3( -1.5, 1.5, 0.0 ) ),
                ArcVertex( ArcVector3( 6.0, 7.0, 0.0 ), ArcVector3( 1.5, 3.0, 0.0 ), backwardVector = ArcVector3( -1.5, -3.0, 0.0 ) )
            ], 
            2 
            )
        ];

    for curve in curve_array:
        
        curve.updateBackwardVectors();

    for curve_index, curve in enumerate( curve_array ):

        for interpolation_method in [ ArcInterpolationMethod.quadratic, ArcInterpolationMethod.spherical ]:
            
            svg_drawing = curve.getSvgDrawing( interpolation_method, 12 );

            method_name = "quadratic" if ( interpolation_method == ArcInterpolationMethod.quadratic ) else "spherical";
            filename = f"curve_{curve_index + 1}_{method_name}_interpolation.svg";

            try:

                with open( filename, 'w' ) as file:
                    file.write( svg_drawing );

                print( f"Generated: {filename}" );

            except Exception as exception:

                print( f"Failed to create: {filename} - {exception}" );

if ( __name__ == "__main__" ):

    main();
