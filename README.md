# i3-dws
[![Latest Version](https://img.shields.io/crates/v/i3-dws.svg)](https://crates.io/crates/i3-dws)

**D**ecoupled **W**orkspace **S**witcher

Simple scirpt written in Rust that enables using workspaces in a manner decoupled from outputs: For every display there are workspaces from 0 to 9. Any suggestions, comments and prs are welcome. 

## Usage
```
USAGE:
    i3-dws [OPTIONS]

OPTIONS:
        --create-cfg               
    -h, --help                     Print help information
        --list-outputs             
        --list-workspaces          
    -m, --move <MV>                Move container to workspace
    -o, --output <OUTPUT>          Change to current workspace on given output
    -V, --version                  Print version information
    -w, --workspace <WORKSPACE>    Workspace to change on current output
```
## Configuration 
Use ```i3-dws --create-cfg``` to get initial workspace config and paste it into your i3 config file (usually in ```~/.config/i3/config``` ). Additional configs may look like that: 
```INI
# change workspace 
bindsym Mod4+1 exec --no-startup-id i3-dws -w 1 
bindsym Mod4+2 exec --no-startup-id i3-dws -w 2
bindsym Mod4+3 exec --no-startup-id i3-dws -w 3
bindsym Mod4+4 exec --no-startup-id i3-dws -w 4
bindsym Mod4+5 exec --no-startup-id i3-dws -w 5
bindsym Mod4+6 exec --no-startup-id i3-dws -w 6
bindsym Mod4+7 exec --no-startup-id i3-dws -w 7
bindsym Mod4+8 exec --no-startup-id i3-dws -w 8
bindsym Mod4+9 exec --no-startup-id i3-dws -w 9
bindsym Mod4+0 exec --no-startup-id i3-dws -w 0

# move focused container to workspace
bindsym Mod4+Shift+1 exec i3-dws -m 1
bindsym Mod4+Shift+2 exec i3-dws -m 2
bindsym Mod4+Shift+3 exec i3-dws -m 3
bindsym Mod4+Shift+4 exec i3-dws -m 4
bindsym Mod4+Shift+5 exec i3-dws -m 5
bindsym Mod4+Shift+6 exec i3-dws -m 6
bindsym Mod4+Shift+7 exec i3-dws -m 7
bindsym Mod4+Shift+8 exec i3-dws -m 8
bindsym Mod4+Shift+9 exec i3-dws -m 9
bindsym Mod4+Shift+0 exec i3-dws -m 10
```
