#/usr/bin/env bash

set -ex

# get the current branch
branch_name=$(git symbolic-ref -q HEAD)
branch_name=${branch_name##refs/heads/}
branch_name=${branch_name:-HEAD}

# stash changes
git add .
git stash push -m 'copy_docs'

# fresh gh-pages branch
git branch -D gh-pages
git checkout -b gh-pages

# generate the docs
rm -rf ./docs ./target/doc
cargo doc --no-deps
cp -r target/doc ./docs

# index page aliases to the right location
echo "<meta http-equiv=\"refresh\" content=\"0; url=rusty_trees\">" > target/doc/index.html

git add -f docs
git commit -m '[copy_docs.sh] Generate Documentation'

# restore old state
git checkout "$branch_name"
git stash pop