# MKLicense

`mklicense` is a short useful command used to quickly generate a license file.
It uses templates and replaces stuff with the arguments invoked.

## Usage

To invoke the command, you can just do :
```
$ mklicense <your_name> mit
                        ^^^
                        the license type
```
and it will generate the MIT license in the folder where you invoked the command.
Refer to the help command :
```
USAGE:
    mklicense.exe [OPTIONS] <author> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --date <date>    Optional year of the license (by default, the date when you invoke the command)
    -o, --out <out>      Optional location of the generated license file (by default, ./LICENSE)
    -p, --proj <proj>    Optional project name, by default project name is not mentionned in the file

ARGS:
    <author>    The name of the author

SUBCOMMANDS:
    bsd     Generate a BSD license
    gpl     Generate a GPL license
    help    Prints this message or the help of the given subcommand(s)
    mit     Generate a MIT license
```

## Todo
The project is not finished yet, I must implement :
- [ ] Other license types (Apache, Creative Commons...)
- [ ] Optional author/organization name (maybe use env variable?)