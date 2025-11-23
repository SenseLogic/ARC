import 'dart:math';
import 'arc_backward_vector_type.dart';
import 'arc_vertex.dart';
import 'arc_interpolation_method.dart';
import 'arc_vector_3.dart';

// -- TYPES

class ArcCurve
{
    // -- ATTRIBUTES

    List<ArcVertex> vertexArray;
    int arcCount;

    // -- CONSTRUCTORS

    ArcCurve(
        this.vertexArray,
        this.arcCount
        );

    // -- INQUIRIES

    ArcVertex getArcFirstVertex(
        int arcIndex
        )
    {
        return vertexArray[ arcIndex ];
    }

    // ~~

    ArcVertex getArcSecondVertex(
        int arcIndex
        )
    {
        return vertexArray[ ( arcIndex + 1 ) % vertexArray.length ];
    }

    // -- OPERATIONS

    void addVertex(
        ArcVertex vertex
        )
    {
        vertexArray.add( vertex );
        arcCount++;
    }

    // ~~

    void updateBackwardVectors(
        )
    {
        for ( int arcIndex = 0; arcIndex < arcCount; ++arcIndex )
        {
            ArcVertex arcSecondVertex = getArcSecondVertex( arcIndex );

            if ( arcSecondVertex.backwardVectorType == ArcBackwardVectorType.symmetrical )
            {
                arcSecondVertex.setSymmetricalBackwardVector();
            }
            else if ( arcSecondVertex.backwardVectorType == ArcBackwardVectorType.residual )
            {
                arcSecondVertex.setResidualBackwardVector( getArcFirstVertex( arcIndex ) );
            }
        }
    }

    // ~~

