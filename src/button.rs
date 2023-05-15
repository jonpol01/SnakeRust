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

    pub fn show(&self, ctx: Context, g: &mut G2d) {
        let rect = rectangle::centered([self.x, self.y, self.w, self.h]);
        let text = Text::new_color([0.0, 0.0, 0.0, 1.0], 22);
        let text_width = text.width(self.text.as_str(), &mut Glyphs::new("arial", g.clone(), TextureSettings::new()).unwrap());
        let text_height = text.line_height() * 1.5; // Margins 

        rectangle([1.0, 1.0, 1.0, 1.0], rect, ctx.transform, g);
        rectangle([0.0, 0.0, 0.0, 1.0], rect, ctx.transform, g);
        text.draw(
            self.text.as_str(),
            &mut Glyphs::new("arial", g.clone(), TextureSettings::new()).unwrap(),
            &DrawState::new_alpha(),
            ctx.trans(self.x, self.y - text_height / 2.0).transform,
            g,
        )
        .unwrap();
    }
}