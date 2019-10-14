#!/bin/bash

set -e

AllWords=$(cargo run --bin print_lesson_words)

mkdir -p publish/words
mkdir -p publish/wavs

for w in $AllWords; do
  say -r 90 -v Samantha --data-format=I32@44100 -o "publish/words/$w.wav" $w
done

cargo run --bin write_lessons

for wf in publish/wavs/*.wav; do
  Outfile=$(basename $wf | sed s/\.wav$/.mp3/)
  ffmpeg -i $wf -acodec libmp3lame -q:a 0 "publish/$Outfile"
done

cargo run --bin write_feed

