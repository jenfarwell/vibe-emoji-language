#!/bin/bash

# EmojiCoder Demo Script
# Shows how the filtering system works with various types of input

echo "=== EmojiCoder: Visual Pidgin Language Demo ==="
echo

echo "🔸 Test 1: Simple concrete concepts"
echo "Input: 'I love you'"
echo "Output:" 
echo "I love you" | cargo run --quiet
echo

echo "🔸 Test 2: Visual descriptions"
echo "Input: 'The big red car is fast'"
echo "Output:"
echo "The big red car is fast" | cargo run --quiet
echo

echo "🔸 Test 3: Multiple sentences"
echo "Input: 'I am happy. You are sad. The cat sleeps.'"
echo "Output:"
echo "I am happy. You are sad. The cat sleeps." | cargo run --quiet
echo

echo "🔸 Test 4: Abstract concepts (heavily filtered)"
echo "Input: 'We believe that knowledge and wisdom are very important for understanding complex philosophical concepts'"
echo "Output:"
echo "We believe that knowledge and wisdom are very important for understanding complex philosophical concepts" | cargo run --quiet
echo

echo "🔸 Test 5: Mixed concrete/abstract"
echo "Input: 'I see the beautiful tree. It has many green leaves that move in the wind.'"
echo "Output:"
echo "I see the beautiful tree. It has many green leaves that move in the wind." | cargo run --quiet
echo

echo "🔸 Test 6: Actions and emotions"
echo "Input: 'The man runs fast. The woman is tired. The child is excited.'"
echo "Output:"
echo "The man runs fast. The woman is tired. The child is excited." | cargo run --quiet
echo

echo "=== Analysis ==="
echo "Notice how:"
echo "- Abstract words (believe, knowledge, wisdom, that, many, etc.) are filtered out"
echo "- Concrete visual concepts (colors, objects, people, actions, emotions) are preserved"  
echo "- The result is a simplified pidgin focusing on observable, physical reality"
echo "- Grammar is minimal but functional: Subject-Verb-Object with visual modifiers"
