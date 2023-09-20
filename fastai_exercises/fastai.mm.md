---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# ML

## | RESOURCES |

### 📖 -- text -- 📖

- `train.csv`

### 👀 -- images -- 👀

- `000000.jpg`

### 🛜 -- globals -- 🛜

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

## | LIBRARY |

### 🐼 -- pandas -- 🐼

#### 🎬 | DataFrame |

- `df`
  - pd.read_csv(path/"train.csv")
    - 'train.csv'

### 🍟 -- fastai -- 🍟

#### 🧱 | DataBlock |

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

##### Dataset

- `dsets`
  - = dblock.datasets(`df`)
    - { splitter }
      - 'train'
      - 'validation'

##### DataLoader

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

#### 🧠 | Learner |

- src/learner.py

##### 👁️‍🗨️ -- vision -- 👁️‍🗨️

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