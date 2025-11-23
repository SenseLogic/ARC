// -- IMPORTS

using System;
using System.Collections.Generic;
using System.Globalization;

// -- TYPES

namespace Arc
{
    public class ArcCurve
    {
        // -- ATTRIBUTES

        public List<ArcVertex>
            vertexArray;
        public int
            arcCount;

        // -- CONSTRUCTORS

        public ArcCurve(
            List<ArcVertex> vertexArray,
            int arcCount
            )
        {
            this.vertexArray = vertexArray;
            this.arcCount = arcCount;
        }

        // -- INQUIRIES

        public ArcVertex GetArcFirstVertex(
            int arcIndex
            )
        {
            return this.vertexArray[ arcIndex ];
        }

        // ~~

        public ArcVertex GetArcSecondVertex(
            int arcIndex
            )
        {
            int secondVertexIndex = arcIndex + 1;

            if ( secondVertexIndex == this.vertexArray.Count )
            {
                secondVertexIndex = 0;
            }

            return this.vertexArray[ secondVertexIndex ];
        }

        // -- OPERATIONS

        public void AddVertex(
            ArcVertex vertex
            )
        {
            this.vertexArray.Add( vertex );
            this.arcCount++;
        }

        // ~~

        public void UpdateBackwardVectors(
            )
        {
            for ( int arcIndex = 0; arcIndex < this.arcCount; ++arcIndex )
            {
                ArcVertex arcSecondVertex = this.GetArcSecondVertex( arcIndex );

                if ( arcSecondVertex.backwardVectorType == ArcBackwardVectorType.symmetrical )
                {
                    arcSecondVertex.SetSymmetricalBackwardVector();
                }
                else if ( arcSecondVertex.backwardVectorType == ArcBackwardVectorType.residual )
                {
                    arcSecondVertex.SetResidualBackwardVector( this.GetArcFirstVertex( arcIndex ) );
                }
            }
        }

        // ~~

