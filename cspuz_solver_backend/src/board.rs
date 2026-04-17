use crate::uniqueness::Uniqueness;
use cspuz_rs::graph;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Compass {
    pub up: Option<i32>,
    pub down: Option<i32>,
    pub left: Option<i32>,
    pub right: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FireflyDir {
    Up,
    Down,
    Left,
    Right,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ItemKind {
    Dot,
    Block,
    Square,
    Triangle,
    Fill,
    Circle,
    FilledCircle,
    SmallCircle,
    SmallFilledCircle,
    SideArrowUp,
    SideArrowDown,
    SideArrowLeft,
    SideArrowRight,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    AboloUpperLeft,
    AboloUpperRight,
    AboloLowerLeft,
    AboloLowerRight,
    PencilUp,
    PencilDown,
    PencilLeft,
    PencilRight,
    Cross,
    Line,
    DottedLine,
    DoubleLine,
    Wall,
    DottedWall,
    DottedHorizontalWall,
    DottedVerticalWall,
    FirewalkCellUnknown,
    FirewalkCellUl,
    FirewalkCellUr,
    FirewalkCellDl,
    FirewalkCellDr,
    FirewalkCellUlDr,
    FirewalkCellUrDl,
    BoldWall,
    Slash,
    Backslash,
    DottedSlash,
    DottedBackslash,
    Plus,
    Text(&'static str),
    TextString(String),
    Num(i32),
    NumUpperLeft(i32),
    NumUpperRight(i32),
    NumLowerLeft(i32),
    NumLowerRight(i32),
    Compass(Compass),
    TapaClue([i32; 4]),
    SudokuCandidateSet(i32, Vec<i32>),
    Firefly(FireflyDir, i32),
    LineTo(i32, i32),
}

impl ItemKind {
    pub fn to_json(&self) -> String {
        match self {
            &ItemKind::Dot => String::from("\"dot\""),
            &ItemKind::Block => String::from("\"block\""),
            &ItemKind::Square => String::from("\"square\""),
            &ItemKind::Triangle => String::from("\"triangle\""),
            &ItemKind::Fill => String::from("\"fill\""),
            &ItemKind::Circle => String::from("\"circle\""),
            &ItemKind::FilledCircle => String::from("\"filledCircle\""),
            &ItemKind::SmallCircle => String::from("\"smallCircle\""),
            &ItemKind::SmallFilledCircle => String::from("\"smallFilledCircle\""),
            &ItemKind::SideArrowUp => String::from("\"sideArrowUp\""),
            &ItemKind::SideArrowDown => String::from("\"sideArrowDown\""),
            &ItemKind::SideArrowLeft => String::from("\"sideArrowLeft\""),
            &ItemKind::SideArrowRight => String::from("\"sideArrowRight\""),
            &ItemKind::ArrowUp => String::from("\"arrowUp\""),
            &ItemKind::ArrowDown => String::from("\"arrowDown\""),
            &ItemKind::ArrowLeft => String::from("\"arrowLeft\""),
            &ItemKind::ArrowRight => String::from("\"arrowRight\""),
            &ItemKind::AboloUpperLeft => String::from("\"aboloUpperLeft\""),
            &ItemKind::AboloUpperRight => String::from("\"aboloUpperRight\""),
            &ItemKind::AboloLowerLeft => String::from("\"aboloLowerLeft\""),
            &ItemKind::AboloLowerRight => String::from("\"aboloLowerRight\""),
            &ItemKind::PencilUp => String::from("\"pencilUp\""),
            &ItemKind::PencilDown => String::from("\"pencilDown\""),
            &ItemKind::PencilLeft => String::from("\"pencilLeft\""),
            &ItemKind::PencilRight => String::from("\"pencilRight\""),
            &ItemKind::Cross => String::from("\"cross\""),
            &ItemKind::Line => String::from("\"line\""),
            &ItemKind::DottedLine => String::from("\"dottedLine\""),
            &ItemKind::DoubleLine => String::from("\"doubleLine\""),
            &ItemKind::Wall => String::from("\"wall\""),
            &ItemKind::BoldWall => String::from("\"boldWall\""),
            &ItemKind::DottedWall => String::from("\"dottedWall\""),
            &ItemKind::Slash => String::from("\"slash\""),
            &ItemKind::Backslash => String::from("\"backslash\""),
            &ItemKind::DottedSlash => String::from("\"dottedSlash\""),
            &ItemKind::DottedBackslash => String::from("\"dottedBackslash\""),
            &ItemKind::Plus => String::from("\"plus\""),
            &ItemKind::DottedHorizontalWall => String::from("\"dottedHorizontalWall\""),
            &ItemKind::DottedVerticalWall => String::from("\"dottedVerticalWall\""),
            &ItemKind::FirewalkCellUnknown => String::from("\"firewalkCellUnknown\""),
            &ItemKind::FirewalkCellUl => String::from("\"firewalkCellUl\""),
            &ItemKind::FirewalkCellUr => String::from("\"firewalkCellUr\""),
            &ItemKind::FirewalkCellDl => String::from("\"firewalkCellDl\""),
            &ItemKind::FirewalkCellDr => String::from("\"firewalkCellDr\""),
            &ItemKind::FirewalkCellUlDr => String::from("\"firewalkCellUlDr\""),
            &ItemKind::FirewalkCellUrDl => String::from("\"firewalkCellUrDl\""),
            &ItemKind::Text(text) => format!("{{\"kind\":\"text\",\"data\":\"{}\"}}", text),
            ItemKind::TextString(text) => format!("{{\"kind\":\"text\",\"data\":\"{}\"}}", text),
            &ItemKind::Num(num) => format!("{{\"kind\":\"text\",\"data\":\"{}\"}}", num),
            &ItemKind::NumUpperLeft(num) => format!(
                "{{\"kind\":\"text\",\"data\":\"{}\",\"pos\":\"upperLeft\"}}",
                num
            ),
            &ItemKind::NumUpperRight(num) => format!(
                "{{\"kind\":\"text\",\"data\":\"{}\",\"pos\":\"upperRight\"}}",
                num
            ),
            &ItemKind::NumLowerLeft(num) => format!(
                "{{\"kind\":\"text\",\"data\":\"{}\",\"pos\":\"lowerLeft\"}}",
                num
            ),
            &ItemKind::NumLowerRight(num) => format!(
                "{{\"kind\":\"text\",\"data\":\"{}\",\"pos\":\"lowerRight\"}}",
                num
            ),
            ItemKind::Compass(compass) => format!(
                "{{\"kind\":\"compass\",\"up\":{},\"down\":{},\"left\":{},\"right\":{}}}",
                compass.up.unwrap_or(-1),
                compass.down.unwrap_or(-1),
                compass.left.unwrap_or(-1),
                compass.right.unwrap_or(-1)
            ),
            ItemKind::TapaClue(clues) => format!(
                "{{\"kind\":\"tapaClue\",\"value\":[{},{},{},{}]}}",
                clues[0], clues[1], clues[2], clues[3]
            ),
            ItemKind::SudokuCandidateSet(size, cands) => format!(
                "{{\"kind\":\"sudokuCandidateSet\",\"size\":{},\"values\":[{}]}}",
                *size,
                cands
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            ItemKind::Firefly(dir, n) => format!(
                "{{\"kind\":\"firefly\",\"dot\":\"{}\",\"value\":{}}}",
                match *dir {
                    FireflyDir::Up => "up",
                    FireflyDir::Down => "down",
                    FireflyDir::Left => "left",
                    FireflyDir::Right => "right",
                },
                n
            ),
            ItemKind::LineTo(dy, dx) => format!(
                "{{\"kind\":\"lineTo\",\"destY\":{},\"destX\":{}}}",
                *dy, *dx
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Item {
    pub y: usize,
    pub x: usize,
    pub color: &'static str,
    pub kind: ItemKind,
}

impl Item {
    pub fn cell(cell_y: usize, cell_x: usize, color: &'static str, kind: ItemKind) -> Item {
        Item {
            y: cell_y * 2 + 1,
            x: cell_x * 2 + 1,
            color,
            kind,
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"y\":{},\"x\":{},\"color\":\"{}\",\"item\":{}}}",
            self.y,
            self.x,
            self.color,
            self.kind.to_json()
        )
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BoardKind {
    Empty,
    Grid,
    OuterGrid,
    DotGrid,
    ColoredGrid(&'static str),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    pub(crate) kind: BoardKind,
    pub(crate) height: usize,
    pub(crate) width: usize,
    pub(crate) data: Vec<Item>,
    pub uniqueness: Uniqueness,
}

impl Board {
    pub fn new(kind: BoardKind, height: usize, width: usize, uniqueness: Uniqueness) -> Board {
        Board {
            kind,
            height,
            width,
            data: vec![],
            uniqueness,
        }
    }

    pub fn push(&mut self, item: Item) {
        self.data.push(item);
    }

    pub fn extend<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = Item>,
    {
        self.data.extend(items);
    }

    pub fn add_grid(&mut self, y: usize, x: usize, height: usize, width: usize) {
        for i in 0..=height {
            for j in 0..width {
                self.push(Item {
                    y: (i + y) * 2,
                    x: (j + x) * 2 + 1,
                    color: "black",
                    kind: if i == 0 || i == height {
                        ItemKind::BoldWall
                    } else {
                        ItemKind::Wall
                    },
                })
            }
        }
        for i in 0..height {
            for j in 0..=width {
                self.push(Item {
                    y: (i + y) * 2 + 1,
                    x: (j + x) * 2,
                    color: "black",
                    kind: if j == 0 || j == width {
                        ItemKind::BoldWall
                    } else {
                        ItemKind::Wall
                    },
                })
            }
        }
    }

    pub fn add_borders(&mut self, borders: &graph::BoolInnerGridEdgesModel, color: &'static str) {
        let height = self.height;
        let width = self.width;
        for y in 0..height {
            for x in 0..width {
                if y < height - 1 && borders.horizontal[y][x] {
                    self.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color,
                        kind: ItemKind::BoldWall,
                    });
                }
                if x < width - 1 && borders.vertical[y][x] {
                    self.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color,
                        kind: ItemKind::BoldWall,
                    });
                }
            }
        }
    }

    pub fn add_borders_as_answer(
        &mut self,
        borders: Option<&graph::BoolInnerGridEdgesIrrefutableFacts>,
    ) {
        let height = self.height;
        let width = self.width;
        for y in 0..height {
            for x in 0..width {
                if y < height - 1 {
                    let mut need_default_edge = true;
                    if let Some(ref border) = borders {
                        if let Some(b) = border.horizontal[y][x] {
                            self.push(Item {
                                y: y * 2 + 2,
                                x: x * 2 + 1,
                                color: "green",
                                kind: if b {
                                    ItemKind::BoldWall
                                } else {
                                    ItemKind::Cross
                                },
                            });
                            if b {
                                need_default_edge = false;
                            }
                        }
                    }
                    if need_default_edge && !matches!(self.kind, BoardKind::ColoredGrid(_)) {
                        self.push(Item {
                            y: y * 2 + 2,
                            x: x * 2 + 1,
                            color: "#cccccc",
                            kind: ItemKind::Wall,
                        });
                    }
                }
                if x < width - 1 {
                    let mut need_default_edge = true;
                    if let Some(ref border) = borders {
                        if let Some(b) = border.vertical[y][x] {
                            self.push(Item {
                                y: y * 2 + 1,
                                x: x * 2 + 2,
                                color: "green",
                                kind: if b {
                                    ItemKind::BoldWall
                                } else {
                                    ItemKind::Cross
                                },
                            });
                            if b {
                                need_default_edge = false;
                            }
                        }
                    }
                    if need_default_edge && !matches!(self.kind, BoardKind::ColoredGrid(_)) {
                        self.push(Item {
                            y: y * 2 + 1,
                            x: x * 2 + 2,
                            color: "#cccccc",
                            kind: ItemKind::Wall,
                        });
                    }
                }
            }
        }
    }

    pub fn add_lines_irrefutable_facts(
        &mut self,
        lines: &graph::BoolGridEdgesIrrefutableFacts,
        color: &'static str,
        skip: Option<&Vec<Vec<bool>>>,
    ) {
        for y in 0..(self.height - 1) {
            for x in 0..self.width {
                if let Some(skip) = skip {
                    if skip[y][x] || skip[y + 1][x] {
                        continue;
                    }
                }
                if let Some(b) = lines.vertical[y][x] {
                    self.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color,
                        kind: if b { ItemKind::Line } else { ItemKind::Cross },
                    });
                }
            }
        }
        for y in 0..self.height {
            for x in 0..(self.width - 1) {
                if let Some(skip) = skip {
                    if skip[y][x] || skip[y][x + 1] {
                        continue;
                    }
                }
                if let Some(b) = lines.horizontal[y][x] {
                    self.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color,
                        kind: if b { ItemKind::Line } else { ItemKind::Cross },
                    });
                }
            }
        }
    }

    pub fn to_json(&self) -> String {
        let kind = "grid";
        let height = self.height;
        let width = self.width;
        let default_style = match self.kind {
            BoardKind::Empty => "empty",
            BoardKind::Grid => "grid",
            BoardKind::OuterGrid => "outer_grid",
            BoardKind::DotGrid => "dots",
            BoardKind::ColoredGrid(_) => "outer_grid",
        };
        let grid_line_items: Vec<Item> = if let BoardKind::ColoredGrid(color) = self.kind {
            let mut items = vec![];
            for y in 0..height {
                for x in 0..width {
                    if y < height - 1 {
                        items.push(Item {
                            y: y * 2 + 2,
                            x: x * 2 + 1,
                            color,
                            kind: ItemKind::Wall,
                        });
                    }
                    if x < width - 1 {
                        items.push(Item {
                            y: y * 2 + 1,
                            x: x * 2 + 2,
                            color,
                            kind: ItemKind::Wall,
                        });
                    }
                }
            }
            items
        } else {
            vec![]
        };
        let data = grid_line_items
            .iter()
            .chain(self.data.iter())
            .map(|item| item.to_json())
            .collect::<Vec<_>>()
            .join(",");
        let uniqueness = match self.uniqueness {
            Uniqueness::Unique => ",\"isUnique\":true",
            Uniqueness::NonUnique => ",\"isUnique\":false",
            Uniqueness::NotApplicable | Uniqueness::NoAnswer => "",
        };
        let has_answer = match self.uniqueness {
            Uniqueness::NoAnswer => false,
            _ => true,
        };
        format!(
            "{{\"kind\":\"{}\",\"height\":{},\"width\":{},\"defaultStyle\":\"{}\",\"hasAnswer\":{},\"data\":[{}]{}}}",
            kind, height, width, default_style, has_answer, data, uniqueness
        )
    }

    pub fn to_text(&self) -> Option<String> {
        const EMPTY_TOKEN: &str = " ";

        fn green_cell_token(kind: &ItemKind) -> Option<String> {
            match kind {
                ItemKind::Block | ItemKind::Fill => Some("#".to_string()),
                ItemKind::Dot => Some(".".to_string()),
                ItemKind::Square => Some("s".to_string()),
                ItemKind::Num(n) => Some(n.to_string()),
                _ => None,
            }
        }

        fn green_edge_token(kind: &ItemKind, is_horizontal: bool) -> Option<&'static str> {
            match kind {
                ItemKind::Cross => Some("x"),
                ItemKind::Line
                | ItemKind::Wall
                | ItemKind::BoldWall
                | ItemKind::DottedLine
                | ItemKind::DoubleLine
                | ItemKind::DottedWall
                | ItemKind::DottedHorizontalWall
                | ItemKind::DottedVerticalWall => {
                    if is_horizontal {
                        Some("-")
                    } else {
                        Some("|")
                    }
                }
                _ => None,
            }
        }

        fn set_token(grid: &mut [Vec<String>], y: usize, x: usize, token: String) -> Option<()> {
            let current = &grid[y][x];
            if current == EMPTY_TOKEN {
                grid[y][x] = token;
                Some(())
            } else if current == &token {
                Some(())
            } else {
                None
            }
        }

        let h = self.height * 2 + 1;
        let w = self.width * 2 + 1;
        let mut grid = vec![vec![EMPTY_TOKEN.to_string(); w]; h];
        if matches!(self.kind, BoardKind::DotGrid) {
            for y in (0..h).step_by(2) {
                for x in (0..w).step_by(2) {
                    grid[y][x] = "+".to_string();
                }
            }
        }

        let mut has_green = false;
        for item in &self.data {
            if item.color != "green" {
                continue;
            }
            if item.y >= h || item.x >= w {
                return None;
            }

            has_green = true;
            let y = item.y;
            let x = item.x;
            if y % 2 == 1 && x % 2 == 1 {
                let token = green_cell_token(&item.kind)?;
                set_token(&mut grid, y, x, token)?;
            } else if y % 2 == 0 && x % 2 == 1 {
                let token = green_edge_token(&item.kind, true)?;
                set_token(&mut grid, y, x, token.to_string())?;
            } else if y % 2 == 1 && x % 2 == 0 {
                let token = green_edge_token(&item.kind, false)?;
                set_token(&mut grid, y, x, token.to_string())?;
            } else {
                return None;
            }
        }

        if !has_green {
            return None;
        }

        let lines = grid
            .into_iter()
            .map(|row| row.join(" ").trim_end().to_string())
            .collect::<Vec<_>>();

        let start = lines.iter().position(|s| !s.is_empty())?;
        let end = lines
            .iter()
            .rposition(|s| !s.is_empty())
            .expect("start implies at least one non-empty line");
        let lines = &lines[start..=end];

        if lines.is_empty() {
            None
        } else {
            Some(lines.join("\n"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Board, BoardKind, Item, ItemKind};
    use crate::decode_and_solve;
    use crate::uniqueness::Uniqueness;

    #[test]
    fn test_to_text_required_puzzles() {
        let urls = [
            "https://puzz.link/p?nurikabe/6/6/m8n8i9u",
            "https://pzprxs.vercel.app/p?fillomino/5/5/g1k34g2h5h4n",
            "https://pzprxs.vercel.app/p?slither/4/4/dgdh2c71",
            "https://pzprxs.vercel.app/p?yajilin/10/10/w32a41b21a21l22e30m21a12b11r20d30g",
            "https://puzz.link/p?heyawake/6/6/aa66aapv0fu0g2i3k",
            "https://puzz.link/p?dbchoco/6/6/pu9hgpe05zu",
            "https://puzz.link/p?evolmino/6/7/i6900910k00005zz1p0008222o",
        ];
        for url in urls {
            let board = decode_and_solve(url.as_bytes()).unwrap();
            let text = board.to_text();
            assert!(text.is_some(), "to_text() returned None for {}", url);
        }
    }

    #[test]
    fn test_to_text_unsupported_item_returns_none() {
        let mut board = Board::new(BoardKind::Grid, 1, 1, Uniqueness::Unique);
        board.push(Item::cell(0, 0, "green", ItemKind::Triangle));
        assert_eq!(board.to_text(), None);
    }
}
