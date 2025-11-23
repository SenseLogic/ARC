// -- TYPES

#[derive( Debug, Clone, Copy )]
pub struct Vector3
{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

// -- IMPLEMENTATIONS

impl Vector3
{
    // -- CONSTRUCTORS
    
    pub fn new(
        x: f64,
        y: f64,
        z: f64
        ) -> Self
    {
        Self { x, y, z }
    }

    // -- OPERATIONS

    pub fn set_quadratic_arc_position_vector(
        &mut self,
        first_position_vector: &Vector3,
        first_forward_vector: &Vector3,
        second_position_vector: &Vector3,
        second_backward_vector: &Vector3,
        interpolation_factor: f64
        )
    {
        let interpolation_factor_complement = 1.0 - interpolation_factor;

        let first_quadratic_position_x = first_position_vector.x + first_forward_vector.x * interpolation_factor;
        let first_quadratic_position_y = first_position_vector.y + first_forward_vector.y * interpolation_factor;
        let first_quadratic_position_z = first_position_vector.z + first_forward_vector.z * interpolation_factor;
        let second_quadratic_position_x = second_position_vector.x + second_backward_vector.x * interpolation_factor_complement;
        let second_quadratic_position_y = second_position_vector.y + second_backward_vector.y * interpolation_factor_complement;
        let second_quadratic_position_z = second_position_vector.z + second_backward_vector.z * interpolation_factor_complement;

        let second_quadratic_position_factor = ( 1.0 - ( std::f64::consts::PI * interpolation_factor ).cos() ) * 0.5;

        self.x = first_quadratic_position_x + ( second_quadratic_position_x - first_quadratic_position_x ) * second_quadratic_position_factor;
        self.y = first_quadratic_position_y + ( second_quadratic_position_y - first_quadratic_position_y ) * second_quadratic_position_factor;
        self.z = first_quadratic_position_z + ( second_quadratic_position_z - first_quadratic_position_z ) * second_quadratic_position_factor;
    }

    // ~~

    pub fn set_spherical_arc_position_vector(
        &mut self,
        first_position_vector: &Vector3,
        first_forward_vector: &Vector3,
        second_position_vector: &Vector3,
        second_backward_vector: &Vector3,
        interpolation_factor: f64
        )
    {
        let first_forward_position_vector_x = first_position_vector.x + first_forward_vector.x;
        let first_forward_position_vector_y = first_position_vector.y + first_forward_vector.y;
        let first_forward_position_vector_z = first_position_vector.z + first_forward_vector.z;

        let second_backward_position_vector_x = second_position_vector.x + second_backward_vector.x;
        let second_backward_position_vector_y = second_position_vector.y + second_backward_vector.y;
        let second_backward_position_vector_z = second_position_vector.z + second_backward_vector.z;

        let first_residual_vector_x = second_position_vector.x - first_forward_position_vector_x;
        let first_residual_vector_y = second_position_vector.y - first_forward_position_vector_y;
        let first_residual_vector_z = second_position_vector.z - first_forward_position_vector_z;
        let second_residual_vector_x = second_backward_position_vector_x - first_position_vector.x;
        let second_residual_vector_y = second_backward_position_vector_y - first_position_vector.y;
        let second_residual_vector_z = second_backward_position_vector_z - first_position_vector.z;

        let first_origin_position_vector_x = first_position_vector.x + first_residual_vector_x;
        let first_origin_position_vector_y = first_position_vector.y + first_residual_vector_y;
        let first_origin_position_vector_z = first_position_vector.z + first_residual_vector_z;
        let second_origin_position_vector_x = second_position_vector.x - second_residual_vector_x;
        let second_origin_position_vector_y = second_position_vector.y - second_residual_vector_y;
        let second_origin_position_vector_z = second_position_vector.z - second_residual_vector_z;

        let interpolation_factor_angle = ( 1.0 - interpolation_factor ) * ( std::f64::consts::PI * 0.5 );
        let interpolation_factor_cosinus = interpolation_factor_angle.cos();
        let interpolation_factor_sinus = interpolation_factor_angle.sin();

        let first_spherical_position_x = first_origin_position_vector_x + first_forward_vector.x * interpolation_factor_cosinus - first_residual_vector_x * interpolation_factor_sinus;
        let first_spherical_position_y = first_origin_position_vector_y + first_forward_vector.y * interpolation_factor_cosinus - first_residual_vector_y * interpolation_factor_sinus;
        let first_spherical_position_z = first_origin_position_vector_z + first_forward_vector.z * interpolation_factor_cosinus - first_residual_vector_z * interpolation_factor_sinus;
        let second_spherical_position_x = second_origin_position_vector_x + second_residual_vector_x * interpolation_factor_cosinus + second_backward_vector.x * interpolation_factor_sinus;
        let second_spherical_position_y = second_origin_position_vector_y + second_residual_vector_y * interpolation_factor_cosinus + second_backward_vector.y * interpolation_factor_sinus;
        let second_spherical_position_z = second_origin_position_vector_z + second_residual_vector_z * interpolation_factor_cosinus + second_backward_vector.z * interpolation_factor_sinus;

        let second_spherical_position_factor = 1.0 - interpolation_factor_sinus;

        self.x = first_spherical_position_x + ( second_spherical_position_x - first_spherical_position_x ) * second_spherical_position_factor;
        self.y = first_spherical_position_y + ( second_spherical_position_y - first_spherical_position_y ) * second_spherical_position_factor;
        self.z = first_spherical_position_z + ( second_spherical_position_z - first_spherical_position_z ) * second_spherical_position_factor;
    }
}
