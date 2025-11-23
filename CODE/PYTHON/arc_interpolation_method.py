# -- IMPORTS

from enum import IntEnum;

# -- TYPES

class ArcInterpolationMethod( IntEnum ):

    # -- CONSTANTS

    quadratic = 0;
    spherical = 1;
    count = 2;
    none = -1;
