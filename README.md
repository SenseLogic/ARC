![](https://github.com/SenseLogic/ARC/blob/master/LOGO/arc.png)

# Arc

Arc interpolation library.

## Overview

This library provides a simple yet innovative interpolation technique that overcomes key limitations of traditional polynomial-based methods.

It is available in the following programming languages :

- C++
- C#
- Dart
- JavaScript
- Python
- Rust

## Definitions

### Vertex

An arc vertex is defined as :

$$\vec{V} = (\vec{P}, \vec{F}, \vec{B})$$

where :

- **$\vec{P}$** ∈ ℝ³ is the position vector
- **$\vec{F}$** ∈ ℝ³ is the forward tangent vector (direction and magnitude)
- **$\vec{B}$** ∈ ℝ³ is the backward tangent vector (direction and magnitude)

The backward vector **$\vec{B}$** can be explicitly provided, or automatically computed.

### Symmetrical backward vector

The backward vector is computed as the negative of the forward vector :

$$\vec{B} = -\vec{F}$$

This creates symmetric tangent behavior at the vertex.

### Residual backward vector

The backward vector is computed from the previous vertex's forward position :

$$\vec{B}_2 = (\vec{P}_1 + \vec{F}_1) - \vec{P}_2$$

This ensures smooth continuity by making the backward vector point from the second vertex toward where the first vertex's forward tangent would reach.

## Spherical Arc Interpolation

The Spherical Arc Interpolation (SAI) method employs a sophisticated dual-curve construction with trigonometric blending that naturally preserves spherical and circular geometric properties. This method can exactly reconstruct circular arcs with proper vertex configuration using residual backward vectors.

![](https://github.com/SenseLogic/ARC/blob/master/CODE/JAVASCRIPT/curve_1_spherical_interpolation.svg)
![](https://github.com/SenseLogic/ARC/blob/master/CODE/JAVASCRIPT/curve_2_spherical_interpolation.svg)
![](https://github.com/SenseLogic/ARC/blob/master/CODE/JAVASCRIPT/curve_9_spherical_interpolation.svg)

### Algorithm

The SAI algorithm proceeds through seven computational phases :

**Phase I : Vector extraction**

$$\vec{P}_1, \vec{F}_1, \vec{P}_2, \vec{B}_2$$

**Phase II : Tangent position computation**

$$\vec{P}_{F1} = \vec{P}_1 + \vec{F}_1$$

$$\vec{P}_{B2} = \vec{P}_2 + \vec{B}_2$$

**Phase III : Residual Vector Calculation**

$$\vec{R}_1 = \vec{P}_2 - \vec{P}_{F1}$$

$$\vec{R}_2 = \vec{P}_{B2} - \vec{P}_1$$

**Phase IV : Origin adjustment**

$$\vec{O}_1 = \vec{P}_1 + \vec{R}_1$$

$$\vec{O}_2 = \vec{P}_2 - \vec{R}_2$$

**Phase V : Trigonometric basis evaluation**

$$\theta = (1 - t) \cdot \pi/2$$

$$c = \cos(\theta)$$

$$s = \sin(\theta)$$

**Phase VI : Dual curve construction**

$$\vec{I}_1(t) = \vec{O}_1 + \vec{F}_1 \cdot c - \vec{R}_1 \cdot s$$

$$\vec{I}_2(t) = \vec{O}_2 + \vec{R}_2 \cdot c + \vec{B}_2 \cdot s$$

**Phase VII : Final interpolation**

$$\beta(t) = 1 - s$$

$$SAI(t) = \vec{I}_1(t) + (\vec{I}_2(t) - \vec{I}_1(t)) \cdot \beta(t)$$

### Mathematical Properties

- **Interpolation** : $SAI(0) = \vec{P}_1$ and $SAI(1) = \vec{P}_2$
- **Smoothness** : Infinitely differentiable on (0,1), C¹ at boundaries
- **Perfect circle reconstruction** : With four properly configured vertices, can exactly represent a unit circle
- **Spherical preservation** : For vertices on a sphere with appropriate tangent vectors, preserves exact radial distance

### Perfect Circle Example

For a unit circle, using four vertices :

$$V_1 = (\vec{P}_1, \vec{F}_1, \vec{B}_1) = ((0, 1, 0), (1, 0, 0), (-1, 0, 0))$$

$$V_2 = (\vec{P}_2, \vec{F}_2, \vec{B}_2) = ((1, 0, 0), (0, -1, 0), (0, 1, 0))$$

$$V_3 = (\vec{P}_3, \vec{F}_3, \vec{B}_3) = ((0, -1, 0), (-1, 0, 0), (1, 0, 0))$$

$$V_4 = (\vec{P}_4, \vec{F}_4, \vec{B}_4) = ((-1, 0, 0), (0, 1, 0), (0, -1, 0))$$

The SAI interpolation between consecutive vertices generates exactly :

$$SAI(t) = (\sin(\pi t/2), \cos(\pi t/2), 0)$$

This is a perfect quarter-circle parameterization with zero approximation error, traversing from (0, 1, 0) at t=0 to (1, 0, 0) at t=1.

### Comparison with Cubic Hermite Interpolation

- **Superior circular fidelity** : Achieves exact circular arc representation
- **Natural constraint handling** : Geometric construction inherently satisfies constraints without requiring numerical optimization
- **No approximation accumulation** : Avoids error accumulation in multi-segment constructions that occurs with polynomial approximations

### Comparison with Cubic Trigonometric Hermite Interpolation

- **Parameter-free operation** : No tuning parameters required, optimal behavior through geometric construction
- **Simpler formulation** : More intuitive geometric interpretation with explicit dual-curve construction

### Comparison with Kochanek-Bartels TCB Splines

- **Superior circular fidelity** : Achieves exact circular representation
- **Simpler formulation** : No tension/continuity/bias parameters to tune
- **Geometric clarity** : All computational steps have clear geometric meaning

### Comparison with NURBS

- **Computational efficiency** : O(1) evaluation per point without rational arithmetic operations
- **Simpler parameterization** : No weight parameters needed
- **Simpler implementation** : More straightforward code structure without rational polynomial evaluation

## Quadratic Arc Interpolation

The Quadratic Arc Interpolation (QAI) is a simpler interpolation method that uses the same vertex structure but does not preserve radial distance.

It simply combines linear interpolation along tangent directions with a cosine-based trigonometric blending factor.

![](https://github.com/SenseLogic/ARC/blob/master/CODE/JAVASCRIPT/curve_1_quadratic_interpolation.svg)
![](https://github.com/SenseLogic/ARC/blob/master/CODE/JAVASCRIPT/curve_2_quadratic_interpolation.svg)
![](https://github.com/SenseLogic/ARC/blob/master/CODE/JAVASCRIPT/curve_9_quadratic_interpolation.svg)

### Algorithm

Given vertices $V_1 = (\vec{P}_1, \vec{F}_1, \vec{B}_1)$ and $V_2 = (\vec{P}_2, \vec{F}_2, \vec{B}_2)$, and interpolation parameter $t \in [0,1]$ :

**Phase I : Linear projection along tangents**

$$\vec{Q}_1(t) = \vec{P}_1 + \vec{F}_1 \cdot t$$

$$\vec{Q}_2(t) = \vec{P}_2 + \vec{B}_2 \cdot (1 - t)$$

**Phase II : Trigonometric blending**

$$\alpha(t) = (1 - \cos(\pi t)) / 2$$

**Phase III : Final interpolation**

$$QAI(t) = \vec{Q}_1(t) + (\vec{Q}_2(t) - \vec{Q}_1(t)) \cdot \alpha(t)$$

### Mathematical Properties

* **Interpolation** : $QAI(0) = \vec{P}_1$ and $QAI(1) = \vec{P}_2$
* **Smoothness** : Infinitely differentiable on (0,1), C¹ at boundaries
* **Blending function** : The cosine-based α(t) provides smooth acceleration/deceleration
* **Geometric interpretation** : Creates a smooth arc by blending two linear trajectories

## Version

0.1

## Author

Eric Pelzer (ecstatic.coder@gmail.com).

## License

This project is licensed under the GNU Lesser General Public License version 3.

See the [LICENSE.md](LICENSE.md) file for details.
