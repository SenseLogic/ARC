#ifndef ARC_VERTEX_HPP
    #define ARC_VERTEX_HPP
    
    // -- IMPORTS

    #include "arc_backward_vector_type.hpp"
    #include "arc_vector_3.hpp"

    // -- TYPES

    class ArcVertex
    {
        // == PUBLIC

        public:

        // -- ATTRIBUTES

        ArcVector3 
            positionVector,
            forwardVector,
            backwardVector;
        ArcBackwardVectorType 
            backwardVectorType;

        // -- CONSTRUCTORS

        ArcVertex(
            ) :
            positionVector( 0.0f, 0.0f, 0.0f ),
            forwardVector( 0.0f, 0.0f, 0.0f ),
            backwardVector( 0.0f, 0.0f, 0.0f ),
            backwardVectorType( ArcBackwardVectorType::custom )
        {
        }

        // ~~

        ArcVertex(
            const ArcVector3 & positionVector,
            const ArcVector3 & forwardVector,
            ArcBackwardVectorType backwardVectorType
            ) :
            positionVector( positionVector ),
            forwardVector( forwardVector ),
            backwardVector( 0.0f, 0.0f, 0.0f ),
            backwardVectorType( backwardVectorType )
        {
            if ( backwardVectorType == ArcBackwardVectorType::symmetrical )
            {
                this->backwardVector.x = -forwardVector.x;
                this->backwardVector.y = -forwardVector.y;
                this->backwardVector.z = -forwardVector.z;
            }
            else if ( backwardVectorType == ArcBackwardVectorType::residual )
            {
                this->backwardVector.x = 0.0f;
                this->backwardVector.y = 0.0f;
                this->backwardVector.z = 0.0f;
            }
        }

        // ~~

        ArcVertex(
            const ArcVector3 & positionVector,
            const ArcVector3 & forwardVector,
            const ArcVector3 & backwardVector
            ) :
            positionVector( positionVector ),
            forwardVector( forwardVector ),
            backwardVector( backwardVector ),
            backwardVectorType( ArcBackwardVectorType::custom )
        {
        }

        // ~~

        ArcVertex(
            const ArcVector3 & positionVector,
            const ArcVector3 & forwardVector,
            const ArcVector3 & backwardVector,
            ArcBackwardVectorType backwardVectorType
            ) :
            positionVector( positionVector ),
            forwardVector( forwardVector ),
            backwardVector( backwardVector ),
            backwardVectorType( backwardVectorType )
        {
            if ( backwardVectorType == ArcBackwardVectorType::symmetrical )
            {
                this->backwardVector.x = -forwardVector.x;
                this->backwardVector.y = -forwardVector.y;
                this->backwardVector.z = -forwardVector.z;
            }
            else if ( backwardVectorType == ArcBackwardVectorType::residual )
            {
                this->backwardVector.x = 0.0f;
                this->backwardVector.y = 0.0f;
                this->backwardVector.z = 0.0f;
            }
        }

        // -- OPERATORS

        ArcVertex& operator=(
            const ArcVertex & other
            )
        {
            if ( this != &other )
            {
                positionVector = other.positionVector;
                forwardVector = other.forwardVector;
                backwardVector = other.backwardVector;
                backwardVectorType = other.backwardVectorType;
            }
            
            return *this;
        }

        // -- OPERATIONS

        void setForwardPositionVector(
            ArcVector3 & forwardPositionVector
            )
        {
            forwardPositionVector.x = positionVector.x + forwardVector.x;
            forwardPositionVector.y = positionVector.y + forwardVector.y;
            forwardPositionVector.z = positionVector.z + forwardVector.z;
        }

        // ~~

        void setBackwardPositionVector(
            ArcVector3 & backwardPositionVector
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
            const ArcVertex & firstVertex
            )
        {
            backwardVector.x = firstVertex.positionVector.x + firstVertex.forwardVector.x - positionVector.x;
            backwardVector.y = firstVertex.positionVector.y + firstVertex.forwardVector.y - positionVector.y;
            backwardVector.z = firstVertex.positionVector.z + firstVertex.forwardVector.z - positionVector.z;
        }
    };
#endif
