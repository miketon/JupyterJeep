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

### | LIBRARY |

#### 🐼 -- pandas -- 🐼

##### 🎬 ==[ DataFrame ]== 🎬

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

##### | BLOCK |

###### 🌎 ==[ DataBlock ]== 🌎

- src/block.py
  - 🔑 Understanding **DataBlock** is key to 🔑
    - moving **BEYOND** fixed applications
    - crafting **NEW** solutions to new PROBLEMS
    - ☑️ @udit-ok : List available Blocks
      - ==[ TransformBlock ]==
        - -- included --
          - 1 - 🔎 **[ ImageBlock ]** 🔎
            - -- image files -- to **grid of pixels**
              - | grid |
                - **tensor**
                  - -- 3d --
                    - width
                    - height
                    - channels(RGBA)
              - | pixels |
                - handles **loading** and **xform** image -> **numeric** formats
                  - -- image --
                    - | single |
                      - **load**
                        - local filesystem
                        - remote server
                      - **xform**
                        - **uniform** basis for comparison
                          - channels
                            - normalize to 0 - 255
                          - resolution
                            - equivalent to table with row/col
                    - | batch |
                      - **generate** augment **batch** -> +generalization
                - | EXAMPLE |

                  - ```python
                      data_block = DataBlock(
                          blocks=(ImageBlock, CategoryBlock), 
                          get_items=get_image_files, 
                          splitter=RandomSplitter(valid_pct=0.2, seed=42),
                          get_y=parent_label,
                          item_tfms=Resize(460),
                          batch_tfms=aug_transforms(size=224, min_scale=0.75) 
                      )
                    ```

                    - (x, y) 🔗
                      - x 🎲 - **ImageBlock** is used to preprocess the independent variables (the images)
                      - y 🎛️ ->
                        - **CategoryBlock** is used to preprocess the dependent variable (labels) for image classification
                        - -💧 loss function 💧-
                          - 'get_y = parent_label'
                    - -- images --
                      - **images loaded** from 'path_to_your_data'
                      - 1 - 👾 ==[ **image upscaled** ]== 👾 -> 460x460
                        - upsized to **reduce lossiness** from batch step (2)
                      - 2 - 🧿 ==[ **batch generate** ]== 🧿
                        - **augment samples** to `improve` model `generalization`
                          - **xform**
                            - `rotate`
                            - `crop`
                            - adjust `hls`
                            - `blur`
                          - **uniform** resized back to 224x224

          - 2 - **[ MaskBlock ]**
            - **segmentation** tasks
              - handles tasks where the target is a **pixel-level** mask
          - 3 - **[ BBoxBlock ]**
            - **object detection** tasks
              - handles tasks where the target is one or more **bounding boxes** in an image
          - 4 - **[ PointBlock ]**
            - **key point** regression
              - handles tasks where the target is one or more **points** in an image's **coordinate system**
          - 5 - 💬 **[ TextBlock ]** 💬
            - handles the **tokenization** and **hashing** of text
              - **tokenization**
                - raw process of splitting text into smaller pieces : tokens
                  - words
                    - 'cats'
                  - sub-words
                    - 'cat'
                    - 's'
                      - plural token
              - **hash**
                - create a **vocabulary** of **tokens**
                  - -- keys --
                    - | index |
                      - alphanumeric characters
                    - | generate |
                      - custom sub_words
                - | EXAMPLE |

                  - ```python
                      data_block = DataBlock(
                          blocks=(TextBlock.from_folder(path), CategoryBlock),
                          get_y = parent_label,
                          get_items=partial(get_text_files, folders=['train', 'test']), 
                          splitter=GrandparentSplitter(valid_name='test')
                      )
                    ```

                    - (x, y) 🔗
                      - x 🎲 - **TextBlock.from_folder(path)** is used to preprocess the independent variables (the text data)
                      - y 🎛️ ->
                        - **CategoryBlock** is used to preprocess the dependent variable (the labels for text classification)
                        - -💧 loss function 💧-
                          - 'get_y = parent_label'
                            - GPT model would use 'get_next_char'
                    - -- text --
                      - **text files are loaded** from the directory specified by 'path'
                      - 🪙 ==[ **tokenized** ]== 🪙
                        - char | subwords | words
                          - @audit Explain trade offs between each ...
                        - 🧇 ==[ **hash** ]== 🧇
                          - a **vocabulary of tokens** is created by assigning a **UID -> each token**
                          - text data is **hashed** to a sequence of these **UID**
                          - this is the **numerical representation** of the text that the model will use

          - 6 - **[ CategoryBlock ]**
            - categorical (**classification**) targets
              - handles converting **categories to codes**
                - @audit - Example of category to code?
          - 7 - **[ MultiCategoryBlock ]**
            - **multiple labels** per item (multi-label classification)
              - handles the **one-hot encoding** needed for such tasks
                - @audit - What is one-hot encoding?
          - 8 - **[ RegressionBlock ]**
            - continuous (**regression**) targets
              - handles tasks where the target is a **single continuous** value
        - -- extend --
          - to generate **custom**
            - @audit - Example of a custom TransformBlock
