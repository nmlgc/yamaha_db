use std::fmt;

use super::voices::*;

pub struct Model {
    name: &'static str,
	year: u16,
	voicesets: &'static [&'static Voices]
}

impl fmt::Display for Model {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(writeln!(f, "Yamaha {} ({}):", self.name, self.year));
		for it in self.voicesets {
			try!(write!(f, "{}", it));
		}
		Ok(())
	}
}

pub static MU5: Model = Model {
	name: "MU5", year: 1994, voicesets: &[&GM, &GSDrums]
};
