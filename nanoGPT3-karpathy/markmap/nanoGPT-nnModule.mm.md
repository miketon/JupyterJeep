---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# NN.MODULE

## 🔑 -- KEY -- 🔑

### -- TRANSFORMER --

#### [ INSIGHTS ]

- -- @**scale** --
  - | PERFORMANCE |
    - | SOFTWARE ARC |
      - batch_size
      - block_size
      - embedding dimension
      - layers
    - | HARDWARE GPU |
      - energy
      - time
      - chunks
- -- @**prediction** --
  - | MEASURE |
    - **NLL** is a good way to MEASURE quality of **predictions** in language models
  - | GENERATE |
    - process of converting logits to probabilities is core of NEXT TOKEN PREDICTION
      - // via softmax and sampling from probabilities
    - 📐 **tri-mask** bounds to current SEQUENCE and prevents SAMPLING from the FUTURE
  - | ATTENTION |
    - | SELF |
      - // samples TOKEN NEIGHBORS in SAME sequence
      - [ self-attention ]
        - solves gathering PAST sequence information in a DATA dependent way
        - allowing TOKENS to learn and interact wrt SEQUENCE
          - PARALLEL and INDEPENDENT production of KEYS and QUERIES for each TOKEN
            - @audit : the embedding table?
          - DOT PRODUCT to generate AFFINITIES and WEIGHTED aggregation in a data dependent manner
            - @audit : cross-entropy and softmax ??
    - | CROSS |
      - // interacts with TOKEN in SEPARATE sequence
      - [ cross-attention ]
        - @audit : Identify where this is used in our BIGRAM model

### 💾 -- ASSETS -- 💾

#### [ TRAINING ]

- [ Data ]
  - **split**
    - 🦠 = ==[ train_data ]==
    - ✅ = ==[ val_data   ]==

#### [ BATCHING ]

##### ⛓️ = ==[ get_batch ]== ⛓️

- `get_batch` (**split** 🦠 )

  - ```python
      data = train_data if split == "train" else val_data
      ix = torch.randint(len(data) - block_size - 1, (batch_size,))
      x = torch.stack([data[i : i + block_size] for i in ix])
      y = torch.stack([data[i + 1 : i + block_size + 1] for i in ix])
    ```

    - **data**
      - 🦠 **train_data** if split == "train"
      - else ✅ **val_data**

  - `returns` **x, y**
    - -> **Tuple**[Tensor, Tensor]
      - **xb** 📥
        - --[inputs]--
          - xb.shape : torch.Size([4, 8])
            - x = x[ **i** ]

            - ```python
                tensor(
                  [[53, 59,  6,  1, 58, 56, 47, 40], # x    
                  ...
                ])
              ```

      - **yb** 🎯
        - --[targets]--
          - yb.shape : torch.Size([4, 8])
            - y = x[ **i+1** ]

            - ```python
                tensor(
                  [[59, 6,  1,  58, 56, 47, 50, 59], # x+1 
                  ...
                ])
              ```

### 🌍 -- MODEL -- 🌍

#### [ TYPE ]

- ✅ | BIGRAM | ✅
  - @embedding_table 🌐
    - @audit : Does nn.Module Bigram have weights and bias in addition to an embedding_table?
    - @audit : BIGRAM can also be implemented as counting and probability?
    - 🧠 **nn.Module**
      - [ Params ]
        - 🌐 self.**token_embedding_table**
          - 🕸️ **nn.Embeddings**
      - [ Methods ]
        - 🚧 `__init__` (self, vocab_size)
        - 🔜 `forward`  (self, idx 👁️‍🗨️, targets=None)
        - ߷ `generate` (self, idx 👁️‍🗨️, max_new_tokens)
- 🛜 🔐 -- OTHERS -- 🔐 🛜
  - 🧧 @follow-up : Types and Params?
  - | ANN |
    - @weights
    - @bias
  - | GPT |
    - @attention
  - | RNN |
    - @gate
  - | ResNet |
    - @residual connections

### -- @MAIN 🛜 --

#### [ INSTANTIATE ]

- -- @python --
  - @🪺
    - `batch_size = 32`
