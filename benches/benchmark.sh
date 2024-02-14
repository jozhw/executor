#!/bin/bash

# takes a --save-to-history argument and if it exists, then it will save to the history log, otherwise it will just run the bench
# compare previous versions using tags
REPO="jozhw/executor"
REPO_NAME="executor"

CURRENT_TAG="v1.0.1"
CURRENT_VER="1.0.1"

PREVIOUS_TAG="v1.0.0"
PREVIOUS_VER="1.0.0"

# keep historical log of the tests
HISTORY_LOG="benches/benchmark_history.csv"

# download the releases
wget -O benches/current_version.tar.gz "https://github.com/$REPO/archive/$CURRENT_TAG.tar.gz"
wget -O benches/previous_version.tar.gz "https://github.com/$REPO/archive/$PREVIOUS_TAG.tar.gz"

# extract releases
tar -zxvf benches/current_version.tar.gz -C benches
tar -zxvf benches/previous_version.tar.gz -C benches

# compile current version code
cd benches/$REPO_NAME-$CURRENT_VER
cargo build --release

# check if the build was successful
if [ $? -ne 0 ]; then
  echo "Error: Failed to build the current version."
  exit 1
fi

# return back to root dir
cd ../..

# compile previous version code
cd benches/$REPO_NAME-$PREVIOUS_VER
cargo build --release

# check if the build was successful
if [ $? -ne 0 ]; then
  echo "Error: Failed to build the previous version."
  exit 1
fi

# return back to root dir
cd ../..

# change to tests dir so that the tempdir will be in the tests dir
cd tests
# create a temporary directory for delete command benching with 5 level nested
TMP_DIR=$(mktemp -d)
mkdir -p "$TMP_DIR/level1/level2/level3/level4/level5"

# create the file to be deleted
touch "$TMP_DIR/level1/level2/level3/level4/level5/file1.txt"

# return back to root dir
cd ..
# test the three subcommands
SEARCHCOMMAND="search"
SEARCH_ARGS="--regex script.\* --path tests/test_data"

EXECUTECOMMAND="execute"
EXECUTE_ARGS="--fname script.sh --path tests/test_data"

DELETECOMMAND="delete"
DELETE_ARGS="--fname file1.txt --path tests$TMP_DIR"

# define the commands for the current version and previous versions
CURRENT_VERSION="./benches/$REPO_NAME-$CURRENT_VER/target/release/executor $SEARCHCOMMAND $SEARCH_ARGS && ./benches/$REPO_NAME-$CURRENT_VER/target/release/executor $EXECUTECOMMAND $EXECUTE_ARGS && ./benches/$REPO_NAME-$CURRENT_VER/target/release/executor $DELETECOMMAND $DELETE_ARGS"
PREVIOUS_VERSION="./benches/$REPO_NAME-$PREVIOUS_VER/target/release/executor $SEARCHCOMMAND $SEARCH_ARGS && ./benches/$REPO_NAME-$PREVIOUS_VER/target/release/executor $EXECUTECOMMAND $EXECUTE_ARGS && ./benches/$REPO_NAME-$PREVIOUS_VER/target/release/executor $DELETECOMMAND $DELETE_ARGS"

# run benchmarks using Hyperfine and store the results
# use --warmup to cache before the benchmark to standardize
hyperfine --warmup 5 --runs 500 --export-csv benches/benchmark_results.csv "$CURRENT_VERSION" "$PREVIOUS_VERSION"

# check if the user wants to save to the history log
if [ "$1" == "--save-to-history" ]; then
  # append the results to the historical log
  cat benches/benchmark_results.csv >> "$HISTORY_LOG"
fi

# clean up temporary directory and extracted files
rm -rf "$TMP_DIR"

cd benches
rm -rf "$REPO_NAME-$CURRENT_VER" "$REPO_NAME-$PREVIOUS_VER" current_version.tar.gz previous_version.tar.gz 
