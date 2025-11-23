// -- IMPORTS

use crate::arc_backward_vector_type::ArcBackwardVectorType;
use crate::arc_interpolation_method::ArcInterpolationMethod;
use crate::arc_vector_3::Vector3;
use crate::arc_vertex::ArcVertex;

// -- TYPES

pub struct ArcCurve
{
    // -- CONSTRUCTORS

    pub vertex_array: Vec<ArcVertex>,
    pub arc_count: usize
}

// -- IMPLEMENTATIONS

impl ArcCurve
{
    // -- CONSTRUCTORS

    pub fn new(
        vertex_array: Vec<ArcVertex>,
        arc_count: usize
        ) -> Self
    {
        Self
        {
            vertex_array,
            arc_count
        }
    }

    // -- INQUIRIES

    pub fn get_arc_first_vertex(
        &self,
        arc_index: usize
        ) -> &ArcVertex
    {
        &self.vertex_array[ arc_index ]
    }

    // ~~

    pub fn get_arc_second_vertex(
        &self,
        arc_index: usize
        ) -> &ArcVertex
    {
        let mut second_vertex_index = arc_index + 1;

        if second_vertex_index == self.vertex_array.len()
        {
            second_vertex_index = 0;
        }

        &self.vertex_array[ second_vertex_index ]
    }

    // -- OPERATIONS

    pub fn add_vertex(
        &mut self,
        vertex: ArcVertex
        )
    {
        self.vertex_array.push( vertex );
        self.arc_count += 1;
    }

    // ~~

    pub fn update_backward_vectors(
        &mut self
        )
    {
        for arc_index in 0 .. self.arc_count
        {
            let second_vertex_index = if arc_index + 1 == self.vertex_array.len() { 0 } else { arc_index + 1 };

            if self.vertex_array[ second_vertex_index ].backward_vector_type == ArcBackwardVectorType::SYMMETRICAL
            {
                self.vertex_array[ second_vertex_index ].set_symmetrical_backward_vector();
            }
            else if self.vertex_array[ second_vertex_index ].backward_vector_type == ArcBackwardVectorType::RESIDUAL
            {
                let first_vertex = ArcVertex {
                    position_vector: self.vertex_array[ arc_index ].position_vector,
                    forward_vector: self.vertex_array[ arc_index ].forward_vector,
                    backward_vector: self.vertex_array[ arc_index ].backward_vector,
                    backward_vector_type: self.vertex_array[ arc_index ].backward_vector_type
                };
                self.vertex_array[ second_vertex_index ].set_residual_backward_vector( &first_vertex );
            }
        }
    }

    // ~~

