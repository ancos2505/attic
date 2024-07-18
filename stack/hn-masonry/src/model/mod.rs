



// API: https://github.com/HackerNews/API


// API: https://hacker-news.firebaseio.com/v0/item/40927438.json?print=pretty
const RECEIVED_ITEM: &str = r#"
{
  "by": "fanf2",
  "descendants": 9,
  "id": 40927438,
  "kids": [40929438, 40929076, 40928990, 40929052, 40929247, 40929146, 40929460],
  "score": 81,
  "time": 1720622523,
  "title": "Weird things I learned while writing an x86 emulator",
  "type": "story",
  "url": "https://www.timdbg.com/posts/useless-x86-trivia/"
}
"#;

// API: https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty

const POST_INDEXES: &str = r#"
[
 40927438, 40921989, 40926734, 40926648, 40928893, 40925025, 40926515, 40928948, 40926411, 40926439,
 40929064, 40929104, 40927481, 40912271, 40891391, 40922739, 40925474, 40920122, 40928756, 40925906,
 40922874, 40926211, 40928450, 40885703, 40916786, 40928503, 40901224, 40914958, 40922740, 40928331,
 40921038, 40916193, 40925471, 40906148, 40889272, 40925896, 40919253, 40920246, 40914475, 40915082,
 40898715, 40911500, 40915005, 40898525, 40920681, 40913138, 40918152, 40883942, 40923986, 40919043,
 40920926, 40907933, 40928469, 40923124, 40928310, 40924472, 40927555, 40906210, 40919505, 40911637,
 40907655, 40928809, 40891003, 40917358, 40909076, 40928290, 40913736, 40921915, 40917312, 40918991,
 40927981, 40927970, 40909327, 40914623, 40907269, 40919762, 40891846, 40918635, 40897435, 40919201,
 40918052, 40888342, 40906401, 40904435, 40916260, 40913793, 40907155, 40912920, 40912684, 40896772,
 40925773, 40910792, 40915225, 40915886, 40902012, 40924936, 40882038, 40908273, 40928675, 40879924,
 40914724, 40927946, 40924915, 40899242, 40909607, 40925163, 40898640, 40920233, 40916748, 40908368,
 40918178, 40894216, 40909334, 40917937, 40911869, 40902740, 40928139, 40887516, 40904862, 40921511,
 40919200, 40880551, 40923079, 40923292, 40924670, 40911339, 40902749, 40908062, 40908985, 40922446,
 40915821, 40907581, 40911816, 40884917, 40919644, 40925456, 40905849, 40927364, 40898313, 40927006,
 40913962, 40909086, 40924690, 40922884, 40927386, 40901623, 40925266, 40928874, 40904407, 40905295,
 40907528, 40920261, 40921165, 40928728, 40897506, 40927274, 40908989, 40916820, 40923715, 40910024,
 40901154, 40906112, 40887521, 40911910, 40914725, 40897518, 40920292, 40903870, 40920159, 40920011,
 40887987, 40905719, 40881077, 40897962, 40908878, 40908178, 40905215, 40919273, 40912075, 40880302,
 40919160, 40904654, 40925913, 40905209, 40918489, 40919707, 40927646, 40899393, 40920812, 40910913,
 40897205, 40895672, 40916724, 40892298, 40899371, 40922209, 40923210, 40876840, 40914350, 40899487,
 40924055, 40877909, 40900087, 40919288, 40926179, 40916584, 40916478, 40878900, 40876453, 40888826,
 40893866, 40920545, 40886972, 40925621, 40891495, 40906877, 40926540, 40898121, 40887806, 40896873,
 40910776, 40909006, 40918652, 40910946, 40912627, 40903690, 40914648, 40926656, 40899309, 40923188,
 40924236, 40919113, 40914145, 40914052, 40906407, 40906393, 40913957, 40922017, 40898118, 40913911,
 40908893, 40922601, 40908348, 40919413, 40920980, 40890035, 40876528, 40923905, 40896470, 40924527,
 40884998, 40895441, 40915112, 40903229, 40887359, 40914340, 40910731, 40911215, 40882881, 40907025,
 40895429, 40892636, 40899281, 40908855, 40924553, 40923171, 40918373, 40883940, 40921941, 40898495,
 40920256, 40912560, 40905670, 40924369, 40909173, 40891643, 40912356, 40882583, 40900029, 40907955,
 40912613, 40907826, 40911990, 40911969, 40907713, 40915269, 40919196, 40907510, 40899520, 40907284,
 40910558, 40928353, 40925317, 40888820, 40914835, 40922570, 40922559, 40905899, 40911256, 40916904,
 40918829, 40917627, 40915837, 40918734, 40909656, 40876848, 40879376, 40883277, 40910447, 40905871,
 40922312, 40890847, 40905692, 40905686, 40893722, 40913503, 40910950, 40899411, 40919648, 40924854,
 40899761, 40917173, 40908534, 40910304, 40905780, 40921336, 40908521, 40895733, 40917858, 40916556,
 40915004, 40892365, 40898844, 40913835, 40912411, 40909773, 40914801, 40922691, 40887955, 40878901,
 40921844, 40880932, 40907552, 40918915, 40888673, 40883839, 40887014, 40909094, 40909390, 40919421,
 40891699, 40891372, 40896102, 40902554, 40892812, 40901985, 40908211, 40892841, 40888085, 40918124,
 40881016, 40912473, 40895167, 40920814, 40891306, 40907024, 40892235, 40912650, 40885155, 40893009,
 40917059, 40887168, 40916984, 40902838, 40906374, 40896110, 40877136, 40906251, 40913390, 40906171,
 40879541, 40904115, 40905904, 40891446, 40913245, 40905922, 40877708, 40877042, 40916486, 40887564,
 40891375, 40914279, 40916305, 40921321, 40887218, 40923650, 40896457, 40905304, 40884131, 40878895,
 40904428, 40905011, 40884531, 40903878, 40877668, 40907691, 40911838, 40888461, 40915641, 40908999,
 40904599, 40917026, 40898383, 40891845, 40918070, 40886385, 40913915, 40921398, 40903461, 40891342,
 40913430, 40924226, 40884878, 40915048, 40906357, 40890018, 40886689, 40881836, 40881672, 40909437,
 40923088, 40891252, 40888282, 40910630, 40910529, 40916983, 40895816, 40908444, 40914384, 40902317,
 40883546, 40905891, 40920060, 40899207, 40891819, 40886954, 40924540, 40916916, 40916905, 40890315,
 40890633, 40914866, 40922548, 40884356, 40922342, 40906747, 40877648, 40899507, 40916462, 40898151,
 40902427, 40916191, 40908145, 40903806, 40896858, 40910401, 40876634, 40922902, 40881224, 40914761,
 40915878, 40927825, 40922459, 40912179, 40896383, 40909102, 40878222, 40901138, 40893522, 40908711,
 40893028, 40891315, 40908319, 40908060, 40920100, 40920497, 40878906, 40913203, 40899724, 40923005,
 40914391, 40915819, 40925037, 40899424, 40899397, 40899321, 40921343, 40885230, 40877337, 40909302
]
"#;


