pub enum PenpaEditorPuzzle {
    Square(PenpaEditorSquare),
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
                if chars[i] == 'z' && i + 2 < chars.len() && chars[i + 2] == ':' {
                    result.push('"');
                    result.push(chars[i]);
                    if i + 1 < chars.len() {
                        result.push(chars[i + 1]);
                    }
                    result.push('"');
                    result.push(':');
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

    let decompressed_data = decompress_url_data(p)?;

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
}
