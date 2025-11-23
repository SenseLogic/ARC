# -- IMPORTS

import math;
from typing import List;
from arc_backward_vector_type import ArcBackwardVectorType;
from arc_vertex import ArcVertex;
from arc_interpolation_method import ArcInterpolationMethod;
from arc_vector_3 import ArcVector3;

# -- TYPES

class ArcCurve:

    # -- CONSTRUCTORS

    def __init__(
        self,
        vertex_array: List[ ArcVertex ],
        arc_count: int
        ) -> None:

        self.vertexArray = vertex_array;
        self.arcCount = arc_count;

    # -- INQUIRIES

    def getArcFirstVertex(
        self,
        arcIndex: int
        ) -> ArcVertex:

        return self.vertexArray[ arcIndex ];

    # ~~

    def getArcSecondVertex(
        self,
        arcIndex: int
        ) -> ArcVertex:

        return self.vertexArray[ ( arcIndex + 1 ) % len( self.vertexArray ) ];

    # -- OPERATIONS

    def addVertex(
        self,
        vertex: ArcVertex
        ) -> None:

        self.vertexArray.append( vertex );
        self.arcCount += 1;

    # ~~

    def updateBackwardVectors(
        self
        ) -> None:

        for arcIndex in range( self.arcCount ):
            arcSecondVertex = self.getArcSecondVertex( arcIndex );

            if ( arcSecondVertex.backwardVectorType == ArcBackwardVectorType.symmetrical ):
                arcSecondVertex.setSymmetricalBackwardVector();
            elif ( arcSecondVertex.backwardVectorType == ArcBackwardVectorType.residual ):
                arcSecondVertex.setResidualBackwardVector( self.getArcFirstVertex( arcIndex ) );

    # ~~

    def getSvgDrawing(
        self,
        interpolationMethod: ArcInterpolationMethod = ArcInterpolationMethod.spherical,
        pointCount: int = 12
        ) -> str:

        vertexPositionVectorArray = [];
        backwardPositionVectorArray = [];
        forwardPositionVectorArray = [];
        interpolatedPositionVectorArray = [];

        backwardPositionVector = ArcVector3( 0.0, 0.0, 0.0 );
        forwardPositionVector = ArcVector3( 0.0, 0.0, 0.0 );
        interpolatedPositionVector = ArcVector3( 0.0, 0.0, 0.0 );

        for vertex in self.vertexArray:
            vertex.setBackwardPositionVector( backwardPositionVector );
            vertex.setForwardPositionVector( forwardPositionVector );

            vertexPositionVectorArray.append( ArcVector3( vertex.positionVector.x, vertex.positionVector.y, vertex.positionVector.z ) );
            backwardPositionVectorArray.append( ArcVector3( backwardPositionVector.x, backwardPositionVector.y, backwardPositionVector.z ) );
            forwardPositionVectorArray.append( ArcVector3( forwardPositionVector.x, forwardPositionVector.y, forwardPositionVector.z ) );

        for arcIndex in range( self.arcCount ):

            firstVertex = self.getArcFirstVertex( arcIndex );
            secondVertex = self.getArcSecondVertex( arcIndex );
            oneOverPointCount = 1.0 / pointCount;

            for pointIndex in range( pointCount + 1 ):
                interpolationFactor = pointIndex * oneOverPointCount;

                if ( interpolationMethod == ArcInterpolationMethod.spherical ):
                    interpolatedPositionVector.setSphericalArcPositionVector( firstVertex.positionVector, firstVertex.forwardVector, secondVertex.positionVector, secondVertex.backwardVector, interpolationFactor );
                elif ( interpolationMethod == ArcInterpolationMethod.quadratic ):
                    interpolatedPositionVector.setQuadraticArcPositionVector( firstVertex.positionVector, firstVertex.forwardVector, secondVertex.positionVector, secondVertex.backwardVector, interpolationFactor );

                else:
                    raise ValueError( f"Invalid interpolation method: {interpolationMethod}" );

                interpolatedPositionVectorArray.append( ArcVector3( interpolatedPositionVector.x, interpolatedPositionVector.y, interpolatedPositionVector.z ) );

        positionVectorArray = (
            vertexPositionVectorArray +
            backwardPositionVectorArray +
            forwardPositionVectorArray +
            interpolatedPositionVectorArray
            );

        minimumPositionVector = ArcVector3( float( 'inf' ), float( 'inf' ), float( 'inf' ) );
        maximumPositionVector = ArcVector3( float( '-inf' ), float( '-inf' ), float( '-inf' ) );

        for positionVector in positionVectorArray:

            minimumPositionVector.x = min( minimumPositionVector.x, positionVector.x );
            minimumPositionVector.y = min( minimumPositionVector.y, positionVector.y );
            minimumPositionVector.z = min( minimumPositionVector.z, positionVector.z );
            maximumPositionVector.x = max( maximumPositionVector.x, positionVector.x );
            maximumPositionVector.y = max( maximumPositionVector.y, positionVector.y );
            maximumPositionVector.z = max( maximumPositionVector.z, positionVector.z );

        boundingBoxWidth = max( maximumPositionVector.x - minimumPositionVector.x, 0.001 );
        boundingBoxHeight = max( maximumPositionVector.y - minimumPositionVector.y, 0.001 );
        margin = 15.0;

        width = 1000.0;
        height = width * ( boundingBoxHeight / boundingBoxWidth );

        xScale = ( width - 2 * margin ) / boundingBoxWidth;
        yScale = ( height - 2 * margin ) / boundingBoxHeight;
        scale = min( xScale, yScale );
        
        pixelSize = 1.0 / scale;
        strokeWidth = 3.0 * pixelSize;
        circleRadius = 6.0 * pixelSize;

        centerX = ( minimumPositionVector.x + maximumPositionVector.x ) / 2.0;
        centerY = ( minimumPositionVector.y + maximumPositionVector.y ) / 2.0;

        translateX = width / 2.0 - centerX * scale;
        translateY = height / 2.0 + centerY * scale;

        svgDrawing = f'<svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">\n';
        svgDrawing += f'  <g transform="translate({translateX}, {translateY}) scale({scale}, -{scale})">\n';

        for i in range( len( self.vertexArray ) ):

            vertex = self.vertexArray[ i ];
            backwardPos = backwardPositionVectorArray[ i ];
            svgDrawing += f'    <line x1="{vertex.positionVector.x}" y1="{vertex.positionVector.y}" x2="{backwardPos.x}" y2="{backwardPos.y}" stroke="rgb(255,192,192)" stroke-width="{strokeWidth}"/>\n';

        for i in range( len( self.vertexArray ) ):
            
            vertex = self.vertexArray[ i ];
            forwardPos = forwardPositionVectorArray[ i ];
            svgDrawing += f'    <line x1="{vertex.positionVector.x}" y1="{vertex.positionVector.y}" x2="{forwardPos.x}" y2="{forwardPos.y}" stroke="rgb(192,192,255)" stroke-width="{strokeWidth}"/>\n';

        for vertexPos in vertexPositionVectorArray:

            svgDrawing += f'    <circle cx="{vertexPos.x}" cy="{vertexPos.y}" r="{2 * circleRadius}" fill="rgb(255,128,255)"/>\n';

        for interpolatedPos in interpolatedPositionVectorArray:

            svgDrawing += f'    <circle cx="{interpolatedPos.x}" cy="{interpolatedPos.y}" r="{circleRadius}" fill="rgb(0,0,255)"/>\n';

        svgDrawing += '  </g>\n</svg>';

        return svgDrawing;