        public string GetSvgDrawing(
            int interpolationMethod = ArcInterpolationMethod.spherical,
            int pointCount = 12
            )
        {
            List<ArcVector3> vertexPositionVectorArray = new List<ArcVector3>();
            List<ArcVector3> backwardPositionVectorArray = new List<ArcVector3>();
            List<ArcVector3> forwardPositionVectorArray = new List<ArcVector3>();
            List<ArcVector3> interpolatedPositionVectorArray = new List<ArcVector3>();

            ArcVector3 backwardPositionVector = new ArcVector3( 0, 0, 0 );
            ArcVector3 forwardPositionVector = new ArcVector3( 0, 0, 0 );
            ArcVector3 interpolatedPositionVector = new ArcVector3( 0, 0, 0 );

            foreach ( ArcVertex vertex in this.vertexArray )
            {
                vertex.SetBackwardPositionVector( ref backwardPositionVector );
                vertex.SetForwardPositionVector( ref forwardPositionVector );

                vertexPositionVectorArray.Add( new ArcVector3( vertex.positionVector ) );
                backwardPositionVectorArray.Add( new ArcVector3( backwardPositionVector ) );
                forwardPositionVectorArray.Add( new ArcVector3( forwardPositionVector ) );
            }

            for ( int arcIndex = 0; arcIndex < this.arcCount; ++arcIndex )
            {
                ArcVertex firstVertex = this.GetArcFirstVertex( arcIndex );
                ArcVertex secondVertex = this.GetArcSecondVertex( arcIndex );
                double oneOverPointCount = 1.0 / pointCount;

                for ( int pointIndex = 0; pointIndex <= pointCount; ++pointIndex )
                {
                    double interpolationFactor = pointIndex * oneOverPointCount;

                    if ( interpolationMethod == ArcInterpolationMethod.spherical )
                    {
                        interpolatedPositionVector.SetSphericalArcPositionVector( firstVertex.positionVector, firstVertex.forwardVector, secondVertex.positionVector, secondVertex.backwardVector, interpolationFactor );
                    }
                    else if ( interpolationMethod == ArcInterpolationMethod.quadratic )
                    {
                        interpolatedPositionVector.SetQuadraticArcPositionVector( firstVertex.positionVector, firstVertex.forwardVector, secondVertex.positionVector, secondVertex.backwardVector, interpolationFactor );
                    }
                    else
                    {
                        throw new Exception( $"Invalid interpolation method: {interpolationMethod}" );
                    }

                    interpolatedPositionVectorArray.Add( new ArcVector3( interpolatedPositionVector ) );
                }
            }

            List<ArcVector3> positionVectorArray = new List<ArcVector3>();
            positionVectorArray.AddRange( vertexPositionVectorArray );
            positionVectorArray.AddRange( backwardPositionVectorArray );
            positionVectorArray.AddRange( forwardPositionVectorArray );
            positionVectorArray.AddRange( interpolatedPositionVectorArray );

            ArcVector3 minimumPositionVector = new ArcVector3( double.PositiveInfinity, double.PositiveInfinity, double.PositiveInfinity );
            ArcVector3 maximumPositionVector = new ArcVector3( double.NegativeInfinity, double.NegativeInfinity, double.NegativeInfinity );

            foreach ( ArcVector3 positionVector in positionVectorArray )
            {
                minimumPositionVector.x = Math.Min( minimumPositionVector.x, positionVector.x );
                minimumPositionVector.y = Math.Min( minimumPositionVector.y, positionVector.y );
                minimumPositionVector.z = Math.Min( minimumPositionVector.z, positionVector.z );
                maximumPositionVector.x = Math.Max( maximumPositionVector.x, positionVector.x );
                maximumPositionVector.y = Math.Max( maximumPositionVector.y, positionVector.y );
                maximumPositionVector.z = Math.Max( maximumPositionVector.z, positionVector.z );
            }

            double boundingBoxWidth = Math.Max( maximumPositionVector.x - minimumPositionVector.x, 0.001 );
            double boundingBoxHeight = Math.Max( maximumPositionVector.y - minimumPositionVector.y, 0.001 );
            double margin = 15;

            double width = 1000;
            double height = width * ( boundingBoxHeight / boundingBoxWidth );

            double xScale = ( width - 2 * margin ) / boundingBoxWidth;
            double yScale = ( height - 2 * margin ) / boundingBoxHeight;
            double scale = Math.Min( xScale, yScale );

            double pixelSize = 1.0 / scale;
            double strokeWidth = 3 * pixelSize;
            double circleRadius = 6 * pixelSize;

            double centerX = ( minimumPositionVector.x + maximumPositionVector.x ) / 2;
            double centerY = ( minimumPositionVector.y + maximumPositionVector.y ) / 2;

            double translateX = width / 2 - centerX * scale;
            double translateY = height / 2 + centerY * scale;

            string svgDrawing = $"<svg width=\"{width.ToString(CultureInfo.InvariantCulture)}\" height=\"{height.ToString(CultureInfo.InvariantCulture)}\" xmlns=\"http://www.w3.org/2000/svg\">\n";
            svgDrawing += $"  <g transform=\"translate({translateX.ToString(CultureInfo.InvariantCulture)}, {translateY.ToString(CultureInfo.InvariantCulture)}) scale({scale.ToString(CultureInfo.InvariantCulture)}, -{scale.ToString(CultureInfo.InvariantCulture)})\">\n";

            for ( int vertexIndex = 0; vertexIndex < this.vertexArray.Count; ++vertexIndex )
            {
                ArcVertex vertex = this.vertexArray[ vertexIndex ];
                ArcVector3 backwardPositionVector_ = backwardPositionVectorArray[ vertexIndex ];

                svgDrawing += $"    <line x1=\"{vertex.positionVector.x.ToString(CultureInfo.InvariantCulture)}\" y1=\"{vertex.positionVector.y.ToString(CultureInfo.InvariantCulture)}\" x2=\"{backwardPositionVector_.x.ToString(CultureInfo.InvariantCulture)}\" y2=\"{backwardPositionVector_.y.ToString(CultureInfo.InvariantCulture)}\" stroke=\"rgb( 255, 192, 192 )\" stroke-width=\"{strokeWidth.ToString(CultureInfo.InvariantCulture)}\"/>\n";
            }

            for ( int vertexIndex = 0; vertexIndex < this.vertexArray.Count; ++vertexIndex )
            {
                ArcVertex vertex = this.vertexArray[ vertexIndex ];
                ArcVector3 forwardPositionVector_ = forwardPositionVectorArray[ vertexIndex ];

                svgDrawing += $"    <line x1=\"{vertex.positionVector.x.ToString(CultureInfo.InvariantCulture)}\" y1=\"{vertex.positionVector.y.ToString(CultureInfo.InvariantCulture)}\" x2=\"{forwardPositionVector_.x.ToString(CultureInfo.InvariantCulture)}\" y2=\"{forwardPositionVector_.y.ToString(CultureInfo.InvariantCulture)}\" stroke=\"rgb( 192, 192, 255 )\" stroke-width=\"{strokeWidth.ToString(CultureInfo.InvariantCulture)}\"/>\n";
            }

            foreach ( ArcVector3 vertexPositionVector in vertexPositionVectorArray )
            {
                svgDrawing += $"    <circle cx=\"{vertexPositionVector.x.ToString(CultureInfo.InvariantCulture)}\" cy=\"{vertexPositionVector.y.ToString(CultureInfo.InvariantCulture)}\" r=\"{(2 * circleRadius).ToString(CultureInfo.InvariantCulture)}\" fill=\"rgb( 255, 128, 255 )\"/>\n";
            }

            foreach ( ArcVector3 interpolatedPositionVector_ in interpolatedPositionVectorArray )
            {
                svgDrawing += $"    <circle cx=\"{interpolatedPositionVector_.x.ToString(CultureInfo.InvariantCulture)}\" cy=\"{interpolatedPositionVector_.y.ToString(CultureInfo.InvariantCulture)}\" r=\"{circleRadius.ToString(CultureInfo.InvariantCulture)}\" fill=\"rgb( 0, 0, 255 )\"/>\n";
            }

            svgDrawing += "  </g>\n</svg>";

            return svgDrawing;
        }
    }
}
