pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: Vec<&str>) -> Self {
        Self {
            headers: headers.iter().map(|s| s.to_string()).collect(),
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    fn col_widths(&self) -> Vec<usize> {
        let mut widths: Vec<usize> = self.headers.iter().map(|h| h.len()).collect();
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }
        widths
    }

    pub fn print_with_fmt<F>(&self, f: F)
    where
        F: Fn(usize, &str) -> String,
    {
        let widths = self.col_widths();

        let header_line: String = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{:<w$}", h, w = widths[i]))
            .collect::<Vec<_>>()
            .join("   ");
        println!("{}", header_line);

        let sep: String = widths
            .iter()
            .map(|&w| "-".repeat(w))
            .collect::<Vec<_>>()
            .join("   ");
        println!("{}", sep);

        for row in &self.rows {
            let row_line: String = row
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    let w = widths.get(i).copied().unwrap_or(cell.len());
                    let padded = format!("{:<w$}", cell, w = w);
                    f(i, &padded)
                })
                .collect::<Vec<_>>()
                .join("   ");
            println!("{}", row_line);
        }
    }

    pub fn print(&self) {
        self.print_with_fmt(|_, padded| padded.to_string());
    }
}
