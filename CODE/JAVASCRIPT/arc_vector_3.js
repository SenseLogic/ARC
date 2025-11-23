// -- FUNCTIONS

export function setQuadraticArcPositionVector(
    arcPositionVector,
    firstPositionVector,
    firstForwardVector,
    secondPositionVector,
    secondBackwardVector,
    interpolationFactor
    )
{
    let interpolationFactorComplement = 1 - interpolationFactor;

    let firstQuadraticPositionX = firstPositionVector.x + firstForwardVector.x * interpolationFactor;
    let firstQuadraticPositionY = firstPositionVector.y + firstForwardVector.y * interpolationFactor;
    let firstQuadraticPositionZ = firstPositionVector.z + firstForwardVector.z * interpolationFactor;
    let secondQuadraticPositionX = secondPositionVector.x + secondBackwardVector.x * interpolationFactorComplement;
    let secondQuadraticPositionY = secondPositionVector.y + secondBackwardVector.y * interpolationFactorComplement;
    let secondQuadraticPositionZ = secondPositionVector.z + secondBackwardVector.z * interpolationFactorComplement;

    let secondQuadraticPositionFactor = ( 1 - Math.cos( Math.PI * interpolationFactor ) ) * 0.5;

    arcPositionVector.x = firstQuadraticPositionX + ( secondQuadraticPositionX - firstQuadraticPositionX ) * secondQuadraticPositionFactor;
    arcPositionVector.y = firstQuadraticPositionY + ( secondQuadraticPositionY - firstQuadraticPositionY ) * secondQuadraticPositionFactor;
    arcPositionVector.z = firstQuadraticPositionZ + ( secondQuadraticPositionZ - firstQuadraticPositionZ ) * secondQuadraticPositionFactor;
}

// ~~

export function setSphericalArcPositionVector(
    arcPositionVector,
    firstPositionVector,
    firstForwardVector,
    secondPositionVector,
    secondBackwardVector,
    interpolationFactor
    )
{
    let firstForwardPositionVectorX = firstPositionVector.x + firstForwardVector.x;
    let firstForwardPositionVectorY = firstPositionVector.y + firstForwardVector.y;
    let firstForwardPositionVectorZ = firstPositionVector.z + firstForwardVector.z;

    let secondBackwardPositionVectorX = secondPositionVector.x + secondBackwardVector.x;
    let secondBackwardPositionVectorY = secondPositionVector.y + secondBackwardVector.y;
    let secondBackwardPositionVectorZ = secondPositionVector.z + secondBackwardVector.z;

    let firstResidualVectorX = secondPositionVector.x - firstForwardPositionVectorX;
    let firstResidualVectorY = secondPositionVector.y - firstForwardPositionVectorY;
    let firstResidualVectorZ = secondPositionVector.z - firstForwardPositionVectorZ;
    let secondResidualVectorX = secondBackwardPositionVectorX - firstPositionVector.x;
    let secondResidualVectorY = secondBackwardPositionVectorY - firstPositionVector.y;
    let secondResidualVectorZ = secondBackwardPositionVectorZ - firstPositionVector.z;

    let firstOriginPositionVectorX = firstPositionVector.x + firstResidualVectorX;
    let firstOriginPositionVectorY = firstPositionVector.y + firstResidualVectorY;
    let firstOriginPositionVectorZ = firstPositionVector.z + firstResidualVectorZ;
    let secondOriginPositionVectorX = secondPositionVector.x - secondResidualVectorX;
    let secondOriginPositionVectorY = secondPositionVector.y - secondResidualVectorY;
    let secondOriginPositionVectorZ = secondPositionVector.z - secondResidualVectorZ;

    let interpolationFactorAngle = ( 1 - interpolationFactor ) * ( Math.PI * 0.5 );
    let interpolationFactorCosinus = Math.cos( interpolationFactorAngle );
    let interpolationFactorSinus = Math.sin( interpolationFactorAngle );

    let firstSphericalPositionX = firstOriginPositionVectorX + firstForwardVector.x * interpolationFactorCosinus - firstResidualVectorX * interpolationFactorSinus;
    let firstSphericalPositionY = firstOriginPositionVectorY + firstForwardVector.y * interpolationFactorCosinus - firstResidualVectorY * interpolationFactorSinus;
    let firstSphericalPositionZ = firstOriginPositionVectorZ + firstForwardVector.z * interpolationFactorCosinus - firstResidualVectorZ * interpolationFactorSinus;
    let secondSphericalPositionX = secondOriginPositionVectorX + secondResidualVectorX * interpolationFactorCosinus + secondBackwardVector.x * interpolationFactorSinus;
    let secondSphericalPositionY = secondOriginPositionVectorY + secondResidualVectorY * interpolationFactorCosinus + secondBackwardVector.y * interpolationFactorSinus;
    let secondSphericalPositionZ = secondOriginPositionVectorZ + secondResidualVectorZ * interpolationFactorCosinus + secondBackwardVector.z * interpolationFactorSinus;

    let secondSphericalPositionFactor = 1 - interpolationFactorSinus;

    arcPositionVector.x = firstSphericalPositionX + ( secondSphericalPositionX - firstSphericalPositionX ) * secondSphericalPositionFactor;
    arcPositionVector.y = firstSphericalPositionY + ( secondSphericalPositionY - firstSphericalPositionY ) * secondSphericalPositionFactor;
    arcPositionVector.z = firstSphericalPositionZ + ( secondSphericalPositionZ - firstSphericalPositionZ ) * secondSphericalPositionFactor;
}
