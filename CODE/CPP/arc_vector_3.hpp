#ifndef ARC_VECTOR_3_HPP
    #define ARC_VECTOR_3_HPP

    // -- TYPES

    class ArcVector3
    {
        // == PUBLIC

        public:

        // -- ATTRIBUTES

        double 
            x, 
            y, 
            z;

        // -- CONSTRUCTORS

        ArcVector3() : 
            x( 0.0 ), 
            y( 0.0 ), 
            z( 0.0 ) 
        {
        }

        // ~~

        ArcVector3( 
            double x, 
            double y, 
            double z 
            ) : 
            x( x ), 
            y( y ), 
            z( z ) 
        { 
        }

        // -- OPERATORS

        ArcVector3 & operator=( const ArcVector3 & other )
        {
            if ( this != &other )
            {
                x = other.x;
                y = other.y;
                z = other.z;
            }
            
            return *this;
        }

        // -- OPERATIONS

        void setQuadraticArcPositionVector(
            const ArcVector3 & firstPositionVector,
            const ArcVector3 & firstForwardVector,
            const ArcVector3 & secondPositionVector,
            const ArcVector3 & secondBackwardVector,
            double interpolationFactor
            );

        // ~~

        void setSphericalArcPositionVector(
            const ArcVector3 & firstPositionVector,
            const ArcVector3 & firstForwardVector,
            const ArcVector3 & secondPositionVector,
            const ArcVector3 & secondBackwardVector,
            double interpolationFactor
            );
    };
#endif
