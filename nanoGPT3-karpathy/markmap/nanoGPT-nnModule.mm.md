---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# NN.MODULE

## INIT

### -- library --

- -- imports --
  - import torch
  - import torch.nn as nn
  - from torch.nn import functional as F
    - ==[ @audit ]== : what is the purpose of `F`???
  - torch.manual_seed(1337)
    - // set the seed for generating random numbers
    - // we are manually setting to `1337` for reproducibility
- **nn.Module**
  - methods
    - 🔰 `__init__` (self, vocab_size)
    - ⏩ `forward`  (self, idx 👁️‍🗨️, targets=None)
    - ߷ `generate` (self, idx 👁️‍🗨️, max_new_tokens)
- **nn.Embeddings**

### -- globals --

#### [ tensor 2d ]

- idx
  - 👁️‍🗨️ **==[ idx ]==**
    - type **2D tensor** (B, T)
      - `idx = torch.zeros((1, 1), dtype=torch.long)`
        - **shape (1, 1)**
          - // intended to **hold a sequence of tokens**
          - (B, T) **indices**
            - **B** is the 🪺 **batch_size**
              - **[8]** in this context
            - **T** is the **length** of **each sequence**
              - not same as 🥚 **block_size**
                - T is the current sequence length of the context during
                generation, and it can change as new tokens are generated
                - 🛑 @audit : I don't understand this lol
        - **type long**
          - because it is **intended to hold integer** values
        - | EXAMPLE |
          - creates a 2D tensor full of zeros with shape (1, 1)
            - **idx** 👁️‍🗨️ is used as the **initial sequence of tokens** to
            feed into the model for generating new tokens.
    - **indexes** to ->
      - **token** in the vocabulary
        - where each type of toy are
          - car
          - doll
          - dice ...etc
      - **vector representation**
        - sticker that descibes the toy
          - color,
          - size,
          - how often we play with it ...etc
          - | EXAMPLE | **gpt 🤖**
            - **[ embedding ]**
              - -- vocab + **idx** 👁️‍🗨️ --
                - idx = id + uid.vector

                - ```sh
                    vocab  id  uid.vector
                    -----  --  -----------
                    King    0  [1.0,  0.7]
                    Queen   1  [1.0, -0.5]
                    Man     2  [0.0,  0.5]
                    Woman   3  [0.0, -0.5]
                  ```

              - -- graph --
                - When our computer sees the word "King"
                  - in addition to the letters K, I, N, G.
                  - it ALSO sees the point [1, 0.5] ...

                - ```sh
                    King  (1,  0.7)      x
                    Man   (0,  0.5)     x

                    Queen (1, -0.5)  x
                    Woman (0, -0.5)  x
                  ```

                  - Looking at these
                  points on a graph :
                    - -- distance --
                      - **royalty**
                        - ~ : King  -> Man
                        - ~ : Queen -> Woman
                      - **gender**
                        - ~ : King -> Queen
                        - ~ : Man  -> Woman
                      - word **algebra**
                        - King - Man + Woman = Queen
                    - -- horizontal --
                      - King and Queen match
                      - Man  and Woman match
                    - -- vertical --
                      - King  and Man   aligned **approximately**
                      - Queen and Woman aligned **exactly**
                    - -- ߷ 🧠 ߷ --
                      - ✅  -- Queens are ONLY FEMALE
                      - ✅  -- Kings aren't exclusively MALE
                    - -- other **semantic relationships** --
                      - **[ Remember ]**
                        - These semantic relationships are **not explicitly programmed** into the model
                        - They are **learned from the data the model is trained on**. The model notices
                        that certain words often appear in similar contexts, and it uses this information
                          to **place similar words close together in the embedding space**
                        - This ability to **learn semantic relationships from data** is one of the things that
                        **makes word embeddings so powerful**
                      - synonyms
                      - antonyms
                      - analogies
                      - part-whole relationships
                      - categories
  - -- generate --
    - | EXAMPLE | **gpt 🤖**
      - **['the', 'cat', 'sat', 'on', 'mat']**
        - // language model that works with
        a [ vocabulary ] of [ 5 ] words
      - **idx** 👁️‍🗨️ = torch.tensor([[0, 1]])
        - // **idx** representing "The cat"
        - // **2D tensor** with **shape (1, 2)**
          - [1] == batch_size (one row)
          - [2] == sequence length, "The" + "cat" == [ 2 tokens ]
      - **-- generate --**
        - **idx** 👁️‍🗨️ = torch.tensor([[0, 1, 2, 3, 4]])
          - // After generation, idx might
          - // representing "The cat sat on mat"
          - // **2d tensor** updated to **shape (1, 5)**
            - (1, 2) "The cat"
            - (1, 5) "The cat sat on a mat"

