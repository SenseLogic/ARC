// -- IMPORTS

import { ArcBackwardVectorType, ArcCurve, ArcInterpolationMethod, ArcVertex, setQuadraticArcPositionVector, setSphericalArcPositionVector } from './arc.js';
import fs from 'fs';

// -- FUNCTIONS

function main(
    )
{
    let curveArray =
        [
            new ArcCurve(
                [
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: 1, z: 0 },
                            forwardVector: { x: 1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 1, y: 0, z: 0 },
                            forwardVector: { x: 0, y: -1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: -1, z: 0 },
                            forwardVector: { x: -1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: -1, y: 0, z: 0 },
                            forwardVector: { x: 0, y: 1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        )
                    ],
                4
                ),
            new ArcCurve(
                [
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: 1, z: 0 },
                            forwardVector: { x: 1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 2, y: 1, z: 0 },
                            forwardVector: { x: 1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 3, y: 0, z: 0 },
                            forwardVector: { x: 0, y: -1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 2, y: -1, z: 0 },
                            forwardVector: { x: -1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: -1, z: 0 },
                            forwardVector: { x: -1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: -1, y: 0, z: 0 },
                            forwardVector: { x: 0, y: 1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        )
                    ],
                6
                ),
            new ArcCurve(
                [
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: 1, z: 0 },
                            forwardVector: { x: 1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.symmetrical
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 2, y: 0, z: 0 },
                            forwardVector: { x: 0, y: -1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.symmetrical
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: -1, z: 0 },
                            forwardVector: { x: -1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.symmetrical
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: -1, y: 0, z: 0 },
                            forwardVector: { x: 0, y: 1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.symmetrical
                        }
                        )
                    ],
                4
                ),
            new ArcCurve(
                [
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: 1, z: 0 },
                            forwardVector: { x: 2, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 2, y: 0, z: 0 },
                            forwardVector: { x: 0, y: -1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: -1, z: 0 },
                            forwardVector: { x: -1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: -1, y: 0, z: 0 },
                            forwardVector: { x: 0, y: 1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        )
                    ],
                4
                ),
            new ArcCurve(
                [
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: 1, z: 0 },
                            forwardVector: { x: 1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.symmetrical
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 2, y: 0, z: 0 },
                            forwardVector: { x: 1, y: -1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.symmetrical
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: -1, z: 0 },
                            forwardVector: { x: -1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.symmetrical
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: -1, y: 0, z: 0 },
                            forwardVector: { x: 0, y: 1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.symmetrical
                        }
                        )
                    ],
                4
                ),
            new ArcCurve(
                [
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: 1, z: 0 },
                            forwardVector: { x: 1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 2, y: 0, z: 0 },
                            forwardVector: { x: 1, y: -1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: -1, z: 0 },
                            forwardVector: { x: -1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: -1, y: 0, z: 0 },
                            forwardVector: { x: 0, y: 1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        )
                    ],
                4
                ),
            new ArcCurve(
                [
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: 1, z: 0 },
                            forwardVector: { x: 1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 2, y: 0, z: 0 },
                            forwardVector: { x: 1.5, y: -1.5, z: 0 },
                            backwardVector: { x: -1.5, y: 1.5, z: 0 }
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: -1, z: 0 },
                            forwardVector: { x: -1, y: 0, z: 0 },
                            backwardVector: { x: 1, y: 0, z: 0 }
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: -1, y: 0, z: 0 },
                            forwardVector: { x: 0, y: 1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        )
                    ],
                4
                ),
            new ArcCurve(
                [
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: 1, z: 0 },
                            forwardVector: { x: 1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 2, y: 0, z: 0 },
                            forwardVector: { x: 1.5, y: -1.5, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 0, y: -1, z: 0 },
                            forwardVector: { x: -1, y: 0, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: -1, y: 0, z: 0 },
                            forwardVector: { x: 0, y: 1, z: 0 },
                            backwardVectorType: ArcBackwardVectorType.residual
                        }
                        )
                    ],
                4
                ),
            new ArcCurve(
                [
                    new ArcVertex(
                        {
                            positionVector: { x: 1, y: 2, z: 0 },
                            forwardVector: { x: 1.5, y: 1.5, z: 0 },
                            backwardVector: { x: -1.5, y: -1.5, z: 0 }
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 3, y: 5, z: 0 },
                            forwardVector: { x: 1.5, y: -1.5, z: 0 },
                            backwardVector: { x: -1.5, y: 1.5, z: 0 }
                        }
                        ),
                    new ArcVertex(
                        {
                            positionVector: { x: 6, y: 7, z: 0 },
                            forwardVector: { x: 1.5, y: 3, z: 0 },
                            backwardVector: { x: -1.5, y: -3, z: 0 }
                        }
                        )
                    ],
                2
                )
            ];

    for ( let curve of curveArray )
    {
        curve.updateBackwardVectors();
    }

    for ( let curveIndex = 0; curveIndex < curveArray.length; curveIndex++ )
    {
        let curve = curveArray[ curveIndex ];

        for ( let interpolationMethod of [ ArcInterpolationMethod.quadratic, ArcInterpolationMethod.spherical ] )
        {
            let svgDrawing = curve.getSvgDrawing( interpolationMethod, 12 );
            let interpolationMethodName = interpolationMethod == ArcInterpolationMethod.quadratic ? 'quadratic' : 'spherical';
            let filepath = `curve_${curveIndex + 1}_${interpolationMethodName}_interpolation.svg`;

            try
            {
                console.log( 'Writing file :', filepath );

                fs.writeFileSync( filepath, svgDrawing );
            }
            catch ( error )
            {
                console.error( 'Error writing file :', filepath, error );
            }
        }
    }
}

// -- STATEMENTS

main();
