
from typing import List
import os

import librosa
import librosa.display
import matplotlib.pyplot as plt
import numpy as np


class SpecLoader:
    def __init__(self, path : str):

        self.path = path 
        self.dirs : List[str] = os.listdir(path)

    def loadAudio(self, plot: bool) -> np.array:

        for i in self.dirs:
            local_dir : List[str] = os.listdir(self.path + "/" + i)
            for l in local_dir:
                # Load an audio file
                y, sr = librosa.load(self.path + "/" + i + "/" + l)

                # Compute the Short-Time Fourier Transform (STFT)
                D = librosa.stft(y)

                # Convert the amplitude to decibels
                S_db = librosa.amplitude_to_db(np.abs(D), ref=np.max)
            
                if plot:
                    self.plotSpec(S_db, sr)
                    
                # return y, sr, S_db


    def plotSpec(self, S_db, sr):
        # Plot the spectrogram
        plt.figure(figsize=(10, 4))
        librosa.display.specshow(S_db, sr=sr, x_axis='time', y_axis='log')
        plt.colorbar(format='%+2.0f dB')
        plt.title('Spectrogram')
        plt.tight_layout()
        plt.show()