- -- @pytorch --
  - @🧠
    - `m = BigramLanguageModel( 65 )`
      - @🌐. -- 🔑 --
        - // generates embeddings table
  - @🧩
    - `optimizer = torch.optim.AdamW(m.parameters(), lr=1e-3)`

#### [ UPDATE PARAMS ]

- [ TRAINING ]
  - @♻️
    - `for steps in range(10,000) :`
      - ⛓️
        - `xb, yb = get_batch("train")`
          - // xb (input),  yb (target
      - 🧠
        - `logits, loss = m (xb, yb)`
          - 🔜
            - // m.forward(idx=xb, target=yb)
      - 🧩
        - `optimizer.zero_grad(set_to_none=True)`
        - `loss.backward()`
          - // getting gradient of loss wrt to model parameters
          - -- 🔑💡 -- CALCULATE **LOSS** 🪬
        - `optimizer.step()`
          - // using gradient to update model parameters
          - -- 🔑💡 -- GRADIENT **UPDATE PARAMS**
            - | @BIGRAM | ✅

#### [ GENERATE ]

- [ PRINT TOKENS ]
  - @👁️‍🗨️
    - idx = torch.zeros((1, 1), dtype=torch.long)
    - 🆗 @udit-ok 🆗 : Why reset idx?
      - ANSWER: ☑️
        - zero-ing idx is essentially an EMPTY prompt
          - We want "neutral" input because we want the output
          to SOLELY based on it's LEARNED parameters
  - @🧠.߷
    - `print(decode(m.generate(idx, max_new_tokens=300, debug=True)[0].tolist()))`
      - -- 🔑💡 -- **GENERATE** TOKENS

## INIT

### -- library --

