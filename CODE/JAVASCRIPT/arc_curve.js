// -- IMPORTS

import { ArcBackwardVectorType } from './arc_backward_vector_type.js';
import { ArcInterpolationMethod } from './arc_interpolation_method.js';
import { setQuadraticArcPositionVector, setSphericalArcPositionVector } from './arc_vector_3.js';

// -- TYPES

export class ArcCurve
{
    // -- CONSTRUCTORS

    constructor(
        vertexArray,
        arcCount
        )
    {
        this.vertexArray = vertexArray;
        this.arcCount = arcCount;
    }

    // -- INQUIRIES

    getArcFirstVertex(
        arcIndex
        )
    {
        return this.vertexArray[ arcIndex ];
    }

    // ~~

    getArcSecondVertex(
        arcIndex
        )
    {
        let secondVertexIndex = arcIndex + 1;

        if ( secondVertexIndex === this.vertexArray.length )
        {
            secondVertexIndex = 0;
        }

        return this.vertexArray[ secondVertexIndex ];
    }

    // -- OPERATIONS

    addVertex(
        vertex
        )
    {
        this.vertexArray.push( vertex );
        this.arcCount++;
    }

    // ~~

    updateBackwardVectors(
        )
    {
        for ( let arcIndex = 0; arcIndex < this.arcCount; ++arcIndex )
        {
            let arcSecondVertex = this.getArcSecondVertex( arcIndex );

            if ( arcSecondVertex.backwardVectorType === ArcBackwardVectorType.symmetrical )
            {
                arcSecondVertex.setSymmetricalBackwardVector();
            }
            else if ( arcSecondVertex.backwardVectorType === ArcBackwardVectorType.residual )
            {
                arcSecondVertex.setResidualBackwardVector( this.getArcFirstVertex( arcIndex ) );
            }
        }
    }

    // ~~