- `dblock`
  - (x, y) 🔗
    - `x` 🎲
      - independent
        - **[ ImageBlock ]** 🔎
    - `y` 🎛️
      - dependent
        - -💧 loss function 💧-
          - 1 - **[ Category ]**
            - Cross Entropy
          - 2 - **[ MultiCategory ]**
            - Binary Cross Entropy
              - BCE
          - 3 - **[ PointBlock ]**
            - @audit : full name?
              - MSE
    - | EXAMPLE |
      - (default) | SINGLE CATEGORY |
        - `dblock =`
          - DataBlock(`get_x`=get_image_path 💾 , `get_y`=get_labels 🏷️ )
            - @audit : Explain why we MUST inline get_x and get_y functions as opposed to using lambdas?

              - ```python
                  dblock = DataBlock(get_x=lambda r:r['fname'], get_y=lambda r:r['labels'])
                ```

              - ANSWER:
                - During **iterative** training of the **Learner**, it's essential to be able
                to save (write) and load (read) the status of 'fname' and 'labels',
                a process known as **serialization**
                  - However, for serialization to work effectively, the functions passed to **get_x** and **get_y** must have specific, indexable **names**
                  - **Lambda** functions are **anonymous** and **do NOT** have a specific and **indexable name**, which makes them unsuitable for serialization
                  - Therefore to ensure **serialization**, we MUST use properly **named functions** instead of lambda functions
                  - This will allow us to save and load **Learner** states without issues
            - **returns**
              - dblock
                - <fastai.data.block.DataBlock at 0x16a355820>
                  - @audit : explain this binary object?
              - dblock.blocks
                - (fastai.data.block.TransformBlock, fastai.data.block.TransformBlock)
                  - @audit : is this training and validation?
            - dblock.**summary**(`df`)
              - 1 - Collecting Items

                - ```sh
                  Setting-up type transforms pipelines
                  Collecting items from            fname          labels  is_valid 
                                        0     000005.jpg           chair      True
                                        ...          ...             ...       ...
                                        5010  009961.jpg             dog     False
                  ```

              - 2 - Setting Up Pipelines

                - ```sh
                    [5011 rows x 3 columns]
                    Found 5011 items
                    2 datasets of sizes 4009,1002
                    Setting up Pipeline: get_image_path 
                    Setting up Pipeline: get_labels
                  ```

                  - `[5011 rows x 3 columns]`
                    - columns
                      - 1 - 'fname'
                      - 2 - 'labels'
                      - 3 - 'is_valid'
                        - is this part of validation set?
                    - rows
                      - 0 ... 5010
                  - `2 datasets` of sizes `4009,1002`
                    - 1 - training set : `4009`
                    - 2 - validate set : `1002`
                  - Setting up Pipeline: `get_image_path`
                  - Setting up Pipeline: `get_labels`

              - 3 - Building One Sample
                - 💾 Pipeline: `get_image_path`
                  - applying `get_image_path` gives
                    - '/Users/mton/.fastai/data/pascal_2007/train/007443.jpg'
                - 🏷️ Pipeline: `get_labels`
                  - applying `get_labels` gives
                    - [chair, sofa]
                - Final sample: (Path('/Users/mton/.fastai/data/pascal_2007/train/007443.jpg'), ['chair', 'sofa'])
                  - -- log --

                    - ```sh
                        Building one sample
                          Pipeline: get_image_path
                            starting from
                              fname       007443.jpg
                        labels      chair sofa
                        is_valid          True
                        Name: 3752, dtype: object
                            applying get_image_path gives
                              /Users/mton/.fastai/data/pascal_2007/train/007443.jpg
                          Pipeline: get_labels
                            starting from
                              fname       007443.jpg
                        labels      chair sofa
                        is_valid          True
                        Name: 3752, dtype: object
                            applying get_labels gives
                              [chair, sofa]

                        Final sample: (Path('/Users/mton/.fastai/data/pascal_2007/train/007443.jpg'), ['chair', 'sofa']) 
                      ```

              - 4 - -- Item --
                - After Item
                  - specify transformations that should be applied after processing each item
                  - such as **upscaling** to reduce loss from **augmented batch generation**
                - Before Batch
                  - **xform** rotate, crop, hls, blur ... etc
                - After Batch
                  - **uniform** resize back to 224x224
                - ... in **this** case we are ONLY converting **image -> ToTensor**

                  - ```sh
                      [5011 rows x 3 columns]
                      Found 5011 items
                      2 datasets of sizes 4009,1002
                      Setting up Pipeline: get_image_path
                      Setting up Pipeline: get_labels
                      Setting up after_item: Pipeline: ToTensor
                      Setting up before_batch: Pipeline: 
                      Setting up after_batch: Pipeline: 
                    ```

              - 5 - Error Message

                - ```sh
                    Could not do one pass in your dataloader, there is something wrong in it. Please see the stack trace below:
                  ```

                  - @audit : Explain the issue?

