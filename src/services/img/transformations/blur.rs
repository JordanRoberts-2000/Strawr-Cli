use crate::services::img::Img;

impl Img {
    pub fn blur(&mut self, blur: u8) {
        self.img = self.img.blur(blur as f32);
    }
}
