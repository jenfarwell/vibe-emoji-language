use std::collections::HashMap;

pub fn get_vocabulary() -> HashMap<&'static str, &'static str> {
    let mut vocab = HashMap::new();
    
    // ESSENTIAL PRONOUNS AND BASIC GRAMMAR (keep minimal set)
    vocab.insert("i", "👤");
    vocab.insert("you", "👤");
    vocab.insert("we", "👥");
    vocab.insert("he", "👨");
    vocab.insert("she", "👩");
    vocab.insert("they", "👥");
    vocab.insert("me", "👤");
    vocab.insert("us", "👥");
    vocab.insert("my", "👤");
    vocab.insert("our", "👥");
    
    // ESSENTIAL VERBS (be, have, basic states)
    vocab.insert("is", "➡️");
    vocab.insert("are", "➡️");
    vocab.insert("am", "➡️");
    vocab.insert("was", "⏪");
    vocab.insert("were", "⏪");
    vocab.insert("have", "✅");
    vocab.insert("has", "✅");
    vocab.insert("had", "✅");
    
    // BASIC SPATIAL/TEMPORAL
    vocab.insert("here", "📍");
    vocab.insert("there", "➡️");
    vocab.insert("now", "⏰");
    vocab.insert("yes", "✅");
    vocab.insert("no", "❌");
    
    // CONCRETE NOUNS - LIVING THINGS
    vocab.insert("cat", "🐈");
    vocab.insert("dog", "🐕");
    vocab.insert("bird", "🐦");
    vocab.insert("fish", "🐟");
    vocab.insert("tree", "🌳");
    vocab.insert("flower", "🌸");
    vocab.insert("animal", "🐾");
    vocab.insert("horse", "🐎");
    vocab.insert("cow", "🐮");
    vocab.insert("pig", "🐖");
    vocab.insert("sheep", "🐑");
    vocab.insert("elephant", "🐘");
    vocab.insert("lion", "🦁");
    vocab.insert("bear", "🐻");
    vocab.insert("rabbit", "🐰");
    vocab.insert("mouse", "🐭");
    vocab.insert("snake", "🐍");
    vocab.insert("frog", "🐸");
    
    // CONCRETE NOUNS - PEOPLE
    vocab.insert("man", "👨");
    vocab.insert("woman", "👩");
    vocab.insert("child", "👶");
    vocab.insert("baby", "👶");
    vocab.insert("friend", "👥");
    vocab.insert("family", "👨‍👩‍👧‍👦");
    
    // CONCRETE NOUNS - OBJECTS
    vocab.insert("house", "🏠");
    vocab.insert("car", "🚗");
    vocab.insert("book", "📖");
    vocab.insert("phone", "📱");
    vocab.insert("computer", "💻");
    vocab.insert("door", "🚪");
    vocab.insert("window", "🪟");
    vocab.insert("table", "📋");
    vocab.insert("chair", "🪑");
    vocab.insert("bed", "🛏️");
    vocab.insert("key", "🔑");
    vocab.insert("money", "💰");
    vocab.insert("bag", "👜");
    vocab.insert("box", "📦");
    vocab.insert("ball", "⚽");
    
    // CONCRETE NOUNS - FOOD
    vocab.insert("food", "🍔");
    vocab.insert("apple", "🍎");
    vocab.insert("banana", "🍌");
    vocab.insert("bread", "🍞");
    vocab.insert("water", "💧");
    vocab.insert("coffee", "☕");
    vocab.insert("tea", "🍵");
    vocab.insert("milk", "🥛");
    vocab.insert("cake", "🎂");
    vocab.insert("pizza", "🍕");
    
    // CONCRETE NOUNS - NATURE
    vocab.insert("sun", "☀️");
    vocab.insert("moon", "🌙");
    vocab.insert("star", "⭐");
    vocab.insert("sky", "☁️");
    vocab.insert("cloud", "☁️");
    vocab.insert("rain", "🌧️");
    vocab.insert("snow", "❄️");
    vocab.insert("wind", "💨");
    vocab.insert("fire", "🔥");
    vocab.insert("water", "💧");
    vocab.insert("mountain", "⛰️");
    vocab.insert("river", "⛲");
    vocab.insert("ocean", "🌊");
    vocab.insert("forest", "🌲");
    vocab.insert("beach", "🏖️");
    vocab.insert("island", "🏝️");
    
    // ACTION VERBS - PHYSICAL ACTIONS
    vocab.insert("walk", "🚶");
    vocab.insert("run", "🏃");
    vocab.insert("jump", "⬆️");
    vocab.insert("sit", "🪑");
    vocab.insert("stand", "🧍");
    vocab.insert("sleep", "😴");
    vocab.insert("eat", "😋");
    vocab.insert("drink", "🥤");
    vocab.insert("cook", "🔥");
    vocab.insert("drive", "🚗");
    vocab.insert("fly", "🕊️");
    vocab.insert("swim", "🏊");
    vocab.insert("dance", "💃");
    vocab.insert("sing", "🎤");
    vocab.insert("read", "👓");
    vocab.insert("write", "✍️");
    vocab.insert("draw", "🎨");
    vocab.insert("build", "🔨");
    vocab.insert("break", "💥");
    vocab.insert("open", "📂");
    vocab.insert("close", "📁");
    vocab.insert("give", "🎁");
    vocab.insert("take", "✋");
    vocab.insert("throw", "🏀");
    vocab.insert("catch", "🤾");
    
    // ACTION VERBS - SENSORY
    vocab.insert("see", "👀");
    vocab.insert("look", "👀");
    vocab.insert("watch", "👀");
    vocab.insert("hear", "👂");
    vocab.insert("listen", "👂");
    vocab.insert("smell", "👃");
    vocab.insert("taste", "👅");
    vocab.insert("touch", "✋");
    vocab.insert("feel", "✋");
    
    // ACTION VERBS - SOCIAL
    vocab.insert("talk", "💬");
    vocab.insert("speak", "🗣️");
    vocab.insert("meet", "🤝");
    vocab.insert("help", "🆘");
    vocab.insert("love", "❤️");
    vocab.insert("like", "👍");
    vocab.insert("hate", "😠");
    vocab.insert("kiss", "💋");
    vocab.insert("hug", "🤗");
    vocab.insert("fight", "⚔️");
    vocab.insert("play", "🎮");
    vocab.insert("work", "💼");
    vocab.insert("learn", "📚");
    vocab.insert("teach", "👨‍🏫");
    
    // VISUAL ADJECTIVES - SIZE
    vocab.insert("big", "🔼");
    vocab.insert("small", "🔽");
    vocab.insert("large", "🔼");
    vocab.insert("little", "🔽");
    vocab.insert("tall", "📏");
    vocab.insert("short", "📐");
    vocab.insert("long", "📏");
    vocab.insert("wide", "⬅️➡️");
    vocab.insert("narrow", "🔽");
    vocab.insert("thick", "⬜");
    vocab.insert("thin", "▫️");
    
    // VISUAL ADJECTIVES - COLORS
    vocab.insert("red", "🔴");
    vocab.insert("blue", "🔵");
    vocab.insert("green", "🟢");
    vocab.insert("yellow", "🟡");
    vocab.insert("orange", "🟠");
    vocab.insert("purple", "🟣");
    vocab.insert("black", "⚫");
    vocab.insert("white", "⚪");
    vocab.insert("pink", "🩷");
    vocab.insert("brown", "🟤");
    
    // VISUAL ADJECTIVES - PHYSICAL PROPERTIES
    vocab.insert("hot", "🔥");
    vocab.insert("cold", "❄️");
    vocab.insert("warm", "🌡️");
    vocab.insert("cool", "🧊");
    vocab.insert("wet", "💧");
    vocab.insert("dry", "🏜️");
    vocab.insert("clean", "🧼");
    vocab.insert("dirty", "🗑️");
    vocab.insert("new", "🆕");
    vocab.insert("old", "👴");
    vocab.insert("young", "👶");
    vocab.insert("fresh", "🌿");
    vocab.insert("heavy", "⚖️");
    vocab.insert("light", "💡");
    vocab.insert("hard", "💎");
    vocab.insert("soft", "☁️");
    vocab.insert("sharp", "🔪");
    vocab.insert("round", "⭕");
    vocab.insert("square", "🟩");
    
    // VISUAL ADJECTIVES - SPEED/MOVEMENT
    vocab.insert("fast", "⚡");
    vocab.insert("slow", "🐌");
    vocab.insert("quick", "⚡");
    vocab.insert("moving", "➡️");
    vocab.insert("still", "⏸️");
    
    // EMOTIONS (highly visual)
    vocab.insert("happy", "😊");
    vocab.insert("sad", "😢");
    vocab.insert("angry", "😠");
    vocab.insert("excited", "🎉");
    vocab.insert("tired", "😴");
    vocab.insert("scared", "😱");
    vocab.insert("surprised", "😲");
    vocab.insert("confused", "🤔");
    vocab.insert("proud", "🏆");
    vocab.insert("embarrassed", "😳");
    vocab.insert("lonely", "😔");
    vocab.insert("bored", "😴");
    vocab.insert("hungry", "🍽️");
    vocab.insert("thirsty", "💧");
    vocab.insert("sick", "🤒");
    vocab.insert("healthy", "💪");
    vocab.insert("strong", "💪");
    vocab.insert("weak", "😩");
    
    // SPATIAL CONCEPTS (essential for pidgin grammar)
    vocab.insert("up", "⬆️");
    vocab.insert("down", "⬇️");
    vocab.insert("left", "⬅️");
    vocab.insert("right", "➡️");
    vocab.insert("in", "📥");
    vocab.insert("out", "📤");
    vocab.insert("on", "⬆️");
    vocab.insert("under", "⬇️");
    vocab.insert("over", "⬆️");
    vocab.insert("near", "👥");
    vocab.insert("far", "🔭");
    vocab.insert("inside", "📥");
    vocab.insert("outside", "📤");
    vocab.insert("front", "➡️");
    vocab.insert("back", "⬅️");
    vocab.insert("top", "⬆️");
    vocab.insert("bottom", "⬇️");
    vocab.insert("middle", "⏺️");
    vocab.insert("around", "🔄");
    vocab.insert("through", "➡️");
    vocab.insert("across", "↔️");
    vocab.insert("between", "⏺️");
    vocab.insert("beside", "👥");
    
    // TEMPORAL (basic time concepts)
    vocab.insert("morning", "🌅");
    vocab.insert("day", "📅");
    vocab.insert("night", "🌙");
    vocab.insert("evening", "🌆");
    vocab.insert("today", "📅");
    vocab.insert("tomorrow", "⏰");
    vocab.insert("yesterday", "⏪");
    vocab.insert("time", "⏰");
    vocab.insert("hour", "🕐");
    vocab.insert("minute", "⏰");
    vocab.insert("week", "📅");
    vocab.insert("month", "📅");
    vocab.insert("year", "📅");
    vocab.insert("before", "⏪");
    vocab.insert("after", "⏩");
    vocab.insert("first", "1️⃣");
    vocab.insert("last", "🔚");
    vocab.insert("next", "➡️");
    vocab.insert("early", "⏰");
    vocab.insert("late", "⏳");
    vocab.insert("soon", "⏰");
    vocab.insert("always", "♾️");
    vocab.insert("never", "🚫");
    vocab.insert("sometimes", "🔄");
    
    // NUMBERS (visual counting)
    vocab.insert("one", "1️⃣");
    vocab.insert("two", "2️⃣");
    vocab.insert("three", "3️⃣");
    vocab.insert("four", "4️⃣");
    vocab.insert("five", "5️⃣");
    vocab.insert("many", "➕");
    vocab.insert("few", "🔽");
    vocab.insert("all", "🌍");
    vocab.insert("some", "🔸");
    vocab.insert("none", "0️⃣");
    vocab.insert("more", "🔼");
    vocab.insert("less", "🔽");
    vocab.insert("most", "🔝");
    vocab.insert("half", "◗");
    vocab.insert("full", "🔵");
    vocab.insert("empty", "⚪");
    
    // SPECIAL CONCEPTS FOR PIDGIN
    vocab.insert("good", "👍");
    vocab.insert("bad", "👎");
    vocab.insert("beautiful", "🌺");
    vocab.insert("ugly", "💀");
    vocab.insert("safe", "🛡️");
    vocab.insert("dangerous", "⚠️");
    vocab.insert("easy", "😌");
    vocab.insert("difficult", "😤");
    vocab.insert("important", "❗");
    vocab.insert("special", "⭐");
    vocab.insert("normal", "😐");
    vocab.insert("different", "🔄");
    vocab.insert("same", "=");
    vocab.insert("true", "✅");
    vocab.insert("false", "❌");
    vocab.insert("real", "🔮");
    vocab.insert("alive", "❤️");
    vocab.insert("dead", "💀");
    vocab.insert("broken", "💥");
    vocab.insert("fixed", "🔧");
    
    // BASIC BODY PARTS (concrete and visible)
    vocab.insert("head", "🧠");
    vocab.insert("face", "😀");
    vocab.insert("eye", "👀");
    vocab.insert("ear", "👂");
    vocab.insert("nose", "👃");
    vocab.insert("mouth", "👄");
    vocab.insert("hand", "✋");
    vocab.insert("finger", "👆");
    vocab.insert("foot", "🦶");
    vocab.insert("leg", "🦵");
    vocab.insert("arm", "💪");
    vocab.insert("heart", "❤️");
    vocab.insert("body", "🧍");
    vocab.insert("hair", "💇");
    vocab.insert("skin", "👋");
    
    // PLACES (concrete locations)
    vocab.insert("home", "🏠");
    vocab.insert("school", "🏫");
    vocab.insert("work", "🏢");
    vocab.insert("store", "🏪");
    vocab.insert("hospital", "🏥");
    vocab.insert("park", "🏞️");
    vocab.insert("city", "🏙️");
    vocab.insert("village", "🏘️");
    vocab.insert("farm", "🚜");
    vocab.insert("garden", "🌻");
    vocab.insert("road", "🛣️");
    vocab.insert("bridge", "🌉");
    
    // WEATHER (visible phenomena)
    vocab.insert("weather", "🌡️");
    vocab.insert("sunny", "☀️");
    vocab.insert("cloudy", "☁️");
    vocab.insert("rainy", "🌧️");
    vocab.insert("snowy", "❄️");
    vocab.insert("windy", "💨");
    vocab.insert("storm", "⛈️");
    vocab.insert("rainbow", "🌈");
    vocab.insert("lightning", "⚡");
    
    // MATERIALS (tangible substances)
    vocab.insert("wood", "🪵");
    vocab.insert("stone", "🪨");
    vocab.insert("metal", "⚙️");
    vocab.insert("glass", "🪟");
    vocab.insert("paper", "📄");
    vocab.insert("plastic", "🥤");
    vocab.insert("cloth", "🧵");
    vocab.insert("sand", "🏖️");
    vocab.insert("dirt", "🌱");
    vocab.insert("mud", "🟤");
    vocab.insert("ice", "🧊");
    vocab.insert("oil", "🛢️");
    
    // ACTIONS WITH OBJECTS
    vocab.insert("cut", "✂️");
    vocab.insert("copy", "📋");
    vocab.insert("paste", "📋");
    vocab.insert("pick", "👆");
    vocab.insert("drop", "⬇️");
    vocab.insert("push", "👐");
    vocab.insert("pull", "🤏");
    vocab.insert("lift", "⬆️");
    vocab.insert("carry", "📦");
    vocab.insert("hold", "✋");
    vocab.insert("release", "👐");
    vocab.insert("squeeze", "🤏");
    vocab.insert("stretch", "↔️");
    vocab.insert("bend", "🔄");
    vocab.insert("fold", "📁");
    vocab.insert("tear", "💥");
    vocab.insert("fix", "🔧");
    vocab.insert("make", "🔨");
    vocab.insert("create", "✨");
    vocab.insert("destroy", "💥");
    vocab.insert("clean", "🧽");
    vocab.insert("wash", "🧼");
    vocab.insert("dry", "🔥");
    
    // Special additions for natural language flow
    vocab.insert("sleep", "😴"); // Make sure sleep is caught
    vocab.insert("sleeps", "😴");
    vocab.insert("moves", "➡️");
    vocab.insert("move", "➡️");
    vocab.insert("leaves", "🍃"); // Could be noun or verb - context matters
    vocab.insert("runs", "🏃");
    vocab.insert("wind", "💨");
    vocab.insert("has", "✅");
    vocab.insert("tired", "😴");
    vocab.insert("excited", "🎉");
    
    // Add missing common patterns
    vocab.insert("beautiful", "🌺");
    vocab.insert("green", "🟢");
    vocab.insert("fast", "⚡");
    vocab.insert("man", "👨");
    vocab.insert("woman", "👩");
    vocab.insert("child", "👶");

    vocab
}
