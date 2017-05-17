# Boiler - Multi-language Code Preprocessor &amp; Module Loader
Have some code that's split into modules, but you don't want to copy-paste, deal with config, etc.?
Boiler to the rescue! Currently at version 1.2. It even supports non-programming languages (ex. English). It's as easy as:
```bash
boil <module-name>
```

## Modules
Modules aren't anything that special. ```boil <module-name>``` just loads whatever the name is + .boil and puts it in the 
.boiled file. It checks relative paths, then the home path (~/.boiler/). Since 1.2 we have added extended irregular 
modules. Example: boil LICENSE would first check the relative path for LICENSE.boil, then the global for LICENSE.boil, 
then the relative for LICENSE. We recommend regular modules to avoid naming conflicts.

## Using the tool
Run ```boiler <filename>``` to boil a file or ```boiler``` to run a file list as specified in boiler.files.txt.

## FAQ
### What are example.txt or crab.boil etc?
They are examples. You can test most of boiler by trying the examples. There is a file list, so running boiler will run
the examples.
### What is metaboil.boil?
As of version 1.2 (metaboiling and irregular modules), loaded files are boiled when loaded. Do NOT let a file boil itself. It'll overflow memory pretty quick, because boiler is really fast.
### How fast is this?
A: A lot quicker then you'd think. On my 2014 Intel i5 2-Core 1.4 base GHz computer running Ubuntu 17.04, the examples in 1.2 clock in at **0.005** seconds total using the time tool in release mode (runs with optimizations in release). In debug mode through cargo run (compilation checking overhead as well), it's about 0.102 seconds. So it's lightning fast. 0.005 includes metaboiling, opening ~7 files, reading all files into memory and back out, interpolation, and a bunch of references and data type conversions.

## Changelog
1.2 (Latest):
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
Maybe Configured Metaboiling - Boiled in from a config file instead of the filesystem, perhaps in TOML.