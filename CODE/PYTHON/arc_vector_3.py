# -- IMPORTS

from __future__ import annotations;
import math;
from typing import Optional;

# -- TYPES

class ArcVector3:

    # -- CONSTRUCTORS

    def __init__(
        self,
        x: float = 0.0,
        y: float = 0.0,
        z: float = 0.0
        ) -> None:

        self.x = x;
        self.y = y;
        self.z = z;

    # -- OPERATIONS

    def setQuadraticArcPositionVector(
        self,
        firstPositionVector: ArcVector3,
        firstForwardVector: ArcVector3,
        secondPositionVector: ArcVector3,
        secondBackwardVector: ArcVector3,
        interpolationFactor: float
        ) -> None:

        interpolationFactorComplement = 1.0 - interpolationFactor;

        firstQuadraticPositionX = firstPositionVector.x + firstForwardVector.x * interpolationFactor;
        firstQuadraticPositionY = firstPositionVector.y + firstForwardVector.y * interpolationFactor;
        firstQuadraticPositionZ = firstPositionVector.z + firstForwardVector.z * interpolationFactor;
        secondQuadraticPositionX = secondPositionVector.x + secondBackwardVector.x * interpolationFactorComplement;
        secondQuadraticPositionY = secondPositionVector.y + secondBackwardVector.y * interpolationFactorComplement;
        secondQuadraticPositionZ = secondPositionVector.z + secondBackwardVector.z * interpolationFactorComplement;

        secondQuadraticPositionFactor = ( 1.0 - math.cos( math.pi * interpolationFactor ) ) * 0.5;

        self.x = firstQuadraticPositionX + ( secondQuadraticPositionX - firstQuadraticPositionX ) * secondQuadraticPositionFactor;
        self.y = firstQuadraticPositionY + ( secondQuadraticPositionY - firstQuadraticPositionY ) * secondQuadraticPositionFactor;
        self.z = firstQuadraticPositionZ + ( secondQuadraticPositionZ - firstQuadraticPositionZ ) * secondQuadraticPositionFactor;

    # ~~

    def setSphericalArcPositionVector(
        self,
        firstPositionVector: ArcVector3,
        firstForwardVector: ArcVector3,
        secondPositionVector: ArcVector3,
        secondBackwardVector: ArcVector3,
        interpolationFactor: float
        ) -> None:

        firstForwardPositionVectorX = firstPositionVector.x + firstForwardVector.x;
        firstForwardPositionVectorY = firstPositionVector.y + firstForwardVector.y;
        firstForwardPositionVectorZ = firstPositionVector.z + firstForwardVector.z;

        secondBackwardPositionVectorX = secondPositionVector.x + secondBackwardVector.x;
        secondBackwardPositionVectorY = secondPositionVector.y + secondBackwardVector.y;
        secondBackwardPositionVectorZ = secondPositionVector.z + secondBackwardVector.z;

        firstResidualVectorX = secondPositionVector.x - firstForwardPositionVectorX;
        firstResidualVectorY = secondPositionVector.y - firstForwardPositionVectorY;
        firstResidualVectorZ = secondPositionVector.z - firstForwardPositionVectorZ;
        secondResidualVectorX = secondBackwardPositionVectorX - firstPositionVector.x;
        secondResidualVectorY = secondBackwardPositionVectorY - firstPositionVector.y;
        secondResidualVectorZ = secondBackwardPositionVectorZ - firstPositionVector.z;

        firstOriginPositionVectorX = firstPositionVector.x + firstResidualVectorX;
        firstOriginPositionVectorY = firstPositionVector.y + firstResidualVectorY;
        firstOriginPositionVectorZ = firstPositionVector.z + firstResidualVectorZ;
        secondOriginPositionVectorX = secondPositionVector.x - secondResidualVectorX;
        secondOriginPositionVectorY = secondPositionVector.y - secondResidualVectorY;
        secondOriginPositionVectorZ = secondPositionVector.z - secondResidualVectorZ;

        interpolationFactorAngle = ( 1.0 - interpolationFactor ) * ( math.pi * 0.5 );
        interpolationFactorCosinus = math.cos( interpolationFactorAngle );
        interpolationFactorSinus = math.sin( interpolationFactorAngle );

        firstSphericalPositionX = firstOriginPositionVectorX + firstForwardVector.x * interpolationFactorCosinus - firstResidualVectorX * interpolationFactorSinus;
        firstSphericalPositionY = firstOriginPositionVectorY + firstForwardVector.y * interpolationFactorCosinus - firstResidualVectorY * interpolationFactorSinus;
        firstSphericalPositionZ = firstOriginPositionVectorZ + firstForwardVector.z * interpolationFactorCosinus - firstResidualVectorZ * interpolationFactorSinus;
        secondSphericalPositionX = secondOriginPositionVectorX + secondResidualVectorX * interpolationFactorCosinus + secondBackwardVector.x * interpolationFactorSinus;
        secondSphericalPositionY = secondOriginPositionVectorY + secondResidualVectorY * interpolationFactorCosinus + secondBackwardVector.y * interpolationFactorSinus;
        secondSphericalPositionZ = secondOriginPositionVectorZ + secondResidualVectorZ * interpolationFactorCosinus + secondBackwardVector.z * interpolationFactorSinus;

        secondSphericalPositionFactor = 1.0 - interpolationFactorSinus;

        self.x = firstSphericalPositionX + ( secondSphericalPositionX - firstSphericalPositionX ) * secondSphericalPositionFactor;
        self.y = firstSphericalPositionY + ( secondSphericalPositionY - firstSphericalPositionY ) * secondSphericalPositionFactor;
        self.z = firstSphericalPositionZ + ( secondSphericalPositionZ - firstSphericalPositionZ ) * secondSphericalPositionFactor;
