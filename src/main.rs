
use quicksilver::{
	geom::Vector,
	graphics::{Color, Font, FontStyle, Image},
	lifecycle::{run, Asset, Settings, State, Window},
	Future, Result,
};

struct Game {
     title: Asset<Image>,
     monoki_font_info: Asset<Image>,

}

impl State for Game {
   fn new() -> Result<Self> {
	let font_monoki = "mononoki-Regular.ttf";
	
	let title = Asset::new(Font::load(font_monoki).and_then(|font| {
	    font.render("Bird's Roguelike", &FontStyle::new(72.0, Color::BLACK))
		}));
	let monoki_font_info = Asset::new(Font::load(font_monoki).and_then(|font| {
	    font.render("Mononoki font by Matthias Tellen, terms: SIL Open Font Licsense 1.1",
			&FontStyle::new(20.0, Color::BLACK),
		)
	  }));
	
	  Ok(Self{
	      title,
	      monoki_font_info,
})
}
	fn update(&mut self, window: &mut Window) -> Result<()> {
	   Ok(())
}
	fn draw(&mut self, window: &mut Window) -> Result<()> {
	   Ok(())
}
}


fn main() {
    let settings = Settings {
	..Default::default()};
	run::<Game>("Bird's Roguelike", Vector::new(800, 600), settings);
}
