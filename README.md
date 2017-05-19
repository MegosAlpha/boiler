# Boiler - Multi-language Code Preprocessor &amp; Module Loader
Have some code that's split into modules, but you don't want to copy-paste, deal with config, etc.?
Boiler to the rescue! Currently at version 1.3.1. It even supports languages as in English. It's as easy as:
```boiler
boil <module-name>
```

## Modules
Modules aren't anything that special. ```boil <module-name>``` just loads according to the module loading path section below and puts it in the name.boiled.ext file. It checks relative paths, then the home path (~/.boiler/). Since 1.2 we have added extended irregular modules, which are just modules without the .boil extension.

## Module Loading Paths
If it starts with a / it's from the root on macOS or Linux. If it starts with C:/ or the drive, it is the drive root on Windows. Otherwise, it checks from the running directory.
1) working directory + '/recipes/' + path + '.boil' (1.3.1)
2) working directory + '/recipes/' + path (irregular modules loaded here) (1.3.1)
3) path + '.boil' (1.1)
4) path (irregular modules loaded here) (1.2)
5) home directory + '/.boiler/' + path + '.boil' (1.1)

## Using the tool
Run ```boiler <filename>``` to boil a file or ```boiler``` to run a file list as specified in boiler.files.txt.

## Directory Layout
README.md, LICENSE, Cargo.toml, Cargo.lock, and .gitignore are all just repository specific. They handle version numbers to an extent, information, and some git rules.
src/ contains the source folder. As of 1.3.1, it is still just main.rs. That's all the code.
recipes/ contain 2 regular modules and 1 irregular module. They are used for the examples.
alertme.js, example.txt, and nametag.cpfffnajs are the examples. They use the recipe files for boiling.
boiler.config.toml is the configuration for the examples in here.
boiler.files.txt is the filelist of what to do without command line arguments.

## FAQ
### What is going on metaboil.boil?
Version 1.2 (metaboiling and irregular modules) and higher cause files to be boiled when loaded. Do NOT let a file boil itself. It'll overflow memory pretty quick, because boiler is really fast. Metaboil is including other boiler files.
### How fast is this?
A: A lot quicker then you'd think. On my 2014 Intel i5 2-Core 1.4 base GHz computer running Ubuntu 17.04, the examples in 1.3.1 clock in at **0.005-0.006** seconds total using the time tool in release mode (runs with optimizations in release). In debug mode, it's about 0.01 seconds. So it's lightning fast. 0.006ish seconds includes metaboiling, opening ~7 files, reading all files into memory and back out, interpolation, loops, configuration and a bunch of references and data type conversions.
### What is a .cpfffnajs?
A fake file format I made for fun. It's really just a joke. You can find its repository [here](https://github.com/MegosAlpha/cpfffnajs).

## Changelog
1.3.1 (Latest):
- Project root search path implemented (/recipes).

1.3:
- Boiling configuration / Configured Metaboiling support (boiler.config.toml).

1.2:
- Metaboiling added
- Irregular modules added
- Metaboiling and Irregular Modules examples added

1.1:
- Filelist Support added
- Filelist and more examples

1.0:
- Regular interpolation of boil files.
- Regular modules.
- Standard paths (.boiler and relative path).
- boil syntax
- Boiler tool
- Examples

0.0.1 (Prototype):
- No features implemented (yet), just the scaffolding for the project

## Coming Soon
Configured Metaboiling - Boiled in from a config file instead of the filesystem, perhaps in TOML. **now in v1.3!**
Project root search path - A path for an entire directory. **now in v1.3.1**