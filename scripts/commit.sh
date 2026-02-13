#!/usr/bin/env bash

# MODEL="opencode/minimax-m2.5-free"
MODEL="opencode/gemini-3-flash"

function show_output() {

  echo "## The output of \`${*}\` is."
  echo
  echo \`\`\`
  "$@"
  echo \`\`\`
}

git add .

cat <<PROMPT | opencode --model "${MODEL}" run

Commit with a Conventional commit message. Do not ask for feedback.

$( show_output git status )

$( show_output git diff --staged --stat )

$( show_output git diff --staged  )

$( show_output git log --oneline -5 )

PROMPT