###### 🛣️ ==[ Dataset ]== 🛣️

- `dsets`
  - DataSet can handle the preprocessing and transformation of data
    - DataSet should inherit **torch.utils.data.Dataset**
      - abstract class req 2 methods impl
        - `__len__`
          - len(dataset) returns the dataset's size
        - `__getitem__`
          - support indexing so that dataset[i] can get the i-th sample
    - Fastai's **DataBlock API** internally wraps these DETAILS

###### 🗺️ ==[ DataLoader ]== 🗺️

- `dls`
  - DataLoader is used to load the data in a way that's
    efficient and convenient for training models
    - composed of
      - 1 - **Dataset**
      - 2 - **Sampler**
      - 3 - **Iterable** for the given DataSet
    - supports
      - 1 - Automatic Batching
      - 2 - Multi-Thread Data Loading
      - 3 - Customizing Loading Order

  - ```python
      dls = data_block.dataloaders(path)
    ```

    - `dataloaders(path)`
      - **path** is the path to your data
      - **dataloaders** method will
        - 1 - follow the instructions in the **DataBlock**
          - includes
            - (x,y) 🔗
            - item xform
            - batch xform
        - 2 - **load and preprocess** the data
        - 3 - **return a DataLoaders** object
          - DataLoaders obj returned will contain:
            - 1 - **Training** DataLoader
            - 2 - **Validation** DataLoader

##### 🧠 ==[ Learner ]== 🧠

- src/learner.py

###### 👁️‍🗨️ -- vision -- 👁️‍🗨️

