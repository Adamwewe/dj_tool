import numpy as np
from decoding_backend.decoding_backend import get_streams, get_tracks

def get_waves(width: int) -> np.ndarray:
    items = get_tracks()
    waves = get_streams(items, 1000)
    return waves
