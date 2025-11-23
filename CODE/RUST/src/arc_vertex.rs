// -- IMPORTS

use crate::arc_backward_vector_type::ArcBackwardVectorType;
use crate::arc_vector_3::Vector3;

// -- TYPES

pub struct ArcVertex
{
    // -- CONSTRUCTORS

    pub position_vector: Vector3,
    pub forward_vector: Vector3,
    pub backward_vector: Vector3,
    pub backward_vector_type: ArcBackwardVectorType
}

// -- IMPLEMENTATIONS

impl ArcVertex
{
    // -- CONSTRUCTORS

    pub fn new(
        position_vector: Vector3,
        forward_vector: Vector3,
        backward_vector: Vector3,
        backward_vector_type: ArcBackwardVectorType
        ) -> Self
    {
        let mut vertex = 
            Self
            {
                position_vector,
                forward_vector,
                backward_vector,
                backward_vector_type
            };

        if backward_vector_type == ArcBackwardVectorType::CUSTOM
        {
            // Keep the provided backward_vector
        }
        else if backward_vector_type == ArcBackwardVectorType::SYMMETRICAL
        {
            vertex.backward_vector = Vector3::new( -forward_vector.x, -forward_vector.y, -forward_vector.z );
        }
        else
        {
            vertex.backward_vector = Vector3::new( 0.0, 0.0, 0.0 );
        }

        vertex
    }

    // -- OPERATIONS

    pub fn set_symmetrical_backward_vector(
        &mut self
        )
    {
        self.backward_vector.x = -self.forward_vector.x;
        self.backward_vector.y = -self.forward_vector.y;
        self.backward_vector.z = -self.forward_vector.z;
    }

    // ~~

    pub fn set_residual_backward_vector(
        &mut self,
        arc_first_vertex: &ArcVertex
        )
    {
        self.backward_vector.x = arc_first_vertex.position_vector.x + arc_first_vertex.forward_vector.x - self.position_vector.x;
        self.backward_vector.y = arc_first_vertex.position_vector.y + arc_first_vertex.forward_vector.y - self.position_vector.y;
        self.backward_vector.z = arc_first_vertex.position_vector.z + arc_first_vertex.forward_vector.z - self.position_vector.z;
    }

    // ~~

    pub fn set_forward_position_vector(
        &self,
        forward_position_vector: &mut Vector3
        )
    {
        forward_position_vector.x = self.position_vector.x + self.forward_vector.x;
        forward_position_vector.y = self.position_vector.y + self.forward_vector.y;
        forward_position_vector.z = self.position_vector.z + self.forward_vector.z;
    }

    // ~~

    pub fn set_backward_position_vector(
        &self,
        backward_position_vector: &mut Vector3
        )
    {
        backward_position_vector.x = self.position_vector.x + self.backward_vector.x;
        backward_position_vector.y = self.position_vector.y + self.backward_vector.y;
        backward_position_vector.z = self.position_vector.z + self.backward_vector.z;
    }
}
