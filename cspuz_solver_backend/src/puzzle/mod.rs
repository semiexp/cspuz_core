use crate::board::Board;

macro_rules! dispatch_enumerate {
    ( $mod:ident, $aliases:expr, $puzzle_kind:expr, $url:expr, $num_max_answers:expr ) => {};
    ( $mod:ident, $aliases:expr, $puzzle_kind:expr, $url:expr, $num_max_answers:expr, $enumerate:ident ) => {
        for alias in $aliases {
            if $puzzle_kind == alias {
                return Some(super::$mod::enumerate($url, $num_max_answers));
            }
        }
    };
}

macro_rules! dispatch_enumerate_list {
    ( $ret:expr, $en_name:expr, $ja_name:expr ) => {};
    ( $ret:expr, $en_name:expr, $ja_name:expr, $enumerable:ident) => {
        $ret.push((String::from($en_name), String::from($ja_name)));
    };
}

macro_rules! puzzle_list {
    ( $mod_name:ident, $( ($mod:ident, $aliases: expr, $en_name:expr, $ja_name:expr $(, $enumerable:ident )? ) ),* $(,)? ) => {
        $(
            pub mod $mod;
        )*

        mod $mod_name {
            pub fn dispatch(puzzle_kind: &str, url: &str) -> Option<Result<super::Board, &'static str>> {
                $(
                    for alias in $aliases {
                        if puzzle_kind == alias {
                            return Some(super::$mod::solve(url));
                        }
                    }
                )*

                None
            }

            #[allow(unused)]
            pub fn dispatch_enumerate(
                #[allow(unused)]
                puzzle_kind: &str,
                #[allow(unused)]
                url: &str,
                #[allow(unused)]
                num_max_answers: usize,
            ) -> Option<Result<(super::Board, Vec<super::Board>), &'static str>> {
                $(
                    dispatch_enumerate!($mod, $aliases, puzzle_kind, url, num_max_answers $(, $enumerable)?);
                )*

                None
            }

            pub fn list_puzzles() -> Vec<(String, String)> {
                vec![
                    $(
                        (String::from($en_name), String::from($ja_name)),
                    )*
                ]
            }

            #[allow(unused)]
            pub fn list_puzzles_with_key() -> Vec<(String, String, String)> {
                vec![
                    $(
                        (String::from($aliases[0]), String::from($en_name), String::from($ja_name)),
                    )*
                ]
            }

            #[allow(unused)]
            pub fn list_puzzles_enumerate() -> Vec<(String, String)> {
                let mut ret = vec![];
                $(
                    dispatch_enumerate_list!(ret, $en_name, $ja_name $(, $enumerable)?);
                )*
                ret
            }
        }
    };
}

pub mod heyawake_internal;

