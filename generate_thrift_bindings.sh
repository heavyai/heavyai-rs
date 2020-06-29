#!/bin/bash

# Run Thrift on an OmniSci source folder, to generate the compiled Thrift bindings.
# This script is intentionally brittle, so that it will get attention if anything changes.

THRIFT_BINARY=thrift
OMNISCI_PATH=$1

# Check for a non-empty input folder
if [ -z "$OMNISCI_PATH" ]; then
  echo 'Expected a path to an OmniSciDB source folder - e.g. generate_thrift_bindings.sh ../omniscidb'
  exit 1
fi

# Check that the Thrift compiler is installed and the version we expect
if ! $THRIFT_BINARY --version | grep -q 'Thrift version 0.13.0'; then
  echo 'Thrift version 0.13.0 not installed'
  exit 1
fi

# Check that the Thrift definitions exist in that folder
if [ ! -f "$OMNISCI_PATH/omnisci.thrift" ]; then
  echo "$OMNISCI_PATH not found"
  exit 1
fi

# Generate the bindings into our src folder
if ! $THRIFT_BINARY -gen rs -recurse -out src "$OMNISCI_PATH/omnisci.thrift"; then
  echo 'Failed to generate Thrift bindings'
  exit 1
fi

# Fix the clippy lines to get rid of the only warnings in the bindings - just an annoyance
fix_clippy () {
  if ! grep -q '#!\[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments, type_complexity))\]' $1; then
    echo "Clippy line not found in $1"
    exit 1
  fi

  if ! perl -0777 -i -pe 's/#!\[cfg_attr\(feature = "cargo-clippy", allow\(too_many_arguments, type_complexity\)\)\]/#![cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments, clippy::type_complexity))]/g' $1; then
    echo "Failed to fix clippy line in $1"
    exit 1
  fi
}

fix_clippy "src/common.rs"
fix_clippy "src/completion_hints.rs"
fix_clippy "src/extension_functions.rs"
fix_clippy "src/omnisci.rs"
fix_clippy "src/serialized_result_set.rs"

# Fix the use declarations in omnisci.rs to have crate:: in front so that they can compile.
# This is patching over open issue https://issues.apache.org/jira/browse/THRIFT-5071
if ! perl -0777 -pi -e 's/use common;\nuse completion_hints;\nuse extension_functions;\nuse serialized_result_set;/use crate::common;\nuse crate::completion_hints;\nuse crate::extension_functions;\nuse crate::serialized_result_set;/gs' src/omnisci.rs; then
  echo "Failed to fix use declarations in omnisci.rs"
  exit 1
fi

# Fix the same in serialized_result_set.rs too
if ! perl -0777 -pi -e 's/use common;/use crate::common;/g' src/serialized_result_set.rs; then
  echo "Failed to fix use declarations in serialized_result_set.rs"
  exit 1
fi

# # Add the commit hash from OmniSciDB for tracking
# if ! sed -i "1i\/\/ Generated from OmniSciDB commit $(cd $OMNISCI_PATH; git log -n 1 --pretty=format:"%H")" src/common.rs src/completion_hints.rs src/extension_functions.rs src/omnisci.rs src/serialized_result_set.rs; then
#   echo "Failed to add commit hash comments"
#   exit 1
# fi

if ! perl -pi -e 'chomp if eof' src/common.rs src/completion_hints.rs src/extension_functions.rs src/omnisci.rs src/serialized_result_set.rs; then
  echo "Failed to chomp newlines"
  exit 1
fi
