#!/bin/bash
function set_alias {
alias $(basename $PWD)='./target/debug/$(basename $PWD)'
echo $(basename $PWD) aliased to ./target/debug/$(basename $PWD)
}
