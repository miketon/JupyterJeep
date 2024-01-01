---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# NLP

- Challenges :
  - 1 - NON-UNIFORM word COUNT per **sentences**
  - 2 - NON-UNIFORM sentence COUNT per **documents**

- Roadmap :
  - 1 - **capture** the FULL vocabulary, i.e. all possible levels of categorical 
  variables
  - 2 - replace each vocabulary term with it's **index** in the vocabulary
  - 3 - **generate** an embedding matrix for each item in the vocab
  - 4 - **embedding matrix** is utilzed as the **1st layer** of the neural  
  network
    - the embedding matrix accepts vocabulary index (@step 2) as input
    - functionally equivalent to one-hot-encoding but faster and more efficient

## Text Processing

### -- DataLoad --

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
  - complex but offers highest potential
    - generalizes **across** languages
    - even adaptable to non-textual data such as :
      - audio
      - ECG
- 3 - Word
  - `white space` separator

#### **WordTokenizer**

- // split text to tokens
- `spacy` = WordTokenizer()
  - WordTokeizer is a Fastai tokenizing library that :
    - 1 - collect subwords
    - 2 - handles context

      - ```python
        first(spacy(['The U.S. dollar $1 is $1.00.']))
        # outputs : (#9) ['The','U.S.','dollar','$','1','is','$','1.00','.']
        ```

- `toks` = first(spacy([txt]))
- print(coll_repr(toks, 30))

### ==[ Embedding Matrix ]==

#### Handle tokens NOT in vocabulary

- 1 - Skip the token
- 2 - Use a special token
- 3 - Break it down
- 4 - Learn the new token
- 5 - Make a new token

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

- This tends to yield BETTER results because :
  - wiki pred => IMDB pred >>> review classifier
    - IMDB set has 50k reviews to improve token prediction perf
    - BEFORE we classify review sentiment
- Fundamental difference between **NLP** and **vision model** fine-tuning
  - 1 - NLP fine-tunes the pretrained model for a DIFFERENT task
    - classification vs sequence
  - 2 - Vision Model fine-tunes to IMPROVE PERF in the same task domain