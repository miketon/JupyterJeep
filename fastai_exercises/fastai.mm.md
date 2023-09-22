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
  - | DATA |
    - 🌎 ==[ DataBlock ]== 🌎
      - `dblock`
        - **BLUEPRINT** to **preprocesss data** for training
          - It doesn't contain the data itself
          - but it contains instructions on how to
            1. **split** the data into training and validation sets
            2. how to **label** the data
            3. what types of **transforms** to apply
          - | EXAMPLE |

            - ```python
                from fastai.vision.all import *

                data_block = DataBlock(
                    blocks=(ImageBlock, CategoryBlock), 
                    get_items=get_image_files, 
                    splitter=RandomSplitter(valid_pct=0.2, seed=42),
                    get_y=parent_label,
                    item_tfms=Resize(460),
                    batch_tfms=aug_transforms(size=224, min_scale=0.75)  
                )
              ```

              - `blocks =` 🔗 ==(x, y)== 🔗
                - **x** | independent |
                  - ImageBlock
                - **y** | dependent |
                  - CategoryBlock
                - @audit : List available Blocks
              - `get_items=fn`
                - **fn** = get_image_files
                  - define a function to **get data**
                    - (images in this case)
              - `splitter = fn(...)`
                - **fn(...)** = RandomSplitter(valid_pct=0.2, seed=42)
                  - define a function to **split** training and validation sets
              - `get_y = fn`
                - **fn** = parent_label
                  - get **label** **y** 🔗 for each **x** 🔗
                    - (in this case the parent DIR is the LABEL)
              - `item_tfms = fn`
                - **fn** = Resize(460)
                  - **item xform**
                    - upscale each item to **reduce lossiness** when batch xform are applied
                    - (in this case we are upscaling all images to 460)
              - `batch_tfms = fn`
                - **fn** = aug_transforms(size=224, min_scale=0.75)
                  - **batch xform**
                    - xform batches to **augment generalization**
                    - (in this case we are augmenting existing training images by crop, rotate, flipping images in the batch)
    - 🗺️ ==[ DataLoader ]== 🗺️
      - `dls`
        - DataLoader is used to load the data in a way that's
         efficient and convenient for training models
          - composed of
            1. **Dataset**
            2. **Sampler**
            3. **Iterable** for the given DataSet
          - supports
            1. Automatic Batching
            2. Multi-Thread Data Loading
            3. Customizing Loading Order
        - | EXAMPLE |

          - ```python
              dls = data_block.dataloaders(path)
            ```

            - `dataloaders(path)`
              - **path** is the path to your data
              - **dataloaders** method will
                - 1 - follow the instructions in the **DataBlock**
                  - includes
                    - (x,y)
                    - item xform
                    - batch xform
                - 2 - **load and preprocess** the data
                - 3 - **return a DataLoaders** object
                  - DataLoaders obj returned will contain:
                    - 1 - **Training** DataLoader
                    - 2 - **Validation** DataLoader

    - 🛣️ ==[ DataSet ]== 🛣️
      - `dsets`
        - DataSet can handle the preprocessing and transformation of data
        - DataSet should inherit **torch.utils.data.Dataset**
          - abstract class req 2 methods impl
            - `__len__`
              - len(dataset) returns the dataset's size
            - `__getitem__`
              - support indexing so that dataset[i] can get the i-th sample
        - Fastai's **DataBlock API** internally wraps these DETAILS
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

            - sum existing columns

              - ```python
                  df['Sum'] = df['Column1'] + df['Column2'] + df['Column3']
                ```

                - print(df.head())

                  - ```sh
                          Column1  Column2  Column3  Sum 
                      0        1        2        3    6
                      1        4        5        6   15
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

##### 🌎 | DataBlock | 🌎

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

###### 🗺️ DataLoader 🗺️

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

###### 🛣️ Dataset 🛣️

- `dsets`
  - = dblock.datasets(`df`) 🗓️
    - { splitter }
      - 'train'
      - 'validation'

##### 🧠 | Learner |

- src/learner.py

###### 👁️‍🗨️ -- vision -- 👁️‍🗨️

- `vision_learner(dls, resnet18)`
  - model
    - defined by
      - 🔗 `(x, y)` 🔗
        - **x** 🔗
          - independent
            - ImageBlock
        - **y** 🔗
          - dependent
            - CategoryBlock
      - ->> loss function <<-
    - returns
      - `nn.Module`
  - 🗺️ DataLoader 🗺️
    - `dls`
  - optimizer
  - loss function

### | APPLICATION |