    pub fn get_svg_drawing(
        &self,
        interpolation_method: ArcInterpolationMethod,
        point_count: usize
        ) -> String
    {
        let mut vertex_position_vector_array = Vec::new();
        let mut backward_position_vector_array = Vec::new();
        let mut forward_position_vector_array = Vec::new();
        let mut interpolated_position_vector_array = Vec::new();

        let mut backward_position_vector = Vector3::new( 0.0, 0.0, 0.0 );
        let mut forward_position_vector = Vector3::new( 0.0, 0.0, 0.0 );
        let mut interpolated_position_vector = Vector3::new( 0.0, 0.0, 0.0 );

        for vertex in &self.vertex_array
        {
            vertex.set_backward_position_vector( &mut backward_position_vector );
            vertex.set_forward_position_vector( &mut forward_position_vector );

            vertex_position_vector_array.push( Vector3::new( vertex.position_vector.x, vertex.position_vector.y, vertex.position_vector.z ) );
            backward_position_vector_array.push( Vector3::new( backward_position_vector.x, backward_position_vector.y, backward_position_vector.z ) );
            forward_position_vector_array.push( Vector3::new( forward_position_vector.x, forward_position_vector.y, forward_position_vector.z ) );
        }

        for arc_index in 0 .. self.arc_count
        {
            let first_vertex = self.get_arc_first_vertex( arc_index );
            let second_vertex = self.get_arc_second_vertex( arc_index );
            let one_over_point_count = 1.0 / point_count as f64;

            for point_index in 0 ..= point_count
            {
                let interpolation_factor = point_index as f64 * one_over_point_count;

                if interpolation_method == ArcInterpolationMethod::SPHERICAL
                {
                    interpolated_position_vector.set_spherical_arc_position_vector( &first_vertex.position_vector, &first_vertex.forward_vector, &second_vertex.position_vector, &second_vertex.backward_vector, interpolation_factor );
                }
                else if interpolation_method == ArcInterpolationMethod::QUADRATIC
                {
                    interpolated_position_vector.set_quadratic_arc_position_vector( &first_vertex.position_vector, &first_vertex.forward_vector, &second_vertex.position_vector, &second_vertex.backward_vector, interpolation_factor );
                }
                else
                {
                    panic!( "Invalid interpolation method: {:?}", interpolation_method );
                }

                interpolated_position_vector_array.push( Vector3::new( interpolated_position_vector.x, interpolated_position_vector.y, interpolated_position_vector.z ) );
            }
        }

        let position_vector_array =
            [
                vertex_position_vector_array.clone(),
                backward_position_vector_array.clone(),
                forward_position_vector_array.clone(),
                interpolated_position_vector_array.clone()
            ].concat();

        let mut minimum_position_vector = Vector3::new( f64::INFINITY, f64::INFINITY, f64::INFINITY );
        let mut maximum_position_vector = Vector3::new( f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY );

        for position_vector in &position_vector_array
        {
            minimum_position_vector.x = minimum_position_vector.x.min( position_vector.x );
            minimum_position_vector.y = minimum_position_vector.y.min( position_vector.y );
            minimum_position_vector.z = minimum_position_vector.z.min( position_vector.z );
            maximum_position_vector.x = maximum_position_vector.x.max( position_vector.x );
            maximum_position_vector.y = maximum_position_vector.y.max( position_vector.y );
            maximum_position_vector.z = maximum_position_vector.z.max( position_vector.z );
        }

        let bounding_box_width = ( maximum_position_vector.x - minimum_position_vector.x ).max( 0.001 );
        let bounding_box_height = ( maximum_position_vector.y - minimum_position_vector.y ).max( 0.001 );
        let margin = 15.0;

        let width = 1000.0;
        let height = width * ( bounding_box_height / bounding_box_width );

        let x_scale = ( width - 2.0 * margin ) / bounding_box_width;
        let y_scale = ( height - 2.0 * margin ) / bounding_box_height;
        let scale = x_scale.min( y_scale );

        let pixel_size = 1.0 / scale;
        let stroke_width = 3.0 * pixel_size;
        let circle_radius = 6.0 * pixel_size;

        let center_x = ( minimum_position_vector.x + maximum_position_vector.x ) / 2.0;
        let center_y = ( minimum_position_vector.y + maximum_position_vector.y ) / 2.0;

        let translate_x = width / 2.0 - center_x * scale;
        let translate_y = height / 2.0 + center_y * scale;

        let mut svg_drawing = format!( "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n", width, height );
        svg_drawing += &format!( "  <g transform=\"translate({}, {}) scale({}, -{})\">\n", translate_x, translate_y, scale, scale );

        for vertex_index in 0 .. self.vertex_array.len()
        {
            let vertex = &self.vertex_array[ vertex_index ];
            let backward_position_vector = &backward_position_vector_array[ vertex_index ];
            svg_drawing += &format!( "    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"rgb( 255, 192, 192 )\" stroke-width=\"{}\"/>\n", vertex.position_vector.x, vertex.position_vector.y, backward_position_vector.x, backward_position_vector.y, stroke_width );
        }

        for vertex_index in 0 .. self.vertex_array.len()
        {
            let vertex = &self.vertex_array[ vertex_index ];
            let forward_position_vector = &forward_position_vector_array[ vertex_index ];
            svg_drawing += &format!( "    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"rgb( 192, 192, 255 )\" stroke-width=\"{}\"/>\n", vertex.position_vector.x, vertex.position_vector.y, forward_position_vector.x, forward_position_vector.y, stroke_width );
        }

        for vertex_position_vector in &vertex_position_vector_array
        {
            svg_drawing += &format!( "    <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"rgb( 255, 128, 255 )\"/>\n", vertex_position_vector.x, vertex_position_vector.y, 2.0 * circle_radius );
        }

        for interpolated_position_vector in &interpolated_position_vector_array
        {
            svg_drawing += &format!( "    <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"rgb( 0, 0, 255 )\"/>\n", interpolated_position_vector.x, interpolated_position_vector.y, circle_radius );
        }

        svg_drawing += "  </g>\n</svg>";

        svg_drawing
    }
}
