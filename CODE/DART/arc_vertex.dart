import 'arc_backward_vector_type.dart';
import 'arc_vector_3.dart';

// -- TYPES

class ArcVertex
{
    // -- ATTRIBUTES

    ArcVector3
        positionVector,
        forwardVector;
    late ArcVector3
        backwardVector;
    ArcBackwardVectorType
        backwardVectorType;

    // -- CONSTRUCTORS

    ArcVertex(
        {
            required this.positionVector,
            required this.forwardVector,
            ArcVector3? backwardVector,
            this.backwardVectorType = ArcBackwardVectorType.custom
        }
        )
    {
        if ( backwardVectorType == ArcBackwardVectorType.custom )
        {
            this.backwardVector =
                backwardVector ??
                ArcVector3(
                    x: 0,
                    y: 0,
                    z: 0,
                    );
        }
        else if ( backwardVectorType == ArcBackwardVectorType.symmetrical )
        {
            this.backwardVector =
                ArcVector3(
                    x: -forwardVector.x,
                    y: -forwardVector.y,
                    z: -forwardVector.z,
                    );
        }
        else
        {
            this.backwardVector =
                ArcVector3(
                    x: 0,
                    y: 0,
                    z: 0,
                    );
        }
    }

    // -- OPERATIONS

    void setForwardPositionVector(
        ArcVector3 forwardPositionVector
        )
    {
        forwardPositionVector.x = positionVector.x + forwardVector.x;
        forwardPositionVector.y = positionVector.y + forwardVector.y;
        forwardPositionVector.z = positionVector.z + forwardVector.z;
    }

    // ~~

    void setBackwardPositionVector(
        ArcVector3 backwardPositionVector
        )
    {
        backwardPositionVector.x = positionVector.x + backwardVector.x;
        backwardPositionVector.y = positionVector.y + backwardVector.y;
        backwardPositionVector.z = positionVector.z + backwardVector.z;
    }

    // ~~

    void setSymmetricalBackwardVector(
        )
    {
        backwardVector.x = -forwardVector.x;
        backwardVector.y = -forwardVector.y;
        backwardVector.z = -forwardVector.z;
    }

    // ~~

    void setResidualBackwardVector(
        ArcVertex firstVertex
        )
    {
        backwardVector.x = firstVertex.positionVector.x + firstVertex.forwardVector.x - positionVector.x;
        backwardVector.y = firstVertex.positionVector.y + firstVertex.forwardVector.y - positionVector.y;
        backwardVector.z = firstVertex.positionVector.z + firstVertex.forwardVector.z - positionVector.z;
    }
}
