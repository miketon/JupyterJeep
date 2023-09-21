---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# ML

## -- CONCEPTS --

### | LOSS FUNCTIONS |

- ->> layers <<-
  - Hidden
    - Sigmoid
      - xform
        - NORMALIZED between 0 and 1
      - output
        - SCALAR
    - Softmax
      - xform
        - probability distribution
          - exponential each value
            - amplifies RELATIVE difference
              - INCREASES picking ONE LEAD category
              - DECREASES probability of TRAILING category
            - ensures ONLY positive values
          - sum all value to 1.0
            - divide by N value
      - output
        - ARRAY
          - @audit : explain
          - [BINARY]
          - [...]
  - Final
    - MNIST
      - xform
        - Categories
          - DISTANCE
      - output
        - Labels
          - BOOL
    - Cross Entropy
      - xform
        - Categories
          - ARRAY
            - Log LikeliHood
      - outpute
        - Labels
          - INDEX
    - Binary Cross Entropy
      - xform
        - Categories
          - ARRAY
      - output
        - Labels
          - INDEX_ARRAY

### | TRAINING |

- init
  - gather data
  - select model architecture
  - select loss function
  - optimize gradient descent
- ->> loop <<-
  - init
    - losses = []
    - epochs = 500
  - loop
    - for i in range(epochs) :
      - set gradients to zero
      - compute predictions
        - forward pass
      - compute loss
      - update parameters
      - store loss to plot
- plot
  - plt.plot(losses)
    - plot losses over time

## -- IMPLEMENT --

### | RESOURCES |

#### 📖 -- text -- 📖

- `train.csv`

#### 👀 -- images -- 👀

- `000000.jpg`

#### 🛜 -- globals -- 🛜

- {pandas} 🐼
  - 🎬 ==[ DataFrame ]== 🎬
    - `df`
- {fastai} 🍟
  - 🎲 ==[ DataBlock ]== 🎲
    - `dblock`
    - `dsets`
    - `dls`
  - 🧠 ==[ Learner ]== 🧠
    - `vision_learner`

### | LIBRARY |

#### 🐼 -- pandas -- 🐼

##### 🎬 | DataFrame |

- 🗓️ `df`
  - pd.read_csv(path/"train.csv")
    - 'train.csv'
      - similar to **SQL** or **spreadsheet**
        - can **source** from a variety of **data inputs**
          - `Dict`
            - | EXAMPLE |

              - ```python
                  data = {
                      'name': ['John', 'Anna', 'Peter', 'Linda'],
                      'age': [28, 24, 35, 32],
                      'city': ['New York', 'Paris', 'Berlin', 'London']
                  }

                  df = pd.DataFrame(data)
                ```

                - print(df)

                  - ```sh
                          name  age       city  
                      0   John   28   New York
                      1   Anna   24      Paris
                      2  Peter   35     Berlin
                      3  Linda   32     London
                    ```

            - ->
              - **Series** (panda)
              - **lists**
              - 1D **ndarrays**

            - ```python
                data = {
                    'Column1': pd.Series([1, 2, 3]),     # Series
                    'Column2': ["one", "two", "three"],  # list
                    'Column3': pd.array([4, 5, 6]),      # 1D ndarray (Pandas Array)
                }

                df = pd.DataFrame(data)
              ```

              - print(df.head())

                - ```sh
                       Column1 Column2  Column3 
                    0        1     one        4
                    1        2     two        5
                    2        3   three        6
                  ```

          - 2D `numpy.ndarray`

            - ```python
                array_2d = np.array([[1, 2, 3], [4, 5, 6]])
                df = pd.DataFrame(array_2d, columns=['Column1', 'Column2', 'Column3'])
              ```

              - print(df.head())

                - ```sh
                       Column1  Column2  Column3 
                    0        1        2        3
                    1        4        5        6
                  ```

          - Another `DataFrame`

            - ```python
                data = {'Column1': [1, 2, 3], 'Column2': [4, 5, 6]} 
                df1 = pd.DataFrame(data)

                # Create a new DataFrame from df1
                df2 = pd.DataFrame(df1)
              ```

              - print(df2.head())

                - ```sh
                       Column1  Column2 
                    0        1        4
                    1        2        5
                    2        3        6
                  ```

      - | TABLE |
        - `.head()`
          - **head()** allows quickly **preview** the first N rows of **DataFrame**
            - table = `df.head()`
              - print(table)
                - default = 5

                - ```sh
                            fname        labels  is_valid  
                    0  000005.jpg         chair      True
                    1  000007.jpg           car      True
                    2  000009.jpg  horse person      True
                    3  000012.jpg           car     False
                    4  000016.jpg       bicycle      True
                  ```

        - `.iloc`
          - **integer-location** based index
          - | ROWS |
            - `df.iloc[0, :]`
              - **POSITION** index
                - `:` is **optional** for row
                  - df.iloc[0]
          - | COLS |
            - `df.iloc[:, 0]`
              - **POSITION** index
                - `df['fname']`
                  - **NAME** index **alt** for column

#### 🍟 -- fastai -- 🍟

##### 🎲 | DataBlock |

- src/block.py
  - 🔑 Understanding **DataBlock** is key to 🔑
    - moving **BEYOND** fixed applications
    - crafting **NEW** solutions to new PROBLEMS
- `dblock`
  - `x`
    - independent
      - 🏁 ==[ ImageBlock ]== 🏁
  - `y`
    - dependent
      - ->> loss function <<-
        - **[ Category ]**
          - Cross Entropy
        - **[ MultiCategory ]**
          - Binary Cross Entropy
            - BCE
        - **[ PointBlock ]**
          - @audit : full name?
            - MSE

###### Dataset

- `dsets`
  - = dblock.datasets(`df`) 🗓️
    - { splitter }
      - 'train'
      - 'validation'

###### DataLoader

- `dls`
  - = dblock.dataloaders(`df`) 🗓️
    - ->> mini-batch <<-
      - collated from Dataset
      - { **item_tfms** }
        - `RandomResizedCrop`
          - must ensure that every item is the same size
  - `show_batch()`
    - @audit : explain nROWS + nCOLS
    - nROWS
    - nCOLS

##### 🧠 | Learner |

- src/learner.py

###### 👁️‍🗨️ -- vision -- 👁️‍🗨️

- `vision_learner(dls, resnet18)`
  - model
    - defined by
      - { var }
        - `x`
          - independent
        - `y`
          - dependent
      - ->> loss function <<-
    - returns
      - `nn.Module`
  - DataLoader
    - `dls`
  - optimizer
  - loss function

### | APPLICATION |