- logits
- targets

#### [ int ]

- vocab_size

### -- class --

- ==[ BigramLanguageModel 🧠 ]==(nn.Module):
  - `self`
    - ==[ token_embedding_table ]==
      - **nn.Embedding**(vocab_size, vocab_size)
        - -- args --
          - **num_embeddings**
          - **embedding_dim**
            - **vocab_size** per embedding
        - -- side effect --
          - creates **embedding layer**
            - a **lookup table** for each **vocab token** and **idx** 👁️‍🗨️
  - `def`
    - 🔰 **`__init__`** (self, vocab_size):
      - **vocab_size** is the size of the vocabulary the model will work with

      - ```python
          super().__init__()
          self.token_embedding_table = nn.Embedding(vocab_size, vocab_size)
        ```

        - self.**token_embedding_table**

    - ⏩ **`forward`** (self, idx 👁️‍🗨️, targets=None):
      - **forward()** method defines how **input** is
      **passed through the layers** of the network

      - ```python
          logits = self.token_embedding_table(idx)  # (B,T,C)
          if targets == None:
            loss = None
          else:
            B, T, C = logits.shape
            logits = logits.view(B * T, C)
            targets = targets.view(B * T)
            loss = F.cross_entropy(logits, targets)
        ```

        - 🧮 **==[ logits ]==**
          - **logits** = self.**token_embedding_table( idx 👁️‍🗨️ )**
            - // (B, T, C)
          - logits.**shape**
            - manually unpacking B, T, C channels
              - | EXAMPLE |
                - // from xb.shape -- 📥 inputs

                - ```python
                  tensor([
                          [53, 59,  6,  1, 58, 56, 47, 40], # Sequence 1
                          [49, 43, 43, 54,  1, 47, 58,  1], # Sequence 2
                          [13, 52, 45, 43, 50, 53,  8,  0], # Sequence 3
                          [ 1, 39,  1, 46, 53, 59, 57, 43]  # Sequence 4
                        ])
                  ```

            - 🪺 ==[ B ]==
              - 🪺 **batch_size**
                - num_rows = [ 4 ]
                  - 4 **sentences** (per batch)
            - 🥚 ==[ T ]==
              - 🥚 **block_size**
                - num_cols = [8]
                  - 8 **words** (sequence length per sentences)
            - 🏷️ ==[ C ]==
              - 🏷️ **vocab_size**
                - num_ids = [65]
                  - 65 **unique ids** (vocabulary)
          - **logits**
            - logits.**view( B * T, C)**
              - ( 🪺 batch_size * 🥚 block_size, 🏷️ vocab_size)
                - This reshaping is done because
                **F.cross_entropy** expects
                  - 🧮 logits (📥 input) @audit ... input v logit
                    - **2D** tensor
                  - 🎯 loss (targets)
                    - **1D** tensor
                - ( **4 * 8** , 65 ) = ( **32***  65 )
                  - reshaping from **3D** to **2D** tensor
                    - // Does NOT help GENERALIZE learning
                    even though it removes batch (sentences)
                    and unfolds to purely words (blocks) @mike
        - `if` targets 👀 == **None**:
          - // This case might happen during inference, when
            we don't have or need target values.
          - loss 🪬 = **None**
        - `else:`
          - Calculate the loss if **targets** are provided
          - 👀 **==[ targets ]==**
            - targets.**view(B * T)**
              - results in a 1D tensor : (B, T) => (B * T)
                - where each element is the "true next token"
          - 🪬 **==[ loss ]==**
            - loss = **F.cross_entropy**( 🧮 logits, 👀 targets )
              - reshaped **logits** and **targets** are passed
              to the **cross entropy** loss function
              - This computes the **loss between** the **network's
              predictions** and the **actual targets**

      - `return` logits 🧮, loss 🪬
        - Finally, the **logits** and the **loss** are
          **returned** from the forward function
          - 🧮 **logits** can be used to **generate predictions**
          - 🪬 **loss** is used during training to **update the model's weights**
    - ߷ **`generate`** (self, idx 👁️‍🗨️, max_new_tokens):

      - args:
        - **idx**
          - a batch of sequences
          - (each sequence is a list of indices)
        - **max_new_tokens**
          - maximum number of new tokens to be generated for each sequence

      - ```python
          for _ in range(max_new_tokens):
            logits, loss = self(idx)
            logits = logits[:, -1, :]  # becomes (B, C)
            probs = F.softmax(logits, dim=1)  # (B, C)
            idx_next = torch.multinomial(probs, num_samples=1)  # (B, 1)
            idx = torch.cat((idx, idx_next), dim=1)  # (B, T+1)
        ```

        - `for _ in range(max_new_tokens) :`
          - self(idx)
            - logits, loss
          - logits
            - logits[:, -1, :]
              - // becomes (B, C)
          - probs
            - F.softmax(logits, dim=1)
              - // (B, C) @audit ... explain
          - idx_next
            - torch.multinomial(probs, num_samples=1)
              - // (B, 1) @audit ... explain
          - idx
            - torch.cat((idx, idx_next), dim=1)
              - // (B, T+1) @audit ... explain

      - `return` idx 👁️‍🗨️

