// -- IMPORTS

use crate::arc::*;
use std::fs;

// -- FUNCTIONS

pub fn test_arc_library(
    )
{
    let mut curve_array = 
        vec![
            ArcCurve::new(
                vec!
                [
                    ArcVertex::new(
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, -1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 0.0, -1.0, 0.0 ),
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    )
                ],
                4
            ),
            ArcCurve::new(
                vec!
                [
                    ArcVertex::new(
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 2.0, 1.0, 0.0 ),
                        Vector3::new( 1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new(3.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, -1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 2.0, -1.0, 0.0 ),
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 0.0, -1.0, 0.0 ),
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    )
                ],
                6
            ),
            ArcCurve::new(
                vec!
                [
                    ArcVertex::new(
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::SYMMETRICAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 2.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, -1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::SYMMETRICAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 0.0, -1.0, 0.0 ),
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::SYMMETRICAL
                    ),
                    ArcVertex::new(
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::SYMMETRICAL
                    )
                ],
                4
            ),
            ArcCurve::new(
                vec!
                [
                    ArcVertex::new(
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 2.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 2.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, -1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 0.0, -1.0, 0.0 ),
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    )
                ],
                4
            ),
            ArcCurve::new(
                vec!
                [
                    ArcVertex::new(
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::SYMMETRICAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 2.0, 0.0, 0.0 ),
                        Vector3::new( 1.0, -1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::SYMMETRICAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 0.0, -1.0, 0.0 ),
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::SYMMETRICAL
                    ),
                    ArcVertex::new(
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::SYMMETRICAL
                    )
                ],
                4
            ),
            ArcCurve::new(
                vec!
                [
                    ArcVertex::new(
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 2.0, 0.0, 0.0 ),
                        Vector3::new( 1.0, -1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 0.0, -1.0, 0.0 ),
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    )
                ],
                4
            ),
            ArcCurve::new(
                vec!
                [
                    ArcVertex::new(
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 2.0, 0.0, 0.0 ),
                        Vector3::new( 1.5, -1.5, 0.0 ),
                        Vector3::new( -1.5, 1.5, 0.0 ),
                        ArcBackwardVectorType::CUSTOM
                    ),
                    ArcVertex::new(
                        Vector3::new( 0.0, -1.0, 0.0 ),
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 1.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::CUSTOM
                    ),
                    ArcVertex::new(
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    )
                ],
                4
            ),
            ArcCurve::new(
                vec!
                [
                    ArcVertex::new(
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    ),
                    ArcVertex::new(
                        Vector3::new( 2.0, 0.0, 0.0 ),
                        Vector3::new( 1.5, -1.5, 0.0 ),
                        Vector3::new( -1.5, 1.5, 0.0 ),
                        ArcBackwardVectorType::CUSTOM
                    ),
                    ArcVertex::new(
                        Vector3::new( 0.0, -1.0, 0.0 ),
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 1.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::CUSTOM
                    ),
                    ArcVertex::new(
                        Vector3::new( -1.0, 0.0, 0.0 ),
                        Vector3::new( 0.0, 1.0, 0.0 ),
                        Vector3::new( 0.0, 0.0, 0.0 ),
                        ArcBackwardVectorType::RESIDUAL
                    )
                ],
                4
            ),
            ArcCurve::new(
                vec!
                [
                    ArcVertex::new(
                        Vector3::new( 1.0, 2.0, 0.0 ),
                        Vector3::new( 1.5, 1.5, 0.0 ),
                        Vector3::new( -1.5, -1.5, 0.0 ),
                        ArcBackwardVectorType::CUSTOM
                    ),
                    ArcVertex::new(
                        Vector3::new( 3.0, 5.0, 0.0 ),
                        Vector3::new( 1.5, -1.5, 0.0 ),
                        Vector3::new( -1.5, 1.5, 0.0 ),
                        ArcBackwardVectorType::CUSTOM
                    ),
                    ArcVertex::new(
                        Vector3::new( 6.0, 7.0, 0.0 ),
                        Vector3::new( 1.5, 3.0, 0.0 ),
                        Vector3::new( -1.5, -3.0, 0.0 ),
                        ArcBackwardVectorType::CUSTOM
                    )
                ],
                2
            )
        ];

    for curve in &mut curve_array
    {
        curve.update_backward_vectors();
    }

    for ( curve_index, curve ) in curve_array.iter().enumerate()
    {
        for interpolation_method in [ ArcInterpolationMethod::QUADRATIC, ArcInterpolationMethod::SPHERICAL ]
        {
            let svg_drawing = curve.get_svg_drawing( interpolation_method, 12 );

            let method_name = if interpolation_method == ArcInterpolationMethod::QUADRATIC { "quadratic" } else { "spherical" };
            let filename = format!( "curve_{}_{}_interpolation.svg", curve_index + 1, method_name );

            match fs::write(&filename, svg_drawing)
            {
                Ok( _ ) => println!( "Writing file : {}", filename ),
                Err( error ) => eprintln!( "Error writing file : {} - {}", filename, error )
            }
        }
    }
}
