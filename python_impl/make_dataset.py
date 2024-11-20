from datasets import load_dataset, Audio, Dataset
import os
from typing import List, Dict
from collections import deque

from torch.utils.data import Dataset, DataLoader
import torchaudio

import torch.nn.functional as F

import random

class MusicLoader(Dataset):
        def __init__(self, path_list,
                    labels, transform: bool=False, 
                    rsr=16_000, seconds=60):
              
                self.path_list = path_list
                self.transform = transform
                self.labels = labels
                self.rsr = rsr
                self.sample_limit = rsr * seconds
 
        def __getitem__(self, index):
                X, sr = torchaudio.load(self.path_list[index], normalize=True)
                transform = torchaudio.transforms.Resample(sr, self.rsr)
                X = transform(X)
                Y = self.labels[index]

                if X.shape[1] > self.sample_limit:
                       X = X[:, :self.sample_limit]
                else:
                        X = F.pad(X,
                                (0, self.sample_limit - X.shape[1]))
                if self.transform:
                       raise NotImplementedError
                
                return X, Y

        def __len__(self):
               return len(self.path_list)


def makeLabel(
        labels :  List[str]
    ): 

    label2id, id2label = dict(), dict()
    size = len(set(labels))
    for label in labels:
        id = random.choice(range(0, size))  #what the fuck is this
        label2id[label] = id
        id2label[str(id)] = label
        
    return label2id, id2label


def createDataset(
        path: str
) -> Audio:
        dirs : List[str] = [items for items in os.walk(path)]
        dataset : Dict[str, str] = {}  #for now trivial with no implementation of nested playlists
        tracks = []
        #TODO : implement a graph with nested playlists later.
        for path, subdirectory, files in dirs:
                if subdirectory:
                        dir_stack = deque(subdirectory)

                for file in files:
                        tracks.append(file) 
                        if dir_stack:
                                dataset[path.replace("\\","/") + "/" + file] = dir_stack[0]

                if not subdirectory and dir_stack:
                        dir_stack.popleft()

        ### update labels here to id
        label2id, id2label = makeLabel(dataset.values())

        for key, value in dataset.items():
                dataset[key] = label2id[value]
        
        # music = Dataset.from_dict({"audio" : dataset.keys(),
        #                 "label" : dataset.values()})\
        #                 .train_test_split(test_size=0.3)\
        #                 .cast_column("audio", Audio())
        
        return dataset, label2id, id2label





