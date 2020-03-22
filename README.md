## idea-clean ##
A tool to recover idea popups in multiscreen setups. 
Sometimes floating popups in IntelliJ products end up in unaccessible locations, expecially while using multiple screen 
with different resolutions or aspect ratios. Upon invoking of cli tool, the position cache will be deleted and the floating 
popups should reappear on the current screen center.

### build ###
You will need [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Clone this repo and cd into it, then do
```
cargo build
```
### usage ###
Invoke with
```
./target/debug/idea-clean /path/to/.idea
```
.idea being the folder containing the settings and the view cache for clion/idea/pycharm projects

### lazy typer ###
```
alias clean="path/to/target/debug/idea-clean /path/to/.idea"
```