use crate::board::Board;

macro_rules! puzzle_list {
    ( $func_name:ident, $( ($mod:ident, $aliases: expr, $en_name:expr, $ja_name:expr ) ),* $(,)? ) => {
        $(
            pub mod $mod;
        )*

        pub fn $func_name(puzzle_kind: &str, url: &str) -> Option<Result<Board, &'static str>> {
            $(
                for alias in $aliases {
                    if puzzle_kind == alias {
                        return Some($mod::solve(url));
                    }
                }
            )*

            None
        }
    };
}

pub mod heyawake_internal;

#[rustfmt::skip]
puzzle_list!(dispatch_puzz_link_puzzle,
    (akari, ["akari"], "Akari", "美術館"),
    (akichiwake, ["akichi"], "Akichiwake", "Akichiwake"),
    (aqre, ["aqre"], "Aqre", "Aqre"),
    (aquapelago, ["aquapelago"], "Aquapelago", "Aquapelago"),
    (araf, ["araf"], "Araf", "相ダ部屋"),
    (archipelago, ["archipelago"], "Archipelago", "Archipelago"),
    (ayeheya, ["ayeheya"], "Ekawayeh (Symmetry Heyawake)", "∀人∃ＨＥＹＡ"),
    (barns, ["barns"], "Barns", "バーンズ"),
    (castle_wall, ["castle"], "Castle Wall", "Castle Wall"),
    (cave, ["cave"], "Cave", "バッグ"),
    (chainedb, ["chainedb"], "Chained Block", "チェンブロ"),
    (chocobanana, ["cbanana"], "Choco Banana", "チョコバナナ"),
    (cocktail, ["cocktail"], "Cocktail Lamp", "カクテルランプ"),
    (coffeemilk, ["coffeemilk"], "Coffee Milk", "コーヒー牛乳"),
    (compass, ["compass"], "Compass", "Compass"),
    (coral, ["coral"], "Coral", "Coral"),
    (creek, ["creek"], "Creek", "クリーク"),
    (curvedata, ["curvedata"], "Curve Data", "カーブデータ"),
    (dbchoco, ["dbchoco"], "Double Choco", "ダブルチョコ"),
    (doppelblock, ["doppelblock"], "Doppelblock", "ビトゥイーン・サム"),
    (evolmino, ["evolmino"], "Evolmino", "シンカミノ"),
    (fillomino, ["fillomino"], "Fillomino", "フィルオミノ"),
    (firefly, ["firefly"], "Firefly", "ホタルビーム"),
    (firewalk, ["firewalk"], "Firewalk", "ファイアウォーク"),
    (fivecells, ["fivecells"], "Fivecells", "ファイブセルズ"),
    (forestwalk, ["forestwalk"], "Forest Walk", "フォレストウォーク"),
    (guidearrow, ["guidearrow"], "Guide Arrow", "ガイドアロー"),
    (hashi, ["hashi"], "Hashiwokakero", "橋をかけろ"),
    (herugolf, ["herugolf"], "Herugolf", "ヘルゴルフ"),
    (heyawake, ["heyawake"], "Heyawake", "へやわけ"),
    (icewalk, ["icewalk"], "Ice Walk", "アイスウォーク"),
    (inverse_litso, ["invlitso"], "Inverse LITSO", "Inverse LITSO"),
    (kakuro, ["kakuro"], "Kakuro", "カックロ"),
    (kouchoku, ["kouchoku"], "Kouchoku", "交差は直交に限る"),
    (kropki, ["kropki"], "Kropki", "Kropki"),
    (kurotto, ["kurotto"], "Kurotto", "クロット"),
    (litherslink, ["lither"], "Litherslink", "Litherslink"),
    (lits, ["lits"], "LITS", "LITS"),
    (lohkous, ["lohkous"], "Lohkous", "Lohkous"),
    (loop_special, ["loopsp"], "Loop Special", "環状線スペシャル"),
    (masyu, ["masyu", "mashu"], "Masyu", "ましゅ"),
    (moonsun, ["moonsun"], "Moon or Sun", "月か太陽"),
    (nagenawa, ["nagenawa"], "Nagenawa", "なげなわ"),
    (nikoji, ["nikoji"], "NIKOJI", "NIKOJI"),
    (norinori, ["norinori"], "Norinori", "のりのり"),
    (nothree, ["nothree"], "No Three", "ノースリー"),
    (nurikabe, ["nurikabe"], "Nurikabe", "ぬりかべ"),
    (nurimaze, ["nurimaze"], "Nurimaze", "ぬりめいず"),
    (nurimisaki, ["nurimisaki"], "Nurimisaki", "ぬりみさき"),
    (pencils, ["pencils"], "Pencils", "ペンシルズ"),
    (polyominous, ["pentominous"], "Pentominous", "Pentominous"),
    (reflect, ["reflect"], "Reflect Link", "リフレクトリンク"),
    (ringring, ["ringring"], "Ring-Ring", "リングリング"),
    (sashigane, ["sashigane"], "Sashigane", "さしがね"),
    (shakashaka, ["shakashaka"], "Shakashaka", "シャカシャカ"),
    (shikaku, ["shikaku"], "Shikaku", "四角に切れ"),
    (shimaguni, ["shimaguni"], "Shimaguni", "島国"),
    (simpleloop, ["simpleloop"], "Simple Loop", "シンプルループ"),
    (slalom, ["slalom"], "Slalom", "スラローム"),
    (slashpack, ["slashpack"], "Slash Pack", "Slash Pack"),
    (slitherlink, ["slither", "slitherlink"], "Slitherlink", "スリザーリンク"),
    (square_jam, ["squarejam"], "Square Jam", "Square Jam"),
    (statue_park, ["statuepark"], "Statue Park", "Statue Park"),
    (stostone, ["stostone"], "Stostone", "ストストーン"),
    (sudoku, ["sudoku"], "Sudoku", "数独"),
    (tapa, ["tapa"], "Tapa", "Tapa"),
    (timebomb, ["timebomb"], "Time Bomb", "時限爆弾"),
    (tontonbeya, ["tontonbeya"], "Tontonbeya", "とんとんべや"),
    (yajilin, ["yajilin", "yajirin"], "Yajilin", "ヤジリン"),
    (yajilin_regions, ["yajilin-regions"], "Yajilin (Regions)", "ヘヤジリン"),
);

#[rustfmt::skip]
puzzle_list!(dispatch_kudamono_puzzle,
    (akari_regions, ["akari-regional"], "Regional Akari", "Regional Akari"),
    (akari_rgb, ["akari-rgb"], "Akari RGB", "Akari RGB"),
    (cross_border_parity_loop, ["cross-border-parity-loop"], "Cross Border Parity Loop", "Cross Border Parity Loop"),
    (crosswall, ["crosswall"], "Cross Wall", "クロスウォール"),
    (hidato, ["hidoku"], "Hidato", "Hidato"),
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
    (the_longest, ["the-longest"], "The Longest", "短辺消失"),
    (tricklayer, ["tricklayer"], "Tricklayer", "Tricklayer"),
);

pub mod double_lits;
