use vizia::prelude::*;

use crate::{data::AppData, events::AppEvent};

pub fn ui_builder(cx: &mut Context) {
	VStack::new(cx, |cx| {
		HStack::new(cx, |cx| {

			VStack::new(cx, |cx| {
				Label::new(cx, "Password Length");
				Textbox::new(cx, AppData::password_length).on_edit(move |cx, text| {
					cx.emit(AppEvent::SetPasswordLength(text.clone()))
				});
			});

			VStack::new(cx, |cx| {
				Label::new(cx, "Number of Digits");
				Textbox::new(cx, AppData::numeric_chars).on_edit(move |cx, text| {
					cx.emit(AppEvent::SetNumericChars(text.clone()))
				});
			});
			
			VStack::new(cx, |cx| {
				Label::new(cx, "Number of Special characters");
				Textbox::new(cx, AppData::special_chars).on_edit(move |cx, text| {
					cx.emit(AppEvent::SetSpecialChars(text.clone()))
				});
			});

			Button::new(cx, |cx| cx.emit(AppEvent::Generate), |cx| Label::new(cx, "Generate")).class("button");
		});
		HStack::new(cx, |cx| {
			Label::new(cx, AppData::password.clone()).class("password");
		}).child_space(Stretch(1.0)).class("password-container");
	}).class("outer");
}