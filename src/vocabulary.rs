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
    vocab.insert("yes", "✅");
    vocab.insert("no", "❌");
    vocab.insert("maybe", "🤔");
    vocab.insert("now", "⏰📍"); // time at this point
    vocab.insert("later", "⏰➡️"); // time moving forward
    vocab.insert("soon", "⏰🔜"); // time approaching quickly
    vocab.insert("today", "📅📍"); // this day specifically
    vocab.insert("tomorrow", "📅➡️"); // next day
    vocab.insert("yesterday", "📅⬅️"); // previous day
    vocab.insert("here", "📍👤"); // at speaker's location
    vocab.insert("there", "📍➡️"); // at distant location
    vocab.insert("everywhere", "📍🌍"); // all locations
    vocab.insert("nowhere", "🚫📍"); // no location
    vocab.insert("somewhere", "📍❓"); // unknown location
    vocab.insert("this", "👤📍🔸"); // near to speaker
    vocab.insert("that", "➡️📍🔸"); // distant from speaker
    vocab.insert("these", "👤📍🔸🔸"); // multiple near speaker
    vocab.insert("those", "➡️📍🔸🔸"); // multiple distant from speaker
    vocab.insert("who", "👤❓"); // person question
    vocab.insert("what", "🔸❓"); // thing question
    vocab.insert("where", "📍❓"); // location question
    vocab.insert("when", "⏰❓"); // time question
    vocab.insert("why", "💭❓"); // reason question
    vocab.insert("how", "🔧❓"); // method question
    vocab.insert("which", "🫵❓"); // selection question
    vocab.insert("whose", "👤🔗❓"); // possession question
    vocab.insert("whom", "👤➡️❓"); // object person question
    
    // Relationship indicators for better context
    vocab.insert("owns", "👤🔗🏠"); // person possesses house
    vocab.insert("belongs", "🔗➡️👤"); // belongs to person
    vocab.insert("contains", "📦📥🔸"); // box contains thing
    vocab.insert("includes", "📦➕🔸"); // box includes thing
    vocab.insert("excludes", "📦➖🔸"); // box excludes thing
    vocab.insert("connects", "🔸🔗🔸"); // thing links to thing
    vocab.insert("separates", "🔸✂️🔸"); // thing cuts from thing
    vocab.insert("leads", "👤🧭➡️👤"); // person guides to person
    vocab.insert("follows", "👤👣➡️👤"); // person tracks to person
    vocab.insert("controls", "👤🎮➡️🔸"); // person commands thing
    vocab.insert("depends", "🔸🔗⬅️🔸"); // thing relies on thing
    vocab.insert("affects", "🔸➡️💫🔸"); // thing influences thing
    vocab.insert("causes", "🔸➡️💥🔸"); // thing creates thing
    vocab.insert("prevents", "🔸🛡️🔸"); // thing blocks thing
    vocab.insert("enables", "🔸🔓🔸"); // thing allows thing
    vocab.insert("requires", "🔸⚠️🔸"); // thing needs thing
    vocab.insert("provides", "🔸📤🔸"); // thing supplies thing
    vocab.insert("creates", "👤✨➡️🔸"); // person makes thing
    vocab.insert("destroys", "👤💥➡️🔸"); // person breaks thing
    vocab.insert("builds", "👤�➡️🔸"); // person constructs thing
    vocab.insert("repairs", "👤🔧➡️🔸"); // person fixes thingvocab.insert("cow", "🐮");
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
    // Verbs - ENHANCED WITH DIRECTIONAL AGENCY
    vocab.insert("am", "👤="); // identity/state of being
    vocab.insert("is", "="); // identity/state of being  
    vocab.insert("are", "👥="); // plural identity/state
    vocab.insert("eat", "�➡️👄"); // person to mouth action
    vocab.insert("eats", "�➡️👄");
    vocab.insert("see", "�️➡️"); // eye directed outward
    vocab.insert("sees", "�️➡️");
    vocab.insert("love", "👤❤️➡️"); // person sending love
    vocab.insert("loves", "👤❤️➡️");
    vocab.insert("run", "👤🏃➡️"); // person running direction
    vocab.insert("runs", "👤🏃➡️");
    vocab.insert("go", "👤➡️"); // person moving direction
    vocab.insert("goes", "👤➡️");
    vocab.insert("come", "👤⬅️"); // person moving toward speaker
    vocab.insert("comes", "�⬅️");
    vocab.insert("give", "👤➡️🎁👤"); // person giving to person
    vocab.insert("gives", "👤➡️🎁�");
    vocab.insert("take", "👤⬅️✋"); // person taking toward self
    vocab.insert("takes", "👤⬅️✋");
    vocab.insert("get", "👤⬅️🫴"); // person acquiring toward self
    vocab.insert("gets", "�⬅️🫴");
    vocab.insert("bring", "👤🫴➡️"); // person carrying toward
    vocab.insert("brings", "👤🫴➡️");
    vocab.insert("send", "👤📤➡️"); // person dispatching outward
    vocab.insert("sends", "�📤➡️");
    vocab.insert("receive", "👤📥⬅️"); // person getting inward
    vocab.insert("receives", "👤📥⬅️");
    vocab.insert("tell", "👤🗣️➡️👤"); // person speaking to person
    vocab.insert("tells", "👤🗣️➡️👤");
    vocab.insert("ask", "👤❓➡️👤"); // person questioning to person
    vocab.insert("asks", "👤❓➡️👤");
    vocab.insert("show", "👤👉➡️👤"); // person indicating to person
    vocab.insert("shows", "👤👉➡️👤");
    vocab.insert("teach", "👤📚➡️👤"); // person educating to person
    vocab.insert("teaches", "👤📚➡️👤");
    vocab.insert("learn", "👤⬅️📚"); // person acquiring knowledge
    vocab.insert("learns", "👤⬅️📚");
    vocab.insert("help", "👤🆘➡️👤"); // person assisting to person
    vocab.insert("helps", "👤🆘➡️👤");
    vocab.insert("hurt", "👤💥➡️👤"); // person harming to person
    vocab.insert("hurts", "👤💥➡️👤");
    vocab.insert("heal", "👤🩹➡️👤"); // person healing to person
    vocab.insert("heals", "👤�➡️👤");
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
    // Prepositions - ENHANCED DIRECTIONAL AND RELATIONAL CLARITY
    vocab.insert("to", "➡️");
    vocab.insert("from", "⬅️");
    vocab.insert("with", "🤝"); // togetherness/accompaniment
    vocab.insert("without", "🚫🤝"); // lack of accompaniment
    vocab.insert("on", "⬆️📍"); // on top of/surface contact
    vocab.insert("in", "📥"); // inside containment
    vocab.insert("at", "📍"); // location/position
    vocab.insert("of", "🔗"); // belonging/possession/origin relationship
    vocab.insert("for", "🎯"); // purpose/benefit/intended recipient
    vocab.insert("by", "👤➡️"); // agency/method/proximity
    vocab.insert("about", "🔄💭"); // concerning/regarding
    vocab.insert("like", "≈"); // similarity/comparison
    vocab.insert("as", "="); // equality/function
    vocab.insert("than", "⚖️➡️"); // comparison indicator
    vocab.insert("into", "➡️📥"); // movement toward inside
    vocab.insert("onto", "➡️⬆️"); // movement toward surface
    vocab.insert("out", "📤"); // movement from inside
    vocab.insert("off", "⬇️🚫"); // movement from surface
    vocab.insert("up", "⬆️");
    vocab.insert("down", "⬇️");
    vocab.insert("above", "⬆️📍"); // higher position
    vocab.insert("below", "⬇️📍"); // lower position
    vocab.insert("behind", "🔙📍"); // rear position
    vocab.insert("ahead", "⏩📍"); // front position
    vocab.insert("against", "�➡️"); // opposition/contact
    vocab.insert("toward", "➡️🎯"); // direction of movement
    vocab.insert("away", "⬅️🏃"); // movement from
    vocab.insert("across", "➡️🌉"); // traversing
    vocab.insert("along", "➡️〰️"); // following a path
    vocab.insert("among", "📍👥"); // within a group
    vocab.insert("between", "👤⏺️�"); // separating two things
    vocab.insert("beside", "👥📍"); // next to
    vocab.insert("beyond", "➡️🔭"); // past/further than
    vocab.insert("within", "📥⏰"); // inside bounds (space/time)
    vocab.insert("throughout", "🔄📍"); // all through
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
    // Pronouns and possessives - CLEARER OWNERSHIP SEMANTICS
    vocab.insert("me", "👤");
    vocab.insert("it", "🔸");
    vocab.insert("we", "👥");
    vocab.insert("us", "👥");
    vocab.insert("he", "👨");
    vocab.insert("she", "👩");
    vocab.insert("they", "👥");
    vocab.insert("our", "👥🔗"); // group possession indicator
    vocab.insert("my", "👤🔗"); // personal possession indicator  
    vocab.insert("your", "👤➡️🔗"); // directed possession (yours)
    vocab.insert("his", "👨🔗"); // male possession
    vocab.insert("her", "👩🔗"); // female possession  
    vocab.insert("hers", "👩🔗");
    vocab.insert("theirs", "👥🔗"); // group possession
    vocab.insert("its", "🔸🔗"); // object possession
    vocab.insert("mine", "👤🔗✋"); // emphatic personal possession
    vocab.insert("yours", "👤➡️🔗✋"); // emphatic directed possession

    // Contractions and common phrases (cleaned versions)
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

    // Essential words for better coverage
    vocab.insert("yes", "✅");
    vocab.insert("no", "❌");
    vocab.insert("maybe", "🤔");
    vocab.insert("now", "⏰");
    vocab.insert("later", "⏳");
    vocab.insert("today", "📅");
    vocab.insert("tomorrow", "📅");
    vocab.insert("yesterday", "📅");
    vocab.insert("here", "📍");
    vocab.insert("there", "👉");
    vocab.insert("this", "�");
    vocab.insert("that", "�");
    vocab.insert("who", "👤");
    vocab.insert("what", "❓");
    vocab.insert("where", "📍");
    vocab.insert("when", "⏰");
    vocab.insert("why", "❓");
    vocab.insert("how", "❓");
    vocab.insert("which", "🤷");
    
    // Direction and movement words
    vocab.insert("up", "⬆️");
    vocab.insert("down", "⬇️");
    vocab.insert("left", "⬅️");
    vocab.insert("right", "➡️");
    vocab.insert("back", "⬅️");
    vocab.insert("forward", "➡️");
    vocab.insert("inside", "📥");
    vocab.insert("outside", "📤");
    vocab.insert("near", "👫");
    vocab.insert("far", "🔭");
    vocab.insert("close", "🤏");
    vocab.insert("open", "🔓");
    
    // Basic actions missing from current vocab
    vocab.insert("get", "🫴");
    vocab.insert("give", "🫱");
    vocab.insert("take", "✋");
    vocab.insert("bring", "🫴");
    vocab.insert("carry", "🧳");
    vocab.insert("hold", "✊");
    vocab.insert("touch", "👋");
    vocab.insert("feel", "✋");
    vocab.insert("push", "👐");
    vocab.insert("pull", "🫷");
    vocab.insert("lift", "🏋️");
    vocab.insert("drop", "💧");
    vocab.insert("throw", "🏀");
    vocab.insert("catch", "🫳");
    vocab.insert("break", "💔");
    vocab.insert("fix", "🔧");
    vocab.insert("clean", "🧹");
    vocab.insert("wash", "🧼");
    vocab.insert("cut", "✂️");
    vocab.insert("join", "🔗");
    vocab.insert("separate", "✂️");
    vocab.insert("connect", "🔗");
    vocab.insert("disconnect", "❌");
    
    // Weather and environment
    vocab.insert("weather", "🌤️");
    vocab.insert("sunny", "☀️");
    vocab.insert("cloudy", "☁️");
    vocab.insert("rainy", "🌧️");
    vocab.insert("snowy", "❄️");
    vocab.insert("windy", "💨");
    vocab.insert("foggy", "🌫️");
    vocab.insert("clear", "☀️");
    vocab.insert("bright", "💡");
    vocab.insert("dark", "🌑");
    
    // Size and quantity descriptors
    vocab.insert("huge", "🟫");
    vocab.insert("tiny", "🔸");
    vocab.insert("enormous", "🦣");
    vocab.insert("giant", "👹");
    vocab.insert("mini", "🤏");
    vocab.insert("full", "📦");
    vocab.insert("empty", "📭");
    vocab.insert("half", "🌗");
    vocab.insert("whole", "⚫");
    vocab.insert("complete", "✅");
    vocab.insert("incomplete", "⏳");
    
    // Common objects missing
    vocab.insert("box", "📦");
    vocab.insert("bag", "👜");
    vocab.insert("bottle", "🍼");
    vocab.insert("cup", "☕");
    vocab.insert("plate", "🍽️");
    vocab.insert("bowl", "🥣");
    vocab.insert("spoon", "🥄");
    vocab.insert("fork", "🍴");
    vocab.insert("knife", "🔪");
    vocab.insert("glass", "🥛");
    vocab.insert("mirror", "🪞");
    vocab.insert("picture", "🖼️");
    vocab.insert("photo", "📷");
    vocab.insert("camera", "📸");
    vocab.insert("television", "📺");
    vocab.insert("radio", "📻");
    vocab.insert("music", "🎵");
    vocab.insert("sound", "🔊");
    vocab.insert("noise", "📢");
    vocab.insert("quiet", "🤫");
    vocab.insert("loud", "📢");
    vocab.insert("silence", "🤐");
    
    // Clothing and accessories
    vocab.insert("clothes", "👕");
    vocab.insert("shirt", "👕");
    vocab.insert("pants", "👖");
    vocab.insert("dress", "👗");
    vocab.insert("shoes", "👞");
    vocab.insert("hat", "👒");
    vocab.insert("coat", "🧥");
    vocab.insert("jacket", "🧥");
    vocab.insert("socks", "🧦");
    vocab.insert("gloves", "🧤");
    vocab.insert("watch", "⌚");
    vocab.insert("glasses", "👓");
    
    // Tools and instruments
    vocab.insert("tool", "🔧");
    vocab.insert("hammer", "🔨");
    vocab.insert("saw", "🪚");
    vocab.insert("screwdriver", "🪛");
    vocab.insert("wrench", "🔧");
    vocab.insert("drill", "🪚");
    vocab.insert("ruler", "📏");
    vocab.insert("pencil", "✏️");
    vocab.insert("eraser", "🗑️");
    vocab.insert("scissors", "✂️");
    vocab.insert("tape", "📹");
    vocab.insert("glue", "🧽");
    
    // Better semantic mappings for problematic words
    vocab.insert("well", "👍");   // "well" as in "good" or "okay"
    vocab.insert("wait", "⏸️");  // pause is more semantic than water well
    vocab.insert("see", "👁️");   // single eye is clearer than two eyes
    vocab.insert("wrong", "❌");  // X mark for wrong
    vocab.insert("right", "✅");  // check mark for correct
    vocab.insert("told", "🗣️");   // speaking face for telling
    vocab.insert("choose", "🫵"); // pointing finger for choosing
    vocab.insert("believe", "🙏"); // hands in prayer for belief
    vocab.insert("someday", "📅🔮"); // calendar + crystal ball for future day
    
    // Containers and spaces
    vocab.insert("room", "🏠");
    vocab.insert("kitchen", "🍳");
    vocab.insert("bedroom", "🛏️");
    vocab.insert("bathroom", "🚿");
    vocab.insert("garage", "🚗");
    vocab.insert("garden", "🌻");
    vocab.insert("yard", "🌱");
    vocab.insert("basement", "⬇️🏠");
    vocab.insert("attic", "⬆️🏠");
    vocab.insert("closet", "👕");
    vocab.insert("drawer", "📦");
    vocab.insert("shelf", "📚");
    vocab.insert("cabinet", "🗄️");

    // Additional essential verbs with better semantic mapping
    vocab.insert("want", "🙏");
    vocab.insert("need", "⚠️");
    vocab.insert("like", "👍");
    vocab.insert("dislike", "👎");
    vocab.insert("prefer", "⭐");
    vocab.insert("choose", "🫵");
    vocab.insert("decide", "🤔");
    vocab.insert("agree", "✅");
    vocab.insert("disagree", "❌");
    vocab.insert("understand", "💡");
    vocab.insert("confuse", "😵");
    vocab.insert("explain", "💬");
    vocab.insert("describe", "📝");
    vocab.insert("show", "👉");
    vocab.insert("hide", "🫥");
    vocab.insert("reveal", "✨");
    vocab.insert("protect", "🛡️");
    vocab.insert("attack", "⚔️");
    vocab.insert("defend", "🛡️");
    vocab.insert("escape", "🏃");
    vocab.insert("chase", "🏃💨");
    vocab.insert("follow", "👣");
    vocab.insert("lead", "🧭");
    vocab.insert("guide", "🧭");
    vocab.insert("lose", "😞");
    vocab.insert("win", "🏆");
    vocab.insert("compete", "🏁");
    vocab.insert("race", "🏃");
    vocab.insert("hurry", "💨");
    vocab.insert("rush", "💨");
    vocab.insert("slow", "🐌");
    vocab.insert("relax", "😌");
    vocab.insert("rest", "😴");
    vocab.insert("wake", "⏰");
    vocab.insert("awake", "👁️");
    vocab.insert("asleep", "😴");
    
    // Materials and textures
    vocab.insert("wood", "🪵");
    vocab.insert("metal", "🔩");
    vocab.insert("plastic", "🧱");
    vocab.insert("glass", "🪟");
    vocab.insert("stone", "🪨");
    vocab.insert("rock", "🪨");
    vocab.insert("sand", "🏖️");
    vocab.insert("dirt", "🌱");
    vocab.insert("mud", "🟤");
    vocab.insert("ice", "🧊");
    vocab.insert("steam", "💨");
    vocab.insert("smoke", "💨");
    vocab.insert("dust", "💨");
    vocab.insert("powder", "💨");
    vocab.insert("liquid", "💧");
    vocab.insert("solid", "🧊");
    vocab.insert("gas", "💨");
    
    // Colors that were missing
    vocab.insert("green", "🟢");
    vocab.insert("yellow", "🟡");
    vocab.insert("orange", "🟠");
    vocab.insert("purple", "🟣");
    vocab.insert("pink", "🩷");
    vocab.insert("brown", "🟤");
    vocab.insert("black", "⚫");
    vocab.insert("white", "⚪");
    vocab.insert("gray", "🩶");
    vocab.insert("grey", "🩶");
    vocab.insert("silver", "🩶");
    vocab.insert("gold", "🟡");
    
    // Shapes and patterns
    vocab.insert("circle", "⭕");
    vocab.insert("square", "⬜");
    vocab.insert("triangle", "🔺");
    vocab.insert("rectangle", "⬜");
    vocab.insert("round", "⭕");
    vocab.insert("flat", "📄");
    vocab.insert("curved", "🌙");
    vocab.insert("straight", "📏");
    vocab.insert("bent", "🪃");
    vocab.insert("twisted", "🌀");
    vocab.insert("spiral", "🌀");
    vocab.insert("pattern", "🔳");
    vocab.insert("design", "🎨");
    vocab.insert("shape", "🔷");
    
    // Family and relationships - MORE SPECIFIC SEMANTIC MAPPINGS
    vocab.insert("family", "👨‍👩‍👧‍👦");
    vocab.insert("parent", "👨‍👩‍👧‍👦⬆️"); // parent relationship upward
    vocab.insert("father", "👨‍👧‍👦"); // man with children - clearer than just man
    vocab.insert("mother", "👩‍👧‍👦"); // woman with children - clearer than just woman  
    vocab.insert("dad", "👨‍👧‍👦");
    vocab.insert("mom", "👩‍👧‍👦");
    vocab.insert("son", "👦⬇️👨"); // boy arrow down from man (child of male)
    vocab.insert("daughter", "👧⬇️👨"); // girl arrow down from man (child of male)
    vocab.insert("brother", "👦🤝👦"); // boys connected, not just two boys
    vocab.insert("sister", "👧🤝👧"); // girls connected, not just two girls
    vocab.insert("sibling", "👦👧🤝"); // mixed gender siblings
    vocab.insert("baby", "👶");
    vocab.insert("grandpa", "👴➡️👨‍👧‍👦"); // old man arrow to father (father's father)
    vocab.insert("grandma", "👵➡️👩‍👧‍👦"); // old woman arrow to mother (mother's mother)
    vocab.insert("grandfather", "�➡️👨‍👧‍👦");
    vocab.insert("grandmother", "�👵➡️👩‍👧‍👦");
    vocab.insert("uncle", "👨�👨‍👧‍👦"); // man connected to father (father's brother)
    vocab.insert("aunt", "👩�👩‍👧‍👦"); // woman connected to mother (mother's sister)
    vocab.insert("cousin", "👫➡️👫"); // people connected to people (children of siblings)
    vocab.insert("husband", "👨💍👩"); // man ring woman (marriage bond)
    vocab.insert("wife", "👩💍👨"); // woman ring man (marriage bond)
    vocab.insert("boyfriend", "👨❤️�"); // man heart woman (romantic but not married)
    vocab.insert("girlfriend", "👩❤️�"); // woman heart man (romantic but not married)
    vocab.insert("partner", "�🤝"); // people connected as equals
    
    // Body parts (essential for actions)
    vocab.insert("body", "🧍");
    vocab.insert("head", "🗣️");
    vocab.insert("face", "😀");
    vocab.insert("eye", "👁️");
    vocab.insert("nose", "👃");
    vocab.insert("mouth", "👄");
    vocab.insert("teeth", "🦷");
    vocab.insert("tongue", "👅");
    vocab.insert("lip", "👄");
    vocab.insert("cheek", "😊");
    vocab.insert("chin", "🫵");
    vocab.insert("forehead", "🤔");
    vocab.insert("hair", "💇");
    vocab.insert("neck", "🧣");
    vocab.insert("shoulder", "🤷");
    vocab.insert("arm", "💪");
    vocab.insert("elbow", "💪");
    vocab.insert("wrist", "⌚");
    vocab.insert("hand", "✋");
    vocab.insert("finger", "👆");
    vocab.insert("thumb", "👍");
    vocab.insert("nail", "💅");
    vocab.insert("chest", "🫁");
    vocab.insert("back", "🔙");
    vocab.insert("stomach", "🤰");
    vocab.insert("waist", "👗");
    vocab.insert("hip", "🕺");
    vocab.insert("leg", "🦵");
    vocab.insert("knee", "🦵");
    vocab.insert("ankle", "🦶");
    vocab.insert("foot", "🦶");
    vocab.insert("toe", "🦶");
    vocab.insert("heel", "👠");
    vocab.insert("skin", "🧴");
    vocab.insert("bone", "🦴");
    vocab.insert("muscle", "💪");
    vocab.insert("blood", "🩸");

    vocab
}