## MAIN

- m = **BigramLanguageModel**( 65 🏷️ )
  - 🧠 **==[ m ]==**
    - `m.forward(xb, yb)`
      - pytorch allows us to call a model like a function
      - This is the same as calling **m.forward(xb, yb)**
        - xb = 📥 inputs
        - yb = 🎯 targets
- logits, loss = m( 📥 xb, 🎯 yb)
  - ==[ logits ]== 🧮
    - logits.shape : torch.Size([32, 65]
      - **-- measure prediction --**
      - used to **measure** the model's prediction
      - | EXAMPLE | **gpt 🤖**
        - -- graph --
          - classify [inputs] [3 batches] => [classes] [5]
          - here is [batch size of 3] and a [vocabulary size of 5]
            - **row**
              - data point in batch [3]
            - **column**
              - [5] class to assign to

        - ```python
            tensor([[ 0.3134, -0.1676,  0.3773, -0.0824, -0.2973],
                    [-0.0856,  0.0987,  0.1772,  0.0565, -0.2135],
                    [ 0.0624,  0.0895,  0.0927, -0.0507, -0.1921]])
          ```

          - | RAW + **UNBOUNDED** |
            - values have no **min/max**
            - and can be positive or negative

        - ```python
            tensor([[0.2175, 0.1338, 0.2301, 0.1450, 0.2736],
                    [0.1823, 0.2194, 0.2372, 0.2109, 0.1502],
                    [0.2108, 0.2171, 0.2182, 0.1882, 0.1657]])
          ```

          - | **NORMALIZED** SOFTMAX |
            - `F.softmax(logits, dim=1)`
            - **convert to probabilities** via softmax
              - all values **between 0 and 1**
              - all **rows sum to 1.0**

  - ==[ loss ]== 🪬
    - tensor(**4.8948**, grad_fn=<**'NllLossBackward0'**>)
      - **-- update weight --**
      - **NllLossBackward0** (backpropagation)
      - used to update model's **weight** during training

## PRE-TRAIN

- -- generate tensor --
  - -- print tensor --
    - `print(decode(m.generate(idx, max_new_tokens=100)[0].tolist()))`

  - ```sh
      SKIcLT;AcELMoTbvZv C?nq-QE33:CJqkOKH-q;:la!oiywkHjgChzbQ?u!3bLIgwevmyFJGUGp
      wnYWmnxKWWev-tDqXErVKLgJ
    ```

    - // output is random garbage because
    we haven't trained the model yet
