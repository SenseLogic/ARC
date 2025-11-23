// -- TYPES

using System;

// -- TYPES

namespace Arc
{
    public struct ArcVector3
    {
        // -- ATTRIBUTES

        public double
            x,
            y,
            z;

        // -- CONSTRUCTORS

        public ArcVector3(
            double x,
            double y,
            double z
            )
        {
            this.x = x;
            this.y = y;
            this.z = z;
        }

        // ~~

        public ArcVector3(
            ArcVector3 other
            )
        {
            this.x = other.x;
            this.y = other.y;
            this.z = other.z;
        }

        // -- OPERATIONS

        public void SetQuadraticArcPositionVector(
            ArcVector3 firstPositionVector,
            ArcVector3 firstForwardVector,
            ArcVector3 secondPositionVector,
            ArcVector3 secondBackwardVector,
            double interpolationFactor
            )
        {
            double interpolationFactorComplement = 1 - interpolationFactor;

            double firstQuadraticPositionX = firstPositionVector.x + firstForwardVector.x * interpolationFactor;
            double firstQuadraticPositionY = firstPositionVector.y + firstForwardVector.y * interpolationFactor;
            double firstQuadraticPositionZ = firstPositionVector.z + firstForwardVector.z * interpolationFactor;
            double secondQuadraticPositionX = secondPositionVector.x + secondBackwardVector.x * interpolationFactorComplement;
            double secondQuadraticPositionY = secondPositionVector.y + secondBackwardVector.y * interpolationFactorComplement;
            double secondQuadraticPositionZ = secondPositionVector.z + secondBackwardVector.z * interpolationFactorComplement;

            double secondQuadraticPositionFactor = ( 1 - Math.Cos( Math.PI * interpolationFactor ) ) * 0.5;

            x = firstQuadraticPositionX + ( secondQuadraticPositionX - firstQuadraticPositionX ) * secondQuadraticPositionFactor;
            y = firstQuadraticPositionY + ( secondQuadraticPositionY - firstQuadraticPositionY ) * secondQuadraticPositionFactor;
            z = firstQuadraticPositionZ + ( secondQuadraticPositionZ - firstQuadraticPositionZ ) * secondQuadraticPositionFactor;
        }

        // ~~

        public void SetSphericalArcPositionVector(
            ArcVector3 firstPositionVector,
            ArcVector3 firstForwardVector,
            ArcVector3 secondPositionVector,
            ArcVector3 secondBackwardVector,
            double interpolationFactor
            )
        {
            double firstForwardPositionVectorX = firstPositionVector.x + firstForwardVector.x;
            double firstForwardPositionVectorY = firstPositionVector.y + firstForwardVector.y;
            double firstForwardPositionVectorZ = firstPositionVector.z + firstForwardVector.z;

            double secondBackwardPositionVectorX = secondPositionVector.x + secondBackwardVector.x;
            double secondBackwardPositionVectorY = secondPositionVector.y + secondBackwardVector.y;
            double secondBackwardPositionVectorZ = secondPositionVector.z + secondBackwardVector.z;

            double firstResidualVectorX = secondPositionVector.x - firstForwardPositionVectorX;
            double firstResidualVectorY = secondPositionVector.y - firstForwardPositionVectorY;
            double firstResidualVectorZ = secondPositionVector.z - firstForwardPositionVectorZ;
            double secondResidualVectorX = secondBackwardPositionVectorX - firstPositionVector.x;
            double secondResidualVectorY = secondBackwardPositionVectorY - firstPositionVector.y;
            double secondResidualVectorZ = secondBackwardPositionVectorZ - firstPositionVector.z;

            double firstOriginPositionVectorX = firstPositionVector.x + firstResidualVectorX;
            double firstOriginPositionVectorY = firstPositionVector.y + firstResidualVectorY;
            double firstOriginPositionVectorZ = firstPositionVector.z + firstResidualVectorZ;
            double secondOriginPositionVectorX = secondPositionVector.x - secondResidualVectorX;
            double secondOriginPositionVectorY = secondPositionVector.y - secondResidualVectorY;
            double secondOriginPositionVectorZ = secondPositionVector.z - secondResidualVectorZ;

            double interpolationFactorAngle = ( 1 - interpolationFactor ) * ( Math.PI * 0.5 );
            double interpolationFactorCosinus = Math.Cos( interpolationFactorAngle );
            double interpolationFactorSinus = Math.Sin( interpolationFactorAngle );

            double firstSphericalPositionX = firstOriginPositionVectorX + firstForwardVector.x * interpolationFactorCosinus - firstResidualVectorX * interpolationFactorSinus;
            double firstSphericalPositionY = firstOriginPositionVectorY + firstForwardVector.y * interpolationFactorCosinus - firstResidualVectorY * interpolationFactorSinus;
            double firstSphericalPositionZ = firstOriginPositionVectorZ + firstForwardVector.z * interpolationFactorCosinus - firstResidualVectorZ * interpolationFactorSinus;
            double secondSphericalPositionX = secondOriginPositionVectorX + secondResidualVectorX * interpolationFactorCosinus + secondBackwardVector.x * interpolationFactorSinus;
            double secondSphericalPositionY = secondOriginPositionVectorY + secondResidualVectorY * interpolationFactorCosinus + secondBackwardVector.y * interpolationFactorSinus;
            double secondSphericalPositionZ = secondOriginPositionVectorZ + secondResidualVectorZ * interpolationFactorCosinus + secondBackwardVector.z * interpolationFactorSinus;

            double secondSphericalPositionFactor = 1 - interpolationFactorSinus;

            x = firstSphericalPositionX + ( secondSphericalPositionX - firstSphericalPositionX ) * secondSphericalPositionFactor;
            y = firstSphericalPositionY + ( secondSphericalPositionY - firstSphericalPositionY ) * secondSphericalPositionFactor;
            z = firstSphericalPositionZ + ( secondSphericalPositionZ - firstSphericalPositionZ ) * secondSphericalPositionFactor;
        }
    }
}
