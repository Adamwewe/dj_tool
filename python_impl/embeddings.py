import torch

from torch.nn import TransformerEncoderLayer, TransformerEncoder
from torch import nn
from torchaudio.transforms import MFCC
import umap

import matplotlib.pyplot as plt

class FeatureExtractor:
    def __init__(self, sample_rate=16_000, 
                 n_mfcc=40, 
                 n_fft=128, 
                 hop_length=None):
        
        self.mfcc = MFCC(sample_rate=sample_rate)

        # self.spectrogram = Spectrogram(n_fft=n_fft, hop_length=hop_length)

    def extract_features(self, audio, device):
        
        mfcc = self.mfcc.to(device)(audio)
        # mfcc = self.mfcc.cuda()(audio) if device == "cuda" else self.mfcc.cpu()  #contribution to pytorch here
        # spec = self.spectrogram(audio)
        # features = torch.stack([mfcc, spec], dim=1)
        return mfcc #features

class CNN(torch.nn.Module):
    def __init__(self, num_classes, num_layers,
                 input_size, d_model, nhead, device, embed_dim=512):
        
        super(CNN, self).__init__()
        self.device = device
        self.feature_extractor = FeatureExtractor()
        self.embed_dim = embed_dim

        self.conv1 = nn.Sequential(
            nn.Conv2d(2, 32, kernel_size=3, stride=1, padding=1),
            nn.BatchNorm2d(32),
            nn.ReLU(inplace=True),
            nn.MaxPool2d(kernel_size=2, stride=2)
        )

        self.conv2 = nn.Sequential(
            nn.Conv2d(32, 64, kernel_size=3, stride=1, padding=1),
            nn.ReLU(inplace=True),
            nn.MaxPool2d(kernel_size=2, stride=2),
        )

        # self.conv3 = nn.Sequential(
        #     nn.Conv2d(64, 128, kernel_size=3, stride=1, padding=1),
        #     nn.ReLU(inplace=True),
        #     nn.MaxPool2d(kernel_size=2, stride=2),
        # )

        # self.linear_proj = nn.Linear(64 * 300 * 2, d_model)

        # encoder_layers = TransformerEncoderLayer(d_model, nhead, dim_feedforward=d_model * 4, dropout=0.1)
        # self.transformer_encoder = TransformerEncoder(encoder_layers, num_layers)

        self.adaptative_pool = nn.AdaptiveAvgPool2d((4, 4))

        flattened_size = 64 * 4 * 4

        # self.classifier = nn.Sequential(
        #     nn.Flatten(),
        #     nn.Linear(flattened_size, 512),
        #     # nn.Linear(64 * 300 * 2, 512),  #for transformer layers
        #     nn.ReLU(inplace=True),
        #     nn.Dropout(p=0.5),
        #     nn.Linear(512, num_classes),
        # )

        self.embeddings = nn.Sequential(
            nn.Flatten(),
            nn.Linear(flattened_size, 512),
            # nn.Linear(64 * 300 * 2, 512),  #for transformer layers
            nn.ReLU(inplace=True),
            nn.Dropout(p=0.5),
            nn.Linear(512, self.embed_dim),
        )


    def forward(self, audio):

        features = self.feature_extractor.extract_features(audio, device=self.device)
        batch_size, channels, height, width = features.size()
        cnn_features1 = self.conv1(features)
        cnn_features2 = self.conv2(cnn_features1)
        # cnn_features3 = self.conv3(cnn_features2)
        adapt = self.adaptative_pool(cnn_features2)
        # output = self.classifier(adapt)
        output = self.embeddings(adapt)
        # cnn_features = cnn_features.view(batch_size, -1)
        # transformer_input = self.linear_proj(cnn_features).unsqueeze(1)
        # transformer_output = self.transformer_encoder(transformer_input)
        # transformer_output = transformer_output.squeeze(1)
        # output = self.classifier(cnn_features)

        return output



def train(model, dataloader, 
          criterion, optimizer, 
          device):
    

    model.to(device).train()
    running_loss = 0.0
    running_accuracy = 0.0


    for audio, labels in dataloader:

        audio = audio.to(device)
        labels = labels.to(device)
        optimizer.zero_grad()
        embeddings = model(audio)
        
        dist_mat = torch.cdist(embeddings, embeddings, p=2)
        anchors = []
        positives = []
        negatives = []
        for i in range(labels.shape[0]):

            anchor_label = labels[i].item()
            anchor_dist = dist_mat[i]

            hardest_positive_idx = (labels == anchor_label).nonzero(as_tuple=True)[0] # all same class indices
            hardest_positive_idx = hardest_positive_idx[hardest_positive_idx != i] # disclude own label
            hardest_positive = hardest_positive_idx[anchor_dist[hardest_positive_idx].argmax()] # label of furthest same class

            # Hardest negative (closest from different class)
            hardest_negative_idx = (labels != anchor_label).nonzero(as_tuple=True)[0] # all diff class indices
            hardest_negative = hardest_negative_idx[anchor_dist[hardest_negative_idx].argmin()] # label of closest different class

            # load selected
            anchors.append(embeddings[i])
            positives.append(embeddings[hardest_positive])
            negatives.append(embeddings[hardest_negative])


        # Convert lists to tensors
        anchors = torch.stack(anchors)
        positives = torch.stack(positives)
        negatives = torch.stack(negatives)


        loss = criterion(anchors, positives, negatives)
        loss.backward()
        optimizer.step()

        running_loss += loss.item()

    epoch_loss = running_loss / len(dataloader)
    return epoch_loss, 1 #epoch_accuracy

def evaluate(model, dataloader, criterion, device):
    model.eval()
    running_loss = 0.0
    running_accuracy = 0.0

    with torch.no_grad():
        for audio, labels in dataloader:
               audio = audio.to(device)
        labels = labels.to(device)
        embeddings = model(audio)
        
        dist_mat = torch.cdist(embeddings, embeddings, p=2)
        anchors = []
        positives = []
        negatives = []
        for i in range(labels.shape[0]):

            anchor_label = labels[i].item()
            anchor_dist = dist_mat[i]

            hardest_positive_idx = (labels == anchor_label).nonzero(as_tuple=True)[0] # all same class indices
            hardest_positive_idx = hardest_positive_idx[hardest_positive_idx != i] # disclude own label
            hardest_positive = hardest_positive_idx[anchor_dist[hardest_positive_idx].argmax()] # label of furthest same class

            # Hardest negative (closest from different class)
            hardest_negative_idx = (labels != anchor_label).nonzero(as_tuple=True)[0] # all diff class indices
            hardest_negative = hardest_negative_idx[anchor_dist[hardest_negative_idx].argmin()] # label of closest different class

            # load selected
            anchors.append(embeddings[i])
            positives.append(embeddings[hardest_positive])
            negatives.append(embeddings[hardest_negative])


        # Convert lists to tensors
        anchors = torch.stack(anchors)
        positives = torch.stack(positives)
        negatives = torch.stack(negatives)


        loss = criterion(anchors, positives, negatives)

        running_loss += loss.item()

    yes =  umap.UMAP(densmap=True).fit_transform(embeddings.detach().cpu())
    plt.scatter(x=yes[:,0], y=yes[:, 1]), 

    for i, txt in enumerate(labels):
        plt.annotate(txt.item(), (yes[i, 0], yes[i, 1]))
    plt.show()
    epoch_loss = running_loss / len(dataloader)
    return epoch_loss, 1 #epoch_accuracy