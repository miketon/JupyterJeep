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
  - 🧱 ==[ DataBlock ]== 🧱
    - `dblock`
    - `dsets`
    - `dls`
  - 🧠 ==[ Learner ]== 🧠
    - `vision_learner`

### | LIBRARY |

#### 🐼 -- pandas -- 🐼

##### 🎬 | DataFrame |

- `df`
  - pd.read_csv(path/"train.csv")
    - 'train.csv'

#### 🍟 -- fastai -- 🍟

##### 🧱 | DataBlock |

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
  - = dblock.datasets(`df`)
    - { splitter }
      - 'train'
      - 'validation'

###### DataLoader

- `dls`
  - = dblock.dataloaders(`df`)
    - ->> mini-batch <<-
      - collated from Dataset
      - { **item_tfms** }
        - `RandomResizedCrop`
          - must ensure that every item is the same size
  - `show_batch()`
    - @audit : explain nrows + ncols
    - nrows
    - ncols

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