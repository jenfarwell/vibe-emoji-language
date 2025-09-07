use std::collections::HashMap;

pub fn get_vocabulary() -> HashMap<&'static str, &'static str> {
    let mut vocab = HashMap::new();
    // Nouns
    vocab.insert("i", "👤");
    vocab.insert("you", "👤");
    vocab.insert("cat", "🐈");
    vocab.insert("dog", "🐕");
    vocab.insert("fish", "🐟");
    vocab.insert("sun", "☀️");
    vocab.insert("moon", "🌙");
    vocab.insert("book", "📖");
    vocab.insert("tree", "🌳");
    vocab.insert("car", "🚗");
    vocab.insert("house", "🏠");
    vocab.insert("computer", "💻");
    vocab.insert("food", "🍔");
    vocab.insert("world", "🌍");
    // More nouns: animals
    vocab.insert("bird", "🐦");
    vocab.insert("horse", "🐎");
    vocab.insert("cow", "🐮");
    vocab.insert("pig", "🐖");
    vocab.insert("sheep", "🐑");
    vocab.insert("elephant", "🐘");
    vocab.insert("lion", "🦁");
    vocab.insert("tiger", "🐅");
    vocab.insert("bear", "🐻");
    vocab.insert("wolf", "🐺");
    vocab.insert("fox", "🦊");
    vocab.insert("deer", "🦌");
    vocab.insert("rabbit", "🐰");
    vocab.insert("mouse", "🐭");
    vocab.insert("rat", "🐀");
    vocab.insert("snake", "🐍");
    vocab.insert("frog", "🐸");
    vocab.insert("turtle", "🐢");
    vocab.insert("shark", "🦈");
    vocab.insert("whale", "🐋");
    vocab.insert("dolphin", "🐬");
    // More nouns: fruits and foods
    vocab.insert("apple", "🍎");
    vocab.insert("banana", "🍌");
    vocab.insert("orange", "🍊");
    vocab.insert("grape", "🍇");
    vocab.insert("strawberry", "🍓");
    vocab.insert("watermelon", "🍉");
    vocab.insert("pizza", "🍕");
    vocab.insert("burger", "🍔");
    vocab.insert("sushi", "🍣");
    vocab.insert("icecream", "🍦");
    vocab.insert("cake", "🎂");
    vocab.insert("bread", "🍞");
    // More nouns: vehicles and transport
    vocab.insert("bike", "🚲");
    vocab.insert("bus", "🚌");
    vocab.insert("train", "🚂");
    vocab.insert("plane", "✈️");
    vocab.insert("ship", "🚢");
    vocab.insert("boat", "⛵");
    // More nouns: places
    vocab.insert("city", "🏙️");
    vocab.insert("mountain", "⛰️");
    vocab.insert("river", "⛲");
    vocab.insert("ocean", "🌊");
    vocab.insert("forest", "🌲");
    vocab.insert("school", "🏫");
    vocab.insert("hospital", "🏥");
    vocab.insert("store", "🏪");
    vocab.insert("park", "🏞️");
    // More nouns: objects
    vocab.insert("phone", "📱");
    vocab.insert("table", "📋");
    vocab.insert("chair", "🪑");
    vocab.insert("door", "🚪");
    vocab.insert("window", "🪟");
    vocab.insert("pen", "🖊️");
    vocab.insert("paper", "📄");
    vocab.insert("clock", "🕐");
    vocab.insert("lamp", "💡");
    // More nouns: people and professions
    vocab.insert("man", "👨");
    vocab.insert("woman", "👩");
    vocab.insert("child", "👶");
    vocab.insert("doctor", "👨‍⚕️");
    vocab.insert("teacher", "👩‍🏫");
    vocab.insert("engineer", "👨‍💻");
    vocab.insert("artist", "👨‍🎨");
    vocab.insert("chef", "👨‍🍳");
    // Verbs
    vocab.insert("am", "➡️"); // is/are/am -> be
    vocab.insert("is", "➡️");
    vocab.insert("are", "➡️");
    vocab.insert("eat", "😋");
    vocab.insert("eats", "😋");
    vocab.insert("see", "👀");
    vocab.insert("sees", "👀");
    vocab.insert("love", "❤️");
    vocab.insert("loves", "❤️");
    vocab.insert("run", "🏃");
    vocab.insert("runs", "🏃");
    vocab.insert("go", "🚶");
    vocab.insert("goes", "🚶");
    vocab.insert("read", "👓");
    vocab.insert("reads", "👓");
    vocab.insert("write", "✍️");
    vocab.insert("writes", "✍️");
    vocab.insert("think", "🤔");
    vocab.insert("thinks", "🤔");
    // More verbs: actions
    vocab.insert("jump", "⬆️");
    vocab.insert("jumps", "⬆️");
    vocab.insert("sleep", "😴");
    vocab.insert("sleeps", "😴");
    vocab.insert("wake", "🌅");
    vocab.insert("wakes", "🌅");
    vocab.insert("sing", "🎤");
    vocab.insert("sings", "🎤");
    vocab.insert("dance", "💃");
    vocab.insert("dances", "💃");
    vocab.insert("swim", "🏊");
    vocab.insert("swims", "🏊");
    vocab.insert("fly", "🕊️");
    vocab.insert("flies", "🕊️");
    vocab.insert("drive", "🚗");
    vocab.insert("drives", "🚗");
    vocab.insert("build", "🔨");
    vocab.insert("builds", "🔨");
    vocab.insert("destroy", "💥");
    vocab.insert("destroys", "💥");
    vocab.insert("learn", "📚");
    vocab.insert("learns", "📚");
    vocab.insert("teach", "👨‍🏫");
    vocab.insert("teaches", "👨‍🏫");
    vocab.insert("work", "💼");
    vocab.insert("works", "💼");
    vocab.insert("play", "🎮");
    vocab.insert("plays", "🎮");
    vocab.insert("laugh", "😂");
    vocab.insert("laughs", "😂");
    vocab.insert("cry", "😢");
    vocab.insert("cries", "😢");
    vocab.insert("fight", "⚔️");
    vocab.insert("fights", "⚔️");
    vocab.insert("help", "🆘");
    vocab.insert("helps", "🆘");
    // Adjectives
    vocab.insert("big", "🔼");
    vocab.insert("small", "🔽");
    vocab.insert("happy", "😊");
    vocab.insert("sad", "😢");
    vocab.insert("red", "🔴");
    vocab.insert("blue", "🔵");
    vocab.insert("bright", "🔆");
    vocab.insert("dark", "🔅");
    // More adjectives
    vocab.insert("tall", "📏");
    vocab.insert("short", "📐");
    vocab.insert("hot", "🔥");
    vocab.insert("cold", "❄️");
    vocab.insert("fast", "⚡");
    vocab.insert("slow", "🐌");
    vocab.insert("beautiful", "🌺");
    vocab.insert("ugly", "💀");
    vocab.insert("new", "🆕");
    vocab.insert("old", "👴");
    vocab.insert("good", "👍");
    vocab.insert("bad", "👎");
    vocab.insert("smart", "🧠");
    vocab.insert("dumb", "🤦");
    vocab.insert("strong", "💪");
    vocab.insert("weak", "😩");
    vocab.insert("young", "👶");
    vocab.insert("rich", "💰");
    vocab.insert("poor", "🪙");
    // Adverbs
    vocab.insert("quickly", "💨");
    vocab.insert("slowly", "🐢");
    vocab.insert("very", "🔥");
    // More adverbs
    vocab.insert("happily", "😊");
    vocab.insert("sadly", "😢");
    vocab.insert("loudly", "🔊");
    vocab.insert("quietly", "🤫");
    vocab.insert("carefully", "🔍");
    vocab.insert("carelessly", "😵");
    vocab.insert("often", "🔄");
    vocab.insert("rarely", "⏸️");
    vocab.insert("always", "♾️");
    vocab.insert("never", "🚫");
    // Prepositions
    vocab.insert("to", "➡️");
    vocab.insert("from", "⬅️");
    vocab.insert("with", "🤝");
    vocab.insert("on", "⬆️");
    vocab.insert("in", "➡️📥");
    vocab.insert("at", "📍");
    // More prepositions
    vocab.insert("under", "⬇️");
    vocab.insert("over", "⬆️");
    vocab.insert("between", "⏺️");
    vocab.insert("beside", "👥");
    vocab.insert("through", "➡️");
    vocab.insert("around", "🔄");
    vocab.insert("before", "⏪");
    vocab.insert("after", "⏩");
    // Tense words
    vocab.insert("will", "⏩"); // Future
    vocab.insert("did", "⏪"); // Past
    vocab.insert("was", "⏪"); // Past
    vocab.insert("were", "⏪"); // Past
    // More tense/modifiers
    vocab.insert("have", "✅");
    vocab.insert("has", "✅");
    vocab.insert("had", "✅");
    vocab.insert("can", "💪");
    vocab.insert("could", "🤔");
    vocab.insert("should", "❓");
    vocab.insert("must", "⚠️");
    // Articles and conjunctions
    vocab.insert("the", "📌");
    vocab.insert("a", "🔸");
    vocab.insert("an", "🔸");
    vocab.insert("and", "➕");
    vocab.insert("or", "❓");
    vocab.insert("but", "➖");
    vocab.insert("if", "❓");
    vocab.insert("when", "⏰");
    // Additional words for better coverage in test sentence
    vocab.insert("hello", "👋");
    vocab.insert("darkness", "🌑");
    vocab.insert("my", "👤");
    vocab.insert("friend", "👥");
    vocab.insert("ive", "👤✅");
    vocab.insert("come", "🚶");
    vocab.insert("meet", "🤝");
    vocab.insert("again", "🔄");

    // Additional words for the new test string and massively expanded lexicon
    // Pronouns and possessives
    vocab.insert("me", "👤");
    vocab.insert("it", "🔸");
    vocab.insert("we", "👥");
    vocab.insert("us", "👥");
    vocab.insert("he", "👨");
    vocab.insert("she", "👩");
    vocab.insert("they", "👥");
    vocab.insert("our", "👥");

    // Contractions and common phrases (cleaned versions)
    vocab.insert("well", "⛲");
    vocab.insert("ive", "👤✅");
    vocab.insert("someday", "⏰");
    vocab.insert("find", "🔍");
    vocab.insert("rainbow", "🌈");
    vocab.insert("connection", "🔗");
    vocab.insert("lovers", "💑");
    vocab.insert("dreamers", "💭👥");

    // More verbs for actions and states
    vocab.insert("come", "👋");
    vocab.insert("meet", "🤝");
    vocab.insert("talk", "💬");
    vocab.insert("walk", "🚶");
    vocab.insert("listen", "👂");
    vocab.insert("know", "🧠");
    vocab.insert("remember", "💾");
    vocab.insert("forget", "😵");
    vocab.insert("hope", "🙏");
    vocab.insert("wish", "⭐");
    vocab.insert("dream", "💤");
    vocab.insert("imagine", "🧠");
    vocab.insert("create", "✨");
    vocab.insert("discover", "🔭");
    vocab.insert("search", "🔍");
    vocab.insert("look", "👀");
    vocab.insert("wait", "⏳");
    vocab.insert("try", "💪");
    vocab.insert("fail", "❌");
    vocab.insert("succeed", "✅");
    vocab.insert("change", "🔄");
    vocab.insert("grow", "🌱");
    vocab.insert("live", "🏠");
    vocab.insert("die", "💀");
    vocab.insert("begin", "▶️");
    vocab.insert("end", "⏹️");
    vocab.insert("continue", "➡️");
    vocab.insert("stop", "⏸️");
    vocab.insert("start", "🚀");
    vocab.insert("finish", "🏁");

    // More nouns: abstract concepts
    vocab.insert("time", "⏰");
    vocab.insert("day", "📅");
    vocab.insert("night", "🌙");
    vocab.insert("life", "❤️");
    vocab.insert("death", "⚰️");
    vocab.insert("love", "❤️");
    vocab.insert("hate", "😠");
    vocab.insert("joy", "😊");
    vocab.insert("pain", "😖");
    vocab.insert("peace", "☮️");
    vocab.insert("war", "⚔️");
    vocab.insert("freedom", "🕊️");
    vocab.insert("hope", "🌈");
    vocab.insert("fear", "😱");
    vocab.insert("dream", "💭");
    vocab.insert("memory", "🧠");
    vocab.insert("future", "🔮");
    vocab.insert("past", "⏪");
    vocab.insert("present", "🎁");
    vocab.insert("idea", "💡");
    vocab.insert("thought", "🤔");
    vocab.insert("feeling", "❤️");
    vocab.insert("emotion", "😀");
    vocab.insert("music", "🎵");
    vocab.insert("song", "🎤");
    vocab.insert("story", "📖");
    vocab.insert("journey", "🗺️");
    vocab.insert("path", "🛤️");
    vocab.insert("road", "🛣️");
    vocab.insert("bridge", "🌉");
    vocab.insert("key", "🔑");
    vocab.insert("secret", "🔒");
    vocab.insert("truth", "✅");
    vocab.insert("lie", "🤥");

    // More nouns: nature and weather
    vocab.insert("sky", "☁️");
    vocab.insert("cloud", "☁️");
    vocab.insert("rain", "🌧️");
    vocab.insert("snow", "❄️");
    vocab.insert("wind", "💨");
    vocab.insert("storm", "⛈️");
    vocab.insert("lightning", "⚡");
    vocab.insert("thunder", "🌩️");
    vocab.insert("flower", "🌸");
    vocab.insert("grass", "🌿");
    vocab.insert("leaf", "🍃");
    vocab.insert("root", "🌱");
    vocab.insert("branch", "🌿");
    vocab.insert("star", "⭐");
    vocab.insert("planet", "🪐");
    vocab.insert("space", "🌌");

    // More adjectives and adverbs
    vocab.insert("true", "✅");
    vocab.insert("false", "❌");
    vocab.insert("real", "🔮");
    vocab.insert("fake", "🤥");
    vocab.insert("long", "📏");
    vocab.insert("high", "⬆️");
    vocab.insert("low", "⬇️");
    vocab.insert("deep", "⬇️");
    vocab.insert("shallow", "⬆️");
    vocab.insert("wide", "⬅️➡️");
    vocab.insert("narrow", "🔽");
    vocab.insert("heavy", "⚖️");
    vocab.insert("light", "💡");
    vocab.insert("hard", "💎");
    vocab.insert("soft", "☁️");
    vocab.insert("wet", "💧");
    vocab.insert("dry", "🏜️");
    vocab.insert("clean", "🧼");
    vocab.insert("dirty", "🗑️");
    vocab.insert("safe", "🛡️");
    vocab.insert("dangerous", "⚠️");
    vocab.insert("easy", "😌");
    vocab.insert("difficult", "😤");
    vocab.insert("possible", "✅");
    vocab.insert("impossible", "🚫");

    // More prepositions and conjunctions
    vocab.insert("for", "🎁");
    vocab.insert("by", "✋");
    vocab.insert("about", "💭");
    vocab.insert("like", "👍");
    vocab.insert("as", "➡️");
    vocab.insert("than", "➡️");
    vocab.insert("so", "🔄");
    vocab.insert("because", "➡️");
    vocab.insert("while", "⏰");
    vocab.insert("during", "⏳");
    vocab.insert("until", "⏸️");
    vocab.insert("since", "⏪");
    vocab.insert("though", "🤔");

    // Numbers and quantifiers
    vocab.insert("one", "1️⃣");
    vocab.insert("two", "2️⃣");
    vocab.insert("three", "3️⃣");
    vocab.insert("many", "➕");
    vocab.insert("few", "🔽");
    vocab.insert("all", "🌍");
    vocab.insert("some", "🔸");
    vocab.insert("none", "0️⃣");
    vocab.insert("every", "♾️");

    // More common words for better coverage
    vocab.insert("just", "⚖️");
    vocab.insert("only", "🔒");
    vocab.insert("even", "⚖️");
    vocab.insert("still", "⏸️");
    vocab.insert("also", "➕");
    vocab.insert("too", "➕");
    vocab.insert("very", "🔥");
    vocab.insert("really", "🔥");
    vocab.insert("much", "💰");
    vocab.insert("more", "🔼");
    vocab.insert("less", "🔽");
    vocab.insert("most", "🔝");
    vocab.insert("least", "🔚");

    // Technology and modern concepts
    vocab.insert("code", "💻");
    vocab.insert("program", "💻");
    vocab.insert("software", "💻");
    vocab.insert("internet", "🌐");
    vocab.insert("website", "🌐");
    vocab.insert("email", "📧");
    vocab.insert("message", "💬");
    vocab.insert("chat", "💬");
    vocab.insert("app", "📱");
    vocab.insert("game", "🎮");
    vocab.insert("video", "📹");
    vocab.insert("movie", "🎬");
    vocab.insert("tv", "📺");
    vocab.insert("news", "📰");
    vocab.insert("social", "👥");
    vocab.insert("media", "📱");
    vocab.insert("post", "📌");
    vocab.insert("share", "📤");
    vocab.insert("like", "👍");
    vocab.insert("comment", "💬");
    vocab.insert("follow", "👥");
    vocab.insert("friend", "👥");

    // Food and drink expansion
    vocab.insert("coffee", "☕");
    vocab.insert("tea", "🍵");
    vocab.insert("water", "💧");
    vocab.insert("milk", "🥛");
    vocab.insert("juice", "🧃");
    vocab.insert("beer", "🍺");
    vocab.insert("wine", "🍷");
    vocab.insert("cook", "🔥");
    vocab.insert("eat", "😋");
    vocab.insert("drink", "🥤");
    vocab.insert("hungry", "🍽️");
    vocab.insert("thirsty", "💧");

    // Emotions and states
    vocab.insert("angry", "😠");
    vocab.insert("excited", "🎉");
    vocab.insert("tired", "😴");
    vocab.insert("bored", "😴");
    vocab.insert("surprised", "😲");
    vocab.insert("confused", "🤔");
    vocab.insert("proud", "🏆");
    vocab.insert("embarrassed", "😳");
    vocab.insert("grateful", "🙏");
    vocab.insert("lonely", "😔");

    // Places and locations
    vocab.insert("home", "🏠");
    vocab.insert("work", "🏢");
    vocab.insert("office", "🏢");
    vocab.insert("restaurant", "🍽️");
    vocab.insert("cafe", "☕");
    vocab.insert("bar", "🍺");
    vocab.insert("beach", "🏖️");
    vocab.insert("island", "🏝️");
    vocab.insert("desert", "🏜️");
    vocab.insert("valley", "🌄");
    vocab.insert("hill", "⛰️");
    vocab.insert("lake", "🏞️");
    vocab.insert("pool", "🏊");

    // Actions and activities
    vocab.insert("exercise", "🏃");
    vocab.insert("run", "🏃");
    vocab.insert("walk", "🚶");
    vocab.insert("hike", "🥾");
    vocab.insert("bike", "🚲");
    vocab.insert("swim", "🏊");
    vocab.insert("yoga", "🧘");
    vocab.insert("meditate", "🧘");
    vocab.insert("read", "📖");
    vocab.insert("write", "✍️");
    vocab.insert("draw", "🎨");
    vocab.insert("paint", "🎨");
    vocab.insert("sing", "🎤");
    vocab.insert("dance", "💃");
    vocab.insert("play", "🎮");
    vocab.insert("watch", "👀");
    vocab.insert("listen", "👂");
    vocab.insert("study", "📚");
    vocab.insert("learn", "📚");
    vocab.insert("teach", "👨‍🏫");

    // Time and events
    vocab.insert("morning", "🌅");
    vocab.insert("afternoon", "🌤️");
    vocab.insert("evening", "🌆");
    vocab.insert("week", "📅");
    vocab.insert("month", "📅");
    vocab.insert("year", "📅");
    vocab.insert("birthday", "🎂");
    vocab.insert("holiday", "🎉");
    vocab.insert("party", "🎉");
    vocab.insert("meeting", "💼");
    vocab.insert("event", "📅");
    vocab.insert("travel", "✈️");
    vocab.insert("vacation", "🏖️");

    // Money and business
    vocab.insert("money", "💰");
    vocab.insert("bank", "🏦");
    vocab.insert("buy", "🛒");
    vocab.insert("sell", "💸");
    vocab.insert("pay", "💳");
    vocab.insert("save", "💾");
    vocab.insert("spend", "💸");
    vocab.insert("job", "💼");
    vocab.insert("career", "💼");
    vocab.insert("business", "🏢");
    vocab.insert("company", "🏢");
    vocab.insert("team", "👥");

    // Health and body
    vocab.insert("health", "🏥");
    vocab.insert("doctor", "👨‍⚕️");
    vocab.insert("medicine", "💊");
    vocab.insert("hospital", "🏥");
    vocab.insert("body", "🧍");
    vocab.insert("head", "🧠");
    vocab.insert("hand", "✋");
    vocab.insert("foot", "🦶");
    vocab.insert("heart", "❤️");
    vocab.insert("eye", "👀");
    vocab.insert("ear", "👂");
    vocab.insert("mouth", "👄");
    vocab.insert("sick", "🤒");
    vocab.insert("healthy", "💪");

    // Education and knowledge
    vocab.insert("school", "🏫");
    vocab.insert("university", "🎓");
    vocab.insert("student", "👩‍🎓");
    vocab.insert("teacher", "👨‍🏫");
    vocab.insert("book", "📖");
    vocab.insert("class", "📚");
    vocab.insert("exam", "📝");
    vocab.insert("grade", "📊");
    vocab.insert("knowledge", "🧠");
    vocab.insert("science", "🔬");
    vocab.insert("math", "➗");
    vocab.insert("history", "📜");
    vocab.insert("art", "🎨");

    // Environment and ecology
    vocab.insert("earth", "🌍");
    vocab.insert("nature", "🌳");
    vocab.insert("tree", "🌳");
    vocab.insert("forest", "🌲");
    vocab.insert("animal", "🐾");
    vocab.insert("plant", "🌱");
    vocab.insert("water", "💧");
    vocab.insert("fire", "🔥");
    vocab.insert("air", "💨");
    vocab.insert("earth", "🌍");
    vocab.insert("recycle", "♻️");
    vocab.insert("pollution", "☢️");
    vocab.insert("climate", "🌡️");
    vocab.insert("global", "🌍");
    vocab.insert("warming", "🔥");

    // Sports and games
    vocab.insert("sport", "⚽");
    vocab.insert("football", "⚽");
    vocab.insert("basketball", "🏀");
    vocab.insert("tennis", "🎾");
    vocab.insert("swimming", "🏊");
    vocab.insert("running", "🏃");
    vocab.insert("team", "👥");
    vocab.insert("win", "🏆");
    vocab.insert("lose", "😞");
    vocab.insert("score", "📊");
    vocab.insert("game", "🎮");
    vocab.insert("chess", "♟️");
    vocab.insert("poker", "🃏");

    // Transportation expansion
    vocab.insert("car", "🚗");
    vocab.insert("truck", "🚚");
    vocab.insert("motorcycle", "🏍️");
    vocab.insert("taxi", "🚕");
    vocab.insert("subway", "🚇");
    vocab.insert("road", "🛣️");
    vocab.insert("street", "🛣️");
    vocab.insert("highway", "🛣️");
    vocab.insert("traffic", "🚦");
    vocab.insert("parking", "🚗");

    // Communication
    vocab.insert("call", "📞");
    vocab.insert("text", "💬");
    vocab.insert("speak", "🗣️");
    vocab.insert("language", "🌐");
    vocab.insert("english", "🇺🇸");
    vocab.insert("word", "📝");
    vocab.insert("sentence", "📝");
    vocab.insert("question", "❓");
    vocab.insert("answer", "❗");

    // Miscellaneous common words
    vocab.insert("yes", "✅");
    vocab.insert("no", "❌");
    vocab.insert("maybe", "🤔");
    vocab.insert("now", "⏰");
    vocab.insert("later", "⏳");
    vocab.insert("today", "📅");
    vocab.insert("tomorrow", "⏰");
    vocab.insert("yesterday", "⏪");
    vocab.insert("here", "📍");
    vocab.insert("there", "➡️");
    vocab.insert("this", "🔸");
    vocab.insert("that", "🔸");
    vocab.insert("who", "👤");
    vocab.insert("what", "❓");
    vocab.insert("where", "📍");
    vocab.insert("when", "⏰");
    vocab.insert("why", "❓");
    vocab.insert("how", "❓");
    vocab.insert("which", "❓");

    vocab
}