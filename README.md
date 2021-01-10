## Kafo

Kafo is an auto sorter for your downloads directory.

### Usage

`config.json` is the configuration file for this application. An example can be found under the name `example.config.json`. Make sure this is renamed to `config.json` when done and in the same directory as the executable. The keys can be found below: 

```
path          : string   : the path to your downloads directory
make_folders  : boolean  : wether or not the program should make the organization folders
move_existing : boolean  : if the program should sort existing files
delay         : integer  : the delay for file checking


dirs          : list     : list of dir objects, defined below

A dir object has the following keys:
name          : string   : the name of the folder to organize the extensions to
exts          : list     : a list of strings containing extensions to organize into the folder
```