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
	sfx: u16,
	kits: u16,
}

impl VoiceCounts {
	pub fn total(&self) -> u16 {
		self.instruments + self.sfx + self.kits
	}
}

impl ops::AddAssign for VoiceCounts {
	fn add_assign(&mut self, other: VoiceCounts) {
		self.instruments += other.instruments;
		self.sfx += other.sfx;
		self.kits += other.kits;
	}
}

impl fmt::Display for VoiceCounts {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			"Total number of voices: {} ({} instruments, {} sound effects, {} kits)",
			self.total(), self.instruments, self.sfx, self.kits
		)
	}
}

impl Voices {
	pub fn voicecount(&self) -> VoiceCounts {
		let mut ret : VoiceCounts = Default::default();
		for it in self.0.iter() {
			match it.msb {
				127 => ret.kits += 1,
				64 => ret.sfx += 1,
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

// XG Level 1 (1994)
pub const XGLevel1: &Voices = &Voices(&[
	&Voice{msb:   0, lsb:   1, prg:   1, name: "Grand Piano KSP"},
	&Voice{msb:   0, lsb:  18, prg:   1, name: "Mellow Grand Piano"},
	&Voice{msb:   0, lsb:  40, prg:   1, name: "Piano Strings"},
	&Voice{msb:   0, lsb:  41, prg:   1, name: "Dream"},
	&Voice{msb:   0, lsb:   1, prg:   2, name: "Bright Piano KSP"},
	&Voice{msb:   0, lsb:   1, prg:   3, name: "Electric Grand Piano KSP"},
	&Voice{msb:   0, lsb:  32, prg:   3, name: "Detuned CP80"},
	&Voice{msb:   0, lsb:  40, prg:   3, name: "Layered CP 1"},
	&Voice{msb:   0, lsb:  41, prg:   3, name: "Layered CP 2"},
	&Voice{msb:   0, lsb:   1, prg:   4, name: "Honky-tonk Piano KSP"},
	&Voice{msb:   0, lsb:   1, prg:   5, name: "Electric Piano 1 KSP"},
	&Voice{msb:   0, lsb:  18, prg:   5, name: "Mellow Electric Piano 1"},
	&Voice{msb:   0, lsb:  32, prg:   5, name: "Chorus Electric Piano 1"},
	&Voice{msb:   0, lsb:  40, prg:   5, name: "Hard Electric Piano"},
	&Voice{msb:   0, lsb:  45, prg:   5, name: "Velocity Crossfade Electric Piano 1"},
	&Voice{msb:   0, lsb:  64, prg:   5, name: "60's Electric Piano 1"},
	&Voice{msb:   0, lsb:   1, prg:   6, name: "Electric Piano 2 KSP"},
	&Voice{msb:   0, lsb:  32, prg:   6, name: "Chorus Electric Piano 2"},
	&Voice{msb:   0, lsb:  33, prg:   6, name: "DX Electric Piano Hard"},
	&Voice{msb:   0, lsb:  34, prg:   6, name: "DX Legend"},
	&Voice{msb:   0, lsb:  40, prg:   6, name: "DX Phase Electric Piano"},
	&Voice{msb:   0, lsb:  41, prg:   6, name: "DX + Analog Electric Piano"},
	&Voice{msb:   0, lsb:  42, prg:   6, name: "DX Koto Electric Piano"},
	&Voice{msb:   0, lsb:  45, prg:   6, name: "Velocity Crossfade Electric Piano 2"},
	&Voice{msb:   0, lsb:   1, prg:   7, name: "Harpsichord KSP"},
	&Voice{msb:   0, lsb:  25, prg:   7, name: "Harpsichord 2"},
	&Voice{msb:   0, lsb:  35, prg:   7, name: "Harpsichord 3"},
	&Voice{msb:   0, lsb:   1, prg:   8, name: "Clavi KSP"},
	&Voice{msb:   0, lsb:  27, prg:   8, name: "Clavi Wah"},
	&Voice{msb:   0, lsb:  64, prg:   8, name: "Pulse Clavi"},
	&Voice{msb:   0, lsb:  65, prg:   8, name: "Pierce Clavi"},
	&Voice{msb:   0, lsb:  64, prg:  11, name: "Orgel"},
	&Voice{msb:   0, lsb:   1, prg:  12, name: "Vibraphone KSP"},
	&Voice{msb:   0, lsb:  45, prg:  12, name: "Hard Vibraphone"},
	&Voice{msb:   0, lsb:   1, prg:  13, name: "Marimba KSP"},
	&Voice{msb:   0, lsb:  64, prg:  13, name: "Sine Marimba"},
	&Voice{msb:   0, lsb:  96, prg:  13, name: "Balafon"},
	&Voice{msb:   0, lsb:  97, prg:  13, name: "Balimba"},
	&Voice{msb:   0, lsb:  98, prg:  13, name: "Log Drums"},
	&Voice{msb:   0, lsb:  96, prg:  15, name: "Church Bells"},
	&Voice{msb:   0, lsb:  97, prg:  15, name: "Carillon"},
	&Voice{msb:   0, lsb:  35, prg:  16, name: "Dulcimer 2"},
	&Voice{msb:   0, lsb:  96, prg:  16, name: "Cimbalom"},
	&Voice{msb:   0, lsb:  97, prg:  16, name: "Santur"},
	&Voice{msb:   0, lsb:  32, prg:  17, name: "Detuned Drawbar Organ"},
	&Voice{msb:   0, lsb:  33, prg:  17, name: "60's Drawbar Organ 1"},
	&Voice{msb:   0, lsb:  34, prg:  17, name: "60's Drawbar Organ 2"},
	&Voice{msb:   0, lsb:  35, prg:  17, name: "80's Drawbar Organ 1"},
	&Voice{msb:   0, lsb:  36, prg:  17, name: "Drawbar Organ 2"},
	&Voice{msb:   0, lsb:  37, prg:  17, name: "60's Drawbar Organ 3"},
	&Voice{msb:   0, lsb:  38, prg:  17, name: "Even Bar Organ"},
	&Voice{msb:   0, lsb:  40, prg:  17, name: "16+2\" 2/3 Organ"},
	&Voice{msb:   0, lsb:  64, prg:  17, name: "Organ Bass"},
	&Voice{msb:   0, lsb:  65, prg:  17, name: "70's Drawbar Organ 2"},
	&Voice{msb:   0, lsb:  66, prg:  17, name: "Cheezy Organ"},
	&Voice{msb:   0, lsb:  67, prg:  17, name: "Drawbar Organ 3"},
	&Voice{msb:   0, lsb:  24, prg:  18, name: "70's Percussive Organ 1"},
	&Voice{msb:   0, lsb:  32, prg:  18, name: "Detuned Percussive Organ"},
	&Voice{msb:   0, lsb:  33, prg:  18, name: "Light Organ"},
	&Voice{msb:   0, lsb:  37, prg:  18, name: "Percussive Organ 2"},
	&Voice{msb:   0, lsb:  64, prg:  19, name: "Rotary Organ"},
	&Voice{msb:   0, lsb:  65, prg:  19, name: "Slow Rotary"},
	&Voice{msb:   0, lsb:  66, prg:  19, name: "Fast Rotary"},
	&Voice{msb:   0, lsb:  32, prg:  20, name: "Church Organ 3"},
	&Voice{msb:   0, lsb:  35, prg:  20, name: "Church Organ 2"},
	&Voice{msb:   0, lsb:  40, prg:  20, name: "Notre Dame"},
	&Voice{msb:   0, lsb:  64, prg:  20, name: "Organ Flute"},
	&Voice{msb:   0, lsb:  65, prg:  20, name: "Tremolo Organ Flute"},
	&Voice{msb:   0, lsb:  40, prg:  21, name: "Puff Organ"},
	&Voice{msb:   0, lsb:  32, prg:  22, name: "Accord It"},
	&Voice{msb:   0, lsb:  32, prg:  23, name: "Harmonica 2"},
	&Voice{msb:   0, lsb:  64, prg:  24, name: "Tango Accordion 2"},
	&Voice{msb:   0, lsb:  16, prg:  25, name: "Nylon Guitar 2"},
	&Voice{msb:   0, lsb:  25, prg:  25, name: "Nylon Guitar 3"},
	&Voice{msb:   0, lsb:  43, prg:  25, name: "Velocity Guitar Harmonics"},
	&Voice{msb:   0, lsb:  96, prg:  25, name: "Ukulele"},
	&Voice{msb:   0, lsb:  16, prg:  26, name: "Steel Guitar 2"},
	&Voice{msb:   0, lsb:  35, prg:  26, name: "12-string Guitar"},
	&Voice{msb:   0, lsb:  40, prg:  26, name: "Nylon & Steel Guitar"},
	&Voice{msb:   0, lsb:  41, prg:  26, name: "Steel Guitar with Body Sound"},
	&Voice{msb:   0, lsb:  96, prg:  26, name: "Mandolin"},
	&Voice{msb:   0, lsb:  18, prg:  27, name: "Mellow Guitar"},
	&Voice{msb:   0, lsb:  32, prg:  27, name: "Jazz Amp"},
	&Voice{msb:   0, lsb:  96, prg:  27, name: "Pedal Steel Guitar"},
	&Voice{msb:   0, lsb:  32, prg:  28, name: "Chorus Guitar"},
	&Voice{msb:   0, lsb:  64, prg:  28, name: "Clean Guitar 2"},
	&Voice{msb:   0, lsb:  40, prg:  29, name: "Funk Guitar 1"},
	&Voice{msb:   0, lsb:  41, prg:  29, name: "Muted Steel Guitar"},
	&Voice{msb:   0, lsb:  43, prg:  29, name: "Funk Guitar 2"},
	&Voice{msb:   0, lsb:  45, prg:  29, name: "Jazz Man"},
	&Voice{msb:   0, lsb:  96, prg:  29, name: "Muted Distortion Guitar"},
	&Voice{msb:   0, lsb:  43, prg:  30, name: "Guitar Pinch"},
	&Voice{msb:   0, lsb:  12, prg:  31, name: "Distorted Rhythm Guitar"},
	&Voice{msb:   0, lsb:  24, prg:  31, name: "Distortion Guitar 2"},
	&Voice{msb:   0, lsb:  35, prg:  31, name: "Distortion Guitar 3"},
	&Voice{msb:   0, lsb:  36, prg:  31, name: "Power Guitar 2"},
	&Voice{msb:   0, lsb:  37, prg:  31, name: "Power Guitar 1"},
	&Voice{msb:   0, lsb:  38, prg:  31, name: "Distorted Fifths"},
	&Voice{msb:   0, lsb:  40, prg:  31, name: "Feedback Guitar"},
	&Voice{msb:   0, lsb:  41, prg:  31, name: "Feedback Guitar 2"},
	&Voice{msb:   0, lsb:  43, prg:  31, name: "Rock Rhythm Guitar 2"},
	&Voice{msb:   0, lsb:  45, prg:  32, name: "Rock Rhythm Guitar 1"},
	&Voice{msb:   0, lsb:  64, prg:  32, name: "Acoustic Harmonics"},
	&Voice{msb:   0, lsb:  65, prg:  32, name: "Guitar Feedback"},
	&Voice{msb:   0, lsb:  66, prg:  32, name: "Guitar Harmonics 2"},
	&Voice{msb:   0, lsb:  40, prg:  33, name: "Jazz Rhythm"},
	&Voice{msb:   0, lsb:  45, prg:  33, name: "Velocity Crossfade Upright Bass"},
	&Voice{msb:   0, lsb:  18, prg:  34, name: "Finger Dark"},
	&Voice{msb:   0, lsb:  27, prg:  34, name: "Flange Bass"},
	&Voice{msb:   0, lsb:  40, prg:  34, name: "Bass & Distorted Electric Guitar"},
	&Voice{msb:   0, lsb:  43, prg:  34, name: "Finger Slap Bass"},
	&Voice{msb:   0, lsb:  45, prg:  34, name: "Finger Bass 2"},
	&Voice{msb:   0, lsb:  64, prg:  34, name: "Jazzy Bass"},
	&Voice{msb:   0, lsb:  65, prg:  34, name: "Modulated Bass"},
	&Voice{msb:   0, lsb:  28, prg:  35, name: "Muted Pick Bass"},
	&Voice{msb:   0, lsb:  32, prg:  36, name: "Fretless Bass 2"},
	&Voice{msb:   0, lsb:  33, prg:  36, name: "Fretless Bass 3"},
	&Voice{msb:   0, lsb:  34, prg:  36, name: "Fretless Bass 4"},
	&Voice{msb:   0, lsb:  96, prg:  36, name: "Synth Fretless"},
	&Voice{msb:   0, lsb:  97, prg:  36, name: "Smooth Fretless"},
	&Voice{msb:   0, lsb:  27, prg:  37, name: "Resonant Slap"},
	&Voice{msb:   0, lsb:  32, prg:  37, name: "Punch Thumb Bass"},
	&Voice{msb:   0, lsb:  43, prg:  38, name: "Velocity Switch Slap"},
	&Voice{msb:   0, lsb:  18, prg:  39, name: "Synth Bass 1 Dark"},
	&Voice{msb:   0, lsb:  20, prg:  39, name: "Fast Resonant Bass"},
	&Voice{msb:   0, lsb:  24, prg:  39, name: "Acid Bass"},
	&Voice{msb:   0, lsb:  35, prg:  39, name: "Clavi Bass"},
	&Voice{msb:   0, lsb:  40, prg:  39, name: "Techno Synth Bass"},
	&Voice{msb:   0, lsb:  64, prg:  39, name: "Orbiter"},
	&Voice{msb:   0, lsb:  65, prg:  39, name: "Square Bass"},
	&Voice{msb:   0, lsb:  66, prg:  39, name: "Rubber Bass"},
	&Voice{msb:   0, lsb:  96, prg:  39, name: "Hammer"},
	&Voice{msb:   0, lsb:   6, prg:  40, name: "Mellow Synth Bass"},
	&Voice{msb:   0, lsb:  12, prg:  40, name: "Sequenced Bass"},
	&Voice{msb:   0, lsb:  18, prg:  40, name: "Click Synth Bass"},
	&Voice{msb:   0, lsb:  19, prg:  40, name: "Synth Bass 2 Dark"},
	&Voice{msb:   0, lsb:  32, prg:  40, name: "Smooth Synth Bass"},
	&Voice{msb:   0, lsb:  40, prg:  40, name: "Modular Synth Bass"},
	&Voice{msb:   0, lsb:  41, prg:  40, name: "DX Bass"},
	&Voice{msb:   0, lsb:  64, prg:  40, name: "X Wire Bass"},
	&Voice{msb:   0, lsb:   8, prg:  41, name: "Slow Violin"},
	&Voice{msb:   0, lsb:   8, prg:  45, name: "Slow Tremolo Strings"},
	&Voice{msb:   0, lsb:  40, prg:  45, name: "Suspense Strings"},
	&Voice{msb:   0, lsb:  40, prg:  47, name: "Yang Chin"},
	&Voice{msb:   0, lsb:   3, prg:  49, name: "Stereo Strings"},
	&Voice{msb:   0, lsb:   8, prg:  49, name: "Slow Strings"},
	&Voice{msb:   0, lsb:  24, prg:  49, name: "Arco Strings"},
	&Voice{msb:   0, lsb:  35, prg:  49, name: "60's Strings"},
	&Voice{msb:   0, lsb:  40, prg:  49, name: "Orchestra"},
	&Voice{msb:   0, lsb:  41, prg:  49, name: "Orchestra 2"},
	&Voice{msb:   0, lsb:  42, prg:  49, name: "Tremolo Orchestra"},
	&Voice{msb:   0, lsb:  45, prg:  49, name: "Velocity Strings"},
	&Voice{msb:   0, lsb:   3, prg:  50, name: "Stereo Slow Strings"},
	&Voice{msb:   0, lsb:   8, prg:  50, name: "Legato Strings"},
	&Voice{msb:   0, lsb:  40, prg:  50, name: "Warm Strings"},
	&Voice{msb:   0, lsb:  41, prg:  50, name: "Kingdom"},
	&Voice{msb:   0, lsb:  64, prg:  50, name: "70's Strings"},
	&Voice{msb:   0, lsb:  65, prg:  50, name: "String Ensemble 3"},
	&Voice{msb:   0, lsb:  27, prg:  51, name: "Resonant Strings"},
	&Voice{msb:   0, lsb:  64, prg:  51, name: "Synth Strings 4"},
	&Voice{msb:   0, lsb:  65, prg:  51, name: "Synth Strings 5"},
	&Voice{msb:   0, lsb:   3, prg:  53, name: "Stereo Choir"},
	&Voice{msb:   0, lsb:  16, prg:  53, name: "Choir Aahs 2"},
	&Voice{msb:   0, lsb:  32, prg:  53, name: "Mellow Choir"},
	&Voice{msb:   0, lsb:  40, prg:  53, name: "Choir Strings"},
	&Voice{msb:   0, lsb:  64, prg:  53, name: "Strings & Choir Aahs"},
	&Voice{msb:   0, lsb:  65, prg:  53, name: "Male Choir Aahs"},
	&Voice{msb:   0, lsb:  64, prg:  54, name: "Voice Doo"},
	&Voice{msb:   0, lsb:  96, prg:  54, name: "Voice Humming"},
	&Voice{msb:   0, lsb:  40, prg:  55, name: "Synth Voice 2"},
	&Voice{msb:   0, lsb:  41, prg:  55, name: "Choral"},
	&Voice{msb:   0, lsb:  64, prg:  55, name: "Analog Voice"},
	&Voice{msb:   0, lsb:  35, prg:  56, name: "Orchestra Hit 2"},
	&Voice{msb:   0, lsb:  64, prg:  56, name: "Impact"},
	&Voice{msb:   0, lsb:  65, prg:  56, name: "Brass Stab"},
	&Voice{msb:   0, lsb:  66, prg:  56, name: "Double Hit"},
	&Voice{msb:   0, lsb:  67, prg:  56, name: "Brass Stab 80"},
	&Voice{msb:   0, lsb:  16, prg:  57, name: "Trumpet 2"},
	&Voice{msb:   0, lsb:  17, prg:  57, name: "Bright Trumpet"},
	&Voice{msb:   0, lsb:  32, prg:  57, name: "Warm Trumpet"},
	&Voice{msb:   0, lsb:  96, prg:  57, name: "Flugel Horn"},
	&Voice{msb:   0, lsb:  18, prg:  58, name: "Trombone 2"},
	&Voice{msb:   0, lsb:  16, prg:  59, name: "Tuba 2"},
	&Voice{msb:   0, lsb:  64, prg:  60, name: "Muted Trumpet 2"},
	&Voice{msb:   0, lsb:   6, prg:  61, name: "French Horn Solo"},
	&Voice{msb:   0, lsb:  32, prg:  61, name: "French Horn 2"},
	&Voice{msb:   0, lsb:  37, prg:  61, name: "Horn Orchestra"},
	&Voice{msb:   0, lsb:  14, prg:  62, name: "Sforzando Brass"},
	&Voice{msb:   0, lsb:  35, prg:  62, name: "Trumpet & Trombone Section"},
	&Voice{msb:   0, lsb:  39, prg:  62, name: "Brass Fall"},
	&Voice{msb:   0, lsb:  40, prg:  62, name: "Brass Section 2"},
	&Voice{msb:   0, lsb:  41, prg:  62, name: "High Brass"},
	&Voice{msb:   0, lsb:  42, prg:  62, name: "Mellow Brass"},
	&Voice{msb:   0, lsb:  12, prg:  63, name: "Quack Brass"},
	&Voice{msb:   0, lsb:  20, prg:  63, name: "Resonant Synth Brass"},
	&Voice{msb:   0, lsb:  24, prg:  63, name: "Poly Brass"},
	&Voice{msb:   0, lsb:  27, prg:  63, name: "Synth Brass 3"},
	&Voice{msb:   0, lsb:  32, prg:  63, name: "Jump Brass"},
	&Voice{msb:   0, lsb:  45, prg:  63, name: "Analog Velocity Brass 1"},
	&Voice{msb:   0, lsb:  64, prg:  63, name: "Analog Brass 1"},
	&Voice{msb:   0, lsb:  18, prg:  64, name: "Soft Brass"},
	&Voice{msb:   0, lsb:  40, prg:  64, name: "Synth Brass 4"},
	&Voice{msb:   0, lsb:  41, prg:  64, name: "Choir Brass"},
	&Voice{msb:   0, lsb:  45, prg:  64, name: "Analog Velocity Brass 2"},
	&Voice{msb:   0, lsb:  64, prg:  64, name: "Analog Brass 2"},
	&Voice{msb:   0, lsb:  40, prg:  66, name: "Sax Section"},
	&Voice{msb:   0, lsb:  43, prg:  66, name: "Hyper Alto Sax"},
	&Voice{msb:   0, lsb:  40, prg:  67, name: "Breathy Tenor Sax"},
	&Voice{msb:   0, lsb:  41, prg:  67, name: "Soft Tenor Sax"},
	&Voice{msb:   0, lsb:  64, prg:  67, name: "Tenor Sax 2"},
	&Voice{msb:   0, lsb:  96, prg:  72, name: "Bass Clarinet"},
	&Voice{msb:   0, lsb:  64, prg:  76, name: "Pan Flute 2"},
	&Voice{msb:   0, lsb:  96, prg:  76, name: "Kawala"},
	&Voice{msb:   0, lsb:   6, prg:  81, name: "Square Lead 2"},
	&Voice{msb:   0, lsb:   8, prg:  81, name: "LM Square"},
	&Voice{msb:   0, lsb:  18, prg:  81, name: "Hollow"},
	&Voice{msb:   0, lsb:  19, prg:  81, name: "Shroud"},
	&Voice{msb:   0, lsb:  64, prg:  81, name: "Mellow"},
	&Voice{msb:   0, lsb:  65, prg:  81, name: "Solo Sine"},
	&Voice{msb:   0, lsb:  66, prg:  81, name: "Sine Lead"},
	&Voice{msb:   0, lsb:   6, prg:  82, name: "Sawtooth Lead 2"},
	&Voice{msb:   0, lsb:   8, prg:  82, name: "Thick Sawtooth"},
	&Voice{msb:   0, lsb:  18, prg:  82, name: "Dynamic Sawtooth"},
	&Voice{msb:   0, lsb:  19, prg:  82, name: "Digital Sawtooth"},
	&Voice{msb:   0, lsb:  20, prg:  82, name: "Big Lead"},
	&Voice{msb:   0, lsb:  24, prg:  82, name: "Heavy Synth"},
	&Voice{msb:   0, lsb:  25, prg:  82, name: "Waspy Synth"},
	&Voice{msb:   0, lsb:  40, prg:  82, name: "Pulse Sawtooth"},
	&Voice{msb:   0, lsb:  41, prg:  82, name: "Dr. Lead"},
	&Voice{msb:   0, lsb:  45, prg:  82, name: "Velocity Lead"},
	&Voice{msb:   0, lsb:  96, prg:  82, name: "Sequenced Analog"},
	&Voice{msb:   0, lsb:  64, prg:  83, name: "Vent Synth"},
	&Voice{msb:   0, lsb:  65, prg:  83, name: "Pure Lead"},
	&Voice{msb:   0, lsb:  64, prg:  84, name: "Rubby"},
	&Voice{msb:   0, lsb:  64, prg:  85, name: "Distorted Lead"},
	&Voice{msb:   0, lsb:  65, prg:  85, name: "Wire Lead"},
	&Voice{msb:   0, lsb:  24, prg:  86, name: "Synth Aahs"},
	&Voice{msb:   0, lsb:  64, prg:  86, name: "Vox Lead"},
	&Voice{msb:   0, lsb:  35, prg:  87, name: "Big Five"},
	&Voice{msb:   0, lsb:  16, prg:  88, name: "Big & Low"},
	&Voice{msb:   0, lsb:  64, prg:  88, name: "Fat & Perky"},
	&Voice{msb:   0, lsb:  65, prg:  88, name: "Soft Whirl"},
	&Voice{msb:   0, lsb:  64, prg:  89, name: "Fantasy"},
	&Voice{msb:   0, lsb:  16, prg:  90, name: "Thick Pad"},
	&Voice{msb:   0, lsb:  17, prg:  90, name: "Soft Pad"},
	&Voice{msb:   0, lsb:  18, prg:  90, name: "Sine Pad"},
	&Voice{msb:   0, lsb:  64, prg:  90, name: "Horn Pad"},
	&Voice{msb:   0, lsb:  65, prg:  90, name: "Rotary Strngs"},
	&Voice{msb:   0, lsb:  64, prg:  91, name: "Poly Pad 80"},
	&Voice{msb:   0, lsb:  65, prg:  91, name: "Click Pad"},
	&Voice{msb:   0, lsb:  66, prg:  91, name: "Analog Pad"},
	&Voice{msb:   0, lsb:  67, prg:  91, name: "Square Pad"},
	&Voice{msb:   0, lsb:  64, prg:  92, name: "Heaven"},
	&Voice{msb:   0, lsb:  65, prg:  92, name: "Light Pad"},
	&Voice{msb:   0, lsb:  66, prg:  92, name: "Itopia"},
	&Voice{msb:   0, lsb:  67, prg:  92, name: "CC Pad"},
	&Voice{msb:   0, lsb:  64, prg:  93, name: "Glacier"},
	&Voice{msb:   0, lsb:  65, prg:  93, name: "Glass Pad"},
	&Voice{msb:   0, lsb:  64, prg:  94, name: "Tine Pad"},
	&Voice{msb:   0, lsb:  65, prg:  94, name: "Pan Pad"},
	&Voice{msb:   0, lsb:  20, prg:  96, name: "Shwimmer"},
	&Voice{msb:   0, lsb:  27, prg:  96, name: "Converge"},
	&Voice{msb:   0, lsb:  64, prg:  96, name: "Polar Pad"},
	&Voice{msb:   0, lsb:  65, prg:  96, name: "Sweepy"},
	&Voice{msb:   0, lsb:  66, prg:  96, name: "Celestial"},
	&Voice{msb:   0, lsb:  45, prg:  97, name: "Clavi Pad"},
	&Voice{msb:   0, lsb:  64, prg:  97, name: "Harmo Rain"},
	&Voice{msb:   0, lsb:  65, prg:  97, name: "African Wind"},
	&Voice{msb:   0, lsb:  66, prg:  97, name: "Carib"},
	&Voice{msb:   0, lsb:  27, prg:  98, name: "Prologue"},
	&Voice{msb:   0, lsb:  64, prg:  98, name: "Ancestral"},
	&Voice{msb:   0, lsb:  65, prg:  98, name: "Rave"},
	&Voice{msb:   0, lsb:  12, prg:  99, name: "Synth Drum Comp"},
	&Voice{msb:   0, lsb:  14, prg:  99, name: "Popcorn"},
	&Voice{msb:   0, lsb:  18, prg:  99, name: "Tiny Bells"},
	&Voice{msb:   0, lsb:  35, prg:  99, name: "Round Glockenspiel"},
	&Voice{msb:   0, lsb:  40, prg:  99, name: "Glockenspiel Chimes"},
	&Voice{msb:   0, lsb:  41, prg:  99, name: "Clear Bells"},
	&Voice{msb:   0, lsb:  42, prg:  99, name: "Chorus Bells"},
	&Voice{msb:   0, lsb:  64, prg:  99, name: "Synth Mallet"},
	&Voice{msb:   0, lsb:  65, prg:  99, name: "Soft Crystal"},
	&Voice{msb:   0, lsb:  66, prg:  99, name: "Loud Glockenspiel"},
	&Voice{msb:   0, lsb:  67, prg:  99, name: "Christmas Bells"},
	&Voice{msb:   0, lsb:  68, prg:  99, name: "Vibraphone Bells"},
	&Voice{msb:   0, lsb:  69, prg:  99, name: "Digital Bells"},
	&Voice{msb:   0, lsb:  70, prg:  99, name: "Air Bells"},
	&Voice{msb:   0, lsb:  71, prg:  99, name: "Bell Harp"},
	&Voice{msb:   0, lsb:  72, prg:  99, name: "Gamelimba"},
	&Voice{msb:   0, lsb:  18, prg: 100, name: "Warm Atmosphere"},
	&Voice{msb:   0, lsb:  19, prg: 100, name: "Hollow Release"},
	&Voice{msb:   0, lsb:  40, prg: 100, name: "Nylon Electric Piano"},
	&Voice{msb:   0, lsb:  64, prg: 100, name: "Nylon Harp"},
	&Voice{msb:   0, lsb:  65, prg: 100, name: "Harp Vox"},
	&Voice{msb:   0, lsb:  66, prg: 100, name: "Atmosphere Pad"},
	&Voice{msb:   0, lsb:  67, prg: 100, name: "Planet"},
	&Voice{msb:   0, lsb:  64, prg: 101, name: "Fantasy Bells"},
	&Voice{msb:   0, lsb:  96, prg: 101, name: "Smokey"},
	&Voice{msb:   0, lsb:  64, prg: 102, name: "Goblins Synth"},
	&Voice{msb:   0, lsb:  65, prg: 102, name: "Creeper"},
	&Voice{msb:   0, lsb:  66, prg: 102, name: "Ring Pad"},
	&Voice{msb:   0, lsb:  67, prg: 102, name: "Ritual"},
	&Voice{msb:   0, lsb:  68, prg: 102, name: "To Heaven"},
	&Voice{msb:   0, lsb:  69, prg: 102, name: "Milky Way"},
	&Voice{msb:   0, lsb:  70, prg: 102, name: "Night"},
	&Voice{msb:   0, lsb:  71, prg: 102, name: "Glisten"},
	&Voice{msb:   0, lsb:  72, prg: 102, name: "Puffy"},
	&Voice{msb:   0, lsb:  96, prg: 102, name: "Bell Choir"},
	&Voice{msb:   0, lsb:   8, prg: 103, name: "Echoes 2"},
	&Voice{msb:   0, lsb:  14, prg: 103, name: "Echo Pan"},
	&Voice{msb:   0, lsb:  64, prg: 103, name: "Echo Bells"},
	&Voice{msb:   0, lsb:  65, prg: 103, name: "Big Pan"},
	&Voice{msb:   0, lsb:  66, prg: 103, name: "Synth Piano"},
	&Voice{msb:   0, lsb:  67, prg: 103, name: "Creation"},
	&Voice{msb:   0, lsb:  68, prg: 103, name: "Star Dust"},
	&Voice{msb:   0, lsb:  69, prg: 103, name: "Resonant & Panning"},
	&Voice{msb:   0, lsb:  64, prg: 104, name: "Starz"},
	&Voice{msb:   0, lsb:  65, prg: 104, name: "Odin"},
	&Voice{msb:   0, lsb:  32, prg: 105, name: "Detuned Sitar"},
	&Voice{msb:   0, lsb:  35, prg: 105, name: "Sitar 2"},
	&Voice{msb:   0, lsb:  96, prg: 105, name: "Tambra"},
	&Voice{msb:   0, lsb:  97, prg: 105, name: "Tamboura"},
	&Voice{msb:   0, lsb:  28, prg: 106, name: "Muted Banjo"},
	&Voice{msb:   0, lsb:  96, prg: 106, name: "Rabab"},
	&Voice{msb:   0, lsb:  97, prg: 106, name: "Gopichant"},
	&Voice{msb:   0, lsb:  98, prg: 106, name: "Oud"},
	&Voice{msb:   0, lsb:  96, prg: 107, name: "Tsugaru"},
	&Voice{msb:   0, lsb:  96, prg: 108, name: "Taisho-kin"},
	&Voice{msb:   0, lsb:  97, prg: 108, name: "Kanoon"},
	&Voice{msb:   0, lsb:  64, prg: 109, name: "Big Kalimba"},
	&Voice{msb:   0, lsb:  64, prg: 112, name: "Shanai 2"},
	&Voice{msb:   0, lsb:  96, prg: 112, name: "Pungi"},
	&Voice{msb:   0, lsb:  97, prg: 112, name: "Hichiriki"},
	&Voice{msb:   0, lsb:  96, prg: 113, name: "Bonang"},
	&Voice{msb:   0, lsb:  97, prg: 113, name: "Altair"},
	&Voice{msb:   0, lsb:  98, prg: 113, name: "Gamelan Gongs"},
	&Voice{msb:   0, lsb:  99, prg: 113, name: "Stereo Gamelan Gongs"},
	&Voice{msb:   0, lsb: 100, prg: 113, name: "Rama Cymbal"},
	&Voice{msb:   0, lsb: 101, prg: 113, name: "Asian Bells"},
	&Voice{msb:   0, lsb:  96, prg: 114, name: "Atarigane"},
	&Voice{msb:   0, lsb:  96, prg: 115, name: "Tablas"},
	&Voice{msb:   0, lsb:  97, prg: 115, name: "Glass Percussion"},
	&Voice{msb:   0, lsb:  98, prg: 115, name: "Thai Bells"},
	&Voice{msb:   0, lsb:  96, prg: 116, name: "Castanets"},
	&Voice{msb:   0, lsb:  96, prg: 117, name: "Gran Cassa"},
	&Voice{msb:   0, lsb:  64, prg: 118, name: "Melodic Tom 2"},
	&Voice{msb:   0, lsb:  65, prg: 118, name: "Real Tom"},
	&Voice{msb:   0, lsb:  66, prg: 118, name: "Rock Tom"},
	&Voice{msb:   0, lsb:  64, prg: 119, name: "Analog Tom"},
	&Voice{msb:   0, lsb:  65, prg: 119, name: "Electro Percussion"},
	&Voice{msb:   0, lsb:  64, prg: 120, name: "Reverse Cymbal 2"},
	&Voice{msb:   0, lsb:  96, prg: 120, name: "Reverse Snare 1"},
	&Voice{msb:   0, lsb:  97, prg: 120, name: "Reverse Snare 2"},
	&Voice{msb:   0, lsb:  98, prg: 120, name: "Reverse Kick 1"},
	&Voice{msb:   0, lsb:  99, prg: 120, name: "Reverse Concert Bass Drum 1"},
	&Voice{msb:   0, lsb: 100, prg: 120, name: "Reverse Tom 1"},
	&Voice{msb:   0, lsb: 101, prg: 120, name: "Reverse Tom 2"},
	&Voice{msb:  64, lsb:   0, prg:   1, name: "Cutting Noise"},
	&Voice{msb:  64, lsb:   0, prg:   2, name: "Cutting Noise 2"},
	&Voice{msb:  64, lsb:   0, prg:   3, name: "Distorted Cutting Noise"},
	&Voice{msb:  64, lsb:   0, prg:   4, name: "String Slap"},
	&Voice{msb:  64, lsb:   0, prg:   5, name: "Bass Slide"},
	&Voice{msb:  64, lsb:   0, prg:   6, name: "Pick Scrape"},
	&Voice{msb:  64, lsb:   0, prg:  17, name: "Flute Key Click"},
	&Voice{msb:  64, lsb:   0, prg:  33, name: "Shower"},
	&Voice{msb:  64, lsb:   0, prg:  34, name: "Thunder"},
	&Voice{msb:  64, lsb:   0, prg:  35, name: "Wind"},
	&Voice{msb:  64, lsb:   0, prg:  36, name: "Stream"},
	&Voice{msb:  64, lsb:   0, prg:  37, name: "Bubble"},
	&Voice{msb:  64, lsb:   0, prg:  38, name: "Feed"},
	&Voice{msb:  64, lsb:   0, prg:  49, name: "Dog"},
	&Voice{msb:  64, lsb:   0, prg:  50, name: "Horse"},
	&Voice{msb:  64, lsb:   0, prg:  51, name: "Bird Tweet 2"},
	&Voice{msb:  64, lsb:   0, prg:  52, name: "Kitty"},
	&Voice{msb:  64, lsb:   0, prg:  53, name: "Growl"},
	&Voice{msb:  64, lsb:   0, prg:  54, name: "Haunted"},
	&Voice{msb:  64, lsb:   0, prg:  55, name: "Ghost"},
	&Voice{msb:  64, lsb:   0, prg:  56, name: "Maou"},
	&Voice{msb:  64, lsb:   0, prg:  65, name: "Phone Call"},
	&Voice{msb:  64, lsb:   0, prg:  66, name: "Door Squeak"},
	&Voice{msb:  64, lsb:   0, prg:  67, name: "Door Slam"},
	&Voice{msb:  64, lsb:   0, prg:  68, name: "Scratch Cut"},
	&Voice{msb:  64, lsb:   0, prg:  69, name: "Scratch Split"},
	&Voice{msb:  64, lsb:   0, prg:  70, name: "Wind Chime"},
	&Voice{msb:  64, lsb:   0, prg:  71, name: "Telephone Ring 2"},
	&Voice{msb:  64, lsb:   0, prg:  81, name: "Car Engine Ignition"},
	&Voice{msb:  64, lsb:   0, prg:  82, name: "Car Tires Squeal"},
	&Voice{msb:  64, lsb:   0, prg:  83, name: "Car Passing"},
	&Voice{msb:  64, lsb:   0, prg:  84, name: "Car Crash"},
	&Voice{msb:  64, lsb:   0, prg:  85, name: "Siren"},
	&Voice{msb:  64, lsb:   0, prg:  86, name: "Train"},
	&Voice{msb:  64, lsb:   0, prg:  87, name: "Jet Plane"},
	&Voice{msb:  64, lsb:   0, prg:  88, name: "Starship"},
	&Voice{msb:  64, lsb:   0, prg:  89, name: "Burst"},
	&Voice{msb:  64, lsb:   0, prg:  90, name: "Roller Coaster"},
	&Voice{msb:  64, lsb:   0, prg:  91, name: "Submarine"},
	&Voice{msb:  64, lsb:   0, prg:  97, name: "Laugh"},
	&Voice{msb:  64, lsb:   0, prg:  98, name: "Scream"},
	&Voice{msb:  64, lsb:   0, prg:  99, name: "Punch"},
	&Voice{msb:  64, lsb:   0, prg: 100, name: "Heartbeat"},
	&Voice{msb:  64, lsb:   0, prg: 101, name: "Footsteps"},
	&Voice{msb:  64, lsb:   0, prg: 102, name: "Applause"},
	&Voice{msb:  64, lsb:   0, prg: 113, name: "MachineGun"},
	&Voice{msb:  64, lsb:   0, prg: 114, name: "Laser Gun"},
	&Voice{msb:  64, lsb:   0, prg: 115, name: "Explosion"},
	&Voice{msb:  64, lsb:   0, prg: 116, name: "Firework"},
	&Voice{msb: 126, lsb:   0, prg:   1, name: "SFX Kit 1"},
	&Voice{msb: 126, lsb:   0, prg:   2, name: "SFX Kit 2"},
]);
