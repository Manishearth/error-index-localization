#!/bin/bash
if [ -z "$1" ]
  then
    echo "No argument supplied, please provide path to rust clone";
    exit 1;
fi

set -e
echo >gen_json/modules.rs
echo "{" >gen_json/register.rs
rm ./gen_json/auto/*
for file in `find $1 -name "error_codes.rs"`; do
name=$(basename $(dirname $file))
if grep -q "register_long_diagnostics" $file; then
    cp $file gen_json/auto/$name.rs
    sed -i "s/^use syntax::.*//" gen_json/auto/$name.rs
    echo "#[path=\"auto/$name.rs\"] mod $name;" >>gen_json/modules.rs
    echo "list.push((\"$name\", crate::$name::map()));" >>gen_json/register.rs
fi
done
echo "}" >>gen_json/register.rs
cd gen_json
cargo run
cd ..
