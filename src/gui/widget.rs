pub trait Widget {
    fn width(&self) -> usize;
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        print!("{}", buffer);
    }
}
