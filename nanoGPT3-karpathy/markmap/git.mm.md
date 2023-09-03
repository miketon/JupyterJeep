---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# Git

- .git
  - branches/
  - filter_repo/
    - analysis/
      - git log --graph --full-history --all --pretty=format:"%h%x09%d%x20%s"
      - `git filter-repo --analyze`
        - blob-shas-and-paths.txt
          - list of **file sizes** and their SHA!!
  - hooks/
  - info/
  - lfs/
  - log/
  - objects/
  - refs/
  - ...
    - // heads
      - HEAD
        - `ref: refs/heads/main`
      - ORIG_HEAD
        - `eb249fcefe84c9d73c45bf4f5f2dc91da7e584c3`
          - `removing git-lfs file`
            - @audit ... 2nd to latest head??
      - FETCH_HEAD
        - `87a60c617ce46d84dd1f1923c92d7404244fddb0
        branch 'main' of https://github.com/miketon/JupyterJeep`
    - COMMIT_EDITMSG
      - `git mm : Fixed repro screw up NOTES lol`
        - // last commit message lol
    - config
      - [core]

        - ```sh
            repositoryformatversion = 0 
            filemode = true
            bare = false
            logallrefupdates = true
            ignorecase = true
            precomposeunicode = true
          ```

      - [submodule]
        - `active = .`
      - [remote "origin"]
        - `url = https://github.com/miketon/JupyterJeep.git`
        - `fetch = +refs/heads/*:refs/remotes/origin/*`
      - [branch "main"]
        - `remote = origin`
        - `merge = refs/heads/main`
      - // lfs
        - [lfs]
          - `repositoryformatversion = 0`
        - [lfs "https://github.com/miketon/JupyterJeep.git/info/lfs"]
          - `access = basic`
    - description
      - `Unnamed repository; edit this file 'description' to name the repository.
        - // repro description
    - index
      - `binary ...djksdjl`
        - @audit ... what is this?
    - packed_refs

      - ```sh
          # pack-refs with: peeled fully-peeled sorted 
          5208ffa362d587d32d0d68f4f2c5a3cd0b4a91c6 refs/heads/main
          87a60c617ce46d84dd1f1923c92d7404244fddb0 refs/remotes/origin/main
          d09eb1d4c269b206499710ee5c6f82675f808a09 refs/stash
        ```

        - @audit ... what is this?

## Project

### Repro

#### R:Sauce

- .git/refs/
  - heads/
    - **main**
      - eb249fcefe84c9d73c45bf4f5f2dc91da7e584c3
      - ref: refs/heads/main
        - // @audit : understand this F*CK UP LOL
          - // git symbolic-ref refs/heads/main_2 refs/heads/main\ 2
    - main 2
      - 85fddf26bc6152bfd0b72b93fc6fafcc085dbe00
      - warning: ignoring ref with broken name refs/heads/main 2
        - // deleted this file to fix
        - // likely a SCREW up with git-lfs attempt
          - // @audit : UNDERSTAND WTH happened here lol
  - remotes/
    - origin/
      - **HEAD**
        - ref: refs/remotes/origin/main
  - tags/
    - @audit : Explain this lol

#### R:Commands

- git-filter-repo
  - `git filter-repo --force --invert-paths --path-match nanoGPT3-karpathy/karpathy-notebook-follow-along.ipynb`
    - @audit : Detached my repro ??? ... FIND A LESS DUMBASS WAY TO FIX
      - **DUMBASS FIX** ... by force pushing to REMOTE NOOOO
        - `git remote -v`
          - origin  <https://github.com/miketon/JupyterJeep> (fetch)
          - origin  <https://github.com/miketon/JupyterJeep> (push)
        - `git remote add origin https://github.com/miketon/JupyterJeep`
          - set remote origin
        - `git push origin main -f`
          - force push from LOCAL
          - FUKK anyone who is sharing this REPRO NOOOOO
            - (fortunately just me)


### Branch

#### B:Sauce

##### Remote

- Github
  - | PUSH |
    - **multipl** SOURCE
      - CONFLICT on MERGE
  - | PULL |
    - **single** SOURCE
      - UPDATE to LATEST TRUTH

##### Local

- Device
  - | PUSH |
    - **single** SOURCE
      - RESOLVE history with Remote
  - | PULL |
    - **single** SOURCE
      - RESOLVE history with Local

#### B:Commands

##### B:Compare

- `git diff <first-branch-name>..<second-branch-name>`

##### B:Filter

- git-filter-branch
- allows you to go through your entire repository commit by
commit and make changes to each commit based on the
instructions you give it
  - HARD to UNDO the changes
  - ALWAYS BACKUP repro before doing this

### Commits

- Snapshot of Project
  - Id (Hash)
  - Message
  - Date
  - Files

#### C:Actions

- | View |
  - Log
    - `git log`
  - Checking Out
    - `git checkout [commit_id]`
      - Goes to this commit state
      - Allows to **view** context of this check in
- | Edit |
  - Reverting
    - `git revert [commit_id]`
  - Amending **Recent**
    - `git commit --amend`
      - change commit message
      - add a file
      - @audit ... remove or edit an existing file?
- | Delete |
  - Resetting
    - `git reset --hard [commit_id]`
      - **permanently** remove all commits after id
      - **CAREFUL** as this **DELETES** commits
- Cherry Picking
  - `git cherry-pick [commit_id]`
    - apply changes from a specific commit id to your current working branch
    - CREATES a new commit on CURRENT branch that INCLUDES id you are cherry picking
    - @audit ... explain more please

#### C:Compare

- `git diff <first-commit-id>..<second-commit-id>`

#### Rebase
