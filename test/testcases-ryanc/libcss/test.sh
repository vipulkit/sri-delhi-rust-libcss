#!/bin/sh

make
./select-auto > output/select-auto.out
./parse2-auto > output/parse2-auto.out
./parse-auto > output/parse-auto.out
./lex-auto > output/lex-auto.out
./number > output/number.out

./test

./csdetect


# csdetect: error in one testcase
# css21: takes too much time. cannot test
# lex: testcase does not test correctly. But lex-auto is a superset of lex
# parse: parse does not test correctly. parse-auto is a superset

# parse-auto: need to check the code.
