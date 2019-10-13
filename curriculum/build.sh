#!/bin/bash

set -e
AllWords=$(cargo run --bin print_lesson_words)
for w in $AllWords; do
  say --data-format=I32@44100 -o "publish/$w.wav" $w
done
