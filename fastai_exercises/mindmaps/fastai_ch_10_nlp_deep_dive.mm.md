---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# NLP

## Text Processing

### DataLoad

- `path` = untar_data(URLs.IMDB)
- `files` = get_text_files(path, folders=[ ... ])
  - `train`
  - `test`
  - `unsup`
- `txt` = files[0].open().read()

### ==[ Tokenize ]==

- Process of splitting text into individual tokens

#### **Unit** Resolution

- 1 - Character
  - `a...z, 0..9 ...etc`
- 2 - ==[ Subword ]==
  - `occasion` => `o` `c` `ca` `sion`
  - most complex and highest potential
    - carries over to **ALL** languages
      - including non-char languages 
        - audio
        - ECG
- 3 - Word
  - `white space` separator

### ==[ Embedding Matrix ]==

## RNN

### Self-supervised

- Training a model using LABELS that are :
  - **EMBEDDED** in the `independent` variable
    - training to predict the next word in a text is an example
  - rather than requiring EXTERNAL labels

#### **ULMFit**

- Universal Language Model Fine-tuning
  - 1 - fine-tune the **sequence based** language model
  - 2 - then fine-tune the **classification** model
- This tends to yield BETTER results