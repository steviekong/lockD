# LockD

LockD is my attempt to build/learn about version control systems from scratch using rust.

I'm using [The Git Parable](https://tom.preston-werner.com/2009/05/19/the-git-parable.htm) as inspiration.

Also using this as an opportunity to learn rust.

My Hope is to build the following features:

- Snapshots
- Tagging
- Branches
- Ignore system
- Merges
- Diffs

## What is done or working?

Currently the following is done in some working state

- [x] `init` which creates a `.repo` directory.
- [x] `commit <message>` which creates creates a snapshot of the current directory with a unique hash as well as a log file containing the hash, author, date and a commit message.

As of now snapshots are full copies of the entire project directory.

The plan is to shift this into some kind of git like structure where file blobs are hashed and stored per commit only if they have been changed.