- -- imports --
  - import torch
  - import torch.nn as nn
    - 🆗 @udit-ok 🆗 : What is the purpose of `nn`???
      - ANSWER: ☑️
        - nn.Module - pytorch 🧠 model : 🚧, 🔜, ߷
          - nn.Embedding ️🕸️ on nn.Module **init** 🚧
  - from torch.nn import functional as F
    - ==[ F ]==
      - 🆗 @udit-ok 🆗 : What is the purpose of `F`???
        - ANSWER: ☑️
          - shorthand for `torch.nn.functional` module
      - 📦 ==[ UNPACK ]== 📦
        - | CHANNEL |
          - B, T, C = logits.**shape**
            - nn.Module
              - `__init__`(self, vocab_size 🏷️ ):
                - 🌐 self.**token_embedding_table**
                  - = 🕸️ nn.**Embedding**(vocab_size 🏷️, vocab_size 🏷️)
            - logits 🧮 = 🌐 self.**token_embedding_table**(idx 👁️‍🗨️) ♿
              - // returns (B, T, C)
          - 2D Tensor
            - (**B * T**, C)
              - logits 🧮 = logits.**view**( B * T, C)
          - 1D Tensor
            - (**B * T**)
              - targets.**view** (B * T)
      - ⚔️ = ==[ cross_entropy ]== ⚔️
        - (🧮, 🎯) => 🪬
          - input `args`
            - 🆗 @udit-ok 🆗 : logits 🧮 and targets 🎯 have **different
             dimensions**, isn't this a **mismatch** ?  Why???
              - ANSWER: ☑️
                - Even though logits and targets have **different
                dimensions**, they are **perfectly compatible** for
                the **cross entropy loss** function because they
                each provide a piece of **information needed**
                to **calculate the loss**
                  - predicted **probabilities**
                    - **2nd dimension** needed for each **[C]**ategory
                    - **probabilities** == each **[C]** per token **softmax** sum to 1
                    - so we can **nll** to reduce **loss** error
                  - **true** classes
                    - store the **actual** id target 🎯
                    - every other **[C]**ategory would have a value of **0**
                    - it's **naive** to store all those channels when the **actual** id target 🎯 is available
                    - **compress** to 1 dimension
            - **logits** 🧮
              - **2D** Tensor (B * T, C)
                - **predicted** probabilities (score per category)
              - | EXAMPLE | **gpt 🤖**

                - **[C]** vocab = **4**
                  - ['cat', 'dog', 'bird', 'fish']
                  - index
                    - [0, 1, 2, 3]
                - **[B]** = **2** Batch (Sentences)
                  - Sentence 1:
                    - **[T]** = **3** Tokens:
                      - **[0, 1, 2]**  # corresponds to "cat dog bird"
                  - Sentence 2:
                    - **[T]** = **3** Tokens:
                      - **[2, 3, 1]**  # corresponds to "bird fish dog"

                - ```python
                    logits = [
                     # ---  ---  ---- ----
                     # cat  dog  bird fish
                     # ---  ---  ---- ----
                     # sentence 1 - 'cat dog bird'
                      [0.7, 0.2, 0.1, 0.0],  # scores for first token
                      [0.2, 0.7, 0.0, 0.1],  # scores for second token
                      [0.1, 0.1, 0.7, 0.1],  # scores for third token
                     # sentence 2 - 'bird fish dog'
                      [0.1, 0.1, 0.7, 0.1],  # scores for fourth token
                      [0.0, 0.0, 0.1, 0.9],  # scores for fifth token
                      [0.2, 0.7, 0.0, 0.1]   # scores for sixth token
                    ]
                  ```

                  - tensor.shape (2x3, 4) // (B * T, C)
                    - The logits tensor might look like this if for
                    **simplicity**, let's assume the model gives **equal
                    scores** to **each class**
                    - NAH, Imma hand edit to approximate training lol

                - F.**cross_entropy**(🧮, 🎯) =>
                 🪬 **cross entropy loss** results
                  1. It applies the **softmax** function to the **logits** 🧮 to convert them into **probabilities**
                  2. Then, it calculates the 🪵 **NLL** of the **true** class
                  3. Specified by **targets** 🎯 under this probabilistic distribution
            - **targets** 🎯
              - **1D Tensor** (B * T, )
                - 🆗 @udit-ok 🆗  : **Difference** between
                 **(B * T, )** and **(B * T)** ?????
                  - ANSWER: ☑️
                    - The difference is **subtle** overall
                      - **(B * T ,)** is more **clearly a 1D Tensor**, the
                      trailing comma indicates it's a **tuple**
                      with a single element
                      - **(B * T)** on the otherhand can be misinterpreted
                      as a **parenthiszed expression**
                - **true** classes (the actual next word)
              - | EXAMPLE | **gpt 🤖**

                - ```python
                    targets = [0, 1, 2, 2, 3, 1] # 'cat', 'dog', 'bird', 'bird', 'fish', 'dog'
                  ```

                  - tensor.shape(2x3, ) // (B * T, )
                  - this is the **true** class, so we do NOT need
                  PROBABILITIES and only need a vector of 1 for
                  **[C]** vocab

                    - **NAIVELY** expanded => **logits** 🧮

                      - ```python
                        targets = [
                        # ---  ---  ---- ----
                        # cat  dog  bird fish
                        #   0    1     2    3  == id
                        # ---  ---  ---- ----
                        # sentence 1 - 'cat dog bird'
                          [1.0, 0.0, 0.0, 0.0],  # scores for first token
                          [0.0, 1.0, 0.0, 0.0],  # scores for second token
                          [0.0, 0.0, 1.0, 0.0],  # scores for third token
                        # sentence 2 - 'bird fish dog'
                          [0.0, 0.0, 1.0, 0.0],  # scores for fourth token
                          [0.0, 0.0, 0.0, 1.0],  # scores for fifth token
                          [0.0, 1.0, 0.0, 0.0]   # scores for sixth token
                        ]
                        ```

          - `returns` the **loss** 🪬 given
            - 🧠 model's **prediction**
            - actual **targets** 🎯
      - 🪵 = ==[ NLL ]== 🪵

        - ```python
            NLL = -sum(y_true * log(y_pred))
          ```

          - | EXAMPLE | **gpt 🤖**

          - ```python
              # True class
              y_true = np.array([0, 1, 0])

              # Predicted probabilities
              y_pred = np.array([0.2, 0.5, 0.3])

              # Compute NLL
              nll = -np.sum(y_true * np.log(y_pred))
            ```

        - 🆗 @udit-ok 🆗 : What is **nll**
          - ANSWER: ☑️
            - **explain** like I am 5
              - **negative likelihood**
                - the more **CORRECT** your guesses
                are the **lower** this value
                - likelihood otoh is higher with correctness,
                this is the opposite of that
              - **log**
                - log **MAGNIFIES** small **differences**
                - small **ERROR** return proportionately **HIGHER LOSS**
            - **nll** is useful for ML **because** :
              - **Measure of Error**
                - NLL **measures** the **discrepancy**/error between
                  the **prediction** and the **actual** values
              - **Handling Probabilities**
                - **true label** = class with **1.0**, and all
                others at 0.0,
                - **NLL measures** this to the
                model's **predicted probability**
              - **Emphasizes Correct Predictions**
                - the **log** part of NLL **heavily penalizes**
                the **model** for being **VERY CONFIDENT**
                but **WRONG**
              - **Optimization**
                - **NLL** is a **smooth and differentiable** function,
                making it suitable for **GRADIENT DESCENT**
                - supports which **direction to adjust** model
                **parameter** to improve **predictions**
  - torch.manual_seed(1337)
    - // set the seed for generating random numbers
    - // we are manually setting to `1337` for reproducibility

