use crate::board::Board;

macro_rules! puzz_link {
    ( $( ($mod:ident, $aliases: expr, $en_name:expr, $ja_name:expr ) ),* $(,)? ) => {
        $(
            pub mod $mod;
        )*

        pub fn dispatch_puzz_link_puzzle(puzzle_kind: &str, url: &str) -> Option<Result<Board, &'static str>> {
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

#[rustfmt::skip]
puzz_link!(
    (akari, ["akari"], "Akari", "美術館"),
    (akichiwake, ["akichi"], "Akichiwake", "Akichiwake"),
    (aqre, ["aqre"], "Aqre", "Aqre"),
    (aquapelago, ["aquapelago"], "Aquapelago", "Aquapelago"),
    (araf, ["araf"], "Araf", "相ダ部屋"),
    (archipelago, ["archipelago"], "Archipelago", "Archipelago"),
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

pub mod akari_regions;
pub mod akari_rgb;
pub mod cross_border_parity_loop;
pub mod crosswall;
pub mod double_lits;
pub mod heyawake;
pub mod hidato;
pub mod kropki_pairs;
pub mod letter_weights;
pub mod milktea;
pub mod multiplication_link;
pub mod parrot_loop;
pub mod seiza;
pub mod slicy;
pub mod sniping_arrow;
pub mod soulmates;
pub mod spokes;
pub mod the_longest;
pub mod tricklayer;
