from transformers import AutoFeatureExtractor, AutoModelForAudioClassification, Trainer, TrainingArguments
from torchaudio.transforms import MFCC
import torch
import numpy as np
from typing import List
import evaluate

def feature_extractor(audio,
                     sample_rate=16_000,
                     n_mfcc=40):
    mfcc_constructor = MFCC(sample_rate=sample_rate, n_mfcc=n_mfcc)
    return mfcc_constructor(audio)
    

class FeatureExtractor:
    def __init__(self, sample_rate, 
                 n_mfcc=40, 
                 n_fft=128, 
                 hop_length=None):
        
        self.mfcc = MFCC(sample_rate=sample_rate, n_mfcc=n_mfcc)
        # self.spectrogram = Spectrogram(n_fft=n_fft, hop_length=hop_length)

    def make_features(self, audio):
        mfcc = self.mfcc(audio)
        # spec = self.spectrogram(audio)
        # features = torch.stack([mfcc, spec], dim=1)
        breakpoint()
        return mfcc #features
    
def compute_metrics(eval_pred):
    accuracy = evaluate.load("accuracy")
    predictions = np.argmax(eval_pred.predictions, axis=1)
    return accuracy.compute(predictions=predictions, references=eval_pred.label_id)
                        

def featExtractor(
        samples : object,
        model_id : str="MIT/ast-finetuned-audioset-10-10-0.4593",
        normalize_bool: bool=True,
        attn_mask_bool: bool=True,
        truncation: bool = True,
        sampling_rate: int = 8000 # halved from 16k
) -> AutoFeatureExtractor:
    
    audio_array = [x["array"] for x in samples["audio"]]
    
    extractor = AutoFeatureExtractor.from_pretrained(
        model_id, do_normalize=normalize_bool, 
        # return_attention_mask=attn_mask_bool,
        sampling_rate=sampling_rate
    )
    
    audio_inputs = extractor(
        audio_array, sampling_rate=extractor.sampling_rate, 
        truncation=truncation
    )
    
    return audio_inputs
    # return extractor, model
    

def trainModel(
        audio_in : None,
        label2id,
        id2label,
        dir : str="models",
        model_id : str="MIT/ast-finetuned-audioset-10-10-0.4593",
        normalize_bool: bool=True,
        attn_mask_bool: bool=True,
        truncation: bool = True,
        sampling_rate: int = 16_000
    ) -> None:

    
    
    feat_extractor = AutoFeatureExtractor.from_pretrained(
        model_id, do_normalize=normalize_bool, 
        # return_attention_mask=attn_mask_bool,
        sampling_rate=sampling_rate
    )

    
    model = AutoModelForAudioClassification.from_pretrained(model_id,
                                                            label2id=label2id, id2label=id2label,
                                                            ignore_mismatched_sizes=True)

    training_args = TrainingArguments(
        output_dir=dir,
        evaluation_strategy="epoch",
        save_strategy="epoch",
        learning_rate=3e-05,
        per_device_eval_batch_size=32,
        gradient_accumulation_steps=4,
        per_device_train_batch_size=32,
        num_train_epochs=32,
        warmup_ratio=0.1,
        logging_steps=10,
        load_best_model_at_end=True,
        metric_for_best_model="accuracy",
        push_to_hub=False,
        # fp16=True
    )
   ## attention mask???
    trainer = Trainer(
            model=model,
            args=training_args,
            train_dataset=audio_in["train"],
            eval_dataset=audio_in["test"],
            tokenizer=feat_extractor,
            compute_metrics=compute_metrics,
        )

    return trainer

# if __name__ == "__main__":
#     # Load model directly

