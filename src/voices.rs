/*
 * Voice set definitions.
 * Names are from XGworks' `melody.ini`.
 */

#![allow(non_upper_case_globals)]

use std::fmt;
use std::ops;

pub struct Voice {
    msb: u8, // 0-based
    lsb: u8, // 0-based
    prg: u8, // 1-based
    name: &'static str,
}

pub struct Voices(&'static [&'static Voice]);

#[derive(Default)]
pub struct VoiceCounts {
	instruments: u16,
	kits: u16,
}

impl VoiceCounts {
	pub fn total(&self) -> u16 {
		self.instruments + self.kits
	}
}

impl ops::AddAssign for VoiceCounts {
	fn add_assign(&mut self, other: VoiceCounts) {
		self.instruments += other.instruments;
		self.kits += other.kits;
	}
}

impl fmt::Display for VoiceCounts {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			"Total number of voices: {} ({} instruments, {} kits)",
			self.total(), self.instruments, self.kits
		)
	}
}

impl Voices {
	pub fn voicecount(&self) -> VoiceCounts {
		let mut ret : VoiceCounts = Default::default();
		for it in self.0.iter() {
			match it.msb {
				127 => ret.kits += 1,
				_ => ret.instruments += 1,
			}
		}
		ret
	}
}

impl fmt::Display for Voice {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:03}-{:03}-{:03} {}", self.msb, self.lsb, self.prg, self.name)
	}
}

impl fmt::Display for Voices {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for it in self.0.iter() {
			try!(write!(f, "{}\n", it));
		}
		Ok(())
	}
}

