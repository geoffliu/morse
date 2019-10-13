#!/bin/bash

set -e

AllWords=$(cargo run --bin print_lesson_words)

mkdir -p publish/words
for w in $AllWords; do
  say --data-format=I32@44100 -o "publish/words/$w.wav" $w
done

cargo run --bin write_lessons