#[rustfmt::skip]
puzzle_list!(puzz_link,
    (akari, ["akari"], "Akari", "美術館"),
    (akichiwake, ["akichi"], "Akichiwake", "Akichiwake"),
    (aqre, ["aqre"], "Aqre", "Aqre"),
    (aquapelago, ["aquapelago"], "Aquapelago", "Aquapelago"),
    (aquarium, ["aquarium"], "Aquarium", "アクアプレース"),
    (araf, ["araf"], "Araf", "相ダ部屋"),
    (archipelago, ["archipelago"], "Archipelago", "Archipelago"),
    (ayeheya, ["ayeheya"], "Ekawayeh (Symmetry Heyawake)", "∀人∃ＨＥＹＡ"),
    (balloon, ["balloon"], "Balloon Box (Revised)", "風船箱 (改訂版)"),
    (battleship, ["battleship"], "Battleship", "Battleship"),
    (barns, ["barns"], "Barns", "バーンズ"),
    (bdwalk, ["bdwalk"], "Building Walk", "ビルウォーク"),
    (bosanowa, ["bosanowa"], "Bosanowa", "ボサノワ"),
    (castle_wall, ["castle"], "Castle Wall", "Castle Wall"),
    (cave, ["cave"], "Cave", "バッグ"),
    (chainedb, ["chainedb"], "Chained Block", "チェンブロ"),
    (chocobanana, ["cbanana"], "Choco Banana", "チョコバナナ"),
    (chocona, ["chocona"], "Chocona", "チョコナ"),
    (circlesquare, ["circlesquare"], "Circles and Squares", "Circles and Squares"),
    (city_space, ["cityspace"], "City Space", "City Space"),
    (cocktail, ["cocktail"], "Cocktail Lamp", "カクテルランプ"),
    (coffeemilk, ["coffeemilk"], "Coffee Milk", "コーヒー牛乳"),
    (compass, ["compass"], "Compass", "Compass"),
    (coral, ["coral"], "Coral", "Coral"),
    (country_road, ["country"], "Country Road", "カントリーロード"),
    (creek, ["creek"], "Creek", "クリーク"),
    (cross_the_streams, ["cts"], "Cross the Streams", "Cross the Streams"),
    (curvedata, ["curvedata"], "Curve Data", "カーブデータ", enumerable),
    (dbchoco, ["dbchoco"], "Double Choco", "ダブルチョコ"),
    (dominion, ["dominion"], "Dominion", "ドミニオン"),
    (doppelblock, ["doppelblock"], "Doppelblock", "ビトゥイーン・サム"),
    (energywalk, ["energywalk"], "Energy Walk", "Energy Walk"),
    (evolmino, ["evolmino"], "Evolmino", "シンカミノ"),
    (fillomino, ["fillomino"], "Fillomino", "フィルオミノ"),
    (firefly, ["firefly"], "Firefly", "ホタルビーム"),
    (firewalk, ["firewalk"], "Firewalk", "ファイアウォーク"),
    (fivecells, ["fivecells"], "Fivecells", "ファイブセルズ"),
    (forestwalk, ["forestwalk"], "Forest Walk", "フォレストウォーク"),
    (fourcells, ["fourcells"], "Fourcells", "フォーセルズ"),
    (geradeweg, ["geradeweg"], "Geradeweg", "グラーデヴェグ"),
    (guidearrow, ["guidearrow"], "Guide Arrow", "ガイドアロー"),
    (hashi, ["hashi"], "Hashiwokakero", "橋をかけろ"),
    (hebiichigo, ["hebi"], "Hebi-Ichigo", "へびいちご"),
    (herugolf, ["herugolf"], "Herugolf", "ヘルゴルフ"),
    (heteromino, ["heteromino"], "Heteromino", "ヘテロミノ"),
    (heyawake, ["heyawake"], "Heyawake", "へやわけ", enumerable),
    (hitori, ["hitori"], "Hitori", "ひとりにしてくれ"),
    (icewalk, ["icewalk"], "Ice Walk", "アイスウォーク"),
    (inverse_litso, ["invlitso"], "Inverse LITSO", "Inverse LITSO"),
    (japanese_sums, ["japanesesums"], "Japanese Sums", "Japanese Sums"),
    (kakuro, ["kakuro"], "Kakuro", "カックロ"),
    (keywest, ["keywest"], "Key West", "キーウエスト"),
    (koburin, ["koburin"], "Koburin", "コブリン"),
    (kouchoku, ["kouchoku"], "Kouchoku", "交差は直交に限る"),
    (kropki, ["kropki"], "Kropki", "Kropki"),
    (kurarin, ["kurarin"], "Kurarin", "クラリン"),
    (kurodoko, ["kurodoko"], "Kurodoko", "黒どこ"),
    (kurotto, ["kurotto"], "Kurotto", "クロット"),
    (lapaz, ["lapaz"], "La Paz", "La Paz"),
    (litherslink, ["lither"], "Litherslink", "Litherslink"),
    (lits, ["lits"], "LITS", "LITS"),
    (lohkous, ["lohkous"], "Lohkous", "Lohkous"),
    (loop_special, ["loopsp"], "Loop Special", "環状線スペシャル"),
    (masyu, ["masyu", "mashu"], "Masyu", "ましゅ"),
    (moonsun, ["moonsun"], "Moon or Sun", "月か太陽"),
    (morningwalk, ["morningwalk"], "Morning Walk", "Morning Walk"),
    (nagenawa, ["nagenawa"], "Nagenawa", "なげなわ"),
    (nanro, ["nanro"], "Nanro", "ナンロー"),
    (nikoji, ["nikoji"], "NIKOJI", "NIKOJI"),
    (nonogram, ["nonogram"], "Nonogram", "お絵かきロジック"),
    (norinori, ["norinori"], "Norinori", "のりのり"),
    (norinuri, ["norinuri"], "Norinori", "海苔ぬり"),
    (nothing, ["nothing"], "All or Nothing", "オールｏｒナッシング"),
    (nothree, ["nothree"], "No Three", "ノースリー"),
    (numcity, ["numcity"], "Number City", "ナンバーシティー"),
    (nurikabe, ["nurikabe"], "Nurikabe", "ぬりかべ", enumerable),
    (nurimaze, ["nurimaze"], "Nurimaze", "ぬりめいず"),
    (nurimisaki, ["nurimisaki"], "Nurimisaki", "ぬりみさき"),
    (nuritwin, ["nuritwin"], "Nuritwin", "ぬりツイン"),
    (pencils, ["pencils"], "Pencils", "ペンシルズ"),
    (pentominous, ["pentominous"], "Pentominous", "Pentominous"),
    (putteria, ["putteria"], "Putteria", "プッテリア"),
    (reflect, ["reflect"], "Reflect Link", "リフレクトリンク"),
    (ringring, ["ringring"], "Ring-Ring", "リングリング"),
    (ripple, ["ripple"], "Ripple Effect", "波及効果"),
    (roma, ["roma"], "Roma", "ろーま"),
    (sansaroad, ["sansaroad"], "Sansa Road", "サンサロード"),
    (sashigane, ["sashigane"], "Sashigane", "さしがね"),
    (school_trip, ["shugaku"], "School Trip", "修学旅行の夜"),
    (sendai, ["sendai"], "Sendai-Miyagi", "宮城県仙台市"),
    (shakashaka, ["shakashaka"], "Shakashaka", "シャカシャカ"),
    (shikaku, ["shikaku"], "Shikaku", "四角に切れ"),
    (shimaguni, ["shimaguni"], "Shimaguni", "島国"),
    (simpleloop, ["simpleloop"], "Simple Loop", "シンプルループ"),
    (slalom, ["slalom"], "Slalom", "スラローム"),
    (slashpack, ["slashpack"], "Slash Pack", "Slash Pack"),
    (slitherlink, ["slither", "slitherlink"], "Slitherlink", "スリザーリンク", enumerable),
    (square_jam, ["squarejam"], "Square Jam", "Square Jam"),
    (star_battle, ["starbattle"], "Star Battle", "スターバトル"),
    (statue_park, ["statuepark"], "Statue Park", "Statue Park"),
    (stostone, ["stostone"], "Stostone", "ストストーン"),
    (sudoku, ["sudoku"], "Sudoku", "数独"),
    (sukoro, ["sukoro"], "Sukoro", "数コロ"),
    (tapa, ["tapa"], "Tapa", "Tapa"),
    (tasquare, ["tasquare"], "Tasquare", "たすくえあ"),
    (tetrochain_k, ["tetrochaink"], "Tetrochain K", "テトロチェーンK"),
    (tetrochain_y, ["tetrochain"], "Tetrochain Y", "テトロチェーンY"),
    (tetrominous, ["tetrominous"], "Tetrominous", "Tetrominous"),
    (tilecity, ["tilecity"], "Tile City", "タイルシティー"),
    (timebomb, ["timebomb"], "Time Bomb", "時限爆弾"),
    (tontonbeya, ["tontonbeya"], "Tontonbeya", "とんとんべや"),
    (waterwalk, ["waterwalk"], "Water Walk", "ウォーターウォーク"),
    (yajikazu, ["yajikazu"], "Yajisan-Kazusan", "やじさんかずさん"),
    (yajilin, ["yajilin", "yajirin"], "Yajilin", "ヤジリン"),
    (yajilin_regions, ["yajilin-regions"], "Yajilin (Regions)", "ヘヤジリン"),
    (yajisoko, ["yajisoko"], "Yajisan-Sokoban", "やじさん倉庫番"),
    (yinyang, ["yinyang"], "Yin-Yang", "しろまるくろまる"),
);

