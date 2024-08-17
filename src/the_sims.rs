fn convert_playstation_2_texture(bytes: &[u8]) -> image::RgbaImage {
    let null_position = bytes.iter().position(|x| *x == 0).unwrap();

    let bytes = &bytes[null_position..];

    let width = usize::from(u16::from_le_bytes(bytes[3..5].try_into().unwrap()));
    let height = usize::from(u16::from_le_bytes(bytes[5..7].try_into().unwrap()));

    let image_bytes = &bytes[21..];

    let texture_type = bytes[7];
    match texture_type {
        0 => crate::playstation_2::decode_rgba8(image_bytes, width, height),
        2 => {
            let palette_count = usize::from(u16::from_le_bytes(bytes[9..11].try_into().unwrap()));
            match palette_count {
                16 => {
                    let palette = &bytes[bytes.len() - 64..];
                    crate::playstation_2::decode_c4(image_bytes, width, height, palette)
                }
                256 => {
                    let palette = &bytes[bytes.len() - 1024..];
                    crate::playstation_2::decode_c8(image_bytes, width, height, palette)
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

pub fn extract_playstation_2_textures(datasets_path: &std::path::Path, output_path: &std::path::Path) {
    std::fs::create_dir_all(output_path).unwrap();

    let datasets = std::fs::read(datasets_path).unwrap();

    let file_list = crate::datasets::list_textures(&datasets, crate::Endianness::Little);

    for (name, id, bytes) in file_list {
        let image = convert_playstation_2_texture(bytes);
        crate::save_texture(image, &name, output_path, !THE_SIMS_ALPHA_TEXTURE_IDS.contains(&id));
    }
}

fn convert_gamecube_texture(bytes: &[u8]) -> image::RgbaImage {
    let null_position = bytes.iter().position(|x| *x == 0).unwrap();

    let bytes = &bytes[null_position..];

    let width = usize::from(u16::from_be_bytes(bytes[3..5].try_into().unwrap()));
    let height = usize::from(u16::from_be_bytes(bytes[5..7].try_into().unwrap()));

    let image_bytes = &bytes[21..];

    let texture_type = bytes[7];
    match texture_type {
        0 => crate::gamecube::decode_cmpr(image_bytes, width, height),
        2 => {
            let palette_count = usize::from(u16::from_be_bytes(bytes[9..11].try_into().unwrap()));
            match palette_count {
                16 => {
                    let palette = &bytes[bytes.len() - 64..];
                    crate::gamecube::decode_i4(image_bytes, width, height, palette)
                }
                256 => {
                    let palette = &bytes[bytes.len() - 1024..];
                    crate::gamecube::decode_i8(image_bytes, width, height, palette)
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

pub fn extract_gamecube_textures(datasets_path: &std::path::Path, output_path: &std::path::Path) {
    std::fs::create_dir_all(output_path).unwrap();

    let datasets = std::fs::read(datasets_path).unwrap();

    let file_list = crate::datasets::list_textures(&datasets, crate::Endianness::Big);

    for (name, id, bytes) in file_list {
        let image = convert_gamecube_texture(bytes);
        crate::save_texture(image, &name, output_path, !THE_SIMS_ALPHA_TEXTURE_IDS.contains(&id));
    }
}

fn convert_xbox_texture(bytes: &[u8]) -> image::RgbaImage {
    let null_position = bytes.iter().position(|x| *x == 0).unwrap();

    let bytes = &bytes[null_position..];

    let width = usize::from(u16::from_le_bytes(bytes[3..5].try_into().unwrap()));
    let height = usize::from(u16::from_le_bytes(bytes[5..7].try_into().unwrap()));

    let image_bytes = &bytes[21..];

    let texture_type = bytes[7];
    match texture_type {
        0 => crate::xbox::decode_rgba(image_bytes, width, height),
        2 => {
            let palette_count = usize::from(u16::from_le_bytes(bytes[9..11].try_into().unwrap()));
            match palette_count {
                256 => {
                    let palette = &bytes[bytes.len() - 1024..];
                    crate::xbox::decode_palette(image_bytes, width, height, palette)
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

pub fn extract_xbox_textures(datasets_path: &std::path::Path, output_path: &std::path::Path) {
    std::fs::create_dir_all(output_path).unwrap();

    let datasets = std::fs::read(datasets_path).unwrap();

    let file_list = crate::datasets::list_textures(&datasets, crate::Endianness::Little);

    for (name, id, bytes) in file_list {
        let image = convert_xbox_texture(bytes);
        crate::save_texture(image, &name, output_path, !THE_SIMS_ALPHA_TEXTURE_IDS.contains(&id));
    }
}

pub fn extract_rle_textures(rletextures_path: &std::path::Path, output_path: &std::path::Path) {
    std::fs::create_dir_all(output_path).unwrap();

    let rletextures = std::fs::read(rletextures_path).unwrap();

    let file_list = crate::arc::read_le(&rletextures);

    for (name, _, bytes) in file_list {
        let image = crate::rle_textures::convert(bytes, false);
        crate::save_texture(image, &name, output_path, false);
    }
}

const THE_SIMS_ALPHA_TEXTURE_IDS: [u32; 785] = [
    1007216883, 1008232004, 1011490186, 1017241056, 1027490000, 1027830898, 1029845720, 1041591381, 10417810,
    1049153561, 1049995869, 1081271696, 1083566872, 109404227, 1100699540, 1103347070, 1108905392, 1113192483,
    1119873414, 1127323391, 113229838, 1133010945, 1134758172, 1143780422, 1152923044, 1166351504, 1174014070,
    1176179994, 1189052072, 1191339711, 1197852211, 1200610566, 1201043482, 1205195629, 1226328230, 1232352795,
    1239677257, 1244389650, 1245278278, 1245417741, 1248132249, 1249571395, 1252095717, 1255837485, 1256090342,
    1257131220, 1257503940, 1259393632, 1262444187, 1269694521, 1269994351, 1273693135, 1274450240, 1276315725,
    1277270157, 1280285465, 1286366364, 1287508534, 1288102648, 1288377036, 1288452373, 1289837041, 1291290073,
    1295545505, 1296367975, 13017408, 1303845632, 1304982175, 1318759385, 133078983, 1331533632, 1336920589,
    1337922196, 1338689808, 1344628705, 1346440349, 1350167590, 1373351017, 1376136015, 1383022668, 1383237537,
    1386033521, 1390588579, 1407477091, 1411746479, 1412058575, 1414048478, 1416251667, 1419810240, 1420886656,
    1424767862, 1428654390, 1433697163, 1435061140, 144191926, 1446678859, 1449305614, 1462182578, 1465381215,
    1465783559, 1468857633, 1473366979, 1474889824, 1480214735, 1483595842, 1487035380, 149102951, 1493472985,
    1493845688, 1500497375, 1503029374, 15114695, 1523302966, 1533038703, 1533373278, 1541183703, 1544393846,
    1545909426, 1546718998, 1547989750, 1548992767, 1549758095, 1549947424, 1554416462, 1554645435, 1556199192,
    1559297877, 1575455884, 1580087523, 1583841953, 1586525931, 1590152064, 1594332643, 1602899597, 1610967325,
    162174801, 1629984905, 163427525, 1642686160, 1642731377, 1649243448, 1655289492, 1664786547, 1673408593,
    1674539178, 1675156705, 167722184, 1677525328, 1683448938, 1685554748, 1686966918, 1687323529, 1691612232,
    1694598327, 1707541197, 1713147342, 1714871382, 1717184190, 1724298698, 172713405, 1729599332, 1742327235,
    1742784790, 1743128456, 1744278501, 1746678542, 1754282234, 1757423489, 1760235619, 1782706693, 1784951381,
    1785236087, 1785796291, 1788357632, 1790648562, 1792437045, 1792778332, 1793460482, 1794228327, 179653103,
    1797095305, 1803342740, 1808817067, 1808890490, 1821158806, 1824922885, 1825309254, 183347545, 1837081202,
    1845543411, 1849748447, 1863522633, 187088903, 1876460084, 1877764757, 1877983896, 1890741598, 1894963838,
    1905958861, 1912319040, 1920407260, 1933271192, 1934622088, 194092111, 1948647159, 1958765649, 1959518161,
    1970764196, 1979750871, 1981458480, 1982543632, 1988914299, 1992090920, 1992242347, 1993684097, 2002685212,
    2009560654, 2010307390, 2011493939, 2014061413, 2024338963, 2032899104, 2036209049, 203987305, 2042925928,
    2049020210, 2052681589, 2053029773, 2059725387, 2061671094, 2064958313, 2071623088, 2082458760, 2082924324,
    2083786102, 2097523083, 2107041810, 2111327258, 2119605054, 2121789640, 21221387, 212263823, 213120098, 2133877158,
    2141420088, 2142436463, 2161850013, 2165970169, 2167138617, 2173834452, 2179075001, 2180208016, 2198440095,
    2203205568, 220635556, 2207948484, 2213497197, 221570880, 221610029, 2227200735, 2227222964, 2228750873,
    2254143153, 2257571744, 2262215845, 2262866179, 2263451111, 2270185921, 2274728794, 2276020238, 2296356619,
    2299246849, 2307708600, 2308678635, 2312590494, 231715389, 2321089862, 2322921011, 2329777508, 2331560374,
    2334132517, 2341259573, 2341279030, 2354571453, 236603892, 2369044831, 2371104946, 2377142653, 2387018257,
    2387263051, 2387356939, 2391058709, 2392716845, 2394543432, 2406272427, 2411915595, 241472884, 2427661703,
    2427920644, 2429852991, 2437902299, 2441164747, 2445326395, 2460391378, 2470513994, 2470589447, 2473298037,
    247887003, 2479533103, 2482634073, 2486169630, 2487013783, 2493555561, 2513302292, 2513944407, 2528106678,
    25394345, 2547819041, 2555844068, 2559192339, 256033428, 2564511149, 2565056444, 2572217392, 2579146228,
    2588499655, 2591764154, 2591845755, 2592196798, 2603132394, 2612183905, 2615024267, 2618949150, 2624275499,
    2624754, 2626785218, 2627411119, 2627509826, 2634272892, 2651229738, 2656720934, 2662840942, 2665877561,
    2667934101, 267568360, 268543566, 2689441662, 2692603900, 2696564270, 2719520579, 2722955761, 2724117146,
    2729939624, 2752207135, 2754366619, 2754556187, 2755072362, 2758856150, 2765347564, 2769062070, 2773058844,
    2788093986, 2790726837, 2797733825, 2806555415, 2810625318, 2812555109, 2827984955, 2836190231, 2844601971,
    2854222844, 2858715097, 2859913541, 2863917679, 2864864804, 2864893301, 2873581897, 2874575490, 2878878606,
    2887499265, 2890185211, 2891548407, 2893771846, 289669205, 2898387169, 2899606039, 2902746137, 2902749735,
    2904361628, 2906161201, 2909284774, 2911415413, 291313192, 2914448751, 2914729667, 2922001328, 292676034,
    2933124395, 29332562, 2936010707, 2944825079, 294516232, 2955931886, 2958314906, 2959729549, 2970502519,
    2985333857, 2987623835, 300312616, 3005348502, 3008900312, 3023284336, 302663140, 3028695178, 3031148623,
    3034615717, 3037354976, 3039454433, 3043897667, 3052894465, 305776020, 3059974510, 3060331268, 3069076626,
    3070502804, 3076629747, 3077232072, 308897742, 3090477159, 3093171836, 3094203662, 3094592662, 3099323433,
    310658188, 3114071745, 3130889100, 3148953073, 3151434702, 3168304488, 3171769308, 3189009872, 3191667735,
    3197602794, 3200775131, 3206277125, 3210944854, 3213631581, 3215549649, 3216633386, 3221427970, 3227948133,
    3230720697, 3231349360, 324031740, 3244601848, 32478091, 3248966318, 3252274669, 3261661653, 3265557324,
    3276131022, 3277859306, 3279685349, 3284688153, 3285457275, 3290589204, 3294249339, 3298465598, 3298525620,
    3303023766, 3303708002, 3305424332, 3306924740, 3308672517, 3322508769, 3324783460, 332711134, 3327564119,
    3343563308, 3345928987, 3378341167, 3388967833, 339579109, 3413722430, 3420706462, 3421527797, 342181420,
    3425438475, 3426073928, 34267919, 34272474, 3434580443, 3441868021, 3442576084, 345553542, 3459051679, 3467261173,
    3467825841, 3469078352, 3473496253, 347668679, 3481223932, 3496462605, 3496678449, 3502132539, 3509503647,
    3532720995, 3547923259, 3549239880, 3572847528, 358208700, 3583802293, 358452159, 3584586420, 3585156041,
    3586278262, 3586454964, 3595880186, 3596232220, 3599436608, 3602611777, 3603060407, 3605105476, 3606918045,
    3612592307, 3620518258, 3625241816, 3633912956, 363759106, 3646252605, 3655814454, 3658812525, 3661607079,
    3665899963, 3669440770, 3669875304, 3677319881, 3679682318, 3685511069, 368653731, 3691171751, 3705095362,
    3713141306, 3716492910, 3723100074, 3723595531, 3726385834, 372675071, 3727396960, 3739061382, 3743059587,
    376089901, 3761983854, 3777638017, 3777779155, 3785691707, 3788446092, 3791205845, 3794464525, 3796562426,
    3811098760, 3812352257, 3812847567, 3822209667, 3823653625, 3829728401, 3830456958, 3859347384, 3859885544,
    3860938752, 3866109681, 3870919216, 3871310918, 3881888001, 3882288716, 3887558208, 3887742429, 3894191469,
    3895184503, 3895797471, 3896263654, 3898516171, 3899115774, 3903955234, 3929597094, 3946757594, 3949257834,
    3954469442, 3959808793, 3964937454, 3974609278, 3990097751, 3990990357, 3996942180, 3998735526, 4007865985,
    401115108, 4012319728, 4024198459, 4024935, 4033164180, 4038751372, 4052287286, 4058721456, 4062451833, 4065304847,
    4068230404, 406894218, 4070264846, 4079904814, 408261476, 4085525899, 4088378789, 4091493007, 4095407365,
    4098835012, 409953985, 4100648395, 4108876283, 4110115042, 4110644604, 4111378166, 411326957, 4119123647,
    4125688322, 4126537960, 4138327788, 4142218031, 4152131868, 415629578, 4162182327, 4162965224, 4165055256,
    4168890676, 417553655, 418077187, 4184959289, 4192286076, 4193624305, 4196872593, 4203802357, 4204334167,
    4204670347, 4206153967, 4206322155, 420782879, 4212583572, 4225541191, 4229405914, 4233426154, 4234824766,
    4236890250, 4236912032, 4238057326, 4249251603, 4250751316, 4250809808, 4258964978, 4266115354, 4268818252,
    4277236286, 4284981157, 4286415847, 4287218951, 4290935882, 4292635320, 434597786, 447633800, 454516837, 465918780,
    477497476, 481343781, 485386940, 490705752, 496577278, 501466342, 50335691, 509155451, 519925788, 528915994,
    529246059, 538275250, 539350939, 542255949, 5497670, 550391901, 557687261, 563242833, 569401448, 582808648,
    591982791, 59374018, 59846215, 605031556, 620615353, 626361102, 627120159, 627776854, 634740547, 638448324,
    642873603, 645101320, 651992319, 653851785, 65931295, 666581509, 66663410, 671552717, 675180585, 679677762,
    683214849, 686129121, 697602777, 698643521, 698720625, 7045294, 709355538, 722117856, 724975488, 725828192,
    72608467, 728476352, 731469071, 733254870, 7335304, 735632034, 739350522, 740550028, 751783946, 754394072,
    756329597, 762048732, 763608004, 764373296, 772634158, 778630473, 783129770, 784588093, 788336269, 795529428,
    801771045, 805711295, 813172339, 822855662, 82541599, 829847645, 835520331, 837610493, 841426323, 844997033,
    845649179, 847269431, 849102219, 868540834, 873723298, 874181057, 876295113, 884974805, 888611831, 893049386,
    896259815, 900481858, 906437155, 912134021, 925626281, 926607369, 927485563, 929342389, 934026351, 937570230,
    940790657, 943654919, 945236217, 945639549, 954959046, 961379836, 968471231, 97218763, 977443284, 978642866,
    982861587, 985852493, 986303700, 994415533, 994659692,
];
