use crate::tui::util::ansi;

bitflags::bitflags! {
    #[derive(Clone)]
    pub struct TextFlags: u16 {
        const NONE          = 0;

        // alignment
        //const ALIGN_CENTER  = 1 << 0;
        //const ALIGN_LEFT    = 1 << 1;
        //const ALIGN_RIGHT   = 1 << 2;
        //const ALIGN_BOTTOM  = 1 << 3;

        // color settings
        const COLOR_BACK    = 1 << 4;
        const COLOR_FORE    = 1 << 5;

        // colors
        const COLOR_BLACK   = 1 << 6;
        const COLOR_RED     = 1 << 7;
        const COLOR_GREEN   = 1 << 8;
        const COLOR_YELLOW  = 1 << 9;
        const COLOR_BLUE    = 1 << 10;
        const COLOR_MAGENTA = 1 << 11;
        const COLOR_CYAN    = 1 << 12;
        const COLOR_WHITE   = 1 << 13;
    }
}

pub struct Text {
    label: String,
    line: u16,
    flags: TextFlags,
    pos_resolve: bool,
    x_pos: u16,
    color: String,
}

impl Text {
    pub fn new(label: String, flags: TextFlags) -> Self {
        Text {
            label,
            line: 0,
            flags: flags.clone(),
            pos_resolve: false,
            x_pos: 0,
            color: Self::resolve_color(flags),
        }
    }

    #[inline]
    fn color_f_or_b(flags: TextFlags, b: &str, f: &str) -> String {
        if flags.contains(TextFlags::COLOR_BACK) { 
            String::from(b) 
        } else {
            String::from(f)
        }
    }

    fn resolve_color(flags: TextFlags) -> String {
        if flags.contains(TextFlags::COLOR_BLACK) {
            Self::color_f_or_b(flags, ansi::ESC_BLACK_B, ansi::ESC_BLACK_F)
        } else if flags.contains(TextFlags::COLOR_RED) {
            Self::color_f_or_b(flags, ansi::ESC_RED_B, ansi::ESC_RED_F)
        } else if flags.contains(TextFlags::COLOR_GREEN) {
            Self::color_f_or_b(flags, ansi::ESC_GREEN_B, ansi::ESC_GREEN_F)
        } else if flags.contains(TextFlags::COLOR_YELLOW) {
            Self::color_f_or_b(flags, ansi::ESC_YELLOW_B, ansi::ESC_YELLOW_F)
        } else if flags.contains(TextFlags::COLOR_BLUE) {
            Self::color_f_or_b(flags, ansi::ESC_BLUE_B, ansi::ESC_BLUE_F)
        } else if flags.contains(TextFlags::COLOR_MAGENTA) {
            Self::color_f_or_b(flags, ansi::ESC_MAGENTA_B, ansi::ESC_MAGENTA_F)
        } else if flags.contains(TextFlags::COLOR_CYAN) {
            Self::color_f_or_b(flags, ansi::ESC_CYAN_B, ansi::ESC_CYAN_F)
        } else if flags.contains(TextFlags::COLOR_WHITE) {
            Self::color_f_or_b(flags, ansi::ESC_WHITE_B, ansi::ESC_WHITE_F)
        } else {
            String::from("")
        }
    }

    pub fn set_line(&mut self, line: u16) {
        self.line = line;
    }

    pub fn line(&self) -> u16 {
        return self.line;
    }

    pub fn label(&self) -> &String {
        return &self.label;
    }

    pub fn color(&self) -> &String {
        return &self.color;
    }
}
