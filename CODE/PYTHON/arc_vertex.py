# -- IMPORTS

from __future__ import annotations;
from typing import Optional;
from arc_backward_vector_type import ArcBackwardVectorType;
from arc_vector_3 import ArcVector3;

# -- TYPES

class ArcVertex:

    # -- CONSTRUCTORS

    def __init__(
        self,
        positionVector: ArcVector3,
        forwardVector: ArcVector3,
        backwardVector: Optional[ ArcVector3 ] = None,
        backwardVectorType: ArcBackwardVectorType = ArcBackwardVectorType.custom
        ) -> None:

        self.positionVector = positionVector;
        self.forwardVector = forwardVector;
        self.backwardVectorType = backwardVectorType;

        if ( backwardVectorType == ArcBackwardVectorType.custom ):
            self.backwardVector = backwardVector if backwardVector else ArcVector3( 0, 0, 0 );
        elif ( backwardVectorType == ArcBackwardVectorType.symmetrical ):
            self.backwardVector = ArcVector3( -forwardVector.x, -forwardVector.y, -forwardVector.z );

        else:
            self.backwardVector = ArcVector3( 0, 0, 0 );

    # -- OPERATIONS

    def setForwardPositionVector(
        self,
        forwardPositionVector: ArcVector3
        ) -> None:

        forwardPositionVector.x = self.positionVector.x + self.forwardVector.x;
        forwardPositionVector.y = self.positionVector.y + self.forwardVector.y;
        forwardPositionVector.z = self.positionVector.z + self.forwardVector.z;

    # ~~

    def setBackwardPositionVector(
        self,
        backwardPositionVector: ArcVector3
        ) -> None:

        backwardPositionVector.x = self.positionVector.x + self.backwardVector.x;
        backwardPositionVector.y = self.positionVector.y + self.backwardVector.y;
        backwardPositionVector.z = self.positionVector.z + self.backwardVector.z;

    # ~~

    def setSymmetricalBackwardVector(
        self
        ) -> None:

        self.backwardVector.x = -self.forwardVector.x;
        self.backwardVector.y = -self.forwardVector.y;
        self.backwardVector.z = -self.forwardVector.z;

    # ~~

    def setResidualBackwardVector(
        self,
        firstVertex: ArcVertex
        ) -> None:

        self.backwardVector.x = firstVertex.positionVector.x + firstVertex.forwardVector.x - self.positionVector.x;
        self.backwardVector.y = firstVertex.positionVector.y + firstVertex.forwardVector.y - self.positionVector.y;
        self.backwardVector.z = firstVertex.positionVector.z + firstVertex.forwardVector.z - self.positionVector.z;
