use std::io::Read;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub struct Input {
    raw_string: String,
}

impl Default for Input {
    #[cfg(feature = "force-read")]
    /// Reads from file that has the same name as binary
    fn default() -> Self {
        let path = std::env::current_exe().unwrap();
        let path = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let path = format!("data/{path}.txt");
        Self::file(path).unwrap()
    }

    #[cfg(not(feature = "force-read"))]
    /// Reads from stdin
    fn default() -> Self {
        Self::stdin()
    }
}

impl Input {
    /// Init Input from stdin
    pub fn stdin() -> Self {
        let mut buffer = String::new();

        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("Error reading from stdin.");

        Self { raw_string: buffer }
    }

    /// Init Input from file
    pub fn file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        Ok(Self {
            raw_string: std::fs::read_to_string(path)?,
        })
    }

    /// Init input from raw text
    pub fn inline(s: &str) -> Self {
        Self {
            raw_string: s.to_owned(),
        }
    }

    /// Get raw string
    pub fn raw(self) -> String {
        self.raw_string
    }

    /// Get cleaned vector of lines
    pub fn lines(self) -> Vec<String> {
        self.raw_string
            .lines()
            .filter(|line| !line.is_empty())
            .map(|x| x.trim().to_owned())
            .collect()
    }

    /// Read data to Type
    ///
    /// This function runs trim and skips empty lines
    pub fn read<T: FromStr>(self) -> Result<T, <T as FromStr>::Err> {
        self.raw_string
            .lines()
            .map(|l| l.trim())
            .filter(|line| !line.is_empty())
            // TODO
            .collect::<Vec<&str>>()
            .join("\n")
            .parse()
    }

    /// Read data to Type using only first line
    ///
    /// This function runs trim and skips empty lines
    pub fn read_one_data<T: FromStr>(self) -> Result<T, <T as FromStr>::Err> {
        self.raw_string
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap()
            .trim()
            .parse()
    }

    /// Read data to Vec of Types
    ///
    /// This function runs trim and skips empty lines
    pub fn read_data<T: FromStr>(self) -> Result<Vec<T>, <T as FromStr>::Err> {
        self.raw_string
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().parse())
            .collect()
    }

    pub fn read_seperated<T: FromStr>(self, sep: char) -> Result<Vec<T>, <T as FromStr>::Err> {
        self.raw_string
            .lines()
            .map(|l| l.trim())
            .filter(|line| !line.is_empty())
            .flat_map(|l| l.split(sep).map(|x| x.parse()))
            .collect()
    }

    /// Read data to Vec of Types where Typed structs are space (new line separated)
    ///
    /// This function runs trim and skips empty lines at end of file
    pub fn read_spaced_data<T: FromStr>(self) -> Result<Vec<T>, <T as FromStr>::Err> {
        let mut v: Vec<T> = Vec::new();
        let mut s = String::new();

        for l in self.raw_string.lines() {
            let l = l.trim();
            if l.is_empty() && !s.trim().is_empty() {
                v.push(s.trim().parse()?);
                s.clear();
            } else {
                s.push_str(&("\n".to_owned() + l));
            }
        }

        if !s.trim().is_empty() {
            v.push(s.trim().parse()?);
        }

        Ok(v)
    }

    /// Read header to Header Type and data to Vec of Types
    ///
    /// This function runs trim and skips empty lines on data parsing but keeps header data intact
    pub fn read_headers_n_data<H: FromStr, T: FromStr>(
        self,
    ) -> Result<(H, Vec<T>), <T as FromStr>::Err>
    where
        H: std::str::FromStr<Err = <T as std::str::FromStr>::Err>,
    {
        let mut v: Vec<T> = Vec::new();
        let mut s = String::new();
        let mut header_readed = false;
        for l in self.raw_string.lines() {
            if header_readed {
                v.push(l.trim().parse()?);
            } else if l.trim().is_empty() && !s.trim().is_empty() {
                header_readed = true;
            } else {
                s.push_str(&("\n".to_owned() + l));
            }
        }

        Ok((s.parse()?, v))
    }

    /// Read header to Header Type and data to Vec of Types
    ///
    /// This function runs trim and skips empty lines on data parsing but keeps header data intact
    pub fn read_headers_n_spaced_data<H: FromStr, T: FromStr>(
        self,
    ) -> Result<(H, Vec<T>), <T as FromStr>::Err>
    where
        H: std::str::FromStr<Err = <T as std::str::FromStr>::Err>,
    {
        let mut v: Vec<T> = Vec::new();
        let mut s = String::new();
        let mut header_readed = 0;
        for (i, l) in self.raw_string.lines().enumerate() {
            if l.trim().is_empty() && !s.trim().is_empty() {
                header_readed = i;
                break;
            } else {
                s.push_str(&("\n".to_owned() + l));
            }
        }

        let h = s.parse()?;
        s.clear();

        for l in self.raw_string.lines().skip(header_readed) {
            let l = l.trim();
            if l.is_empty() && !s.trim().is_empty() {
                v.push(s.trim().parse()?);
                s.clear();
            } else {
                s.push_str(&("\n".to_owned() + l));
            }
        }

        if !s.trim().is_empty() {
            v.push(s.trim().parse()?);
        }

        Ok((h, v))
    }

    pub fn read_n_split_once_on_empty_line<H: FromStr, T: FromStr>(
        self,
    ) -> Result<(Vec<H>, Vec<T>), <T as FromStr>::Err>
    where
        H: std::str::FromStr<Err = <T as std::str::FromStr>::Err>,
    {
        let mut empty_line_reached = false;
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();

        for l in self.raw_string.lines() {
            if empty_line_reached {
                v2.push(l.trim().parse()?);
            } else if l.trim().is_empty() {
                empty_line_reached = true;
            } else {
                v1.push(l.trim().parse()?);
            }
        }

        Ok((v1, v2))
    }
}

/*
/// Returns raw input data that is not trimmed
pub fn input() -> String {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer).unwrap();

    buffer
}

pub fn read_data<T>(filename: &str) -> std::io::Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(filename)?;
    let v = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse::<T>().unwrap())
        .collect();
    Ok(v)
}
*/
