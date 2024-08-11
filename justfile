_list:
  @just --list --no-aliases

alias fmt := format
format:
  dprint fmt

test:
  ./__test__/test.sh
