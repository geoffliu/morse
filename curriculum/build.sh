#!/bin/bash

set -e
AllWords=$(cargo run --bin print_lesson_words)
for w in $AllWords; do
  espeak -w "publish/$w.wav" $w
done
