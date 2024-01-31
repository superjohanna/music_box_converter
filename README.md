# music_box_converter
> Converts MIDI files into music box scores that are ready to print.

![images/example_output_svg](/images/example_output.png)

This is a command-line program that converts MIDI files to scores that can be printed for any music box.

If you are unsure of how to create a MIDI file for this program to consume, I recommend [Musescore](https://musescore.org/), which is a free and open source music notation program that has a MIDI export function.

## Building

  - To build this just install rust via the [Rust toolchain installer](https://rustup.rs/)
  - Run 'cargo build --release'
  - The binary will be under './target/release/music_box_converter' (Path might vary depending on the OS)

You can also download a pre-compiled binary and run that.

## Usage

### Basic usage

```bash
# Convert midi (*.mid|*.midi) => svg (*.svg)
music_box_converter convert --input 'Path to your midi file' --output 'Path to your output folder'
```
### Commands

There are two commands for this:
```bash
music_box_converter convert
music_box_converter config
```

#### Convert

The convert option lets you convert a midi file to a svg file. To do this it need a couple of things:

 1. Your midi file
 2. An output directory
 3. A json file with your music box configuration
 4. A json file with general settings for svg writing

This project provides both files. More in the [Configuration](#configuration) part.  
There are the following options:

##### Required

  - -i, --input  \<FILE>  
  The input file to use.
  - -o, --output \<FILE>   
  The output directory to output to.

##### Optional

  - -s, --settings \<FILE> &emsp;&emsp;&emsp;&emsp;&emsp;&emsp; 
  Specifies which settings file to use.  
  [default: ./settings.json]
  - -b, --box \<FILE> &emsp;&emsp;&emsp;&emsp; &emsp; &emsp; &emsp;
  Specifies which box file to use.  
  [default: ./box.json]
  - -T, --track \<TRACK_NUMBER>&emsp;&emsp;
  Specifies which track from the midi file to use. Zero-based.  
  [default: 0]
  - -O, --midi-out &emsp; &emsp; &emsp; &emsp; &emsp;&emsp;&emsp;&emsp;When set outputs a midi file with transposed notes on one track and the original track.
  - -t, --transpose &emsp; &emsp; &emsp; &emsp; &emsp; &emsp; &emsp;
  Wether to transpose notes that can't normally be played.
  - -v, --verbose... &emsp; &emsp; &emsp; &emsp; &emsp; &emsp; &emsp;
  Increases verbosity. Can be used multiple times to raise log level.
  - -q, --quiet &emsp; &emsp; &emsp; &emsp; &emsp; &emsp; &emsp;&emsp;&emsp;
  No Output. Exclusive to verbosity
  - &emsp; &nbsp;--force &emsp; &emsp; &emsp; &emsp; &emsp; &emsp; &emsp;&emsp;&emsp;
  Allows to output into the current working directory.
  - -h, --help &emsp; &emsp; &emsp; &emsp; &emsp; &emsp; &emsp; &emsp;&emsp;
  Print help
  - -V, --version &emsp; &emsp; &emsp; &emsp; &emsp; &emsp; &emsp;&emsp;
  Print version

Short options that don't require a value can be put together
```bash
# Verbose is set to two, transpose is set and track number is 1
music_box_converter convert -i 'PATH' -o 'PATH' -vvtT 1
```

#### Config

The config option lets you edit a settings.json file with a gui editor. It only has three optional option:
  
  - -s, --settings \<FILE> &emsp; &emsp; &emsp; &emsp; &emsp;
  Specifies which settings file to use.  
  [default: ./settings.json]
  - -h, --help &emsp; &emsp; &emsp; &emsp; &emsp; &emsp; &emsp; &emsp; &emsp;
  Print help
  - -V, --version &emsp; &emsp; &emsp; &emsp; &emsp; &emsp; &emsp; &emsp;
  Print version

## Configuration

There are two files you can change to your liking. The first is the settings.json file.

### settings.json

This file is read by the convert program and includes settings primarly for svg writing. This file is best edited with the [config](#config) command and so this part is dedicated to the config editor.

![Picture of the editor](/images/editor_gui.png "Credit for the background: Alexander Grey")
The editor, Credit: [background image](https://www.pexels.com/photo/assorted-color-sequins-1191710/)


To change between settings use the up and down arrow keys or alternatively ^E and ^D respectively. You can use ^L to clear the line and ^S to open a pop up dialogue for saving and ^O to open a dialogue for opening a file. The path that is displayed will be the path that was used for opening or saving a file.

Note: Should the editor crash the terminal will be messed up. I recommend just creating a new instance. To fix this I somehow need to catch a panic and I don't really know how to do this just yet.

The editor includes tips and help for each item. Should you find that there aren't enough options and settings for you to tweak don't hesitate to write me an e-mail. I might take a while to respond because I don't read my e-mails frequently enough.



## Links
This was inspired by a similar project by starbeamrainbowlabs:
[Repo](https://git.starbeamrainbowlabs.com/sbrl/MusicBoxConverter/src/branch/main/),
[Blog](https://starbeamrainbowlabs.com/blog/article.php?article=posts%2F469-musicboxconverter.html)
