// -- IMPORTS

using Arc;

// -- TYPES

namespace Arc
{
    public class ArcVertex
    {
        // -- ATTRIBUTES

        public ArcVector3
            positionVector,
            forwardVector,
            backwardVector;
        public int
            backwardVectorType;

        // -- CONSTRUCTORS

        public ArcVertex(
            ArcVector3? positionVector = null,
            ArcVector3? forwardVector = null,
            ArcVector3? backwardVector = null,
            int backwardVectorType = ArcBackwardVectorType.custom
            )
        {
            this.positionVector = positionVector ?? new ArcVector3();
            this.forwardVector = forwardVector ?? new ArcVector3();
            this.backwardVectorType = backwardVectorType;

            if ( backwardVectorType == ArcBackwardVectorType.custom )
            {
                this.backwardVector = backwardVector ?? new ArcVector3();
            }
            else if ( backwardVectorType == ArcBackwardVectorType.symmetrical )
            {
                this.backwardVector = new ArcVector3( -forwardVector?.x ?? 0, -forwardVector?.y ?? 0, -forwardVector?.z ?? 0 );
            }
            else
            {
                this.backwardVector = new ArcVector3( 0, 0, 0 );
            }
        }

        // -- OPERATIONS

        public void SetSymmetricalBackwardVector(
            )
        {
            this.backwardVector.x = -this.forwardVector.x;
            this.backwardVector.y = -this.forwardVector.y;
            this.backwardVector.z = -this.forwardVector.z;
        }

        // ~~

        public void SetResidualBackwardVector(
            ArcVertex arcFirstVertex
            )
        {
            this.backwardVector.x = arcFirstVertex.positionVector.x + arcFirstVertex.forwardVector.x - this.positionVector.x;
            this.backwardVector.y = arcFirstVertex.positionVector.y + arcFirstVertex.forwardVector.y - this.positionVector.y;
            this.backwardVector.z = arcFirstVertex.positionVector.z + arcFirstVertex.forwardVector.z - this.positionVector.z;
        }

        // ~~

        public void SetForwardPositionVector(
            ref ArcVector3 forwardPositionVector
            )
        {
            forwardPositionVector.x = this.positionVector.x + this.forwardVector.x;
            forwardPositionVector.y = this.positionVector.y + this.forwardVector.y;
            forwardPositionVector.z = this.positionVector.z + this.forwardVector.z;
        }

        // ~~

        public void SetBackwardPositionVector(
            ref ArcVector3 backwardPositionVector
            )
        {
            backwardPositionVector.x = this.positionVector.x + this.backwardVector.x;
            backwardPositionVector.y = this.positionVector.y + this.backwardVector.y;
            backwardPositionVector.z = this.positionVector.z + this.backwardVector.z;
        }
    }
}
