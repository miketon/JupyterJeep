---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# SOTA

- ->> **Model Engineer** <<-
  - | SPEED |
    - Faster training iterations => **mores** experiments
      - 1 - **cut** down **data** set
        - aim for ~ **minutes** per iteration
      - 2 - **simplify** model
        - ☑️ @udit-ok : example?
          - | ANSWER |
            - reduce # layers
            - reduce # parameters
  - | DIVERSITY |
    - Maximize **utility** of **smaller** data set
      - dataset given is rarely the dataset you want
        - (for your specific task)
    - ImageNet/**Imagenette**
      - **small** subset of ImageNet by fastai
        - categories 10 vs 1000
          - 3 hours of manual curation
- ->> **Product Engineer** <<-
  - | PERFORMANCE |
    - **ImageNet**
      - train on large data set for production
- ->> ==**@ 🔑 key concepts @**== <<- 📶
  - 1 - -- normalization --
    - **critical** to working with **pretrained** models
      - ==[ statistics for normalization ]==
        - get for pretrained model
        - bundle with model you **distribute**
    - a preprocessing step to **standardize input** data
    - improves training effectiveness
      - @audit-ok : explain?
  - 2 - -- data augmentation --
    - **Mixup** - Adds **uncertainty**
      - ==[ helps model learn and handle uncertainty ]==
        - **images** blend fractional => track **mix ratio**
          - [0.3, 0.7] vs [0.0, 1.0]
        - **labels** discrete => add noise **probability**
          - values approach, but never reach 0.0 or 1.0
  - 3 - -- progressive resizing --
    - **gradually** using **larger images** on each iteration
      - **speeds** up training completion when **resolution low**
      - improves model **performance** on **resolution raised**
      - this works because **features** are independent of resolution
        - parameters learned are the same at each res step
          - **early** layers find **edges and gradients**
          - later layers find noses and sunsets
        - effectively **transfer** learning
          - we can use **fine_tune** after resolution progression
      - ==[ performance ceiling truncated at resolution of image saved on disk ]==
    - ==[ stability-plasticity dilemma ]==
      - **catastrophic forgetting**
        - transfer learning are at **risk** if the **base model**
        were **NOT also progressively trained**
        - because we are adding resolution cross attention learning
          - ☑️ @udit-ok : would the progressive resolution need to
          also match the base model to mitigate this risk?
            - | ANSWER |
              - mitigate **forgetting**
                - 1 - Fine-Tuning
                  - **epoch level**
                    - Train ONLY the LAST few layers when increasing resolution
                    - The **base layers** are **frozen** or are updated at a slower rate
                      - this **slows** forgetting of **vocabulary** level features
                      - while allow **learning** of higher level **context** features
                - 2 - Elastic Weight Consolidation
                  - **feature level**
                    - EWC
                      - identifies the most important feature to the task
                      - modulates rate of change to those features (protects)
                - 3 - Learning Rate Scheduling
                  - **hyperparameter level**
                    - smaller learning rate reduces risk of forgetting
                    by ensuring that updates are subtle
                - 4 - Use of Mix Image Sizes
                  - **dataset level**
                    - ensure that the model trains on a variety of resolution in the data set
                    - gives model a chance to maintain ability to see features at scale
  - 4 - -- test time augmentation --
    - @audit : why is this at **inference** time?
