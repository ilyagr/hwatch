use ncurses::*;

use cmd::Result;
use std::cmp;

#[derive(Clone)]
pub struct WatchPad {
    pub result: Result,
    pub result_diff_output: String,

    pub screen: WINDOW,
    pub pad: WINDOW,
    pub pad_lines: i32,
    pub pad_position: i32,
}

impl WatchPad {
    // set default value
    pub fn new(_screen: WINDOW) -> Self {
        Self {
            result: Result::new(),
            result_diff_output: String::new(),

            screen: _screen,
            pad: newpad(0, 0),
            pad_lines: 0,
            pad_position: 0,
        }
    }

    // count pad line
    pub fn create_pad(&mut self, output_type: i32) {
        let mut max_x = 0;
        let mut max_y = 0;

        getmaxyx(self.screen, &mut max_y, &mut max_x);

        let mut _pad_lines_result = 0;
        let mut _pad_lines_output = 0;

        // set result output type(stdout/stderr/output)
        let output = self.get_output(output_type);
        let output_text = output.split("\n");

        for _output_line in output_text {
            _pad_lines_result += get_pad_lines(_output_line.to_string(), max_x - 23);
        }

        for _output_line in self.result_diff_output.clone().split("\n") {
            _pad_lines_output += get_pad_lines(_output_line.to_string(), max_x - 23);
        }

        self.pad_lines = cmp::max(_pad_lines_result, _pad_lines_output + 1);
        self.pad = newpad(self.pad_lines.clone(), max_x - 23);
    }

    pub fn update(&mut self, diff_mode: i32, output_type: i32) {
        let output = self.get_output(output_type);
        let output_text = output.split("\n");

        for line in output_text {
            if diff_mode == 2 {
                let mut _output_line = &format!("  {}\n", line);
                wprintw(self.pad, _output_line);
            } else {
                let mut _output_line = &format!("{}\n", line);
                wprintw(self.pad, _output_line);
            }
        }
    }

    pub fn update_output_pad_char(&mut self, _char: String, _reverse: bool, _color_code: i16) {
        if _reverse {
            wattron(self.pad, A_REVERSE());
            self.update_ouput_pad_char_color(_char, _color_code);
            wattroff(self.pad, A_REVERSE());
        } else {
            self.update_ouput_pad_char_color(_char, _color_code);
        }
    }

    fn update_ouput_pad_char_color(&mut self, _char: String, _color_code: i16) {
        if _color_code != 0 {
            wattron(self.pad, COLOR_PAIR(_color_code));
            wprintw(self.pad, &format!("{}", _char));
            wattroff(self.pad, COLOR_PAIR(_color_code));
        } else {
            wprintw(self.pad, &format!("{}", _char));
        }
    }

    fn get_output(&mut self, output_type: i32) -> String {
        let mut output = String::new();
        match output_type {
            ::IS_OUTPUT => output = self.result.output.clone(),
            ::IS_STDOUT => output = self.result.stdout.clone(),
            ::IS_STDERR => output = self.result.stderr.clone(),
            _ => {}
        };
        return output;
    }

    pub fn draw_output_pad(&mut self) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(self.screen, &mut max_y, &mut max_x);
        prefresh(self.pad, self.pad_position, 0, 2, 0, max_y - 1, max_x - 23);
    }

    pub fn scroll_up(&mut self) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(self.screen, &mut max_y, &mut max_x);

        if self.pad_lines > max_y && self.pad_position > 0 {
            self.pad_position -= 1;
            prefresh(self.pad, self.pad_position, 0, 2, 0, max_y - 1, max_x - 23);
        }
    }

    pub fn scroll_down(&mut self) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(self.screen, &mut max_y, &mut max_x);

        if self.pad_lines > max_y && self.pad_position < (self.pad_lines - max_y + 2 - 1) {
            self.pad_position += 1;
            prefresh(self.pad, self.pad_position, 0, 2, 0, max_y - 1, max_x - 23);
        }
    }

    pub fn resize(&mut self) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(self.screen, &mut max_y, &mut max_x);
        resizeterm(max_y, max_x);
        prefresh(self.pad, self.pad_position, 0, 2, 0, max_y - 1, max_x - 23);
    }

    pub fn exit(&self) {
        endwin();
    }
}

// get pad lines from string
fn get_pad_lines(_string: String, _width: i32) -> i32 {
    let char_vec: Vec<char> = _string.chars().collect();
    let mut _char_count = 0;
    let mut _line_count = 1;

    for ch in char_vec {
        if ch.to_string().len() > 1 {
            _char_count += 2;
        } else {
            _char_count += 1;
        }

        if _char_count == _width {
            _line_count += 1;
            _char_count = 0;
        }
    }
    return _line_count;
}

// @TODO:
//    下のコードを参考に、ANSIカラーコードからNcurses向けのカラーコードへの変換処理を実装する
//    example)
//    https://github.com/viseztrance/flow/blob/f34f34210f9bfcded8ae6c6740ab2f2fe2aa28c9/src/utils/ansi_decoder.rs
// fn convert_ansi2ncurses_color() {}
