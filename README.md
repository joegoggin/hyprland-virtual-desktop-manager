# Hyprland Virtual Desktop Manager 

## Description

Hyprland Virtual Desktop Manager is a command-line interface (CLI) tool 
designed to enhance and extend the capabilities of the existing 
[hyprctl](https://wiki.hyprland.org/Configuring/Using-hyprctl/) 
utility provided by [Hyprland](https://hyprland.org). This application 
aims to deliver improved functionality for users with multi-monitor 
Hyprland setups. 

During the process of configuring my own Hyprland environment, I found 
it necessary to create numerous scripts to achieve the level of virtual 
desktop management I required. To streamline this experience and provide 
a more robust solution, I consolidated these scripts and rewrote them in 
Rust, resulting in a comprehensive tool for efficient virtual desktop 
management on Hyprland.

## Usage

### `initialize-workspaces`

**Command:** `hyprland-vdm initialize-workspaces`

The `initialize-workspaces` command configures the current workspaces on 
each monitor to align with the expectations of the tool. This utility 
manages workspaces by incrementing and decrementing their respective ID 
numbers. For proper functionality, it is essential that workspaces are 
correctly initialized; otherwise, the tool may not operate as intended.

To automate this process, add the following line to your Hyprland 
configuration file:

```
exec-once = hyprland-vdm initialize-workspaces
```

This ensures that workspace initialization occurs automatically each time 
Hyprland starts.
