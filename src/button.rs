use piston_window::*;

pub struct win_Button {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    text: String,
}

impl win_Button {
    pub fn new(x: f64, y: f64, w: f64, h: f64, t: &str) -> Self {
        Self {
            x,
            y,
            w,
            h,
            text: t.to_string(),
        }
    }

    pub fn collide(&self, x: f64, y: f64) -> bool {
        x >= self.x - self.w / 2.0
            && x <= self.x + self.w / 2.0
            && y >= self.y - self.h / 2.0
            && y <= self.y + self.h / 2.0
    }

    pub fn show(&self, ctx: Context, gfx: &mut G2d, glyphs: &mut Glyphs) {
        let text_width = self
            .text
            .chars()
            .map(|c| glyphs.character(c).unwrap().h_metrics().advance_width * 1.2)
            .sum::<f64>();
        let text_height = glyphs.line_height();
        let rect = rectangle::centered([self.x, self.y, self.w, self.h]);

        rectangle([1.0; 4], rect, ctx.transform, gfx);
        rectangle(
            [0.0, 0.0, 0.0, 1.0],
            rect.border(1.0),
            ctx.transform,
            gfx,
        );

        text::Text::new_color([0.0; 4], 18)
            .draw(
                &self.text,
                glyphs,
                &draw_state::DrawState::new_alpha(),
                ctx.trans(self.x - text_width / 2.0, self.y + text_height / 2.0)
                    .transform,
                gfx,
            )
            .unwrap();
    }
}