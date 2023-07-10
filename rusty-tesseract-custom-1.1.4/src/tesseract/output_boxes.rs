use super::*;
use core::fmt;

#[derive(Debug, PartialEq)]
pub struct BoxOutput {
    pub output: String,
    pub boxes: Vec<Box>,
}

impl fmt::Display for BoxOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.output)
    }
}

#[derive(Debug, PartialEq)]
pub struct Box {
    pub symbol: String,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
    pub top: i32,
    pub page: i32,
}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {}",
            self.symbol, self.left, self.bottom, self.right, self.top, self.page
        )
    }
}

impl FromLine for Box {
    fn from_line(line: &str) -> Option<Self> {
        let mut x = line.split_whitespace();

        Some(Box {
            symbol: x.next()?.to_string(),
            left: parse_next(&mut x)?,
            bottom: parse_next(&mut x)?,
            right: parse_next(&mut x)?,
            top: parse_next(&mut x)?,
            page: parse_next(&mut x)?,
        })
    }
}

pub fn image_to_boxes(image: &Image, args: &Args) -> TessResult<BoxOutput> {
    let mut command = create_tesseract_command(image, args)?;
    command.arg("makebox");

    let output = run_tesseract_command(&mut command)?;
    let boxes = string_to_boxes(&output)?;
    Ok(BoxOutput { output, boxes })
}

fn string_to_boxes(output: &str) -> TessResult<Vec<Box>> {
    output
        .lines()
        .into_iter()
        .map(|line| Box::parse(line.into()))
        .collect::<_>()
}

#[cfg(test)]
mod tests {
    use crate::{output_boxes::string_to_boxes, tesseract::*};

    #[test]
    fn test_string_to_boxes() {
        let result = string_to_boxes("L 18 26 36 59 0");
        assert_eq!(
            *result.unwrap().first().unwrap(),
            Box {
                symbol: String::from("L"),
                left: 18,
                bottom: 26,
                right: 36,
                top: 59,
                page: 0
            }
        )
    }

    #[test]
    fn test_image_to_boxes() {
        let img = Image::from_path("img/string.png").unwrap();
        let mut image_to_boxes_args = Args::default();
        image_to_boxes_args.psm = 6;

        let result = image_to_boxes(&img, &image_to_boxes_args).unwrap();
        assert_eq!(
            result.boxes,
            string_to_boxes(
                r#"L 18 26 36 59 0
                O 35 25 70 60 0
                R 75 26 98 59 0
                E 103 26 122 59 0
                M 127 26 162 59 0
                I 181 26 214 59 0
                P 203 25 226 60 0
                S 216 25 263 60 0
                U 252 25 280 60 0
                M 269 26 304 59 0
                D 323 26 352 59 0
                O 355 25 390 60 0
                L 395 26 413 59 0
                O 413 25 448 60 0
                R 453 26 476 59 0
                S 490 25 511 60 0
                I 514 26 518 59 0
                T 521 26 540 59 0
                A 553 26 586 59 0
                M 589 26 624 59 0
                E 630 26 649 59 0
                T 652 26 671 59 0"#
            )
            .unwrap()
        );
    }

    #[test]
    fn test_string_to_boxes_parse_error() {
        let result = string_to_boxes("L 18 X 36 59 0");
        assert_eq!(
            result,
            Err(TessError::ParseError(
                "invalid line 'L 18 X 36 59 0'".into()
            ))
        )
    }
}
