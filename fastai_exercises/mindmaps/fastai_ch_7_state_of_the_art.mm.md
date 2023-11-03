---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# SOTA

- ->> **model engineer** <<-
  - | SPEED |
    - ~**minutes** training iteration == **mores** experiments!
      - 1 - **cut** down **data** set
      - 2 - **simplify** model
        - @audit-ok : example?
  - | DIVERSITY |
    - Get the **most** out of a **smaller** data set
      - dataset given is rarely the dataset you want
    - ImageNet/**Imagenette**
      - **small** subset of ImageNet
        - 3 hours of curation by fastai
          - categories 10 vs 1000
- ->> **product engineer** <<-
  - | PERFORMANCE |
    - **ImageNet**
      - train on large data set for production
- ->> ==**@ 🔑 key concepts @**== <<- 📶
  - 1 - -- normalization --
  - 2 - -- data augmentation --
    - **Mixup** - Adds **uncertainty** to images and label
      - blend images so model can **learn uncertainty**
        - [0.3, 0.7] vs [0.0, 1.0]
      - labels => shift from one-hot encoding to high threshold
        - @audit-ok : word this better ^
  - 3 - -- progressive resizing --
  - 4 - -- test time augmentation --