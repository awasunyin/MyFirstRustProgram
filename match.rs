enum MachineState {
	Normal,
	Comment,
	Upper,
	Lower,
}

fn machine_cycle(state: &MachineState, character: char) -> (Option<char>, MachineState) {

	use std::ascii::AsciiExt; 

	match state {
		&MachineState::Normal => {
			match character {
				'#' => {
					return (None, MachineState::Comment)
				}

				'^' => {
					return (None, MachineState::Upper);
				}
				'_' => {
					return (None, MachineState::Lower);
				}
				_ => {
					//All above first, otherwise the rest
					//Wrapping it in an option
					return (Some(character), MachineState::Normal);
				}
			}
		}
		
		&MachineState::Comment => { 

			if character == '#' {

				return (None, MachineState::Normal);

			} else {

				return (None, MachineState::Comment);
			}
		}

		&MachineState::Upper => {

			if character == '^' {

				return (None, MachineState::Normal);

			} else {

				return (Some(character.to_ascii_uppercase()), MachineState::Upper);
			}
		}

		&MachineState::Lower => {

			if character == '_' {

				return (None, MachineState::Normal);

			} else {

				return (Some(character.to_ascii_lowercase()), MachineState::Lower);
			}
		}
	};
}

fn main() {

	let mut state = MachineState::Normal;
	let input = "this is some _Lower CASe_ code, #this is a comment, # ^and this is part is upper case^";
	let mut processed_string = String::new();

	for character in input.chars() {

		let (output, new_state) = machine_cycle(&state, character); 

		match output {

			Some(c) => {

				processed_string.push(c);
			}

			None => {

			}
		};

		state = new_state;
	}

	println!("Input:\t{}\nOutput:\t{}", input, processed_string);
}
