#ifndef ARC_CURVE_HPP
    #define ARC_CURVE_HPP

    // -- IMPORTS

    #include <cmath>
    #include <limits>
    #include <string>
    #include <vector>
    #include "arc_backward_vector_type.hpp"
    #include "arc_interpolation_method.hpp"
    #include "arc_vector_3.hpp"
    #include "arc_vertex.hpp"

    // -- TYPES

    class ArcCurve
    {
        // == PUBLIC

        public:

        // -- ATTRIBUTES

        std::vector<ArcVertex> 
            vertexArray;
        int 
            arcCount;

        // -- CONSTRUCTORS

        ArcCurve(
            const std::vector<ArcVertex> & vertexArray,
            int arcCount
            ) :
            vertexArray( vertexArray ),
            arcCount( arcCount )
        {
        }

        // -- INQUIRIES

        ArcVertex & getArcFirstVertex(
            int arcIndex
            )
        {
            return vertexArray[ arcIndex ];
        }

        // ~~

        ArcVertex & getArcSecondVertex(
            int arcIndex
            )
        {
            return vertexArray[ ( arcIndex + 1 ) % vertexArray.size() ];
        }

        // -- OPERATIONS

        void addVertex(
            const ArcVertex & vertex
            )
        {
            vertexArray.push_back( vertex );
            arcCount++;
        }

        // ~~

        void updateBackwardVectors(
            )
        {
            for ( int arcIndex = 0; arcIndex < arcCount; ++arcIndex )
            {
                ArcVertex & arcSecondVertex = this->getArcSecondVertex( arcIndex );

                if ( arcSecondVertex.backwardVectorType == ArcBackwardVectorType::symmetrical )
                {
                    arcSecondVertex.setSymmetricalBackwardVector();
                }
                else if ( arcSecondVertex.backwardVectorType == ArcBackwardVectorType::residual )
                {
                    arcSecondVertex.setResidualBackwardVector( this->getArcFirstVertex( arcIndex ) );
                }
            }
        }

        // ~~

        std::string getSvgDrawing(
            ArcInterpolationMethod interpolationMethod = ArcInterpolationMethod::spherical,
            int pointCount = 12
            );
    };
#endif
