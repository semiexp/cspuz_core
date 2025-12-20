pub enum PenpaEditorPuzzle {
    Square(PenpaEditorSquare),
    Pyramid(PenpaEditorPyramid),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Fill(i32),
    Symbol(Symbol),
    Text(Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text {
    pub text: String,
    pub color_id: i32,
    pub style: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub color_id: i32,
    pub name: String,
    pub style_id: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PenpaEditorSquare {
    height: usize,
    width: usize,
    cells: Vec<Vec<Vec<Item>>>,
}

impl PenpaEditorSquare {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            cells: vec![vec![vec![]; width]; height],
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get_cell(&self, y: usize, x: usize) -> &[Item] {
        &self.cells[y][x]
    }

    pub fn add_cell_item(&mut self, y: usize, x: usize, item: Item) {
        self.cells[y][x].push(item);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PenpaEditorPyramid {
    size: usize,
    cells: Vec<Vec<Vec<Item>>>,
    outside: Vec<Item>,
}

impl PenpaEditorPyramid {
    pub fn new(size: usize) -> Self {
        let mut cells = vec![];
        for i in 0..size {
            cells.push(vec![vec![]; i + 1]);
        }

        Self {
            size,
            cells,
            outside: vec![],
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get_cell(&self, y: usize, x: usize) -> &[Item] {
        &self.cells[y][x]
    }

    pub fn get_outside(&self) -> &[Item] {
        &self.outside
    }

    pub fn add_cell_item(&mut self, y: usize, x: usize, item: Item) {
        self.cells[y][x].push(item);
    }

    pub fn add_outside_item(&mut self, item: Item) {
        self.outside.push(item);
    }
}

#[allow(unused)]
#[cfg(not(target_arch = "wasm32"))]
fn decompress_url_data(data: &str) -> Result<String, &'static str> {
    use base64::Engine;
    use flate2::read::ZlibDecoder;
    use flate2::Decompress;
    use std::io::Read;

    // base64 decode
    let decoded_bytes = match base64::engine::general_purpose::STANDARD.decode(data) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Base64 decoding failed"),
    };

    // zlib decompress
    let mut decoder = ZlibDecoder::new_with_decompress(&decoded_bytes[..], Decompress::new(false));
    let mut decompressed_data = String::new();
    match decoder.read_to_string(&mut decompressed_data) {
        Ok(_) => Ok(decompressed_data),
        Err(_) => Err("Zlib decompression failed"),
    }
}

fn preprocess_json(json: &str) -> String {
    let mut result = String::new();
    let mut is_quote = false;
    let chars: Vec<char> = json.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '\\' {
            result.push(chars[i]);
            if i + 1 < chars.len() {
                result.push(chars[i + 1]);
                i += 1;
            }
        } else if chars[i] == '"' {
            is_quote = !is_quote;
            result.push(chars[i]);
        } else {
            if !is_quote {
                if chars[i] == 'z'
                    && i + 2 < chars.len()
                    && (chars[i + 2] == ':' || chars[i + 2] == ',')
                {
                    result.push('"');
                    result.push(chars[i]);
                    if i + 1 < chars.len() {
                        result.push(chars[i + 1]);
                    }
                    result.push('"');
                    result.push(chars[i + 2]);
                    i += 2;
                    i += 1;
                    continue;
                }
            }
            result.push(chars[i]);
        }
        i += 1;
    }

    result
}

pub fn decode_penpa_editor_url(url: &str) -> Result<PenpaEditorPuzzle, &'static str> {
    let decompressed_data;

    if url.starts_with("penpa-edit-predecoded:") {
        decompressed_data = url["penpa-edit-predecoded:".len()..].to_string();
    } else {
        #[cfg(target_arch = "wasm32")]
        {
            panic!("raw penpa-edit URLs are not supported in wasm32 target");
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let prefix = "https://opt-pan.github.io/penpa-edit/";
            if !url.starts_with(prefix) {
                return Err("Invalid URL prefix");
            }
            let data = &url[prefix.len()..];

            let p = match data.find("&p=") {
                Some(pos) => &data[pos + 3..],
                None => return Err("Missing &p= in URL"),
            };

            let p = match p.find("&") {
                Some(pos) => &p[..pos],
                None => p,
            };

            decompressed_data = decompress_url_data(p)?;
        }
    }

    let lines = decompressed_data.split("\n").collect::<Vec<_>>();
    if lines.len() < 4 {
        return Err("Insufficient data lines");
    }

    let header = lines[0].split(',').collect::<Vec<_>>();
    if header.is_empty() {
        return Err("Board type missing");
    }
    let body_preproc = preprocess_json(&lines[3]);
    let body = json::parse(&body_preproc).map_err(|_| "JSON parsing failed")?;

    if header[0] == "square" {
        Ok(PenpaEditorPuzzle::Square(decode_penpa_editor_data_square(
            &header, body,
        )?))
    } else if header[0] == "pyramid" {
        Ok(PenpaEditorPuzzle::Pyramid(
            decode_penpa_editor_data_pyramid(&header, &lines[1], body)?,
        ))
    } else {
        Err("Unsupported board type")
    }
}

fn decode_penpa_editor_data_square(
    header: &[&str],
    body: json::JsonValue,
) -> Result<PenpaEditorSquare, &'static str> {
    if header.len() < 3 {
        return Err("Insufficient header data for square board");
    }
    let height: usize = header[2].parse().map_err(|_| "Invalid height")?;
    let width: usize = header[1].parse().map_err(|_| "Invalid width")?;

    let mut ret = PenpaEditorSquare::new(height, width);

    let cell_position = |ki: usize| -> Option<(usize, usize)> {
        if ki < 10 + 2 * width {
            return None;
        }
        let y = (ki - 10 - 2 * width) / (width + 4);
        let x = (ki - 10 - 2 * width) % (width + 4);
        if y < height && x < width {
            Some((y, x))
        } else {
            None
        }
    };

    {
        // fills
        let fill_data = &body["zS"];
        if !fill_data.is_object() {
            return Err("Invalid cell data");
        }
        for (k, v) in fill_data.entries() {
            let ki = k.parse::<usize>().map_err(|_| "Invalid cell key")?;
            let fill_value = v.as_i32().ok_or("Invalid fill value")?;

            if let Some((y, x)) = cell_position(ki) {
                ret.add_cell_item(y, x, Item::Fill(fill_value));
            }
        }
    }
    {
        // texts
        let symbol_data = &body["zN"];
        if !symbol_data.is_object() {
            return Err("Invalid cell data");
        }
        for (k, v) in symbol_data.entries() {
            let ki = k.parse::<usize>().map_err(|_| "Invalid cell key")?;
            if !v.is_array() {
                return Err("Invalid cell value");
            }

            if let Some((y, x)) = cell_position(ki) {
                let text = v[0].as_str().ok_or("Invalid text")?.to_string();
                let color_id = v[1].as_i32().ok_or("Invalid color_id")?;
                let style = v[2].as_str().ok_or("Invalid style")?.to_string();

                ret.add_cell_item(
                    y,
                    x,
                    Item::Text(Text {
                        color_id,
                        text,
                        style,
                    }),
                );
            }
        }
    }
    {
        // symbols
        let symbol_data = &body["zY"];
        if !symbol_data.is_object() {
            return Err("Invalid cell data");
        }
        for (k, v) in symbol_data.entries() {
            let ki = k.parse::<usize>().map_err(|_| "Invalid cell key")?;
            if !v.is_array() {
                return Err("Invalid cell value");
            }

            if let Some((y, x)) = cell_position(ki) {
                let color_id = v[0].as_i32().ok_or("Invalid color_id")?;
                let name = v[1].as_str().ok_or("Invalid symbol_name")?.to_string();
                let style_id = v[2].as_i32().ok_or("Invalid style_id")?;

                ret.add_cell_item(
                    y,
                    x,
                    Item::Symbol(Symbol {
                        color_id,
                        name,
                        style_id,
                    }),
                );
            }
        }
    }

    Ok(ret)
}

fn decode_penpa_editor_data_pyramid(
    header: &[&str],
    margin_line: &str,
    body: json::JsonValue,
) -> Result<PenpaEditorPyramid, &'static str> {
    if header.len() < 3 {
        return Err("Insufficient header data for square board");
    }
    let height: usize = header[2].parse().map_err(|_| "Invalid height")?;
    let width: usize = header[1].parse().map_err(|_| "Invalid width")?;
    if height != width {
        return Err("Pyramid board must have equal height and width");
    }

    let margin = margin_line[1..(margin_line.len() - 1)]
        .parse::<usize>()
        .map_err(|_| "Invalid margin")?;
    if !(margin == 0 || margin == 1) {
        return Err("Unsupported margin value");
    }

    let mut ret = PenpaEditorPyramid::new(height - margin * 2);

    let mut row_starts = vec![];
    {
        let start;
        let count;

        if margin == 0 {
            start = 5 * height / 2 + 10;
            count = height;
        } else {
            start = 5 * height / 2 + 2 * height + 18;
            count = height - 2;
        }

        row_starts.push(start);
        for i in 1..count {
            let diff = if i % 2 == 1 { height + 3 } else { height + 4 };
            row_starts.push(row_starts[i - 1] + diff);
        }
    }

    let size = ret.size();
    let cell_position = |ki: usize| -> Option<(usize, usize)> {
        for y in 0..size {
            let row_start = row_starts[y];
            if ki >= row_start && ki < row_start + y + 1 {
                let x = ki - row_start;
                return Some((y, x));
            }
        }
        None
    };
    let mut add_item = |ki: usize, item: Item| {
        if let Some((y, x)) = cell_position(ki) {
            ret.add_cell_item(y, x, item);
        } else {
            ret.add_outside_item(item);
        }
    };

    {
        // fills
        let fill_data = &body["zS"];
        if !fill_data.is_object() {
            return Err("Invalid cell data");
        }
        for (k, v) in fill_data.entries() {
            let ki = k.parse::<usize>().map_err(|_| "Invalid cell key")?;
            let fill_value = v.as_i32().ok_or("Invalid fill value")?;

            add_item(ki, Item::Fill(fill_value));
        }
    }
    {
        // texts
        let symbol_data = &body["zN"];
        if !symbol_data.is_object() {
            return Err("Invalid cell data");
        }
        for (k, v) in symbol_data.entries() {
            let ki = k.parse::<usize>().map_err(|_| "Invalid cell key")?;
            if !v.is_array() {
                return Err("Invalid cell value");
            }

            let text = v[0].as_str().ok_or("Invalid text")?.to_string();
            let color_id = v[1].as_i32().ok_or("Invalid color_id")?;
            let style = v[2].as_str().ok_or("Invalid style")?.to_string();

            add_item(
                ki,
                Item::Text(Text {
                    color_id,
                    text,
                    style,
                }),
            );
        }
    }
    {
        // symbols
        let symbol_data = &body["zY"];
        if !symbol_data.is_object() {
            return Err("Invalid cell data");
        }
        for (k, v) in symbol_data.entries() {
            let ki = k.parse::<usize>().map_err(|_| "Invalid cell key")?;
            if !v.is_array() {
                return Err("Invalid cell value");
            }

            let color_id = v[0].as_i32().ok_or("Invalid color_id")?;
            let name = v[1].as_str().ok_or("Invalid symbol_name")?.to_string();
            let style_id = v[2].as_i32().ok_or("Invalid style_id")?;

            add_item(
                ki,
                Item::Symbol(Symbol {
                    color_id,
                    name,
                    style_id,
                }),
            );
        }
    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_square_cells() {
        let url = "https://opt-pan.github.io/penpa-edit/#m=edit&p=7VRBb5swFL7nV0w+vwOYkqW+dV2zS9ata6YqQihyEtqgQtwZWCdH+e99fiCBgUnboVoPk+WnL997ef6M/bn4UUmdwBRCCGbggY+DT6fA+QwC/4ym14xlWmaJeAcXVblXGgHAl/kc7mVWJJOoqYonR3MuzA2YTyJiPgPGcfosBnMjjuazMCswt5hi4CO3qIs4wqsW3lHeosua9D3E1w1GuEK4TfU2S9aLmvkqIrMEZtf5QP+2kOXqZ8IaHfb3VuWb1BIbWeJmin361GSKaqceq6bWj09gLmq5tyNyg1auhbVci0bk2l28stzz+HTCz/4NBa9FZLV/b+GshbfiyDhnwkd8jTgIbYMAFdEJIeuLI8YV5sIAcwFK6MjG1JwKOMUldgUTUPxI0aMYUlxQzRXFO4qXFM8oTqnmvdX1h8przV2NryQn4pxsUI/w73A8idABrFDZuqj0vdzieZJB8MiQO1T5JtEOlSn1lKUHty59OCidjKYsmewexuo3Su963Z9lljlEbXiHqo/YoUqN167zW2qtnh0ml+XeITpX1OmUHEpXQCldifJR9lbL2z2fJuwXoxlxfJiA4xf+/8D8kwfGHoL31sz61uTQ/VV61PxIj/gf2VGfN/zA6sgPTG0XHPoa2RFrI9t3N1JDgyM58Dhyv7G57dp3ulXVN7tdauB3u1TX8lE8eQE=";
        let decoded = decode_penpa_editor_url(url).unwrap();

        #[allow(unreachable_patterns)]
        let decoded = match decoded {
            PenpaEditorPuzzle::Square(sq) => sq,
            _ => panic!("Expected square puzzle"),
        };
        assert_eq!(decoded.height(), 5);
        assert_eq!(decoded.width(), 6);
        for y in 0..5 {
            for x in 0..6 {
                if (y, x) == (0, 0) {
                    assert_eq!(decoded.get_cell(y, x), &[Item::Fill(1)]);
                } else if (y, x) == (1, 3) {
                    assert_eq!(
                        decoded.get_cell(y, x),
                        &[Item::Text(Text {
                            text: "3".to_string(),
                            color_id: 1,
                            style: "1".to_string(),
                        })]
                    );
                } else if (y, x) == (3, 1) {
                    assert_eq!(
                        decoded.get_cell(y, x),
                        &[Item::Symbol(Symbol {
                            color_id: 3,
                            name: "circle_L".to_string(),
                            style_id: 1,
                        })]
                    );
                } else {
                    assert!(decoded.get_cell(y, x).is_empty());
                }
            }
        }
    }

    #[test]
    fn test_decode_pyramid_cells() {
        let url = "https://opt-pan.github.io/penpa-edit/#m=edit&p=7ZTBb5swFMbv+Ssmn98BQ0gXblnX7JJl69KpqhCKnIQ2qBBnBtaOKP9733vQUQcm7bKqhwn55eOHsT9sf9n/MipLNjDCy3fAAYmXh4qa9IfciNN1lRRpHLyDSVlstUEB8GU6hVuV5vEgdKLBoRoH1SVUn4JQSAHCxSZFBNVlcKg+B9UcqgU+EuAhm9WdXJQXrbzm56TOaygd1PNGo7xBuU7MOo2Xs5p8DcLqCgTN84HfJiky/TMWjQ+6X+tslRBIk1382MC83Oj7sukmoyNUk9rp4tkpTdA4JdONU5K1U1I9TukD/p3TcXQ84mJ/Q6/LICTb31v5vpWL4CC8oQg81HPU7ogGmKCZel+Ez8B7BthNBgesN1ynXF2uVzgiVB7Xj1wdrj7XGfe54HrN9ZzrkOuI+5yRp790Xft9BTuh68MYv56ahDOu7a8E/7eS0SDEEy5ynS7z0tyqNW4aH33cHGS7MlvFxkKp1nvaQwsmdztt4t5HBOPNXV//lTabk9EfVJpaIP9RKmO/XB8/CxUGz9aLe2WMfrBIpoqtBVaqwODn22RvjxTvCttAoWyL6l6dzJa133wciEfBLXTBHYGLK/z/D+QV/0Bo6Z23Fsi3ZodPrTa9kUfck3qkvelueCfgyDtRpgm7aUbaE2ikp5lG1I01wk6ykf0h3DTqab7J1WnEaapOymmql0EPo8ET";
        let decoded = decode_penpa_editor_url(url).unwrap();

        #[allow(unreachable_patterns)]
        let decoded = match decoded {
            PenpaEditorPuzzle::Pyramid(sq) => sq,
            _ => panic!("Expected pyramid puzzle"),
        };
        assert_eq!(decoded.size(), 6);
        assert_eq!(
            decoded.get_outside(),
            &[Item::Text(Text {
                text: "A".to_string(),
                color_id: 1,
                style: "1".to_string(),
            })]
        );
        for y in 0..6 {
            for x in 0..=y {
                if (y, x) == (1, 0) {
                    assert_eq!(decoded.get_cell(y, x), &[Item::Fill(3)]);
                } else if (y, x) == (3, 3) {
                    assert_eq!(
                        decoded.get_cell(y, x),
                        &[Item::Text(Text {
                            text: "3".to_string(),
                            color_id: 1,
                            style: "1".to_string(),
                        })]
                    );
                } else {
                    assert!(decoded.get_cell(y, x).is_empty());
                }
            }
        }
    }
}