#[rustfmt::skip]
puzzle_list!(kudamono,
    (akari_regions, ["akari-regional"], "Regional Akari", "Regional Akari"),
    (akari_rgb, ["akari-rgb"], "Akari RGB", "Akari RGB"),
    (cross_border_parity_loop, ["cross-border-parity-loop"], "Cross Border Parity Loop", "Cross Border Parity Loop"),
    (crosswall, ["crosswall"], "Cross Wall", "クロスウォール"),
    (hidato, ["hidoku"], "Hidato", "Hidato"),
    (knossos, ["knossos"], "Knossos", "Knossos"),
    (kropki_pairs, ["kropki-pairs"], "Kropki Pairs", "Kropki Pairs"),
    (letter_weights, ["letter-weights"], "Letter Weights", "Letter Weights"),
    (milktea, ["milk-tea"], "Milk Tea", "Milk Tea"),
    (multiplication_link, ["multiplication-link"], "Multiplication Link", "掛け算リンク"),
    (parrot_loop, ["parrot-loop"], "Parrot Loop", "Parrot Loop"),
    (seiza, ["seiza"], "Seiza", "星座になれたら"),
    (slicy, ["slicy"], "SLICY", "SLICY"),
    (sniping_arrow, ["sniping-arrow"], "Sniping Arrow", "スナイピングアロー"),
    (soulmates, ["soulmates"], "Soulmates", "ソウルメイツ"),
    (spokes, ["spokes"], "Spokes", "Spokes"),
    (tamago_town, ["tamago-town"], "Tamago Town", "Tamago Town"),
    (tetrochain_ctb, ["tetrochain-ctb"], "Tetrochain CTB", "Tetrochain CTB"),
    (the_longest, ["the-longest"], "The Longest", "短辺消失"),
    (tricklayer, ["tricklayer"], "Tricklayer", "Tricklayer"),
    (windows, ["windows"], "Windows", "Windows"),
);

