// -- IMPORTS

import { ArcBackwardVectorType } from './arc_backward_vector_type.js';

// -- TYPES

export class ArcVertex
{
    // -- CONSTRUCTORS

    constructor(
        {
            positionVector,
            forwardVector,
            backwardVector,
            backwardVectorType = ArcBackwardVectorType.custom
        } = {}
        )
    {
        this.positionVector = { ...positionVector };
        this.forwardVector = { ...forwardVector };
        this.backwardVectorType = backwardVectorType;

        if ( backwardVectorType === ArcBackwardVectorType.custom )
        {
            this.backwardVector =  { ...backwardVector };
        }
        else if ( backwardVectorType === ArcBackwardVectorType.symmetrical )
        {
            this.backwardVector = { x: -forwardVector.x, y: -forwardVector.y, z: -forwardVector.z };
        }
        else
        {
            this.backwardVector = { x: 0, y: 0, z: 0 };
        }
    }

    // -- OPERATIONS

    setSymmetricalBackwardVector(
        )
    {
        this.backwardVector.x = -this.forwardVector.x;
        this.backwardVector.y = -this.forwardVector.y;
        this.backwardVector.z = -this.forwardVector.z;
    }

    // ~~

    setResidualBackwardVector(
        arcFirstVertex
        )
    {
        this.backwardVector.x = arcFirstVertex.positionVector.x + arcFirstVertex.forwardVector.x - this.positionVector.x;
        this.backwardVector.y = arcFirstVertex.positionVector.y + arcFirstVertex.forwardVector.y - this.positionVector.y;
        this.backwardVector.z = arcFirstVertex.positionVector.z + arcFirstVertex.forwardVector.z - this.positionVector.z;
    }

    // ~~

    setForwardPositionVector(
        forwardPositionVector
        )
    {
        forwardPositionVector.x = this.positionVector.x + this.forwardVector.x;
        forwardPositionVector.y = this.positionVector.y + this.forwardVector.y;
        forwardPositionVector.z = this.positionVector.z + this.forwardVector.z;
    }

    // ~~

    setBackwardPositionVector(
        backwardPositionVector
        )
    {
        backwardPositionVector.x = this.positionVector.x + this.backwardVector.x;
        backwardPositionVector.y = this.positionVector.y + this.backwardVector.y;
        backwardPositionVector.z = this.positionVector.z + this.backwardVector.z;
    }
}
