import 'dart:math';

// -- TYPES

class ArcVector3
{
    // -- ATTRIBUTES

    double 
        x, 
        y, 
        z;
    
    // -- CONSTRUCTORS

    ArcVector3(
        {
            this.x = 0.0,
            this.y = 0.0,
            this.z = 0.0
        }
        );

    // ~~

    ArcVector3 copy(
        )
    {
        return ArcVector3( x: x, y: y, z: z );
    }

    // -- OPERATIONS

    void setQuadraticArcPositionVector(
        ArcVector3 firstPositionVector,
        ArcVector3 firstForwardVector,
        ArcVector3 secondPositionVector,
        ArcVector3 secondBackwardVector,
        double interpolationFactor
        )
    {
        double interpolationFactorComplement = 1.0 - interpolationFactor;

        double firstQuadraticPositionX = firstPositionVector.x + firstForwardVector.x * interpolationFactor;
        double firstQuadraticPositionY = firstPositionVector.y + firstForwardVector.y * interpolationFactor;
        double firstQuadraticPositionZ = firstPositionVector.z + firstForwardVector.z * interpolationFactor;
        double secondQuadraticPositionX = secondPositionVector.x + secondBackwardVector.x * interpolationFactorComplement;
        double secondQuadraticPositionY = secondPositionVector.y + secondBackwardVector.y * interpolationFactorComplement;
        double secondQuadraticPositionZ = secondPositionVector.z + secondBackwardVector.z * interpolationFactorComplement;

        double secondQuadraticPositionFactor = ( 1.0 - cos( pi * interpolationFactor ) ) * 0.5;

        x = firstQuadraticPositionX + ( secondQuadraticPositionX - firstQuadraticPositionX ) * secondQuadraticPositionFactor;
        y = firstQuadraticPositionY + ( secondQuadraticPositionY - firstQuadraticPositionY ) * secondQuadraticPositionFactor;
        z = firstQuadraticPositionZ + ( secondQuadraticPositionZ - firstQuadraticPositionZ ) * secondQuadraticPositionFactor;
    }

    // ~~

    void setSphericalArcPositionVector(
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

        double interpolationFactorAngle = ( 1.0 - interpolationFactor ) * ( pi * 0.5 );
        double interpolationFactorCosinus = cos( interpolationFactorAngle );
        double interpolationFactorSinus = sin( interpolationFactorAngle );

        double firstSphericalPositionX = firstOriginPositionVectorX + firstForwardVector.x * interpolationFactorCosinus - firstResidualVectorX * interpolationFactorSinus;
        double firstSphericalPositionY = firstOriginPositionVectorY + firstForwardVector.y * interpolationFactorCosinus - firstResidualVectorY * interpolationFactorSinus;
        double firstSphericalPositionZ = firstOriginPositionVectorZ + firstForwardVector.z * interpolationFactorCosinus - firstResidualVectorZ * interpolationFactorSinus;
        double secondSphericalPositionX = secondOriginPositionVectorX + secondResidualVectorX * interpolationFactorCosinus + secondBackwardVector.x * interpolationFactorSinus;
        double secondSphericalPositionY = secondOriginPositionVectorY + secondResidualVectorY * interpolationFactorCosinus + secondBackwardVector.y * interpolationFactorSinus;
        double secondSphericalPositionZ = secondOriginPositionVectorZ + secondResidualVectorZ * interpolationFactorCosinus + secondBackwardVector.z * interpolationFactorSinus;

        double secondSphericalPositionFactor = 1.0 - interpolationFactorSinus;

        x = firstSphericalPositionX + ( secondSphericalPositionX - firstSphericalPositionX ) * secondSphericalPositionFactor;
        y = firstSphericalPositionY + ( secondSphericalPositionY - firstSphericalPositionY ) * secondSphericalPositionFactor;
        z = firstSphericalPositionZ + ( secondSphericalPositionZ - firstSphericalPositionZ ) * secondSphericalPositionFactor;
    }
}
