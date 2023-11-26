---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# from

## fastcore

### foundation

- ==L==
  - **supercharged** version of **List**
  - List addendum
    - 1 - Array indexing
      - L can be indexed with 
        - a list of indices
        - a boolean mask
        - another L object
      - | EXAMPLE |

        - ```python
          a = L(range(10))
          a[[1,2,3]] # L([1, 2, 3])
          a[[True]*5 + [False]*5] # L[0, 1, 2, 3, 4]
          ```

    - 2 - Item Assignment
    - 3 - Appending and Concatenating
    - 4 - Mapping and Filtering
    - 5 - Sorting
    - 6 - Attribute access
    - 7 - @audit : more???

## fastai

### data

#### external

URLs

### collab

CollabDataLoaders

### learner

### losses