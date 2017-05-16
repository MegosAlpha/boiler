# Boiler - Multi-language Code Preprocessor &amp; Module Loader

Have some code that's split into modules, but you don't want to copy-paste, deal with config, etc.?
Boiler to the rescue! Currently at version 1.1. It even supports non-programming languages (ex. English). It's as easy as:
```
boil <module-name>
```

## Modules
Modules aren't anything that special. ```boil <module-name>``` just loads whatever the name is + .boil and puts it in the 
.boiled file. It checks relative paths, then the home path (~/.boiler/).

## Using the tool
Run ```boiler <filename>``` to boil a file or ```boiler``` to run a file list as specified in boiler.files.txt.