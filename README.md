# to

A program written in Rust that helps you to navigate quickly through many project folders.

![To Showcase](http://dennis-schneider.com/downloads/to_showcase.gif)

## Installation

* Install Rust (https://www.rust-lang.org/)
* Clone the project
* Navigate to the project folder and run `make install`
* It will install the binary to the `bin` folder in your home directory
* Add the following function to your .bashrc or .zshrc:

```
function to() {
  cd `~/bin/to $1`
}
```

* Now create a `.to` folder in your home directory
* Create a `paths.cfg` file in there
* Fill in your absolute project folder paths ending with `/`. Example:

```
/home/username/Projects/Work/
/home/username/Projects/Private/
```

## Usage

* Type `to FOLDER_NAME` in your terminal

If more than one project folder containing the given folder name is found, the
program will prompt you to choose a folder. `To` does a partial match on
your folder name.


## Debugging

If you descover a problem, run the `to` binary without the function from the
.bashrc / .zshrc to see the error output.
