---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# NanoGPT

## Resources

### **Pytorch** ==[ Library ]==

#### `import`

- ```python
    import sys
    from pathlib import Path

    # @note 🧠 : In Jupyter cell use `Path.cwd().parent` 
    # to get the parent directory 
    # of the current working directory
    parent_directory = str(Path.cwd().parent)
    if parent_directory not in sys.path:
        sys.path.append(parent_directory)

    from utils.dataframe.util_dataframe_table import UtilDataFrameTable as df_table
  ```

### ==[ Gathered ]== **Data**

#### `wget`

- ```sh
    !wget https://raw.githubusercontent.com/karpathy/char-rnn/master/data/tinyshakespeare/input.txt
  ```

#### ==[ chars ]==

- ```python
    chars = sorted(
      # Build an unordered collection of unique elements
      # ALL duplicate characters are removed from 'text' 
      # to achieve this
      set(text)
    )
  ```

  - --[smoke test]--

    1. `open`
        - // read it to inspect it

        - ```python
            with open("input.txt", "r", encoding="utf-8") as f:
              text = f.read()
          ```

    2. `len`
        - // print length of `text`

        - ```python
            print(f"length of dataset in characters : {len(text)}")
          ```

    3. `print`
        - // print first 100 characters

          - ```sh
              First Citizen:
              Before we proceed any further, hear me speak.

              All:
              Speak, speak.

              First Citizen:
              You
            ```

        - ```python
            print(text[:100])
          ```

### **Token** ==[ Table ]==

  1. --[characters]-- 💬

      - !$&',-.3:;?ABCDEFGHIJKLMNOPQRST
        UVWXYZabcdefghijklmnopqrstuvwxyz

        - ```sh
              0 1 2 3 4 5 6 7 8 9 10 11
            0 0 \n 1  2 ! 3 $ 4 & 5 '
            1 6 , 7 - 8 . 9 3 10 : 11 ;
            2 12 ? 13 A 14 B 15 C 16 D 17 E
            3 18 F 19 G 20 H 21 I 22 J 23 K
            4 24 L 25 M 26 N 27 O 28 P 29 Q
            5 30 R 31 S 32 T 33 U 34 V 35 W
            6 36 X 37 Y 38 Z 39 a 40 b 41 c
            7 42 d 43 e 44 f 45 g 46 h 47 i
            8 48 j 49 k 50 l 51 m 52 n 53 o
            9 54 p 55 q 56 r 57 s 58 t 59 u
            10 60 v 61 w 62 x 63 y 64 z 0 0
          ```

          - `input_phrase` = "Bee3"
          - `print(encode(input_phrase))`

              - ```sh
                  [14, 43, 43, 9]
                  Bee3
                ```

  2. --[vocab_size]-- 🏷️
      - ==[ 65 ]==

## Model

### Data

#### **[ Sets ]**

- `n = int(0.9 * len(data))`
  - // Let's split up the data into train and validation sets
- --[ split ]--
  - ==[ train_data ]== 💾
    - data[:n]
      - // first 90% of the data is training data
  - ==[ val_data ]== 💡
    - data[n:]
      - // the rest is for validation

#### | Tokenize |

##### -- | mapping | --

###### `chars`

- | TRADE-OFFS |
  - 💬 -- character --
    - id table
      - ⛛ (chars)
    - vocabulary
      - 🔺
  - -- subword --
    - id table
      - 🔺 (tokens)
    - vocabulary
      - ⛛

###### `ints`

- 🏷️ ==[ vocab_size ]==
  - ==[ 65 ]==

##### ==[ Encode ]==

- ```python
    # Encoder : takes a string (list of characters) and 
    # returns a list of integers
    encode = lambda s: [stoi[c] for c in s]
  ```

  - ` stoi `

    - ```python
        stoi = {ch: i for i, ch in enumerate(chars)}
      ```

      - // string to integer

    - ✅
      - **input_phrase** = `"Bee3"`
      - encode(**input_phrase**)
        - [14, 43, 43, 9]
    - ❌
      - **input_phrase** = `"Bee7"`
      - encode(**input_phrase**)
        - ⛔ --[ERROR]-- ⛔
          - // **Shakespeare** text does **NOT** have
            **ANY** numerical characters **other than 3**
          - // FOR REALS, check the [characters] 💬 table BRO!
            - --[09]-- 3 😤