### ♿ -- globals -- ♿

#### [ int ]

- 🪺 = ==**[ B ]**== atch
  - 🪺 **batch_size**
    - num_rows = **[ 4 ]**
      - 4 **sentences** (batch) processed in **parallel**
- 🥚 = ==**[ T ]**== okens
  - 🥚 **block_size**
    - num_cols = **[ 8 ]**
      - 8 **words** token length per batch (sentences)
- 🏷️ = ==**[ C ]**== ategories
  - 🏷️ **vocab_size**
    - num_ids = **[ 65 ]**
      - 65 **unique word ids** (categories)

#### [ nn.Module ]

- 🧠 = ==**[ m ]**==
  - BigramLanguageModel(**nn.Module**)

##### [ nn.Embeddings ]

- 🌐 = ==**[ m.token_embedding_table ]**==
  - m.init()
    - 🆗 @udit-ok 🆗 : ... is 🕸️ nn.Embeddings GLOBALLY ACCESSIBLE???
      - ANSWER: ☑️
        - NO, m.token_embedding_table PERSISTS @🧠 methods like forward and generate
        - BUT it isn't GLOBALLY accessible outside of 🧠

#### [ torch.Optim ]

- 🧩 = ==**[ optimizer ]**==

### -- heap --

#### [ tensor 2d ]

##### -- input --

- 👁️‍🗨️ = **==[ idx ]==**
  - -- forward --
    - 📥 **xb**
      - **-- 🛜 👁️‍🗨️ 🛜 --**
        - in the **forward** pass => 📥 **xb** as 👁️‍🗨️ **idx**
          because 📥 **xb** is **also** a 2D tensor **(B, T)**
    - type **2D tensor** (B, T)
      - `idx = torch.zeros((1, 1), dtype=torch.long)`
        - **shape (1, 1)**
          - // intended to **hold a sequence of tokens**
          - (B, T) **indices**
            - **B** is the 🪺 **batch_size**
              - **[8]** in this context
            - **T** is the **length** of **each sequence**
              - **!NOT!** same as 🥚 **block_size**
                - because **T** is the **current sequence length** of
                the context during generation, and it can change as
                new tokens are generated
                - however, 🥚 **block_size** is the max length of **T**
        - **type long**
          - because it is **intended to hold integer** values
        - | EXAMPLE |
          - creates a 2D tensor full of zeros with shape (1, 1)
            - 👁️‍🗨️ **idx** is used as the **initial sequence of tokens** to
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
    - in the generate pass we building the **idx**
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
- = 🎯 ==**[ targets ]**==
  - **yb** 🎯

##### -- evaluated --