#[rustfmt::skip]
puzzle_list!(penpa_edit,
    (castle_walker, ["castle_walker"], "Castle Walker", "Castle Walker"),
    (exercise, ["exercise"], "Exercise", "Exercise"),
    (pyramid, ["pyramid"], "Pyramid", "ピラミッド"),
    (pyramid_climbers, ["pyramid_climbers"], "Pyramid Climbers", "Pyramid Climbers"),
);

pub mod double_lits;

pub fn dispatch_puzz_link(puzzle_kind: &str, url: &str) -> Option<Result<Board, &'static str>> {
    puzz_link::dispatch(puzzle_kind, url)
}

pub fn dispatch_puzz_link_enumerate(
    puzzle_kind: &str,
    url: &str,
    num_max_answers: usize,
) -> Option<Result<(Board, Vec<Board>), &'static str>> {
    puzz_link::dispatch_enumerate(puzzle_kind, url, num_max_answers)
}

pub fn dispatch_kudamono(
    puzzle_kind: &str,
    puzzle_variant: &str,
    url: &str,
) -> Option<Result<Board, &'static str>> {
    if let Some(res) = kudamono::dispatch(puzzle_kind, url) {
        return Some(res);
    }

    if puzzle_kind == "lits" && puzzle_variant == "double" {
        return Some(double_lits::solve(url));
    }

    None
}

pub fn dispatch_penpa_edit(puzzle_kind: &str, url: &str) -> Option<Result<Board, &'static str>> {
    penpa_edit::dispatch(puzzle_kind, url)
}

pub fn list_puzzles_for_solve() -> Vec<(String, String)> {
    let mut puzzles = Vec::new();

    puzzles.extend(puzz_link::list_puzzles());

    puzzles.extend(kudamono::list_puzzles());
    puzzles.push(("Double LITS".to_string(), "Double LITS".to_string()));

    puzzles.extend(penpa_edit::list_puzzles());

    puzzles.sort();

    puzzles
}

pub fn list_puzzles_for_enumerate() -> Vec<(String, String)> {
    let mut puzzles = Vec::new();

    puzzles.extend(puzz_link::list_puzzles_enumerate());

    puzzles.sort();

    puzzles
}

pub fn list_penpa_edit_puzzles() -> Vec<(String, String, String)> {
    penpa_edit::list_puzzles_with_key()
}
