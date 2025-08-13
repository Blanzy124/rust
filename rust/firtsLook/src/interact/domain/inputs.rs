
pub struct LoginInput  
{
	input: String,
	error_input: String,
	any_error: bool
}

impl Default for LoginInput 
{
	fn default() -> Self{
		LoginInput {
			input: String::new(),
			error_input: String::new(),
			any_error: false
		}
	}
}

impl LoginInput 
{
	//SETTERS
	pub fn set_input(&mut self, input_: &str) 
	{	
		self.input = input_.to_string();

	}

	pub fn set_error(&mut self, error_: &str)
	{
		self.error_input = error_.to_string();
	}

	pub fn set_any_error(&mut self, set: bool)
	{
		self.any_error = set;
	}

	//GETTERS
	pub fn get_input(&self) -> &str
	{
		&self.input
	}

	pub fn get_error_input(&self) -> &str
	{
		&self.error_input
	}

	
	//METHODS
	pub fn reset_any_error(&mut self)
	{
		self.any_error = false;
	}

	pub fn check_any_error(&self) -> bool
	{

		self.any_error
	}
}

