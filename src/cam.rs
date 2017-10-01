use screen::{Screen, Drawable};
use vec2::Vec2;

pub struct Cam {
    pub pos: Vec2,
}

impl Drawable for Cam {
    fn render(&self) -> Screen {
        let mut target = Screen::new();

        let (x, y) = target.project(&self.pos);
        // ##p##
        // #   #
        //  # #
        // #####
        // #   #
        // #   #
        // #   #
        // #####

        // ##p##
        target.put(x - 2, y, [0, 0, 0]);
        target.put(x - 1, y, [0, 0, 0]);
        target.put(x, y, [0, 0, 0]);
        target.put(x + 1, y, [0, 0, 0]);
        target.put(x + 2, y, [0, 0, 0]);

        // #   #
        target.put(x - 2, y + 1, [0, 0, 0]);
        target.put(x + 2, y + 1, [0, 0, 0]);

        //  # #
        target.put(x - 1, y + 2, [0, 0, 0]);
        target.put(x + 1, y + 2, [0, 0, 0]);

        // #####
        target.put(x - 2, y + 3, [0, 0, 0]);
        target.put(x - 1, y + 3, [0, 0, 0]);
        target.put(x, y + 3, [0, 0, 0]);
        target.put(x + 1, y + 3, [0, 0, 0]);
        target.put(x + 2, y + 3, [0, 0, 0]);

        // #   #
        target.put(x - 2, y + 4, [0, 0, 0]);
        target.put(x + 2, y + 4, [0, 0, 0]);

        // #   #
        target.put(x - 2, y + 5, [0, 0, 0]);
        target.put(x + 2, y + 5, [0, 0, 0]);

        // #   #
        target.put(x - 2, y + 6, [0, 0, 0]);
        target.put(x + 2, y + 6, [0, 0, 0]);

        // #####
        target.put(x - 2, y + 7, [0, 0, 0]);
        target.put(x - 1, y + 7, [0, 0, 0]);
        target.put(x, y + 7, [0, 0, 0]);
        target.put(x + 1, y + 7, [0, 0, 0]);
        target.put(x + 2, y + 7, [0, 0, 0]);

        target
    }
}
