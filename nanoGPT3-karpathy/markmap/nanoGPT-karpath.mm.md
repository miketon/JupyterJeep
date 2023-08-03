---
markmap:
   colorFreezeLevel: 2
   maxWidth: 500
---

# NanoGPT

## Resources

### Library

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

### Text

#### `wget`

-

  ```sh
    !wget https://raw.githubusercontent.com/karpathy/char-rnn/master/data/tinyshakespeare/input.txt
  ```

  1. smoke test

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

## Model

### Data

#### ==[ Tokenize ]==

##### | mapping |

###### `chars`

- -- character --
  - ⛛ id table ⛛
  - 🔺 vocabulary 🔺
- -- subword --
  - 🔺 id table 🔺⬆️
  - ⛛ vocabulary ⛛

###### `ints`

#### ==[ Encode Decode ]==

#### | Block Size |

### Train

#### | Attention |

##### | Self |

##### | Cross |

#### | Mask |

##### Soft Max

##### Tril

### Inference
