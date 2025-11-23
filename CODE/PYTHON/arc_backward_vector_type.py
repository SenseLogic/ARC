# -- IMPORTS

from enum import IntEnum;

# -- TYPES

class ArcBackwardVectorType( IntEnum ):

    # -- CONSTANTS

    custom = 0;
    symmetrical = 1;
    residual = 2;
    count = 3;
    none = -1;