// General MIDI (1991)
pub const GM: Voices = Voices(&[
	&Voice{msb:   0, lsb:   0, prg:   1, name: "Grand Piano"},
	&Voice{msb:   0, lsb:   0, prg:   2, name: "Bright Piano"},
	&Voice{msb:   0, lsb:   0, prg:   3, name: "Electric Grand Piano"},
	&Voice{msb:   0, lsb:   0, prg:   4, name: "Honky-tonk Piano"},
	&Voice{msb:   0, lsb:   0, prg:   5, name: "Electric Piano 1"},
	&Voice{msb:   0, lsb:   0, prg:   6, name: "Electric Piano 2"},
	&Voice{msb:   0, lsb:   0, prg:   7, name: "Harpsichord"},
	&Voice{msb:   0, lsb:   0, prg:   8, name: "Clavi"},
	&Voice{msb:   0, lsb:   0, prg:   9, name: "Celesta"},
	&Voice{msb:   0, lsb:   0, prg:  10, name: "Glockenspiel"},
	&Voice{msb:   0, lsb:   0, prg:  11, name: "Music Box"},
	&Voice{msb:   0, lsb:   0, prg:  12, name: "Vibraphone"},
	&Voice{msb:   0, lsb:   0, prg:  13, name: "Marimba"},
	&Voice{msb:   0, lsb:   0, prg:  14, name: "Xylophone"},
	&Voice{msb:   0, lsb:   0, prg:  15, name: "Tubular Bells"},
	&Voice{msb:   0, lsb:   0, prg:  16, name: "Dulcimer"},
	&Voice{msb:   0, lsb:   0, prg:  17, name: "Drawbar Organ"},
	&Voice{msb:   0, lsb:   0, prg:  18, name: "Percussive Organ"},
	&Voice{msb:   0, lsb:   0, prg:  19, name: "Rock Organ"},
	&Voice{msb:   0, lsb:   0, prg:  20, name: "Church Organ"},
	&Voice{msb:   0, lsb:   0, prg:  21, name: "Reed Organ"},
	&Voice{msb:   0, lsb:   0, prg:  22, name: "Accordion"},
	&Voice{msb:   0, lsb:   0, prg:  23, name: "Harmonica"},
	&Voice{msb:   0, lsb:   0, prg:  24, name: "Tango Accordion"},
	&Voice{msb:   0, lsb:   0, prg:  25, name: "Nylon Guitar"},
	&Voice{msb:   0, lsb:   0, prg:  26, name: "Steel Guitar"},
	&Voice{msb:   0, lsb:   0, prg:  27, name: "Jazz Guitar"},
	&Voice{msb:   0, lsb:   0, prg:  28, name: "Clean Guitar"},
	&Voice{msb:   0, lsb:   0, prg:  29, name: "Muted Guitar"},
	&Voice{msb:   0, lsb:   0, prg:  30, name: "Overdriven Guitar"},
	&Voice{msb:   0, lsb:   0, prg:  31, name: "Distortion Guitar"},
	&Voice{msb:   0, lsb:   0, prg:  32, name: "Guitar Harmonics"},
	&Voice{msb:   0, lsb:   0, prg:  33, name: "Acoustic Bass"},
	&Voice{msb:   0, lsb:   0, prg:  34, name: "Finger Bass"},
	&Voice{msb:   0, lsb:   0, prg:  35, name: "Pick Bass"},
	&Voice{msb:   0, lsb:   0, prg:  36, name: "Fretless Bass"},
	&Voice{msb:   0, lsb:   0, prg:  37, name: "Slap Bass 1"},
	&Voice{msb:   0, lsb:   0, prg:  38, name: "Slap Bass 2"},
	&Voice{msb:   0, lsb:   0, prg:  39, name: "Synth Bass 1"},
	&Voice{msb:   0, lsb:   0, prg:  40, name: "Synth Bass 2"},
	&Voice{msb:   0, lsb:   0, prg:  41, name: "Violin"},
	&Voice{msb:   0, lsb:   0, prg:  42, name: "Viola"},
	&Voice{msb:   0, lsb:   0, prg:  43, name: "Cello"},
	&Voice{msb:   0, lsb:   0, prg:  44, name: "Contrabass"},
	&Voice{msb:   0, lsb:   0, prg:  45, name: "Tremolo Strings"},
	&Voice{msb:   0, lsb:   0, prg:  46, name: "Pizzicato Strings"},
	&Voice{msb:   0, lsb:   0, prg:  47, name: "Orchestral Harp"},
	&Voice{msb:   0, lsb:   0, prg:  48, name: "Timpani"},
	&Voice{msb:   0, lsb:   0, prg:  49, name: "Strings 1"},
	&Voice{msb:   0, lsb:   0, prg:  50, name: "Strings 2"},
	&Voice{msb:   0, lsb:   0, prg:  51, name: "Synth Strings 1"},
	&Voice{msb:   0, lsb:   0, prg:  52, name: "Synth Strings 2"},
	&Voice{msb:   0, lsb:   0, prg:  53, name: "Choir Aahs "},
	&Voice{msb:   0, lsb:   0, prg:  54, name: "Voice Oohs"},
	&Voice{msb:   0, lsb:   0, prg:  55, name: "Synth Voice"},
	&Voice{msb:   0, lsb:   0, prg:  56, name: "Orchestra Hit"},
	&Voice{msb:   0, lsb:   0, prg:  57, name: "Trumpet "},
	&Voice{msb:   0, lsb:   0, prg:  58, name: "Trombone"},
	&Voice{msb:   0, lsb:   0, prg:  59, name: "Tuba"},
	&Voice{msb:   0, lsb:   0, prg:  60, name: "Muted Trumpet"},
	&Voice{msb:   0, lsb:   0, prg:  61, name: "French Horn"},
	&Voice{msb:   0, lsb:   0, prg:  62, name: "Brass Section"},
	&Voice{msb:   0, lsb:   0, prg:  63, name: "Synth Brass 1"},
	&Voice{msb:   0, lsb:   0, prg:  64, name: "Synth Brass 2"},
	&Voice{msb:   0, lsb:   0, prg:  65, name: "Soprano Sax"},
	&Voice{msb:   0, lsb:   0, prg:  66, name: "Alto Sax"},
	&Voice{msb:   0, lsb:   0, prg:  67, name: "Tenor Sax"},
	&Voice{msb:   0, lsb:   0, prg:  68, name: "Baritone Sax"},
	&Voice{msb:   0, lsb:   0, prg:  69, name: "Oboe"},
	&Voice{msb:   0, lsb:   0, prg:  70, name: "English Horn"},
	&Voice{msb:   0, lsb:   0, prg:  71, name: "Bassoon"},
	&Voice{msb:   0, lsb:   0, prg:  72, name: "Clarinet"},
	&Voice{msb:   0, lsb:   0, prg:  73, name: "Piccolo"},
	&Voice{msb:   0, lsb:   0, prg:  74, name: "Flute"},
	&Voice{msb:   0, lsb:   0, prg:  75, name: "Recorder"},
	&Voice{msb:   0, lsb:   0, prg:  76, name: "Pan Flute"},
	&Voice{msb:   0, lsb:   0, prg:  77, name: "Blown Bottle"},
	&Voice{msb:   0, lsb:   0, prg:  78, name: "Shakuhachi"},
	&Voice{msb:   0, lsb:   0, prg:  79, name: "Whistle"},
	&Voice{msb:   0, lsb:   0, prg:  80, name: "Ocarina"},
	&Voice{msb:   0, lsb:   0, prg:  81, name: "Square Lead"},
	&Voice{msb:   0, lsb:   0, prg:  82, name: "Sawtooth Lead"},
	&Voice{msb:   0, lsb:   0, prg:  83, name: "Calliope Lead"},
	&Voice{msb:   0, lsb:   0, prg:  84, name: "Chiff Lead"},
	&Voice{msb:   0, lsb:   0, prg:  85, name: "Charang Lead"},
	&Voice{msb:   0, lsb:   0, prg:  86, name: "Voice Lead"},
	&Voice{msb:   0, lsb:   0, prg:  87, name: "Fifths Lead"},
	&Voice{msb:   0, lsb:   0, prg:  88, name: "Bass & Lead"},
	&Voice{msb:   0, lsb:   0, prg:  89, name: "New Age Pad"},
	&Voice{msb:   0, lsb:   0, prg:  90, name: "Warm Pad"},
	&Voice{msb:   0, lsb:   0, prg:  91, name: "Poly Synth Pad"},
	&Voice{msb:   0, lsb:   0, prg:  92, name: "Choir Pad"},
	&Voice{msb:   0, lsb:   0, prg:  93, name: "Bowed Pad"},
	&Voice{msb:   0, lsb:   0, prg:  94, name: "Metallic Pad"},
	&Voice{msb:   0, lsb:   0, prg:  95, name: "Halo Pad"},
	&Voice{msb:   0, lsb:   0, prg:  96, name: "Sweep Pad"},
	&Voice{msb:   0, lsb:   0, prg:  97, name: "Rain"},
	&Voice{msb:   0, lsb:   0, prg:  98, name: "Sound Track"},
	&Voice{msb:   0, lsb:   0, prg:  99, name: "Crystal"},
	&Voice{msb:   0, lsb:   0, prg: 100, name: "Atmosphere"},
	&Voice{msb:   0, lsb:   0, prg: 101, name: "Brightness"},
	&Voice{msb:   0, lsb:   0, prg: 102, name: "Goblins"},
	&Voice{msb:   0, lsb:   0, prg: 103, name: "Echoes"},
	&Voice{msb:   0, lsb:   0, prg: 104, name: "Sci-Fi"},
	&Voice{msb:   0, lsb:   0, prg: 105, name: "Sitar"},
	&Voice{msb:   0, lsb:   0, prg: 106, name: "Banjo"},
	&Voice{msb:   0, lsb:   0, prg: 107, name: "Shamisen"},
	&Voice{msb:   0, lsb:   0, prg: 108, name: "Koto"},
	&Voice{msb:   0, lsb:   0, prg: 109, name: "Kalimba"},
	&Voice{msb:   0, lsb:   0, prg: 110, name: "Bagpipe"},
	&Voice{msb:   0, lsb:   0, prg: 111, name: "Fiddle"},
	&Voice{msb:   0, lsb:   0, prg: 112, name: "Shanai"},
	&Voice{msb:   0, lsb:   0, prg: 113, name: "Tinkle Bell"},
	&Voice{msb:   0, lsb:   0, prg: 114, name: "Agogo"},
	&Voice{msb:   0, lsb:   0, prg: 115, name: "Steel Drums"},
	&Voice{msb:   0, lsb:   0, prg: 116, name: "Woodblock"},
	&Voice{msb:   0, lsb:   0, prg: 117, name: "Taiko Drum"},
	&Voice{msb:   0, lsb:   0, prg: 118, name: "Melodic Tom"},
	&Voice{msb:   0, lsb:   0, prg: 119, name: "Synth Drum"},
	&Voice{msb:   0, lsb:   0, prg: 120, name: "Reverse Cymbal"},
	&Voice{msb:   0, lsb:   0, prg: 121, name: "Fret Noise"},
	&Voice{msb:   0, lsb:   0, prg: 122, name: "Breath Noise"},
	&Voice{msb:   0, lsb:   0, prg: 123, name: "Seashore"},
	&Voice{msb:   0, lsb:   0, prg: 124, name: "Bird Tweet"},
	&Voice{msb:   0, lsb:   0, prg: 125, name: "Telephone Ring"},
	&Voice{msb:   0, lsb:   0, prg: 126, name: "Helicopter"},
	&Voice{msb:   0, lsb:   0, prg: 127, name: "Applause"},
	&Voice{msb:   0, lsb:   0, prg: 128, name: "Gunshot"},
]);

// Additional drum kits from Roland GS (1991)
pub const GSDrums: &Voices = &Voices(&[
	&Voice{msb: 127, lsb:   0, prg:   1, name: "Standard Kit"},
	&Voice{msb: 127, lsb:   0, prg:   9, name: "Room Kit"},
	&Voice{msb: 127, lsb:   0, prg:  17, name: "Rock Kit"},
	&Voice{msb: 127, lsb:   0, prg:  25, name: "Electro Kit"},
	&Voice{msb: 127, lsb:   0, prg:  26, name: "Analog Kit"},
	&Voice{msb: 127, lsb:   0, prg:  33, name: "Jazz Kit"},
	&Voice{msb: 127, lsb:   0, prg:  41, name: "Brush Kit"},
	&Voice{msb: 127, lsb:   0, prg:  49, name: "Symphony Kit"},
]);