    getSvgDrawing(
        interpolationMethod = ArcInterpolationMethod.spherical,
        pointCount = 12
        )
    {
        let vertexPositionVectorArray = [];
        let backwardPositionVectorArray = [];
        let forwardPositionVectorArray = [];
        let interpolatedPositionVectorArray = [];

        let backwardPositionVector = { x: 0, y: 0, z: 0 };
        let forwardPositionVector = { x: 0, y: 0, z: 0 };
        let interpolatedPositionVector = { x: 0, y: 0, z: 0 };

        for ( let vertex of this.vertexArray )
        {
            vertex.setBackwardPositionVector( backwardPositionVector );
            vertex.setForwardPositionVector( forwardPositionVector );

            vertexPositionVectorArray.push( { ...vertex.positionVector } );
            backwardPositionVectorArray.push( { ...backwardPositionVector } );
            forwardPositionVectorArray.push( { ...forwardPositionVector } );
        }

        for ( let arcIndex = 0; arcIndex < this.arcCount; ++arcIndex )
        {
            let firstVertex = this.getArcFirstVertex( arcIndex );
            let secondVertex = this.getArcSecondVertex( arcIndex );
            let oneOverPointCount = 1 / pointCount;

            for ( let pointIndex = 0; pointIndex <= pointCount; ++pointIndex )
            {
                let interpolationFactor = pointIndex * oneOverPointCount;

                if ( interpolationMethod === ArcInterpolationMethod.spherical )
                {
                    setSphericalArcPositionVector( interpolatedPositionVector, firstVertex.positionVector, firstVertex.forwardVector, secondVertex.positionVector, secondVertex.backwardVector, interpolationFactor );
                }
                else if ( interpolationMethod === ArcInterpolationMethod.quadratic )
                {
                    setQuadraticArcPositionVector( interpolatedPositionVector, firstVertex.positionVector, firstVertex.forwardVector, secondVertex.positionVector, secondVertex.backwardVector, interpolationFactor );
                }
                else
                {
                    throw new Error( `Invalid interpolation method: ${interpolationMethod}` );
                }

                interpolatedPositionVectorArray.push( { ...interpolatedPositionVector } );
            }
        }

        let positionVectorArray =
            [
                ...vertexPositionVectorArray,
                ...backwardPositionVectorArray,
                ...forwardPositionVectorArray,
                ...interpolatedPositionVectorArray
            ];

        let minimumPositionVector = { x: Infinity, y: Infinity, z: Infinity };
        let maximumPositionVector = { x: -Infinity, y: -Infinity, z: -Infinity };

        for ( let positionVector of positionVectorArray )
        {
            minimumPositionVector.x = Math.min( minimumPositionVector.x, positionVector.x );
            minimumPositionVector.y = Math.min( minimumPositionVector.y, positionVector.y );
            minimumPositionVector.z = Math.min( minimumPositionVector.z, positionVector.z );
            maximumPositionVector.x = Math.max( maximumPositionVector.x, positionVector.x );
            maximumPositionVector.y = Math.max( maximumPositionVector.y, positionVector.y );
            maximumPositionVector.z = Math.max( maximumPositionVector.z, positionVector.z );
        }

        let boundingBoxWidth = Math.max( maximumPositionVector.x - minimumPositionVector.x, 0.001 );
        let boundingBoxHeight = Math.max( maximumPositionVector.y - minimumPositionVector.y, 0.001 );
        let margin = 15;

        let width = 1000;
        let height = width * ( boundingBoxHeight / boundingBoxWidth );

        let xScale = ( width - 2 * margin ) / boundingBoxWidth;
        let yScale = ( height - 2 * margin ) / boundingBoxHeight;
        let scale = Math.min( xScale, yScale );

        let pixelSize = 1 / scale;
        let strokeWidth = 3 * pixelSize;
        let circleRadius = 6 * pixelSize;

        let centerX = ( minimumPositionVector.x + maximumPositionVector.x ) / 2;
        let centerY = ( minimumPositionVector.y + maximumPositionVector.y ) / 2;

        let translateX = width / 2 - centerX * scale;
        let translateY = height / 2 + centerY * scale;

        let svgDrawing = `<svg width="${width}" height="${height}" xmlns="http://www.w3.org/2000/svg">\n`;
        svgDrawing += `  <g transform="translate(${translateX}, ${translateY}) scale(${scale}, -${scale})">\n`;

        for ( let vertexIndex = 0; vertexIndex < this.vertexArray.length; ++vertexIndex )
        {
            let vertex = this.vertexArray[ vertexIndex ];
            let backwardPositionVector = backwardPositionVectorArray[ vertexIndex ];
            svgDrawing += `    <line x1="${vertex.positionVector.x}" y1="${vertex.positionVector.y}" x2="${backwardPositionVector.x}" y2="${backwardPositionVector.y}" stroke="rgb( 255, 192, 192 )" stroke-width="${strokeWidth}"/>\n`;
        }

        for ( let vertexIndex = 0; vertexIndex < this.vertexArray.length; ++vertexIndex )
        {
            let vertex = this.vertexArray[ vertexIndex ];
            let forwardPositionVector = forwardPositionVectorArray[ vertexIndex ];
            svgDrawing += `    <line x1="${vertex.positionVector.x}" y1="${vertex.positionVector.y}" x2="${forwardPositionVector.x}" y2="${forwardPositionVector.y}" stroke="rgb( 192, 192, 255 )" stroke-width="${strokeWidth}"/>\n`;
        }

        for ( let vertexPositionVector of vertexPositionVectorArray )
        {
            svgDrawing += `    <circle cx="${vertexPositionVector.x}" cy="${vertexPositionVector.y}" r="${2 * circleRadius}" fill="rgb( 255, 128, 255 )"/>\n`;
        }

        for ( let interpolatedPositionVector of interpolatedPositionVectorArray )
        {
            svgDrawing += `    <circle cx="${interpolatedPositionVector.x}" cy="${interpolatedPositionVector.y}" r="${circleRadius}" fill="rgb( 0, 0, 255 )"/>\n`;
        }

        svgDrawing += `  </g>\n</svg>`;

        return svgDrawing;
    }
}
