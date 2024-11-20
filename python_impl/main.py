
import numpy as np

from pathlib import Path


import os
import torch
from torch.utils.data import DataLoader
from typing import List

from make_spec import SpecLoader
from make_dataset import createDataset, MusicLoader
from extract_audio_features import featExtractor, trainModel, FeatureExtractor
from make_dataset import makeLabel
from embeddings import CNN, train, evaluate

import time

EPOCHS = 64
os.environ["CUDA_LAUNCH_BLOCKING"] = "1"
os.environ["TORCH_USE_CUDA_DSA"] = "1"

if __name__ == "__main__":

    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    path : str = "C:/Users/6090249/Desktop/Learning/RustyMusic/Playlist"
    save_path = "C:/Users/6090249/Desktop/Learning/RustyMusic/models"
    # path = "E:/Track selections/New sorting"
    dataset, label2id, id2label = createDataset(path)
    n_classes = len(label2id.keys())
    
    start = time.time()
    music = []
    labels = []



    music = MusicLoader(list(dataset.keys()), 
                        list(dataset.values()))
    # for path, label in dataset.items():

        # music.append(feature_extractor(sr, wf))

        # label.append(label)

    train_dataloader = DataLoader(music, batch_size=32, shuffle=True)
    val_dataloader = DataLoader(music, batch_size=32)

    model = CNN(num_classes=n_classes, num_layers=2, input_size=128, d_model=256, nhead=4, device=device)

    # criterion = torch.nn.CrossEntropyLoss()
    criterion = torch.nn.TripletMarginLoss()
    optimizer = torch.optim.Adam(model.parameters(), lr=0.001)

    for epoch in range(EPOCHS):
        train_loss, train_acc = train(model, train_dataloader, criterion, optimizer, device)
        val_loss, val_acc = evaluate(model, val_dataloader, criterion, device)
        print(f"Epoch[{((epoch)/EPOCHS) * 100}] %, Train Loss: {train_loss:.4f}, Train Acc: {train_acc:.4f}, Val Loss: {val_loss:.4f}, Val Acc: {val_acc:.4f}")
    end = time.time()
    breakpoint()
    
    audio_in = dataset.map(featExtractor,
                           batched=True)
    label2id_train, label2id_test = makeLabel(audio_in["train"]["label"]), makeLabel(audio_in["test"]["label"])
    audio_in["train"]["label"] = label2id_train
    audio_in["test"]["label"] = label2id_test
    audio_in =  audio_in.remove_columns("audio")

    # hugging face loader takes 340 secs


    # for i in range(yes["train"].num_rows):
    #     print(yes["train"][i]["audio"]["array"])
        
    # print((end-start))

    
    # with torch.no_grad():
    #     trainer = trainModel(
    #         audio_in=audio_in,
    #         label2id=label2id,
    #         id2label=id2label
    #     ).train()  ## no label 2 id function hence the error
    
    breakpoint()




    # below in case transformers dont work!
    # s = SpecLoader(path)
    # y, sr, S_db = s.loadAudio()
    # s.plotSpec(S_db, sr)