- `vision_learner(dls, resnet18)`
  - model
    - defined by
      - 🔗 `(x, y)` 🔗
        - **x** 🎲
          - independent
            - ImageBlock 🔎
        - **y** 🎛️
          - dependent
            - CategoryBlock
      - -💧 loss function 💧-
    - returns
      - `nn.Module`
  - 🗺️ DataLoader 🗺️
    - `dls`
  - optimizer
  - 💧 loss function 💧

### | RESOURCES |

#### 📖 -- text -- 📖

- `train.csv`

#### 👀 -- images -- 👀

- | Folder |
  - `path`
    - = `untar_data`(URLs.PASCAL_2007)
    - 'Path('/Users/mton/.fastai/data/pascal_2007')'
      - PASCAL dataset can have multiple labels per image
  - | File |
    - `000000.jpg`
    - ...

#### 🛜 -- globals -- 🛜

##### {pandas} 🐼

- 🎬 | DataFrame | 🎬
  - 🗓️ `df`
    - pd.read_csv(path/"train.csv")
      - `train.csv`

##### {fastai} 🍟

- | DATA |
  - 🌎 | DataBlock | 🌎
    - `dblock`
      - **BLUEPRINT** to **preprocesss data** for training
        - It doesn't contain the data itself
        - but it contains instructions on how to
          - 💡 🥠 💡
            - 1 - **extract** items from the **input** data
            - 2 - **split** the data into training and validation sets
            - 3 - apply needed **transforms**
              - correlate **label**
              - **augment** to improve **generalization**
              - **uniform resize** for batching
            - 4 - generate **batches** for the model to **consume**
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
              - **x** 🎲 | independent |
                - ImageBlock 🔎
              - **y** 🎛️ | dependent |
                - CategoryBlock
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
            - 🫒 | CPU |
              - `item_tfms = fn`
                - **item xform**
                  - **fn** = Resize(460)
                    - upscale each item to **reduce lossiness** when batch xform are applied
                    - (in this case we are upscaling all images to 460)
            - 🫐 | GPU |
              - `batch_tfms = fn`
                - **batch xform**
                  - **fn** = aug_transforms(size=224, min_scale=0.75)
                    - xform batches to **augment generalization**
                    - (in this case we are augmenting existing training images by crop, rotate, flipping images in the batch)
    - 🛣️ | DataSet | 🛣️
      - `dsets`
        - = dblock.datasets(`df`) 🗓️
          - { splitter }
            - 1 - 'train'
            - 2 - 'validation'
  - 🗺️ | DataLoader | 🗺️
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

- 🧠 | Learner | 🧠
  - `vision_learner`

#### 📐 -- unit tests -- 📐

##### {common}

- | Import |
  - import `os`
    - functions for **interacting** with the operating system
  - from IPython.display
    - import
      - `display`
        - displays objects in Jupyter notebook
      - `Markdown`
        - used to display FORMATTED text
  - from PIL
    - (Python Imaging Library)
    - import `Image`
      - used for **opening**, **xforming** and **saving** image file formats
- | File Path |
  - `source_data_path` = '/Users/mton/.fastai/data/pascal_2007/train'
    - @audit ... this has /train hardcoded, verify that we are also handling valid ok
    - path to where our image data is located (local)
- | Tables |
  - **DataFrame**
    - train_idx, valid_idx = splitter(`df`)
      - list of indices for training and validation sets
      - used **generate** train/validation set **from pre-split df** (dataframe)
    - `train_df` = `df`.iloc[train_idx]
    - `valid_df` = `df`.iloc[train_idx]

##### {display}

- | LABEL |

  - ```python
      idxs = torch.where(dsets[img_id][1]==1.0)[0]
      idx_to_labels = dsets.vocab[idxs]
    ```

    - `idxs` is list of indices **where** dsets[img_id]`[1]`==1.0
      - dsets [img_id] **[...]**
        - [0]
          - image -- object --
            - PILImage mode=RGB size=334x500
        - **[1]**
          - -- label -- index
            - TensorMultiCategory([...])
    - `idx_to_labels`
      - dsets.`vocab`[idxs]

