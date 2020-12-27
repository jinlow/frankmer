use figlet_rs::FIGfont;

/// Container to handle the figlet text.
pub struct TimeText {
    fig_text: String,
    font: FIGfont,
}

impl TimeText {
    /// Create new text object.
    pub fn new(text: String, font: FIGfont) -> Self {
        let fig_text = match font.convert(text.as_str()) {
            Some(x) => format!("{}", x),
            None => String::from(""),
        };
        TimeText { fig_text, font }
    }
    /// Update the text in the TimeText struct.
    pub fn update_text(&mut self, text: String) {
        self.fig_text = self.gen_fig_text(text);
    }
    /// Generate the figure text
    fn gen_fig_text(&self, text: String) -> String {
        match self.font.convert(text.as_str()) {
            Some(x) => format!("{}", x),
            None => String::from(""),
        }
    }
    /// Length of text.
    pub fn text_length(&self) -> u16 {
        match self.fig_text.find("\n") {
            Some(x) => x as u16,
            None => 0,
        }
    }
    /// Text height.
    pub fn text_height(&self) -> u16 {
        (self.fig_text.matches("\n").count() + 1) as u16
    }
    /// Pad the text.
    pub fn pad_left_right(&self, pad_str: &str, pad_length: usize) -> String {
        let fill_str = format!("{:indent$}", pad_str, indent = pad_length);
        let fig_text_str = format!("{}{}", fill_str, self.fig_text);
        let fill_str = "\n".to_owned() + &fill_str;
        fig_text_str.replace("\n", fill_str.as_str())
    }
}
