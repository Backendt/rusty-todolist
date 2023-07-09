# Rusty-todolist
### Rusty-todolist is a CLI program made to manage notes.
Disclaimer: This is my first time using Rust. This project was made to learn the Rust language.

Add note: todo add <TITLE> <CONTENT>
e.g `todo add "Test note" "This is a test note."`

Remove note(s): todo delete <TITLE>
e.g `todo delete "Test note"`
e.g `todo --regex delete "Test.*"`

List notes: todo list

Read note(s): todo read <TITLE>
e.g `todo read "Test note"`
e.g `todo --regex read "*"`

### Troubleshooting
Notes are stored in "$HOME/.todos". The HOME environment variable has to be set.