    String getSvgDrawing(
        {
            ArcInterpolationMethod interpolationMethod = ArcInterpolationMethod.spherical,
            int pointCount = 12
        }
        )
    {
        List<ArcVector3> vertexPositionVectorArray = <ArcVector3>[];
        List<ArcVector3> backwardPositionVectorArray = <ArcVector3>[];
        List<ArcVector3> forwardPositionVectorArray = <ArcVector3>[];
        List<ArcVector3> interpolatedPositionVectorArray = <ArcVector3>[];

        ArcVector3 backwardPositionVector = ArcVector3();
        ArcVector3 forwardPositionVector = ArcVector3();
        ArcVector3 interpolatedPositionVector = ArcVector3();

        for ( ArcVertex vertex in vertexArray )
        {
            vertex.setBackwardPositionVector( backwardPositionVector );
            vertex.setForwardPositionVector( forwardPositionVector );

            vertexPositionVectorArray.add( vertex.positionVector.copy() );
            backwardPositionVectorArray.add( backwardPositionVector.copy() );
            forwardPositionVectorArray.add( forwardPositionVector.copy() );
        }

        for ( int arcIndex = 0; arcIndex < arcCount; ++arcIndex )
        {
            ArcVertex firstVertex = getArcFirstVertex( arcIndex );
            ArcVertex secondVertex = getArcSecondVertex( arcIndex );
            double oneOverPointCount = 1.0 / pointCount;

            for ( int pointIndex = 0; pointIndex <= pointCount; ++pointIndex )
            {
                double interpolationFactor = pointIndex * oneOverPointCount;

                if ( interpolationMethod == ArcInterpolationMethod.spherical )
                {
                    interpolatedPositionVector.setSphericalArcPositionVector(
                        firstVertex.positionVector,
                        firstVertex.forwardVector,
                        secondVertex.positionVector,
                        secondVertex.backwardVector,
                        interpolationFactor,
                        );
                }
                else if ( interpolationMethod == ArcInterpolationMethod.quadratic )
                {
                    interpolatedPositionVector.setQuadraticArcPositionVector(
                        firstVertex.positionVector,
                        firstVertex.forwardVector,
                        secondVertex.positionVector,
                        secondVertex.backwardVector,
                        interpolationFactor,
                        );
                }
                else
                {
                    throw ArgumentError( 'Invalid interpolation method: $interpolationMethod' );
                }

                interpolatedPositionVectorArray.add( interpolatedPositionVector.copy() );
            }
        }

        List<ArcVector3> positionVectorArray =
            [
                ...vertexPositionVectorArray,
                ...backwardPositionVectorArray,
                ...forwardPositionVectorArray,
                ...interpolatedPositionVectorArray,
            ];

        ArcVector3 minimumPositionVector =
            ArcVector3(
                x: double.infinity,
                y: double.infinity,
                z: double.infinity,
                );
        
        ArcVector3 maximumPositionVector =
            ArcVector3(
                x: double.negativeInfinity,
                y: double.negativeInfinity,
                z: double.negativeInfinity,
                );

        for ( ArcVector3 positionVector in positionVectorArray )
        {
            minimumPositionVector.x = min( minimumPositionVector.x, positionVector.x );
            minimumPositionVector.y = min( minimumPositionVector.y, positionVector.y );
            minimumPositionVector.z = min( minimumPositionVector.z, positionVector.z );
            maximumPositionVector.x = max( maximumPositionVector.x, positionVector.x );
            maximumPositionVector.y = max( maximumPositionVector.y, positionVector.y );
            maximumPositionVector.z = max( maximumPositionVector.z, positionVector.z );
        }

        double boundingBoxWidth = max( maximumPositionVector.x - minimumPositionVector.x, 0.001 );
        double boundingBoxHeight = max( maximumPositionVector.y - minimumPositionVector.y, 0.001 );
        double margin = 15.0;

        double width = 1000.0;
        double height = width * ( boundingBoxHeight / boundingBoxWidth );

        double xScale = ( width - 2 * margin ) / boundingBoxWidth;
        double yScale = ( height - 2 * margin ) / boundingBoxHeight;
        double scale = min( xScale, yScale );
        
        double pixelSize = 1.0 / scale;
        double strokeWidth = 3.0 * pixelSize;
        double circleRadius = 6.0 * pixelSize;

        double centerX = ( minimumPositionVector.x + maximumPositionVector.x ) / 2.0;
        double centerY = ( minimumPositionVector.y + maximumPositionVector.y ) / 2.0;

        double translateX = width / 2.0 - centerX * scale;
        double translateY = height / 2.0 + centerY * scale;

        StringBuffer svgDrawing = StringBuffer();
        svgDrawing.writeln( '<svg width="$width" height="$height" xmlns="http://www.w3.org/2000/svg">' );
        svgDrawing.writeln( '  <g transform="translate($translateX, $translateY) scale($scale, -$scale)">' );

        for ( int vertexIndex = 0; vertexIndex < vertexArray.length; ++vertexIndex )
        {
            ArcVertex vertex = vertexArray[ vertexIndex ];
            ArcVector3 backwardPositionVector = backwardPositionVectorArray[ vertexIndex ];
            svgDrawing.writeln( '    <line x1="${vertex.positionVector.x}" y1="${vertex.positionVector.y}" x2="${backwardPositionVector.x}" y2="${backwardPositionVector.y}" stroke="rgb( 255, 192, 192 )" stroke-width="$strokeWidth"/>' );
        }

        for ( int vertexIndex = 0; vertexIndex < vertexArray.length; ++vertexIndex )
        {
            ArcVertex vertex = vertexArray[ vertexIndex ];
            ArcVector3 forwardPositionVector = forwardPositionVectorArray[ vertexIndex ];
            svgDrawing.writeln( '    <line x1="${vertex.positionVector.x}" y1="${vertex.positionVector.y}" x2="${forwardPositionVector.x}" y2="${forwardPositionVector.y}" stroke="rgb( 192, 192, 255 )" stroke-width="$strokeWidth"/>' );
        }

        for ( ArcVector3 vertexPositionVector in vertexPositionVectorArray )
        {
            svgDrawing.writeln( '    <circle cx="${vertexPositionVector.x}" cy="${vertexPositionVector.y}" r="${2 * circleRadius}" fill="rgb( 255, 128, 255 )"/>' );
        }

        for ( ArcVector3 interpolatedPositionVector in interpolatedPositionVectorArray )
        {
            svgDrawing.writeln( '    <circle cx="${interpolatedPositionVector.x}" cy="${interpolatedPositionVector.y}" r="$circleRadius" fill="rgb( 0, 0, 255 )"/>' );
        }

        svgDrawing.writeln( '  </g>' );
        svgDrawing.writeln( '</svg>' );

        return svgDrawing.toString();
    }
}
