use crate::uniqueness::Uniqueness;
use cspuz_rs::graph;

#[derive(PartialEq, Eq)]
pub struct Compass {
    pub up: Option<i32>,
    pub down: Option<i32>,
    pub left: Option<i32>,
    pub right: Option<i32>,
}

#[derive(PartialEq, Eq)]
pub enum FireflyDir {
    Up,
    Down,
    Left,
    Right,
}

#[allow(unused)]
#[derive(PartialEq, Eq)]
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
pub enum BoardKind {
    Empty,
    Grid,
    OuterGrid,
    DotGrid,
}

pub struct Board {
    kind: BoardKind,
    height: usize,
    width: usize,
    data: Vec<Item>,
    uniqueness: Uniqueness,
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
        };
        let data = self
            .data
            .iter()
            .map(|item| item.to_json())
            .collect::<Vec<_>>()
            .join(",");
        let uniqueness = match self.uniqueness {
            Uniqueness::Unique => ",\"isUnique\":true",
            Uniqueness::NonUnique => ",\"isUnique\":false",
            Uniqueness::NotApplicable => "",
        };
        format!(
            "{{\"kind\":\"{}\",\"height\":{},\"width\":{},\"defaultStyle\":\"{}\",\"data\":[{}]{}}}",
            kind, height, width, default_style, data, uniqueness
        )
    }
}