- 🧮 = **==[ logits ]==**
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
- 🪬 = **==[ loss ]==**
  - tensor( 4.8948, **grad_fn**=<`NllLossBackward0`>)
    - **-- update weight --**
    - used to update model's **weight** during training
    - | EXAMPLE | **gpt 🤖**
      - **[ grad_fn ]**
        - **NllLossBackward0** @⚔️
          - used in **classification** tasks
          - Negative Log Likelihood loss operation
        - 🛜🔐 -- OTHERS -- 🔐🛜
          - 🧧 @follow-up : Loss Types
          - AddBackward0
            - for the addition operation
          - MulBackward0
            - the multiplication operation
          - MeanBackward0
            - function for the mean operation
          - AccumulateGrad
            - .backward() function on a tensor to compute the gradients
          - Conv2DBackward
            - used in Convolutional Neural Networks (CNNs)
          - LinearBackward
            - used in Fully Connected layers of Neural Networks

### -- class --

#### ==[ BigramLanguageModel 🧠 ]==(nn.Module)

- `self`
  - 🌐 = ==[ token_embedding_table ]==
    - 🕸️ **nn.Embedding**(vocab_size, vocab_size)
      - -- args --
        - **num_embeddings**
        - **embedding_dim**
          - **vocab_size** per embedding
      - -- side effect --
        - creates **embedding layer**
          - a **lookup table** for each **vocab token** and **idx** 👁️‍🗨️
