// -- IMPORTS

using System;
using System.Collections.Generic;
using System.IO;

// -- FUNCTIONS

namespace Arc
{
    public class Test
    {
        public static void Main(
            )
        {
            List<ArcCurve> curveArray =
                new List<ArcCurve>
                {
                    new ArcCurve(
                        new List<ArcVertex>
                        {
                            new ArcVertex(
                                new ArcVector3( 0, 1, 0 ),
                                new ArcVector3( 1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 1, 0, 0 ),
                                new ArcVector3( 0, -1, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 0, -1, 0 ),
                                new ArcVector3( -1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( -1, 0, 0 ),
                                new ArcVector3( 0, 1, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                )
                        },
                        4
                        ),
                    new ArcCurve(
                        new List<ArcVertex>
                        {
                            new ArcVertex(
                                new ArcVector3( 0, 1, 0 ),
                                new ArcVector3( 1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 2, 1, 0 ),
                                new ArcVector3( 1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 3, 0, 0 ),
                                new ArcVector3( 0, -1, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 2, -1, 0 ),
                                new ArcVector3( -1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 0, -1, 0 ),
                                new ArcVector3( -1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( -1, 0, 0 ),
                                new ArcVector3( 0, 1, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                )
                        },
                        6
                        ),
                    new ArcCurve(
                        new List<ArcVertex>
                        {
                            new ArcVertex(
                                new ArcVector3( 0, 1, 0 ),
                                new ArcVector3( 1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.symmetrical
                                ),
                            new ArcVertex(
                                new ArcVector3( 2, 0, 0 ),
                                new ArcVector3( 0, -1, 0 ),
                                null,
                                ArcBackwardVectorType.symmetrical
                                ),
                            new ArcVertex(
                                new ArcVector3( 0, -1, 0 ),
                                new ArcVector3( -1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.symmetrical
                                ),
                            new ArcVertex(
                                new ArcVector3( -1, 0, 0 ),
                                new ArcVector3( 0, 1, 0 ),
                                null,
                                ArcBackwardVectorType.symmetrical
                                )
                        },
                        4
                        ),
                    new ArcCurve(
                        new List<ArcVertex>
                        {
                            new ArcVertex(
                                new ArcVector3( 0, 1, 0 ),
                                new ArcVector3( 2, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 2, 0, 0 ),
                                new ArcVector3( 0, -1, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 0, -1, 0 ),
                                new ArcVector3( -1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( -1, 0, 0 ),
                                new ArcVector3( 0, 1, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                )
                        },
                        4
                        ),
                    new ArcCurve(
                        new List<ArcVertex>
                        {
                            new ArcVertex(
                                new ArcVector3( 0, 1, 0 ),
                                new ArcVector3( 1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.symmetrical
                                ),
                            new ArcVertex(
                                new ArcVector3( 2, 0, 0 ),
                                new ArcVector3( 1, -1, 0 ),
                                null,
                                ArcBackwardVectorType.symmetrical
                                ),
                            new ArcVertex(
                                new ArcVector3( 0, -1, 0 ),
                                new ArcVector3( -1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.symmetrical
                                ),
                            new ArcVertex(
                                new ArcVector3( -1, 0, 0 ),
                                new ArcVector3( 0, 1, 0 ),
                                null,
                                ArcBackwardVectorType.symmetrical
                                )
                        },
                        4
                        ),
                    new ArcCurve(
                        new List<ArcVertex>
                        {
                            new ArcVertex(
                                new ArcVector3( 0, 1, 0 ),
                                new ArcVector3( 1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 2, 0, 0 ),
                                new ArcVector3( 1, -1, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 0, -1, 0 ),
                                new ArcVector3( -1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( -1, 0, 0 ),
                                new ArcVector3( 0, 1, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                )
                        },
                        4
                        ),
                    new ArcCurve(
                        new List<ArcVertex>
                        {
                            new ArcVertex(
                                new ArcVector3( 0, 1, 0 ),
                                new ArcVector3( 1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 2, 0, 0 ),
                                new ArcVector3( 1.5, -1.5, 0 ),
                                new ArcVector3( -1.5, 1.5, 0 ),
                                ArcBackwardVectorType.custom
                                ),
                            new ArcVertex(
                                new ArcVector3( 0, -1, 0 ),
                                new ArcVector3( -1, 0, 0 ),
                                new ArcVector3( 1, 0, 0 ),
                                ArcBackwardVectorType.custom
                                ),
                            new ArcVertex(
                                new ArcVector3( -1, 0, 0 ),
                                new ArcVector3( 0, 1, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                )
                        },
                        4
                        ),
                    new ArcCurve(
                        new List<ArcVertex>
                        {
                            new ArcVertex(
                                new ArcVector3( 0, 1, 0 ),
                                new ArcVector3( 1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 2, 0, 0 ),
                                new ArcVector3( 1.5, -1.5, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( 0, -1, 0 ),
                                new ArcVector3( -1, 0, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                ),
                            new ArcVertex(
                                new ArcVector3( -1, 0, 0 ),
                                new ArcVector3( 0, 1, 0 ),
                                null,
                                ArcBackwardVectorType.residual
                                )
                        },
                        4
                        ),
                    new ArcCurve(
                        new List<ArcVertex>
                        {
                            new ArcVertex(
                                new ArcVector3( 1, 2, 0 ),
                                new ArcVector3( 1.5, 1.5, 0 ),
                                new ArcVector3( -1.5, -1.5, 0 ),
                                ArcBackwardVectorType.custom
                                ),
                            new ArcVertex(
                                new ArcVector3( 3, 5, 0 ),
                                new ArcVector3( 1.5, -1.5, 0 ),
                                new ArcVector3( -1.5, 1.5, 0 ),
                                ArcBackwardVectorType.custom
                                ),
                            new ArcVertex(
                                new ArcVector3( 6, 7, 0 ),
                                new ArcVector3( 1.5, 3, 0 ),
                                new ArcVector3( -1.5, -3, 0 ),
                                ArcBackwardVectorType.custom
                                )
                        },
                        2
                        )
                };

            foreach ( ArcCurve curve in curveArray )
            {
                curve.UpdateBackwardVectors();
            }

            for ( int curveIndex = 0; curveIndex < curveArray.Count; curveIndex++ )
            {
                ArcCurve curve = curveArray[ curveIndex ];

                foreach ( int interpolationMethod in new int[] { ArcInterpolationMethod.quadratic, ArcInterpolationMethod.spherical } )
                {
                    string svgDrawing = curve.GetSvgDrawing( interpolationMethod, 12 );
                    string interpolationMethodName = interpolationMethod == ArcInterpolationMethod.quadratic ? "quadratic" : "spherical";
                    string filepath = $"curve_{curveIndex + 1}_{interpolationMethodName}_interpolation.svg";

                    try
                    {
                        Console.WriteLine( "Writing file : " + filepath );

                        File.WriteAllText( filepath, svgDrawing );
                    }
                    catch ( Exception error )
                    {
                        Console.WriteLine( "Error writing file : " + filepath + " " + error.Message );
                    }
                }
            }
        }
    }
}