- | IMAGE |
  - 📄 ==[ show_image ]== 📄
    - (fname, label)

      - ```python
          def show_image(fname, label='none'):
            img = Image.open(os.path.join(source_data_path, fname)).resize(256,256) 
            display(img)
            display(Markdown(f'{fname} -- [{label}]'))
        ```

        - **img**
          - -- load --
            - Image.**open**(path_to_image)
              - path_to_image
                - **os**.path.join( ... , ... )
                  - **source_data_path**
                  - fname
          - -- xform --
            - .resize(256, 256)
          - -- show --
            - **display**(img)
            - display(**Markdown**(f'{} -- {}'))
              - fname
              - label

  - 📰 ==[ show_image_from_set ]== 📰
    - (df, dsets, img_id)

      - ```python
          def show_image_from_set(df, dsets, img_id=0):
            assert(img_id < len(dsets)), f'id {img_id} must be less than {len(dsets)}' 
            # multi-label, get index of all labels active for this image id
            idxs = torch.where(dsets[img_id][1]=1.0)[0]
            # vocab takes id int values to return human readable text labels
            idx_to_labels = dsets.vocab[idxs]
            show_image(df.iloc[img_id]['fname'], idx_to_labels)
        ```

        - **show_image**
          - ( img_path, label_string)
            - -- img_path --
              - **df**.iloc [id] [name_col]
                - **id** = img_id
                - **name_col** = 'fname'
                  - | EXAMPLE |
                    - train.csv
                      - [0]
                        - **'fname'**
                      - [1]
                        - 'labels'
                      - [2]
                        - 'is_valid'
            - -- label_string --

  - | EXAMPLE |

    - ```python
        # Training set
        show_image_from(train_df, dsets.train, 12)
      ```

      - ```sh
          000048.jpg -- [['bird', 'person']] --
        ```

### | APPLICATION |

#### -- vars --

##### **const**

- | Dir |
  - `path`
    - 'Path('/Users/mton/.fastai/data/pascal_2007')'
- | DataSet |
  - `train`
  - `valid`
- | DataFrame (Table) |
  - -- row --
    - @audit : example of row index access?
  - -- col --
    - `fname`
      - 'fname'
        - the image file name
    - `labels`
      - 'labels'
        - the list of labels

#### -- functions --

- 💾 ==[ get_image_path(row) ]== 💾

  - ```python
      def get_image_path(row):
        # we actually need the path name to open the image 
        return path/'train'/row['fname']
    ```

- 🏷️ ==[ get_labels(row) ]== 🏷️

  - ```python
      def get_labels(row):
        # we actually need to split the labels on spaces 
        return row['labels'].split(' ')
    ```

- ✂️ ==[ splitter(dataformat) ]== ✂️

  - ```python
    def splitter(dataformat):
        "splits the set into train or valid based on the 'is_valid' column" 
        train = dataformat.index[~dataformat['is_valid']].tolist()
        valid = dataformat.index[dataformat['is_valid']].tolist()
        return train, valid
    ```

    - `~`
      - bitwise `NOT` operator
        - ~True = False
        - ~False = True
    - dataformat.index[**~dataformat**['is_valid']].tolist()
      - collects a list of **training** data
      - is_valid == **False**
    - dataformat.index[**dataformat**['is_valid']].tolist()
      - collects a list of **validation** data
      - is_valid == **True**

#### -- **main** --

##### -- import --

- `pd`
  - import **pandas** as **pd**
- `RandomResizedCrop`
  - from fastai.vision.augment import RandomResizedCrop

##### -- var --

- `df`
  - pd.`read_csv`(path/'train.csv')
    - **load** csv into dataframe **table**
    - df.**head()**
      - **print** head (5 default)

      - ```sh
                  fname        labels  is_valid 
          0  000005.jpg         chair      True
          1  000007.jpg           car      True
          2  000009.jpg  horse person      True
          3  000012.jpg           car     False
          4  000016.jpg       bicycle      True
        ```

