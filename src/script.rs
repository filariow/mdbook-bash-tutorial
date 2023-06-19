use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{BufRead,BufReader};

pub(crate) struct Step {
    comments: Vec<String>,
    code: Vec<String>,
    format: String,
}

#[allow(dead_code)]
pub(crate) struct Script {
    shebang: Option<String>,
    steps: Vec<Step>,
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.steps
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("");

        // match &self.shebang {
        //     Some(sb) => write!(f, "{}\n{}", sb, s),
        //     None => write!(f, "{}", s),
        // }
        write!(f, "{}", s)
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let comments = self.comments
            .iter()
            .map(|x| x
                 .trim_start_matches('#')
                 .trim_start()
                 .to_string())
            .collect::<Vec<String>>()
            .join("");
        let code = self.code.join("\t");

        write!(f, "1. {}\t```{}\n\t{}\t```\n", comments, self.format, code)
    }
}

fn read_file(file_path: &str) -> BufReader<File>
{
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read file from {}", file_path)
    };

    let buffer = BufReader::new(file);
    return buffer
}

pub(crate) fn parse(file_path: &str) -> Script {
    let mut file_reader = read_file(file_path);
    let shebang  = skip_shebang(&mut file_reader).expect("error looking for Shebang ");

    let mut steps = vec![];
    loop {
        match read_step(&mut file_reader) {
            Ok(x) => {
                match x {
                    Some(step) => steps.push(step),
                    None => break,
                }
            },
            Err(_) => panic!("error parsing step"),
        }
    }

    return Script {
        shebang,
        steps,
    }
}

fn skip_shebang(script: &mut BufReader<File>) -> Result<Option<String>, std::io::Error>
{
    let mut first_line = String::new();
    script.read_line(&mut first_line).expect("Unable to read first line");

    if first_line.starts_with("#!") {
        return Ok(Some(first_line.to_string()));
    }

    let l = first_line.len() as i64;
    script.seek_relative(-1 * l)?;
    return Ok(None)
}

fn read_step(script: &mut BufReader<File>) -> Result<Option<Step>, std::io::Error>
{
    let mut comments = vec![];
    let mut code = vec![];

    loop {
        let mut l = String::new();
        let r = script.read_line(&mut l)?;
        if r == 0 {
            if comments.is_empty() {
                return Ok(None);
            }

            let s = Step{
                comments,
                code,
                format: "bash".to_string(),
            };
            return Ok(Some(s));
        }

        if l
            .trim_end_matches('\r')
            .trim_end_matches('\n')
            .is_empty() {
            continue
        }

        let is_comment = l.starts_with("#");
        if !comments.is_empty() && is_comment {
            match script.seek_relative(-1 * l.len() as i64) {
                Ok(x) => x,
                Err(_) => panic!("error resetting file reader"),
            }
            break
        }

        if l
            .trim_end_matches('\r')
            .trim_end_matches('\n')
            .ends_with("# mdbash: skip-line") {
            continue
        }

        if is_comment {
            comments.push(l);
        } else {
            code.push(l);
        }
    }

    if comments.is_empty() {
        return Ok(None)
    }

    let s = Step{
        comments,
        code,
        format: "bash".to_string(),
    };
    return Ok(Some(s));
}
