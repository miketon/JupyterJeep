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

  2. --[vocab size]-- 🏷️
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

- 🏷️ vocab size
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
  - -- consts --
    - **torch.manual_seed(`1337`)**
      - // inlined to 'leet' to ensure reproducibility of the results
    - ==[ batch_size = 4 ]== 🪺
      - // number of independent sequences processed in parallel
    - ==[ block_size = 8 ]== 🥚
      - // maximum character context length for prediction
        - // given that we can't load the entire data set into memory
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

              - ==[ x ]== 📥
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

### Train

#### | Attention |

##### | Self |

##### | Cross |

#### | Mask |

##### Soft Max

##### Tril

### Inference
