---
markmap:
   colorFreezeLevel: 2
   maxWidth: 500
---

# NanoGPT

## Resources

### **Pytorch** ==[ Library ]==

#### `import`

-

  ```python
    import sys
    from pathlib import Path

    # @note 🧠 : In Jupyter cell use `Path.cwd().parent` to get the parent directory 
    # of the current working directory
    parent_directory = str(Path.cwd().parent)
    if parent_directory not in sys.path:
        sys.path.append(parent_directory)

    from utils.dataframe.util_dataframe_table import UtilDataFrameTable as df_table
   ```

### ==[ Gathered ]== **Data**

#### `wget`

-

  ```sh
    !wget https://raw.githubusercontent.com/karpathy/char-rnn/master/data/tinyshakespeare/input.txt
  ```

#### ==[ chars ]==

-

  ```python
    chars = sorted(
      # Build an unordered collection of unique elements
      # ALL duplicate characters are removed from 'text' 
      # to achieve this
      set(text)
    )
  ```

  1. --[smoke test]--

      - `open`
        - // read it to inspect it

        -

          ```python
            with open("input.txt", "r", encoding="utf-8") as f:
              text = f.read()
          ```

      - `len`
        - // print length of `text`

        -

          ```python
            print(f"length of dataset in characters : {len(text)}")
          ```

      - `print`
        - // print first 100 characters

        -

          ```python
            print(text[:100])
          ```

### **Token** ==[ Table ]==

  1. --[characters]-- 💬

      - !$&',-.3:;?ABCDEFGHIJKLMNOPQRST
        UVWXYZabcdefghijklmnopqrstuvwxyz

        -

          ```sh
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

          - 1. `input_phrase = "Bee3"`
          - 2. `print(encode(input_phrase))`

                -

                  ```sh
                    [14, 43, 43, 9]
                    Bee3
                  ```

  2. --[vocab size]-- 📖
      - ==[ 65 ]==

## Model

### Data

#### | Split |

- `n = int(0.9 * len(data))`
  - // Let's split up the data into train and validation sets
- | **sets** |
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
  - -- character -- 💬
    - id table
      - ⛛
    - vocabulary
      - 🔺
  - -- subword --
    - id table
      - 🔺
    - vocabulary
      - ⛛

###### `ints`

- vocab size 📖
  - ==[ 65 ]==

##### ==[ Encode ]==

-

  ```python
    # Encoder : takes a string (list of characters) and 
    # returns a list of integers
    encode = lambda s: [stoi[c] for c in s]
  ```

  1. encode
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

##### ==[ Decode ]==

-

  ```python
    # Decoder :takes a list of integers and outputs a 
    # string (list of characters)
    decode = lambda l: "".join([itos[i] for i in l])
  ```

  1. decode
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
    - --[data.shape]-- torch.Size([1115394])
      - 0..5
        - --[18]-- F
          --[47]-- i
          --[56]-- r
          --[57]-- s
          --[58]-- t
    - --[data.dtype]-- torch.int64

##### LANGUAGE MODEL

- **[ train_data ]** 💾
  - ==[ batch_size = 8 ]== 🪺
    - // number of independent sequences processed in parallel
  - ==[ block_size = 8 ]== 🥚
    - // maximum character context length for prediction
      - // given that we can't load the entire data set into memory
    - `8` is a good **default**
      - // training on **shorter sequences** yield
        - more memory and **compute efficient**
        - smaller composable tokens more likely to **generalize**
        - may **miss** on deeper correlation though

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

          -

            ```sh
              -- torch.Size([8]) -- 
              -- [x_0] -- batch => (train_data[:block_size])
              tensor([18, 47, 56, 57, 58,  1, 15, 47])
                 0  1  2  3  4  5  6  7
              0 18 47 56 57 58  1 15 47
              0  F  i  r  s  t     C  i
            ```

      - `x_1`
        - x_1 = train_data[block_size :block_size*2]
        - // next **block_size** 🥚 from **train_data** 💾

          -

            ```sh
              -- [x_1] -- batch => (train_data[block_size :block_size*2])
              tensor([58, 47, 64, 43, 52, 10,  0, 14])
                 0  1  2  3  4  5  6  7
              0 58 47 64 43 52 10  0 14
              0  t  i  z  e  n  : \n  B
            ```

      -

        ```python
          for t in range(block_size):
        ```

### Train

#### | Attention |

##### | Self |

##### | Cross |

#### | Mask |

##### Soft Max

##### Tril

### Inference