##### ==[ Decode ]==

- ```python
    # Decoder :takes a list of integers and outputs a 
    # string (list of characters)
    decode = lambda l: "".join([itos[i] for i in l])
  ```

  - ` itos `

    - ```python
        itos = {i: ch for i, ch in enumerate(chars)}
      ```

      - // integer to string

    - ✅
      - **output_phrase** = `[14, 43, 43, 9]`
      - decode(**output_phrase**)
        - "Bee3"

#### | Tensor |

##### import

- `import`
  - import torch
    - // let's encode the entire shakespeare text
    dataset and store it in a torch.Tensor
  - `data = torch.tensor(encode(text), dtype=torch.int64)`
    - --[data.shape]-- torch.Size([1115394]) 📖
      - 0 .. 5
        - --[18]-- F
          --[47]-- i
          --[56]-- r
          --[57]-- s
          --[58]-- t
          - 1,115,394 📖
    - --[data.dtype]-- torch.int64

##### LANGUAGE MODEL

- **[ train_data ]** 💾
  - -- global --
    - ==[ batch_size = 4 ]== 🪺
      - // number of independent sequences processed in parallel
    - ==[ block_size = 8 ]== 🥚
      - // **maximum character** context **length**
       for prediction processing
        - // given that we can't load the
         entire data set into memory
      - `8` is a good **default**
        - // training on **shorter sequences** yield
          - more memory and **compute efficient**
          - smaller composable tokens more likely to **generalize**
          - may **miss** on deeper correlation though

  - -- methods --
    - def ==[ get_batch() ]== 🧮

      - ```python
          get_batch(split) -> Tuple[torch.Tensor, torch.Tensor]:
        ```

        - ==[ data ]==

          - ```python
               data = train_data if split == "train" else val_data
            ```

            - 💾 train_data
            - 💡 val_data

        - | 1D Tensor |
          - // dimension
            - **batch_size**[4] 🪺  of random integers
          - ==[   ix ]==

            - ```python
                ix = torch.randint(len(data) - block_size - 1, (batch_size,))
              ```

              - **random offset**
                - // Prevents Model from MEMORIZING character POSITION
                  so we can GENERALIZE to ONLY neighboring characters
                  - // generate random offset for each sequence in the batch ...
                  - // - DO NOT EXCEED unit BATCH SIZE
        - | 2D Tensors |
          - // dimension
            - **block_size**[8] 🥚 --characters-- long
            - **batch_size**[4] 🪺 --input-- sequences
          - ==[    x ]==

            - ```python
                x = torch.stack([data[i : i + block_size] for i in ix])
              ```

          - ==[    y ]==

            - ```python
                y = torch.stack([data[i + 1 : i + block_size + 1] for i in ix])
              ```

          - ==[ return ]==

            - ```python
                return x, y
              ```

              - ==[ xb ]== 📥
                - --[inputs]--
                - xb.shape : torch.Size([4, 8])

                  - ```sh
                      tensor([[53, 59,  6,  1, 58, 56, 47, 40],
                              [49, 43, 43, 54,  1, 47, 58,  1],
                              [13, 52, 45, 43, 50, 53,  8,  0],
                              [ 1, 39,  1, 46, 53, 59, 57, 43]])
                    ```

                    - ```sh
                          0  1  2  3  4  5  6  7 
                        0 53 59  6  1 58 56 47 40
                        -  o  u  ,     t  r  i  b
                        1 49 43 43 54  1 47 58  1
                        -  k  e  e  p     i  t 
                        2 13 52 45 43 50 53  8  0
                        -  A  n  g  e  l  o  . \n
                        3  1 39  1 46 53 59 57 43
                        -     a     h  o  u  s  e 
                      ```

              - ==[ yb ]== 🎯
                - --[targets]--
                - yb.shape : torch.Size([4, 8])

                  - ```sh
                      tensor([[59,  6,  1, 58, 56, 47, 40, 59],
                              [43, 43, 54,  1, 47, 58,  1, 58],
                              [52, 45, 43, 50, 53,  8,  0, 26],
                              [39,  1, 46, 53, 59, 57, 43,  0]])
                    ```

                    - ```sh
                          0  1  2  3  4  5  6  7 
                        0 59  6  1 58 56 47 40 59
                        -  u  ,     t  r  i  b  u
                        1 43 43 54  1 47 58  1 58
                        -  e  e  p     i  t     t
                        2 52 45 43 50 53  8  0 26
                        -  n  g  e  l  o  . \n  N
                        3 39  1 46 53 59 57 43  0
                        -  a     h  o  u  s  e \n 
                      ```

  - -- sliced --
    - LLM is essentially built on the
    **block_size** relationships between
      - **CURRENT** BLOCK
        - ==[ input ]== => current **[ `8` characters ]**
      - **NEXT** CHARACTER
        - ==[ target ]== => next **[ `1` character ]**
    - `train_data[:block_size+1]`
      - tensor([18, 47, 56, 57, 58,  1, 15, 47, 58])
      - `+1` because we need to predict the next character

  - -- predicting --
    - the next character in a sequence
      - even across `batches`
    - **[ sliding window ]**
      - `x_0`
        - train_data[:block_size]
        - // first **block_size** 🥚 from **train_data** 💾

          - ```sh
              -- torch.Size([8]) -- 
              -- [x_0] -- batch => (train_data[:block_size])
              tensor([18, 47, 56, 57, 58,  1, 15, 47])
                 0  1  2  3  4  5  6  7
              0 18 47 56 57 58  1 15 47
              0  F  i  r  s  t     C  i
            ```

      - `x_1`
        - train_data[block_size :block_size*2]
        - // next **block_size** 🥚 from **train_data** 💾

          - ```sh
              -- [x_1] -- batch => (train_data[block_size :block_size*2])
              tensor([58, 47, 64, 43, 52, 10,  0, 14])
                 0  1  2  3  4  5  6  7
              0 58 47 64 43 52 10  0 14
              0  t  i  z  e  n  : \n  B
            ```

      - ```python
          for t in range(block_size):
        ```

        - ```python
              context = x_0[: t + 1] 
          ```

          - The **context** is created by taking the characters from
          BEGINNING of **x** up to and INCLUDING the character at
          position **t**

        - ```python
              target = y[t]
          ```

          - The corresponding **target** character for the current
            **context** is the character at position **t** in the
            output sequence **y**

        - | output |

          - ```sh
               @t   context | x_0 | target | y[t] |
              ----  -------         ------
              [ 0]     18     [F]     47     [i]    => tensor([18]) the target is 47
              [ 1]     47     [i]     56     [r]    => tensor([18, 47]) the target is 56
              [ 2]     56     [r]     57     [s]    => tensor([18, 47, 56]) the target is 57
              [ 3]     57     [s]     58     [t]    => tensor([18, 47, 56, 57]) the target is 58
              [ 4]     58     [t]      1     [ ]    => tensor([18, 47, 56, 57, 58]) the target is 1
              [ 5]      1     [ ]     15     [C]    => tensor([18, 47, 56, 57, 58,  1]) the target is 15
              [ 6]     15     [C]     47     [i]    => tensor([18, 47, 56, 57, 58,  1, 15]) the target is 47
              [ 7]     47     [i]     58     [t]    => tensor([18, 47, 56, 57, 58,  1, 15, 47]) the target is 58
            ```

            - | mask | ==[ @audit ]== ...explain this more
              - tri 📐

