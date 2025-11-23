// -- IMPORTS

import 'dart:io';
import 'arc.dart';

// -- FUNCTIONS

void main( 
    )
{
    List<ArcCurve> curveArray =
        [
            ArcCurve(
                [
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 1.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 0.0, y: -1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: -1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                ],
                4
                ),
            ArcCurve(
                [
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 2.0, y: 1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 3.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 0.0, y: -1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 2.0, y: -1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: -1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                ],
                6
                ),
            ArcCurve(
                [
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.symmetrical
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 2.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 0.0, y: -1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.symmetrical
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: -1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.symmetrical
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.symmetrical
                        ),
                ],
                4
                ),
            ArcCurve(
                [
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 2.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 2.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 0.0, y: -1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: -1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                ],
                4
                ),
            ArcCurve(
                [
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.symmetrical
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 2.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.0, y: -1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.symmetrical
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: -1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.symmetrical
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.symmetrical
                        ),
                ],
                4
                ),
            ArcCurve(
                [
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 2.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.0, y: -1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: -1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                ],
                4
                ),
            ArcCurve(
                [
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 2.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.5, y: -1.5, z: 0.0 ),
                        backwardVector: ArcVector3( x: -1.5, y: 1.5, z: 0.0 )
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: -1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        backwardVector: ArcVector3( x: 1.0, y: 0.0, z: 0.0 )
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                ],
                4
                ),
            ArcCurve(
                [
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 2.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.5, y: -1.5, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 0.0, y: -1.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: -1.0, y: 0.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 0.0, y: 1.0, z: 0.0 ),
                        backwardVectorType: ArcBackwardVectorType.residual
                        ),
                ],
                4
                ),
            ArcCurve(
                [
                    ArcVertex(
                        positionVector: ArcVector3( x: 1.0, y: 2.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.5, y: 1.5, z: 0.0 ),
                        backwardVector: ArcVector3( x: -1.5, y: -1.5, z: 0.0 )
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 3.0, y: 5.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.5, y: -1.5, z: 0.0 ),
                        backwardVector: ArcVector3( x: -1.5, y: 1.5, z: 0.0 )
                        ),
                    ArcVertex(
                        positionVector: ArcVector3( x: 6.0, y: 7.0, z: 0.0 ),
                        forwardVector: ArcVector3( x: 1.5, y: 3.0, z: 0.0 ),
                        backwardVector: ArcVector3( x: -1.5, y: -3.0, z: 0.0 )
                        ),
                ],
                2
                ),
        ];

    for ( ArcCurve curve in curveArray )
    {
        curve.updateBackwardVectors();
    }

    for ( int curveIndex = 0; curveIndex < curveArray.length; curveIndex++ )
    {
        ArcCurve curve = curveArray[ curveIndex ];

        for ( ArcInterpolationMethod interpolationMethod in [ ArcInterpolationMethod.quadratic, ArcInterpolationMethod.spherical ] )
        {
            String svgDrawing =
                curve.getSvgDrawing(
                    interpolationMethod: interpolationMethod,
                    pointCount: 12,
                    );

            String methodName = interpolationMethod == ArcInterpolationMethod.quadratic ? 'quadratic' : 'spherical';
            String filename = 'curve_${curveIndex + 1}_${methodName}_interpolation.svg';

            try
            {
                File( filename ).writeAsStringSync( svgDrawing );
                print( 'Generated: $filename' );
            }
            catch ( e )
            {
                print( 'Failed to create: $filename - $e' );
            }
        }
    }

    print( '\nAll SVG files generated successfully!' );
}
