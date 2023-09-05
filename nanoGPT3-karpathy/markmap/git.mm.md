---
markmap:
   colorFreezeLevel: 2
   maxWidth: 0
---

# Git

## -- **.git** --

- DO NOT EDIT OR DELETE FILES IN THIS DIRECTORY
  - What are some common signs of repository corruption that might
  require manual intervention?
    - Inability to clone, push, or pull
    - Inability to check out or switch branches
    - Error messages
      - "bad object"
      - "invalid object"
      - "checksum failure"
      - "reference broken"
    - Missing commits or branches
    - Strange behavior of Git commands
      - 'git log' might not show the entire commit history
      - 'git status' might show incorrect information
  - rare ==[ EXCEPTIONS ]==
    - **Recovering** a deleted branch
    - Manually editing **hooks**
    - Cleaning up a repository with **git-filter-repo**
      - **Purging** from ENTIRE commit history
        - **sensitive data** NEEDS to be **scrubbed**
        - **large files** accidentally checked in
    - Repairing a **corrupted repository**
      - This could involve operations such as
        - removing damaged objects
        - manipulating
          - HEAD
          - index
          - refs
    - -- Suggestions --
      - Back up your repository
      - Identify the problem
        - `git fsck --full` to check the integrity of your Git repository
        - checks the connectivity and validity of the objects in the database
      - Resolve dangling blobs, commits, and trees
        - If git fsck reports "dangling" blobs, commits, or trees
          - These are objects that aren't referenced anywhere in your repository
          - They can be cleaned up with the `git gc --prune=now` command
            - which removes unreachable objects
            - Dangles are not necessarily a sign of corruption,
            but removing them reduces the PROBLEM SURFACE AREA
      - Repair missing or broken objects
        - If 'git fsck' reports missing or broken
        objects, you have a more serious problem
        - You might be able to recover these objects
        if they exist in another clone of the repository
          1. Identify the SHA1 hash of the missing objects.
          2. Copy the missing objects to the corrupted repository
          3. Remove bad objects
          4. Repair corrupted packfiles

### -- .g/folders --

- Most **important** part of a **Git repository**
  - It's what gets **copied** when you **clone a repository** from **remote**
    - Hidden directory in a Git repository that contains
    all the information necessary for version control
    - This is the directory where Git stores all the
    **metadata** and **object database** for your **project**
- ==| core |==
  - hooks/
    - **pre-commit** and **pre-receive** hooks
      - contains **client** or **server-side** scripts that you can
      use to **trigger actions** at certain points in Git's execution
  - info/
    - contains the **global excludes file** for the **repository**
  - log/
    - contains record of how the **tip of branches** have changed
      - information is used by commands like **git reflog** to help
      you **recover** lost commits
  - objects/
    - (compressed and stored by their SHA1 hash)
      - directory **stores all** the **content** for your
      repository, including **files and directories**
      - Deleting or modifying objects can lead to loss of
        - commits
        - branches
        - tags
        - file history
  - refs/
    - contains **pointers to commit objects**
      - @audit - Explain what that means
- | optional |
  - branches/
    - -- DEPRECATED --
      - is no longer used for new repositories
      - used by early versions of Git to hold references to heads of branches
      - @audit : verify that this is TRUE!
  - lfs/
    - **Git LFS** (Large File Storage) to store **pointers to large files** that are not stored directly in the repository
  - filter_repo/
    - analysis/
      - git log --graph --full-history --all --pretty=format:"%h%x09%d%x20%s"
      - `git filter-repo --analyze`
        - blob-shas-and-paths.txt
          - list of **file sizes** and their SHA!!

### -- .g/files --

- -- head --
  - HEAD
    - `ref: refs/heads/main`
      - reference to the last commit in the currently check-out branch
  - ORIG_HEAD
    - **backup reference** to the state of **HEAD** at the time of the **last operation** that moved it
    - `eb249fcefe84c9d73c45bf4f5f2dc91da7e584c3`
      - `removing git-lfs file`
  - FETCH_HEAD
    - **reference to the tip of the branch** fetched during the **last git fetch** or git **pull**
    - `87a60c617ce46d84dd1f1923c92d7404244fddb0
    branch 'main' of https://github.com/miketon/JupyterJeep`
- -- repros --
  - description
    - `Unnamed repository; edit this file 'description' to name the repository.
      - repro description
        - file is only used by the **GitWeb** program, so you can **safely ignore** it
  - config
    - stores the **configuration for the Git repository**, including the repository's
      - remotes,
      - branches,
      - and other settings
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
- -- commits --
  - COMMIT_EDITMSG
    - `git mm : Fixed repro screw up NOTES lol`
      - // last commit message lol
- index
  - `djksdjl...`
    - 🆗 @udit-ok 🆗 : Explain this
    - ANSWER: ☑️
      - This file is used by Git to keep track of files and directories
      - It's a binary containing
        - sorted list of path names, each with permissions
        - the SHA1 of a blob object
      - Represents the staging area
        - Git uses this file to efficiently determine what
        has changed in your working directory
        - If you delete or modify this file, you might
          - lose staged changes
          - disrupt Git's ability to track changes
- packed_refs

  - ```sh
      # pack-refs with: peeled fully-peeled sorted 
      5208ffa362d587d32d0d68f4f2c5a3cd0b4a91c6 refs/heads/main
      87a60c617ce46d84dd1f1923c92d7404244fddb0 refs/remotes/origin/main
      d09eb1d4c269b206499710ee5c6f82675f808a09 refs/stash
    ```

    - 🆗 @udit-ok 🆗 : What is this?
    - ANSWER: ☑️
      - file contains references to objects in **packfiles**
      - **packfiles** are a space-efficient storage format for objects

## Project

### Repro

#### R:Sauce

- .git/refs/
  - heads/
    - // This is a file that contains a reference to the last commit (the HEAD) of the branch
    - **main**
      - eb249fcefe84c9d73c45bf4f5f2dc91da7e584c3
      - ref: refs/heads/main
    - main 2
      - 85fddf26bc6152bfd0b72b93fc6fafcc085dbe00
      - warning: ignoring ref with broken name refs/heads/main 2
        - 🆗 @udit-ok 🆗 : **deleted** this file to fix
          - ANSWER: ☑️
            - Git doesn't handle branch names with spaces well unless they're properly escaped or quoted
            - // Sloppy git lfs attempt created this throwaway branch LOL
  - remotes/
    - origin/
      - **HEAD**
        - ref: refs/remotes/origin/main
  - tags/
    - 🆗 @udit-ok 🆗 : Explain what are tags
      - ANSWER: ☑️
        - a tag is a pointer or reference to a specific commit
        - typically used to capture a point in history that is used for a marked version release
          - v1.0, v2.0 ... etc
        - **Types** of Tags
          - **Lightweight** tags are just a pointer to a specific commit : bookmark
          - **Annotated** tags are stored as full objects in the Git database

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
- format-patch
  - `git format-patch origin/master`
    - When repro is CORRUPT : **reseting from REMOTE** and
    patching changes in **abandoned LOCAL**

      - ```sh
          mv *.patch /path/to/fresh/repo 
          cd /path/to/fresh/repo
          git apply *.patch
        ```

        - This will create a series of patch files in the current directory
        - one for each commit that is in your current branch but not in origin/master

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
