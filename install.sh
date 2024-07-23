#! /usr/bin/bash 
cargo b -r
cp target/release/project .
mv project searchfor
sudo cp searchfor ~/../../bin/ 