- `dblock`
  - `dsets`
    - dblock.**datasets**(`df`)
      - split dataframe into 2 sets :
        - 1 - training
          - dsets.**train[0]**
            - '(Path('/Users/mton/.fastai/data/pascal_2007/train/007045.jpg'), ['person', 'cow'])'
            - (PILImage mode=RGB size=500x375,
              TensorMultiCategory([0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0.,
                        0., 0., 0., 0., 0.]))
            - --[idx_to_labels]--
              ['car'] dsets.train[0] PILImage mode=RGB size=500x375
        - 2 - validation
          - dsets.**valid[0]**
            - '(Path('/Users/mton/.fastai/data/pascal_2007/train/008759.jpg'), ['bicycle'])'
            - (PILImage mode=RGB size=500x375,
              TensorMultiCategory([0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 1.,
                        0., 0., 0., 1., 0.]))
            - --[idx_to_labels]--
              ['car', 'person', 'train'] dsets.valid[0] PILImage mode=RGB size=500x375
      - `.vocab`
        - **vocab** is generated **pre-split** with the ENTIRE dataset
          - by **convention** the vocab for both **.train** and **.valid** SHOULD be the SAME
          - this allows the model to **index** the **same vocabulary** during BOTH **training** and **validation**
        - print(...)

          - ```sh
              # -- [len] --
              20
              # -- [vocab] --
              ['aeroplane', 'bicycle', 'bird', 'boat', 'bottle', 'bus', 'car', 'cat',
              'chair', 'cow', 'diningtable', 'dog', 'horse', 'motorbike', 'person',
              'pottedplant', 'sheep', 'sofa', 'train', 'tvmonitor'] 
            ```

            - train
              - len(dsets.train.vocab)
              - dsets.train.vocab
            - vocab
              - len(desets.valid.vocab)
              - dsets.valid.vocab

    - `idxs`

      - ```python
          # idxs = indexes where the value is 1.0
          idxs = torch.where(dsets.train[0][1]==1.0)[0] # type: ignore
        ```

        - 'TensorMultiCategory([6])'

    - `idx_to_labels`

      - ```python
          idx_to_labels = dsets.train.vocab[idxs]
        ```

        - '(#1) ['car']'

##### -- 🖨️ { ... } 🖨️ --

###### | DATA |

- 🌎 `dblock =`
  - | MULTI CATEGORY |
    - (with split/augment)

    - ```python
        DataBlock{
          blocks = (ImageBlock, MultiCategoryBlock),
          splitter = splitter, # split based on 'is_valid' column
          get_x = get_image_path,
          get_y = get_labels,
          item_tfms = RandomResizedCrop(128, min_scale=0.35)
        }
      ```

- 🗺️ `dls =`

  - ```python
      dls = dblock.dataloaders(df)
    ```

    - -> create a DataLoaders obj
      - 1 - from `df` DataFrame
        - path to directory
        - index list of items
        - ...
      - 2 - according to **blueprint** defined in `dblock`
    - -> **returns** DataLoaders to `dls`
      - **DataLoaders** 2 objs
        - **training** DataLoader
        - **validation** DataLoader
      - each obj are responsible for **fetching batches** of data for their respective **phase**

  - | SHOW |
    - dls.**show_batch**(nrows=1, ncols=3)

###### 🧠 | LEARNER |

- [ **encapsulates** ]->
  - 1 - Model
  - 2 - Dataloader
  - 3 - Loss Function

- -- import --
  - from fastai
    - .vision
      - .all
        - @audit-issue : ❌ : fastai.vision.learner FAILS, why do we have use fastai.vision.all instead?
        - `vision_learner`
        - `cnn_learner`
          - used in fastai book : DEPRECATE for vision_learner ^ instead
      - .models
        - `resnet18`

- `learn`

  - ```python
      learn = vision_learner(dls, resnet18)
    ```