- `def`
  - 🚧 **`__init__`** (self, vocab_size):
    - **vocab_size** is the size of the vocabulary the model will work with

    - ```python
        super().__init__()
        self.token_embedding_table = nn.Embedding(vocab_size, vocab_size)
      ```

      - self.**token_embedding_table**
  - 🔜 **`forward`** (self, idx 👁️‍🗨️, targets=None):
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

      - **logits** = self.**token_embedding_table( idx 👁️‍🗨️ )**
        - 🌐 @embedding_table ♿
          - **idx**    .shape - **torch.Size([4, 8])**
          - **logits** .shape - **torch.Size([4, 8, 65])**
        - // (B, T, C)
          - **-- 🛜 🔐 🛜 --**
            - 🆗 @udit-ok 🆗 : we **MUST** always get **logits**
            - `generate`() implicitly depends on this via self(idx)
      - `if` targets 👀 == **None**:
        - // This case might happen during inference, when
          we don't have or need target values.
        - loss 🪬 = **None**
      - `else:`
        - 📦 | @UNPACK | ⚔️
          - 🧮 **==[ logits ]==**
            - logits.**shape**
              - **unpack** to B, T, C channels
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

            - **logits**
              - logits.**view( B * T, C)**
                - // **repack** to B * T, C channels
                - ( 🪺 batch_size * 🥚 block_size, 🏷️ vocab_size)
                  - This reshaping is done because
                  ⚔️ **F.cross_entropy** expects
                    - 🧮 logits (📥 input)
                      - 🆗 @udit-ok 🆗 : ... input v logit?
                        - ANSWER: ☑️
                          - The BOUND is tensor.shape : (B*T, C)
                          - batch -> xb input -> m.logits is contextual to our model
                      - **2D** tensor
                    - 🎯 loss (targets)
                      - **1D** tensor
                  - ( **4 * 8** , 65 ) = ( **32***  65 )
                    - reshaping from **3D** to **2D** tensor
                      - // Does NOT help GENERALIZE learning
                      even though it removes batch (sentences)
                      and unfolds to purely words (blocks) @mike
        - Calculate the loss if **targets** are provided
        - 👀 **==[ targets ]==**
          - 🆗 @udit-ok 🆗 Where are **targets** sourced from?
            - ANSWER: ☑️
            - **get_batch** ⛓️ returns (xb, **yb** 🎯 )
              - // ♿ **yb** is stored at | MAIN | **scope**
            - **targets** 👀 <<< **yb** 🎯 when 🧠 **m** (xb, **yb**)
          - targets.**view(B * T)**
            - results in a 1D tensor : (B, T) => (B * T)
              - where each element is the "true next token"
        - 🪬 **==[ loss ]==**
          - loss = ⚔️ **F.cross_entropy**( 🧮 logits, 👀 targets )
            - reshaped **logits** and **targets** are passed
            to the **cross entropy** loss function
            - This computes the **loss between** the **network's
            predictions** and the **actual targets**
    - `return` logits 🧮, loss 🪬
      - 🆗 @udit-ok 🆗 : ... where is it returned to ???
        - ANSWER: ☑️
          - | MAIN.**TRAINING** ♿ | **loop** ♻️
            - ♻️ | FORWARD PASS |
              - logits, loss = **m**(xb, yb)
            - ♻️ | COMPUTE GRADIENTS |
              - loss.**backward()**
            - ♻️ | UPDATE WEIGHTS |
              - optimizer.**step()**
            - ♻️ | CLEAR GRADIENTS |
              - optimizer.**zero_grad()**
                - (for next iteration)
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
          logits, _ = self(idx) # logits, discarding loss = NONE
          # focus only on the last[-1] [T]ime step ... we are :
          # (B, T, C) -> (B * T, C) where B * T = B * T[-1] 
          # ... B != T[-1]
          logits = logits[:, -1, :]  # becomes (B, C)
          # bag of marbles - for each marble a prob sums to 1.0
          probs = F.softmax(logits, dim=1)  # (B, C)
          # calculate the next likely marble
          idx_next = torch.multinomial(probs, num_samples=1)  # (B, 1)
          # add marble to sequence .i.e. generate next token
          idx = torch.cat((idx, idx_next), dim=1)  # (B, T+1)
      ```

      - `for _ in range(max_new_tokens) :`
        - logits
          - self(**idx**)
            - ~== 🔜 **forward**(self, idx, None)
              - self(idx) is effectively calling `nn.Module.__call()__`
                - // + additional hooks before and after
                - // so not exactly the same, but close enough
            - **logits** , _
              - 🆗 @udit-ok 🆗
                - discarding **loss**, because will always be **NONE**
                  - the '_' **underscore** character indicates
                  a variable that is **NOT USED**
          - `logits[:, -1, :]`
            - (B,T,C) => (B, C)
              - shape :  **torch.Size([1, 1, 65])**
              - shape :  **torch.Size([1, 65])**
              - [: -1, :] = B[:] T[-1] C[:]
                - 🆗 @udit-ok 🆗
                  - Selects the LAST set of [T]ime step in each [B]atch :
                  (B, T, C) => (B * T, C) **where B * T = B * T[-1]**
                    - ' :' = select ALL [B]atch
                    - '-1' = select the LAST [T]ime entry
                    - ' :' = select ALL [C]ategory
                      - | EXAMPLE |

                        - ```python
                          [   [B]   [T]
                              ---   ---   C0   C1   C2   C3  [C]
                                          --   --   --   --  ---
                              B0 [
                                    T0  [1.0, 2.0, 3.0, 4.0]
                                    T1  [9.0, 9.2, 9.5, 9.7] 
                                    # Last time step in     - B0 - 
                                    T2  [5.0, 6.0, 7.0, 8.0]
                                ]

                              B1 [
                                    T0  [9.1, 9.3, 9.6, 9.8] 
                                    T1  [5.1, 6.1, 7.1, 8.1]
                                    # Last time step in     - B1 -  
                                    T2  [1.1, 2.1, 3.1, 4.1]
                                ]
                          ]
                          ```

                          - The set of [C]ategory logits for the LAST
                          [T]ime step in each [B]atch are
                            - | B0:T(-1) | **T2** == [5.0, 6.0, 7.0, 8.0]
                            - | B1:T(-1) | **T2** == [1.1, 2.1, 3.1, 4.1]
        - 🎱 probs - shape :  **torch.Size([1, 65])**
          - F.**softmax**(logits, dim=1)
            - | **logits** |
              - shape : **torch.Size([1, 65])**
              - starts out as raw, unnormalized predictions or output of model
              - **softmax** takes a **range** of
              values and **map** them where
                - the **sum** of all element == **1.0**
                - each **element** is a valid
                probability between **0.0 and 1.0**
            - | **dim** |
              - batch of data with dimensions [batch_size, vocab], **dim=1**
              **softmax** will **apply to each row** i.e. each individual sample
              in the batch
                - 0 = columns
                - 1 = rows
                - @audit : Explain dimensions higher than a table, 3D ?
            - | EXAMPLE |

              - ```python
                  [
                    [5.0, 6.0, 7.0, 8.0], # B0:T(-1)  
                    [1.1, 2.1, 3.1, 4.1]  # B1:T(-1)
                  ]
                ```

                - F.softmax(logits, dim=1)

                  - ```python
                      tensor([
                              [0.0321, 0.0871, 0.2369, 0.6439], 
                              [0.0321, 0.0871, 0.2369, 0.6439]
                            ])
                    ```

                    - 🆗 @udit-ok 🆗 : Why are BOTH ROWS EQUAL???
                    - ANSWER: ☑️
                      - The **RELATIVE DIFFERENCES** in each ROW are the SAME,
                      each probability is **1.0 MORE** than the **PREVIOUS**
                      - BECAUSE :
                        - softmax EXPONENTIATES it's input so ALL LOGITS are POSITIVE
                        - @ all POSITIVE logits become LARGER
                        - @ all NEGATIVE logits become SMALLER (but stay positive)
                        - **RATIO** between positive numbers REMAIN the **SAME**
                          - when softmax **normalizes** exponentiated
                          logits so they **SUM to 1.0**, the resulting
                          'prob' **maintain** the exp **RATIO**

              - ```python
                  [ # random tensor
                    [0.0560, 0.4175, 0.3114, 0.9418], 
                    [0.9193, 0.1046, 0.0022, 0.0089]
                  ]
                ```

                - F.softmax(logits, dim=1)

                  - ```python
                      tensor([
                              [0.1626, 0.2334, 0.2099, 0.3942], 
                              [0.4455, 0.1972, 0.1780, 0.1792]
                            ])
                    ```

        - **idx_next** - shape : **torch.Size([1, 1])**
          - torch.**multinomial**(probs, num_samples=1)
            - **multinomial**
              - 🆗 @udit-ok 🆗 :
                - samples from 🎱 'probs' distribution (bag of marbles)
                  - 'probs' == **bag of marbles** 🎱
                    - The marbles in the bag are NOT equally distributed
                    - There are more of some colors and less of others
                    - This UNEVEN distribution will be represented in the 'probs' tensor
                  - Each COLOR [Category] has a probability associated with it
                    - So, if the 'probs' tensor indicates that the "red"
                    class has a high probability
                      - it's like the bag has many red marbles,  
                      - and you're more likely to pull out a red marble.
              - inputs
                - 'num_samples'
                  - 🆗 @udit-ok 🆗 :
                    - Tasks where > 1 samples are beneficial
                      - Beam Search
                      - Ensemble Methods
                      - Uncertainty Qualifications
                      - Diversity Promotion
                      - Reinforcement Learning
                - 'probs' is expected to be a valid probability
                  - must be positive values ONLY
                    - pytorch will throw ERROR
                  - all entries must SUM up to 1.0
                    - no ERROR thrown, but implicit drift because
                    probability constraint not adhered to
              - `returns` **idx_next**
                - **picking** 1 (num_samples) **marble** 🎱 from each item in the batch
                - the index (marble) chosen for each item
                  - | EXAMPLE |

                    - ```python
                        probs = torch.tensor([
                          # [C]      0    1    2    3    [B]atch
                          #        ---  ---  ---  ---    ---
                                  [0.1, 0.2, 0.5, 0.2], #  0
                                  [0.3, 0.3, 0.2, 0.2], #  1
                                  [0.2, 0.2, 0.2, 0.4], #  2
                        ])

                        idx_next = torch.multinomial(probs, num_samples=1)
                        print(idx_next)
                      ```

                      - ```sh
                          tensor([
                            [2], # 0.5
                            [0], # 0.3 ... [1] also valid  
                            [3], # 0.4  
                          ])
                        ```

                        - Actual [C]ategory returned is ==[ NON-DETERMINISTIC ]==
                        when 'prob' values are **EQUAL**
                        - We returned **0** in this case, even though **1** ALSO equals **0.3**
                          - Rerun doesn't guarantee **0**

        - **idx**
          - torch.**cat**((idx, idx_next), dim=1)
            - cat() is a function that **combines**
            **tensors** along a specific **dimension**
              - | EXAMPLE |

                - ```python
                    idx = torch.tensor([
                      [1, 2, 3]
                    ])

                    idx_next = torch.tensor([
                      [4]
                    ])
                  ```

                  - idx = torch.cat((idx, idx_next), dim=1)
                  - print(idx)

                    - ```sh
                        tensor([[1, 2, 3, 4]])
                      ```

    - `return` idx 👁️‍🗨️
      - idx.shape :  **torch.Size([1, 301])**
        - returning **generated text**
        <= **max_new_tokens** (300)

## 🛜 MAIN 🛜

### ⛓️ **get_batch** ( =="train"== )

- 📥 = ==[ xb ]==
  - --[inputs]-- // ♿  `idx` 👁️‍🗨️
    - xb.**shape** : torch.Size(**[32, 8]**)
      - row [32]
      - columns [8]
- 🎯 = ==[ yb ]==
  - --[targets]-- // ♿ `targets` 👀
    - yb.**shape** : torch.Size(**[32, 8]**)
      - row [32]
      - columns [8]

### 🧠 ==[ m ]== // ♿

- m = **BigramLanguageModel**( 65 🏷️ )
- 🧠 m( 📥 **xb** 👁️‍🗨️, 🎯 **yb** 👀)
  - This is the same as calling
    - 🔜 m.**forward**( 📥 xb, 🎯 yb )
      - **-- 🛜 👁️‍🗨️ 🛜 --**
        - 🆗 @udit-ok 🆗 : forward signature expects `idx` not `xb` EXPLAIN THIS!
          - ANSWER: ☑️
            - forward(self, **idx**, targets=None) ...
            - **xb** 📥 is compatible with **idx** because
            they are both 2D tensor **(B, T)**
    - pytorch allows us to call a model like a function
  - `returns`
    - **[ logits ]** 🧮
    - **[ loss ]** 🪬

### ♿ | TRAINING | ♿

#### -- init --

- 🧩 ==[ optimizer ]==
  - torch.optim.**AdamW**( 🧠 **m**.parameters(), **lr**=`1e-3`)
    - create pytorch optimizer
    - 🆗 @udit-ok 🆗 : Explain why AdamW > SGD and 1e-3
      - ANSWER: ☑️
        - AdamW > SGD because it has VARIABLE STEP SIZE, this reduces
        overshoot and DECREASES NOISE during training as a result
        - '1e-3' is a HEURISTIC that was found to work well through
        TRIAL and ERROR
- 🪺 **batch_size** = `32`

#### ♻️ = ==[ loop ]== = ♻️

- for `steps` in range( `100,000` ):

  - ```python
      xb, yb = get_batch("train")
      logits, loss = m(xb, yb)
      optimizer.zero_grad(set_to_none=True)
      loss.backward()
      optimizer.step()
    ```

    - // sample batch of data
      - ⛓️ **get_batch** ("train")
        - xb 📥
        - yb 🎯

    - // evaluate the loss
      - **m** 🧠 ( xb 📥, yb 🎯)
        - logits 🧮
        - **loss** 🪬
    - // zero-ing out gradient from previous step
      - 🧩 optimizer.**zero_grad**(set_to_none=True)
        - 🆗 @udit-ok 🆗 : Why zero out grad
          - ANSWER: ☑️
        - 🆗 @udit-ok 🆗 : Why set_to_none=True
          - ANSWER: ☑️
            - // Diff is how memory are handled after the gradient is zero-ed out
            - True
              - biased to **memory** perf
              - clear out memory > 1 cycle
              - **LARGER** MODELS are more **MEMORY CONSTRAINED**
            - False
              - biased to **cpu** perf
              - uses a bit more memory
    - // getting gradient of loss wrt to model parameters
      - 🪬 loss.**backward()**
    - // using gradient to update model parameters
      - 🧩 optimizer.**step()**
