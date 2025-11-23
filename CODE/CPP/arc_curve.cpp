// -- IMPORTS

#include <sstream>
#include <stdexcept>
#include "arc_curve.hpp"

// -- OPERATIONS

std::string ArcCurve::getSvgDrawing(
    ArcInterpolationMethod interpolationMethod,
    int pointCount
    )
{
    auto
        vertexPositionVectorArray = std::vector<ArcVector3>(),
        backwardPositionVectorArray = std::vector<ArcVector3>(),
        forwardPositionVectorArray = std::vector<ArcVector3>(),
        interpolatedPositionVectorArray = std::vector<ArcVector3>();

    ArcVector3
        backwardPositionVector( 0.0f, 0.0f, 0.0f ),
        forwardPositionVector( 0.0f, 0.0f, 0.0f ),
        interpolatedPositionVector( 0.0f, 0.0f, 0.0f );

    for ( auto & vertex : vertexArray )
    {
        vertex.setBackwardPositionVector( backwardPositionVector );
        vertex.setForwardPositionVector( forwardPositionVector );

        vertexPositionVectorArray.push_back( vertex.positionVector );
        backwardPositionVectorArray.push_back( backwardPositionVector );
        forwardPositionVectorArray.push_back( forwardPositionVector );
    }

    for ( int arcIndex = 0; arcIndex < arcCount; ++arcIndex )
    {
        auto firstVertex = getArcFirstVertex( arcIndex );
        auto secondVertex = getArcSecondVertex( arcIndex );
        double oneOverPointCount = 1.0 / pointCount;

        for ( int pointIndex = 0; pointIndex <= pointCount; ++pointIndex )
        {
            double interpolationFactor = pointIndex * oneOverPointCount;

            if ( interpolationMethod == ArcInterpolationMethod::spherical )
            {
                interpolatedPositionVector.setSphericalArcPositionVector(
                    firstVertex.positionVector,
                    firstVertex.forwardVector,
                    secondVertex.positionVector,
                    secondVertex.backwardVector,
                    interpolationFactor
                    );
            }
            else if ( interpolationMethod == ArcInterpolationMethod::quadratic )
            {
                interpolatedPositionVector.setQuadraticArcPositionVector(
                    firstVertex.positionVector,
                    firstVertex.forwardVector,
                    secondVertex.positionVector,
                    secondVertex.backwardVector,
                    interpolationFactor
                    );
            }
            else
            {
                throw std::runtime_error( "Invalid interpolation method: " + std::to_string( static_cast<int>( interpolationMethod ) ) );
            }

            interpolatedPositionVectorArray.push_back( interpolatedPositionVector );
        }
    }

    std::vector<ArcVector3> positionVectorArray;
    positionVectorArray.insert( positionVectorArray.end(), vertexPositionVectorArray.begin(), vertexPositionVectorArray.end() );
    positionVectorArray.insert( positionVectorArray.end(), backwardPositionVectorArray.begin(), backwardPositionVectorArray.end() );
    positionVectorArray.insert( positionVectorArray.end(), forwardPositionVectorArray.begin(), forwardPositionVectorArray.end() );
    positionVectorArray.insert( positionVectorArray.end(), interpolatedPositionVectorArray.begin(), interpolatedPositionVectorArray.end() );

    ArcVector3 minimumPositionVector( std::numeric_limits<double>::infinity(), std::numeric_limits<double>::infinity(), std::numeric_limits<double>::infinity() );
    ArcVector3 maximumPositionVector( -std::numeric_limits<double>::infinity(), -std::numeric_limits<double>::infinity(), -std::numeric_limits<double>::infinity() );

    for ( auto & positionVector : positionVectorArray )
    {
        minimumPositionVector.x = std::min( minimumPositionVector.x, positionVector.x );
        minimumPositionVector.y = std::min( minimumPositionVector.y, positionVector.y );
        minimumPositionVector.z = std::min( minimumPositionVector.z, positionVector.z );
        maximumPositionVector.x = std::max( maximumPositionVector.x, positionVector.x );
        maximumPositionVector.y = std::max( maximumPositionVector.y, positionVector.y );
        maximumPositionVector.z = std::max( maximumPositionVector.z, positionVector.z );
    }

    double boundingBoxWidth = std::max( maximumPositionVector.x - minimumPositionVector.x, 0.001 );
    double boundingBoxHeight = std::max( maximumPositionVector.y - minimumPositionVector.y, 0.001 );
    double margin = 15.0;

    double width = 1000.0;
    double height = width * ( boundingBoxHeight / boundingBoxWidth );

    double xScale = ( width - 2 * margin ) / boundingBoxWidth;
    double yScale = ( height - 2 * margin ) / boundingBoxHeight;
    double scale = std::min( xScale, yScale );

    double pixelSize = 1.0 / scale;
    double strokeWidth = 3.0 * pixelSize;
    double circleRadius = 6.0 * pixelSize;

    double centerX = ( minimumPositionVector.x + maximumPositionVector.x ) / 2.0;
    double centerY = ( minimumPositionVector.y + maximumPositionVector.y ) / 2.0;

    double translateX = width / 2.0 - centerX * scale;
    double translateY = height / 2.0 + centerY * scale;

    std::ostringstream svgDrawing;
    svgDrawing << "<svg width=\"" << width << "\" height=\"" << height << "\" xmlns=\"http://www.w3.org/2000/svg\">\n";
    svgDrawing << "  <g transform=\"translate(" << translateX << ", " << translateY << ") scale(" << scale << ", -" << scale << ")\">\n";

    for ( size_t vertexIndex = 0; vertexIndex < vertexArray.size(); ++vertexIndex )
    {
        auto & vertex = vertexArray[ vertexIndex ];
        auto & backwardPositionVector = backwardPositionVectorArray[ vertexIndex ];
        svgDrawing << "    <line x1=\"" << vertex.positionVector.x << "\" y1=\"" << vertex.positionVector.y << "\" x2=\"" << backwardPositionVector.x << "\" y2=\"" << backwardPositionVector.y << "\" stroke=\"rgb( 255, 192, 192 )\" stroke-width=\"" << strokeWidth << "\"/>\n";
    }

    for ( size_t vertexIndex = 0; vertexIndex < vertexArray.size(); ++vertexIndex )
    {
        auto & vertex = vertexArray[ vertexIndex ];
        auto & forwardPositionVector = forwardPositionVectorArray[ vertexIndex ];
        svgDrawing << "    <line x1=\"" << vertex.positionVector.x << "\" y1=\"" << vertex.positionVector.y << "\" x2=\"" << forwardPositionVector.x << "\" y2=\"" << forwardPositionVector.y << "\" stroke=\"rgb( 192, 192, 255 )\" stroke-width=\"" << strokeWidth << "\"/>\n";
    }

    for ( auto & vertexPositionVector : vertexPositionVectorArray )
    {
        svgDrawing << "    <circle cx=\"" << vertexPositionVector.x << "\" cy=\"" << vertexPositionVector.y << "\" r=\"" << 2 * circleRadius << "\" fill=\"rgb( 255, 128, 255 )\"/>\n";
    }

    for ( auto & interpolatedPositionVector : interpolatedPositionVectorArray )
    {
        svgDrawing << "    <circle cx=\"" << interpolatedPositionVector.x << "\" cy=\"" << interpolatedPositionVector.y << "\" r=\"" << circleRadius << "\" fill=\"rgb( 0, 0, 255 )\"/>\n";
    }

    svgDrawing << "  </g>\n</svg>";

    return svgDrawing.str();
}
