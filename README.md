# Turing Smart Screen CLI

This is a command line interface for the Turing Smart Screen.

## Examples

- `tss text -f MyFont.ttf -t "Hello World"` - Display "Hello World" on the screen using the font file MyFont.ttf
- `tss text -f MyFont.ttf -t "Hello World\nThis is a test."` - Display "Hello World" and "This is a test." on seperate lines on the screen using the font file MyFont.ttf
- `tss test` - Tests the connection to the screen
- `tss clear` - Clears the screen
- `tss draw -p <path>` - Draws the image at the specified path on the screen

## Additional parameters

All of these parameters are optional and go BEFORE the command.

- `-p` or `--portrait` - Display the text or image in portrait mode (default is landscape)
- `-b <number>` or `--brightness <number>` - Set the brightness of the screen (0-100)
