use vizia::prelude::*;
use crate::{events::AppEvent, logic::generate_password};

#[derive(Lens)]
pub struct AppData{
	pub password: String,
	pub password_length: String,
	pub special_chars: String,
	pub numeric_chars: String
}

impl Model for AppData {
	fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
		event.map(|app_event, _| match app_event {
			AppEvent::SetNumericChars(length) => {
				if let Ok(n) = length.clone().trim().parse::<i32>(){
					self.numeric_chars = n.to_string();
				} else if length.is_empty() {
					self.numeric_chars = "0".to_string()
				}
			},
			
			AppEvent::SetPasswordLength(length) => {
				if let Ok(n) = length.clone().trim().parse::<i32>(){
					self.password_length = n.to_string();
				} else if length.is_empty() {
					self.password_length = "0".to_string()
				}
			},
			
			AppEvent::SetSpecialChars(length) => {
				if let Ok(n) = length.clone().trim().parse::<i32>(){
					self.special_chars = n.to_string();
				} else if length.is_empty() {
					self.special_chars = "0".to_string()
				}
			}

			AppEvent::Generate => {
				let mut password_length = 0;
				if let Ok(n) = self.password_length.clone().parse::<i32>() {
					password_length = n;
				}

				let mut special_chars: i32 = 0;				
				if let Ok(n) = self.special_chars.clone().parse::<i32>() {
					special_chars = n;
				}

				let mut numeric_chars: i32 = 0;
				
				if let Ok(n) = self.numeric_chars.clone().parse::<i32>() {
					numeric_chars = n;
				}

				
				if (special_chars + numeric_chars) > password_length || password_length == 0{
					println!("Password length must be greater than amount of special characters and digits");
					return
				}
				
				self.password = generate_password(password_length, special_chars, numeric_chars)
			}
		})
	}
}