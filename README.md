# music_box_converter
> Converts MIDI files into music box scores that are ready to print.

This is a command-line program that converts MIDI files to scores that can be printed for any music box.

If you are unsure of how to create a MIDI file for this program to consume, I recoomend [Musescore](https://musescore.org/), which is a free and open source music notation program that has a MIDI export function.

## Building
  -To build this just install rust via the [Rust toolchain installer](https://rustup.rs/)
  -Run 'cargo build --release'
  -The binary will be under '/target/release/music_box_converter' (Path might vary depending on the OS)

You can also download a pre-compiled binary and run that.

## Usage
```bash
#Convert MIDI => SVG
./music_box_converter --input song.midi --output /out/song
# Note: The program will create multiple pages so it is recommended to pass a folder path for the resulting svgs.
# Further Note: The program will automatically name them with the number i in this format: '{Your Path}_{The current number}.svg'. so you don't need to append .svg
```

I'll add more commands once I have this finished

## Links
This was inspired by a similar project by starbeamrainbowlabs:
[Git Repo](https://git.starbeamrainbowlabs.com/sbrl/MusicBoxConverter/src/branch/main/)
[Blog](https://starbeamrainbowlabs.com/blog/article.php?article=posts%2F469-musicboxconverter.html)
