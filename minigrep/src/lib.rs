extern crate termion;

use std::fmt;
use std::fs;
use std::io;

use termion::color;

pub enum OutputType {
    Plain,
    Color,
    FilesWithMatch,
    FilesWithoutMatch,
}

impl fmt::Display for OutputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            OutputType::Plain => write!(f, "plain"),
            OutputType::Color => write!(f, "color"),
            OutputType::FilesWithMatch => write!(f, "files_with_match"),
            OutputType::FilesWithoutMatch => write!(f, "files_without_match"),
        }
    }
}

pub struct Config {
    pub query: String,
    pub filenames: Vec<String>,
    pub output_type: OutputType,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "query={} fnames=[{}] ot={}", 
               &self.query,
               &self.filenames.join(","),
               &self.output_type,
               )
    }
}

pub fn run(config: Config) {
    for filename in &config.filenames {
        run_one(filename, &config).expect("cannot open file(s)");
    }
}

fn run_one(filename: &str, config: &Config) -> io::Result<()> {
    let contents = fs::read_to_string(filename)?;

    let mut zero_matches = true;
    for mt in search(&config.query, &contents) {
        zero_matches = false;

        // Add a filename prefix if we're searching more than one.
        let mut prefix = "".to_string();
        if config.filenames.len() > 1 {
            match &config.output_type {
                OutputType::Color => {
                    prefix = format!("{}{}{}:", 
                                    color::Fg(color::Magenta),
                                    &filename,
                                    color::Fg(color::Reset),
                                    ).to_string();
                },
                _ => {
                    prefix = format!("{}:", 
                                    &filename,
                                    ).to_string();
                }
            }
        }

        match config.output_type {

            // Matching files only?  Print the name and stop processing further
            // matches.
            OutputType::FilesWithMatch => {
                println!("{}", filename);
                return Ok(());
            }

            // This case is handled after the search is complete.
            OutputType::FilesWithoutMatch => {
            }

            // Plain output?  Simply print the matching line.
            OutputType::Plain => println!("{}{}", &prefix, mt.line),

            // For color output, clone the string and consume it, inserting
            // color escapes into the match positions as we go.
            OutputType::Color => {
                let mut dup = mt.line.clone();
                for pair in mt.matches {
                    let mut chunk : String = dup.drain(..pair.0).collect();
                    print!("{}{}{}", 
                           prefix,
                           color::Fg(color::Reset),
                           chunk);
                    chunk = dup.drain(..(pair.1 - pair.0)).collect();
                    print!("{}{}", 
                           color::Fg(color::Red),
                           chunk);

                    // Only want the file prefix for the first chunk in the
                    // line.  Unset it for any further chunks (if any).
                    prefix = "".to_string();
                }
                print!("{}{}{}", 
                       color::Fg(color::Reset),
                       dup,
                       "\n");
            }
        }
    }

    match config.output_type {
        OutputType::FilesWithoutMatch => {
            if zero_matches {
                println!("{}", filename);
            }
        },
        _ => {}
    }
    Ok(())
}

pub struct Match {
    // the line itself
    line: String,
    // begin/end position of each match in the line
    matches: Vec<(usize, usize)>,
}

pub fn search(query: &str, contents: &str) -> Vec<Match> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            // Copy the matching line and search it for matches.
            let mut dup = line.to_string();
            let mut matches = Vec::<(usize, usize)>::new();
            loop {
                let res = dup.find(query);
                match res {
                    None => break,
                    Some(i) => {
                        // Grab the positions of every match, then consume just
                        // past the end f it.
                        let begin = i;
                        let end = i + query.len();
						matches.push((begin, end));	
                        dup.drain(..end);
                    }
                }
            }
            results.push(Match{
                line: line.to_string(), 
                matches: matches,
			});
        }
    }
    results
}

// TODO: super need to update the below....
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
            );
    }
}
