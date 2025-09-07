# EmojiCoder: A Visual Pidgin Language

EmojiCoder transforms English text into a simplified emoji-based pidgin language using computer science and linguistic heuristics to filter out abstract concepts that don't translate well to visual symbols.

## Philosophy

This pidgin language is designed around the principle that **only concrete, visualizable concepts should be represented**. Abstract words, complex grammar, and linguistic constructs that don't have clear visual metaphors are filtered out, leaving a core vocabulary of:

- **Concrete nouns** (objects, animals, people)
- **Physical actions** (verbs that can be seen/performed)
- **Visual properties** (colors, sizes, emotions)
- **Essential grammar** (pronouns, basic verbs like "is/are")
- **Spatial/temporal concepts** (directions, basic time)

## Filtering Heuristics

### What Gets Included ✅
- **Physical objects**: 🐈 (cat), 🏠 (house), 🚗 (car)
- **Actions you can see**: 🏃 (run), 😋 (eat), 💃 (dance)
- **Visual properties**: 🔴 (red), 🔼 (big), 😊 (happy)
- **Essential pronouns**: 👤 (I/you/me), 👥 (we/they)
- **Basic existence**: ➡️ (is/are), ✅ (have/has)
- **Spatial relations**: ⬆️ (on/up), ⬇️ (under/down), 📍 (here)

### What Gets Filtered Out ❌
- **Abstract concepts**: "belief", "knowledge", "connection"
- **Complex grammar**: "the", "and", "but", "because", "very"
- **Modal verbs**: "would", "could", "should", "might"
- **Adverbs ending in -ly**: "quickly", "carefully"
- **Relative clauses**: "that", "which", "who" (when used as relatives)
- **Prepositions without spatial meaning**: "of", "for", "by"

## Grammar Rules

### Sentence Structure
The pidgin follows a simplified Subject-Verb-Object structure with visual modifiers:

```
👤 ❤️ 👤     (I love you)
🔼 🔴 🚗     (big red car)
🐈 😴 ⬆️ 📋  (cat sleeps on table)
```

### Sentence Separation
Multiple sentences are separated by `•`:
```
👤 ➡️ 😊 • 👤 ➡️ 😢   (I am happy. You are sad.)
```

### Word Order
- **Adjectives before nouns**: 🔼 🔴 🚗 (big red car)
- **Simple verb phrases**: 👤 ❤️ 👤 (I love you)
- **Spatial relations**: 🐈 ⬆️ 📋 (cat on table)

## Core Vocabulary Categories

### People & Pronouns
- 👤 = I, you, me (singular person)
- 👥 = we, they, us (multiple people)
- 👨 = he, man
- 👩 = she, woman
- 👶 = child, baby, young

### Essential Verbs
- ➡️ = is, are, am (existence/state)
- ⏪ = was, were (past state)
- ✅ = have, has, had (possession/completion)
- ❤️ = love
- 👀 = see, look, watch
- 😋 = eat

### Physical Actions
- 🚶 = walk, go
- 🏃 = run
- 😴 = sleep
- 🤝 = meet, help
- ✋ = take, hold
- 💬 = talk, speak

### Concrete Objects
- 🏠 = house, home
- 🚗 = car
- 🐈 = cat
- 🌳 = tree
- 📖 = book
- 💻 = computer

### Colors & Properties
- 🔴 = red
- 🔵 = blue
- 🟢 = green
- 🔼 = big, large
- 🔽 = small, little
- ⚡ = fast, quick
- 🐌 = slow

### Emotions
- 😊 = happy
- 😢 = sad
- 😠 = angry
- 😱 = scared, afraid
- 🎉 = excited
- 😴 = tired, bored

### Spatial & Temporal
- ⬆️ = up, on, over
- ⬇️ = down, under
- ➡️ = right, to, there
- ⬅️ = left, from, back
- 📍 = here, at
- ⏰ = now, time
- 📅 = today, day

## Example Translations

### Simple Sentences
```
English: "I am happy"
Emoji: 👤 ➡️ 😊

English: "The big red car"
Emoji: 🔼 🔴 🚗

English: "I love you"
Emoji: 👤 ❤️ 👤
```

### Complex Input (Filtered)
```
English: "So we've been told and some choose to believe it, I know they're wrong, wait and see"
Emoji: 👤 👀 • 👤

Filtered out: "so", "been", "told", "and", "some", "choose", "believe", "it", "know", "wrong", "wait", "see"
Kept: "I" (👤), "they" → filtered as abstract, "me" → becomes second 👤
```

## Technical Implementation

### Word Categorization Algorithm
1. **Essential Check**: Is it a core pronoun or basic verb?
2. **Abstract Filter**: Is it in the list of abstract/grammatical words?
3. **Concrete Check**: Is it a physical object, visible action, or visual property?
4. **Morphological Analysis**: Does the word ending suggest it should be filtered?
   - `-ly` adverbs → filtered
   - `-ing/-ed` verbs → may be kept if visual
   - Unknown words → filtered by default

### Composition Fallback
If a word isn't in the vocabulary, the system attempts to split it into recognizable parts:
```
"sunlight" → "sun" (☀️) + "light" (💡) = ☀️💡
```

### Safety Philosophy
**When in doubt, filter out.** The pidgin prioritizes clarity and visual communication over comprehensive translation. It's better to have a short, clear emoji sentence than a long string full of unclear symbols.

## Use Cases

- **Language learning aid**: Focus on concrete vocabulary
- **Visual communication**: Cross-cultural basic communication
- **Accessibility**: Simple visual representation of concepts
- **Creative writing**: Constrained language for artistic expression
- **Meditation on language**: What concepts are truly essential?

## Future Improvements

- **Contextual filtering**: Same word, different meanings based on context
- **Emoji combinations**: More sophisticated compound concepts
- **Grammar particles**: Minimal grammatical markers for clarity
- **Cultural emoji variants**: Region-specific emoji mappings
- **Negation system**: Simple way to express "not"

## Philosophy Notes

This pidgin demonstrates that human communication can be stripped down to a surprisingly small set of concrete, visual concepts while still maintaining meaningful expression. It forces both speaker and listener to focus on the essential, observable aspects of experience rather than abstract reasoning or complex grammatical relationships.

The filtering process reveals how much of our everyday language consists of linguistic "scaffolding" that, while useful for nuanced expression, may not be necessary for basic communication of concrete ideas and physical experiences.
