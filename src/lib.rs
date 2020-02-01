
use std::fs::read_to_string;
use std::fs::write;

/**
 * This allows a type to be used with Jata.
 * This makes sure that a file can be translated to a string.
 * It also needs to be able to parse a String into itself
 */
pub trait JataType where Self: std::marker::Sized {

	/**
	 * This takes a string and converts it to the type.
	 * Returns None if the String is not a correct representation of the type.
	 */
	fn read(representation: String) -> Option<Self>;

	/**
	 * Converts the type to a String.
	*/
	fn to_str(&self) -> String;
}


#[derive(Default)]
/**
 * This is a struct containing a path to a file.
 * It takes a generic type which defines what type the file contains.
 */
pub struct JataFile<T: JataType + Default> {

	/** A path to the file */
	path: String,

	/** What is currently believed to be the value */
	value: T
}

impl<T> JataFile<T> where T: JataType + Default {

	/**
	 * The default constructor which doesn't initiate any fields
	 */
	pub fn new() -> Self {
		Self::default()
	}

	/**
	 * Checks and returns the current value of the file.
	 * Returns None if there is an error when reading the file,
	 * or if the file isn't a valid representation of the type.
	 */
	pub fn check_value(self) -> Option<T> {
		match read_to_string(self.path) {
			Ok(s) => T::read(s),
			Err(_e) => None
		}
	}

	/**
	 * Checks the value of the file and sets the current value if applicable.
	 * Returns Some(()) if the check was successful.
	 * Return None if there was a problem with reading or parsing the file.
	 */
	pub fn reset_value(&mut self) -> Option<()> {
		match read_to_string(self.path.clone()) {
			Ok(s) => match T::read(s) {
				Some(t) => {self.value = t; Some(())},
				None => None
			},
			Err(_e) => None
		}
	}

	/**
	 * Sets the value and writes the value to the file.
	 * Returns Some(()) if the write was successful
	 * Returns None if there was an error in writing to the file
	 */
	pub fn set_value(&mut self, value: T) -> Option<()> {
		self.value = value;
		let string_rep = self.value.to_str();
		match write(self.path.clone(), string_rep) {
			Ok(_o) => Some(()),
			Err(_e) => None
		}
	}
}