##### TRAINING

###### INIT

- **nn.Module**
  - -- imports --
    - import torch
    - import torch.nn as nn
    - from torch.nn import functional as F
      - ==[ @audit ]== : what is the purpose of `F`???
    - torch.manual_seed(1337)
      - // set the seed for generating random numbers
      - // we are manually setting to `1337` for reproducibility
  - -- class --
    - ==[ BigramLanguageModel 🧠 ]==(nn.Module):
      - `def`
        - **`__init__`** (self, vocab_size):
          - **vocab_size** is the size of the vocabulary the model will work with

          - ```python
              super().__init__()
              self.token_embedding_table = nn.Embedding(vocab_size, vocab_size)
            ```

            - ==[ token_embedding_table ]==
              - -- args --
                - **num_embeddings**
                - **embedding_dim**
                  - **vocab_size** per embedding
              - -- side effect --
                - creates **embedding layer**
                - simple **lookup table** used to **map** :
                  - each **token in the vocabulary**
                  - to a **vector representation**

        - **`forward`** (self, idx, targets=None):

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

          - `return` logits, loss
        - **`generate`** (self, idx, max_new_tokens):

          - ```python
              for _ in range(max_new_tokens):
                logits, loss = self(idx)
                logits = logits[:, -1, :]  # becomes (B, C)
                probs = F.softmax(logits, dim=1)  # (B, C)
                idx_next = torch.multinomial(probs, num_samples=1)  # (B, 1)
                idx = torch.cat((idx, idx_next), dim=1)  # (B, T+1)
            ```

          - `return` idx
  - -- main --
    - m = BigramLanguageModel(65)
      - 🧠 ==[ m ]==
        - `m.forward(xb, yb)`
          - // pytorch allows us to call a model like a function
          - // - this is the same as calling m.forward(xb, yb)
            - xb = 📥 inputs
            - yb = 🎯 targets
      - **65** == 🏷️ **vocab_size**
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

      - ==[ loss ]== ⚖️
        - tensor(**4.8948**, grad_fn=<**'NllLossBackward0'**>)
          - **-- update weight --**
          - **NllLossBackward0** (backpropagation)
          - used to update model's **weight** during training
  - | PRE-TRAIN |
    - -- generate tensor --
      - ==[ idx ]== 🎚️
        - | EXAMPLE | **gpt 🤖**
          - **['the', 'cat', 'sat', 'on', 'mat']**
            - // language model that works with
            a [ vocabulary ] of [ 5 ] words
          - **idx** = torch.tensor([[0, 1]])
            - // **idx** representing "The cat"
            - // **2D tensor** with **shape (1, 2)**
              - [1] == batch_size (one row)
              - [2] == sequence length, "The" + "cat" == [ 2 tokens ]
          - **-- generate --**
            - idx = torch.tensor([[0, 1, 2, 3, 4]])
              - // After generation, idx might
              - // representing "The cat sat on mat"
              - // **2d tensor** updated to **shape (1, 5)**
                - (1, 2) "The cat"
                - (1, 5) "The cat sat on a mat"
        - **(B,T)** array of indices in the current context
          - **B** is the 🪺 **batch_size**
            - **[8]** in this context
          - **T** is the **length** of **each sequence**
            - not same as 🥚 **block_size**
              - T is the current sequence length of the context during
              generation, and it can change as new tokens are generated
              - 🛑 @audit : I don't understand this lol
        - `idx = torch.zeros((1, 1), dtype=torch.long)`
          - creates a 2D tensor full of zeros with shape (1, 1)
            - **idx** is used as the **initial sequence of tokens** to
            feed into the model for generating new tokens.
          - (token indices) tensor is of:
            - **shape (1, 1)**
              - because it is **intended to hold a sequence of tokens**
            - **type long**
              - because it is **intended to hold integer** values
    - -- print tensor --
      - `print(decode(m.generate(idx, max_new_tokens=100)[0].tolist()))`

      - ```sh
          SKIcLT;AcELMoTbvZv C?nq-QE33:CJqkOKH-q;:la!oiywkHjgChzbQ?u!3bLIgwevmyFJGUGp
          wnYWmnxKWWev-tDqXErVKLgJ
        ```

        - // output is random garbage because
        we haven't trained the model yet

###### STEP

- **batch_size** = 32
  - // @audit increased from `4`?

- ```python
    for steps in range(100000):
  ```

  - ```python
       xb, yb = get_batch("train")
    ```

  - ```python
      logits, loss = m(xb, yb)
    ```

  - ```python
      optimizer.zero_grad(set_to_none=True)
    ```

  - ```python
      loss.backward()
    ```

  - ```python
      optimizer.step()
    ```

###### POST

- ```python
    idx = torch.zeros((1, 1), dtype=torch.long)
  ```

  - -- print --

    - ```python
        print(decode(m.generate(idx, max_new_tokens=300)[0].tolist()))
      ```

      - ```sh
          Ofows ht IUS:
          S:

          ING flvenje ssutefr,
          M:
          War cl igagimous pray whars:
          Panalit I It aithit terised the. by fonau buaror VOubed spo mng as chathab llll:
          Ware,

          ee her,
          Thooured aly y hind Idimashat-owhrees s, share hathure Anfaneof f s llon!
        ```

### Train

#### | Attention |

##### | Self |

##### | Cross |

#### | Mask |

##### Soft Max

##### Tril

### Inference
