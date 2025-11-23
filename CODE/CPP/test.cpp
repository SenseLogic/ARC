// -- IMPORTS

#include <fstream>
#include <iostream>
#include <string>
#include "arc.hpp"

// -- FUNCTIONS

int main(
    int argc,
    char * argv[]
    )
{
    std::vector<ArcCurve> curveArray =
        {
            ArcCurve(
                {
                    ArcVertex(
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcVector3( 1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 1.0f, 0.0f, 0.0f ),
                        ArcVector3( 0.0f, -1.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 0.0f, -1.0f, 0.0f ),
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        )
                },
                4
                ),
            ArcCurve(
                {
                    ArcVertex(
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcVector3( 1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 2.0f, 1.0f, 0.0f ),
                        ArcVector3( 1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 3.0f, 0.0f, 0.0f ),
                        ArcVector3( 0.0f, -1.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 2.0f, -1.0f, 0.0f ),
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 0.0f, -1.0f, 0.0f ),
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        )
                },
                6
                ),
            ArcCurve(
                {
                    ArcVertex(
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcVector3( 1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::symmetrical
                        ),
                    ArcVertex(
                        ArcVector3( 2.0f, 0.0f, 0.0f ),
                        ArcVector3( 0.0f, -1.0f, 0.0f ),
                        ArcBackwardVectorType::symmetrical
                        ),
                    ArcVertex(
                        ArcVector3( 0.0f, -1.0f, 0.0f ),
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::symmetrical
                        ),
                    ArcVertex(
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcBackwardVectorType::symmetrical
                        )
                },
                4
                ),
            ArcCurve(
                {
                    ArcVertex(
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcVector3( 2.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 2.0f, 0.0f, 0.0f ),
                        ArcVector3( 0.0f, -1.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 0.0f, -1.0f, 0.0f ),
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        )
                },
                4
                ),
            ArcCurve(
                {
                    ArcVertex(
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcVector3( 1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::symmetrical
                        ),
                    ArcVertex(
                        ArcVector3( 2.0f, 0.0f, 0.0f ),
                        ArcVector3( 1.0f, -1.0f, 0.0f ),
                        ArcBackwardVectorType::symmetrical
                        ),
                    ArcVertex(
                        ArcVector3( 0.0f, -1.0f, 0.0f ),
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::symmetrical
                        ),
                    ArcVertex(
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcBackwardVectorType::symmetrical
                        )
                },
                4
                ),
            ArcCurve(
                {
                    ArcVertex(
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcVector3( 1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 2.0f, 0.0f, 0.0f ),
                        ArcVector3( 1.0f, -1.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 0.0f, -1.0f, 0.0f ),
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        )
                },
                4
                ),
            ArcCurve(
                {
                    ArcVertex(
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcVector3( 1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 2.0f, 0.0f, 0.0f ),
                        ArcVector3( 1.5f, -1.5f, 0.0f ),
                        ArcVector3( -1.5f, 1.5f, 0.0f )
                        ),
                    ArcVertex(
                        ArcVector3( 0.0f, -1.0f, 0.0f ),
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcVector3( 1.0f, 0.0f, 0.0f )
                        ),
                    ArcVertex(
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        )
                },
                4
                ),
            ArcCurve(
                {
                    ArcVertex(
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcVector3( 1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 2.0f, 0.0f, 0.0f ),
                        ArcVector3( 1.5f, -1.5f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( 0.0f, -1.0f, 0.0f ),
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        ),
                    ArcVertex(
                        ArcVector3( -1.0f, 0.0f, 0.0f ),
                        ArcVector3( 0.0f, 1.0f, 0.0f ),
                        ArcBackwardVectorType::residual
                        )
                },
                4
                ),
            ArcCurve(
                {
                    ArcVertex(
                        ArcVector3( 1.0f, 2.0f, 0.0f ),
                        ArcVector3( 1.5f, 1.5f, 0.0f ),
                        ArcVector3( -1.5f, -1.5f, 0.0f )
                        ),
                    ArcVertex(
                        ArcVector3( 3.0f, 5.0f, 0.0f ),
                        ArcVector3( 1.5f, -1.5f, 0.0f ),
                        ArcVector3( -1.5f, 1.5f, 0.0f )
                        ),
                    ArcVertex(
                        ArcVector3( 6.0f, 7.0f, 0.0f ),
                        ArcVector3( 1.5f, 3.0f, 0.0f ),
                        ArcVector3( -1.5f, -3.0f, 0.0f )
                        )
                },
                2
                )
        };

    for ( auto & curve : curveArray )
    {
        curve.updateBackwardVectors();
    }

    for ( size_t curveIndex = 0; curveIndex < curveArray.size(); ++curveIndex )
    {
        auto & curve = curveArray[ curveIndex ];

        for ( auto interpolationMethod : { ArcInterpolationMethod::quadratic, ArcInterpolationMethod::spherical } )
        {
            auto svgDrawing = curve.getSvgDrawing( interpolationMethod, 12 );
            auto methodName = ( interpolationMethod == ArcInterpolationMethod::quadratic ) ? "quadratic" : "spherical";
            auto filename = "curve_" + std::to_string( curveIndex + 1 ) + "_" + methodName + "_interpolation.svg";

            std::ofstream file( filename );

            if ( file.is_open() )
            {
                file << svgDrawing;
                file.close();

                std::cout << "Writing file : " << filename << "\n";
            }
            else
            {
                std::cerr << "Error writing file: " << filename << "\n";
            }
        }
    }

    return 0;
}
