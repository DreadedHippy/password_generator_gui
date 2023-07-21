pub enum AppEvent {
	Generate,
	SetPasswordLength(String),
	SetSpecialChars(String),
	SetNumericChars(String)
}