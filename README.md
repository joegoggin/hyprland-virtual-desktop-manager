# Hyprland Workspace Manager 

## Description

Hyprland Workspace Manager is a command-line interface (CLI) tool 
designed to enhance and extend the capabilities of the existing 
[hyprctl](https://wiki.hyprland.org/Configuring/Using-hyprctl/) 
utility provided by [Hyprland](https://hyprland.org). This application 
aims to deliver improved functionality for users with multi-monitor 
Hyprland setups. 

During the process of configuring my own Hyprland environment, I found 
it necessary to create numerous scripts to achieve the level of workspace 
management I required. To streamline this experience and provide 
a more robust solution, I consolidated these scripts and rewrote them in 
Rust, resulting in a comprehensive tool for efficient workspace management 
on Hyprland.

## Installation

### Step 1

Clone the repository 

```
$ git clone https://github.com/joegoggin/hyprland-workspace-manager.git
```

### Step 2

Change directory to the `hyprland-workspace-manager` directory.

```
$ cd hyprland-workspace-manager 
```

### Step 3

Run install script

**Note:** This command requires you to have the command runner 
[just](https://github.com/casey/just) installed.

```
$ just install
```

### Step 4

Add the following line to your `.bashrc` 

```
export PATH="$HOME/.local/bin:$PATH"
```

### Step 5

Setup Hyprland Workspace Manager configuration

```
$ hyprland-workspace-manager config 
```

## Usage

### `config`

**Command:** `hyprland-workspace-manager config`

The `config` command assists you in creating a configuration file for Hyprland 
Workspace Manager. This tool automatically detects your connected monitors and 
allows you to assign a key (a nickname used to refer to the monitor) for each 
monitor for use with other commands. Once generated, the configuration file is 
saved to `~/.config/hyprland-workspace-manager/config.json`.

In addition to creating the configuration file, the `config` command updates 
your Hyprland configuration to map the appropriate workspaces to each monitor.
The application applies default assumptions regarding workspace assignments. 
Without these configurations, certain commands may not function correctly. By 
default, each monitor is assigned five workspaces, though this can be adjusted
within your configuration file.

### `focus`

#### `monitor`

**Command:** `hyprland-workspace-manager focus monitor <KEY>`

The `focus monitor` command allows you to focus a specific monitor based on the
key assigned in your configuration. For example, if a monitor is assigned the 
key `left`, running `hyprland-workspace-manager focus monitor left` will shift focus to that
monitor.

#### `next-workspace`

**Command:** `hyprland-workspace-manager focus next-workspace`

The `focus next-workspace` command shifts focus to the next workspace relative to 
the focused monitor. For example, if each monitor is assigned 
five workspaces (the default amount) and you are on the first workspace on that
monitor, running `hyprland-workspace-manager focus next-workspace` will move focus to the 
second workspace on that monitor. When executed from the fifth (last) workspace, 
the command cycles back to the first workspace assigned to that monitor.

#### `prev-workspace`

**Command:** `hyprland-workspace-manager focus prev-workspace`

The `focus prev-workspace` command shifts focus to the previous workspace relative
to your focused monitor. For example, if each monitor is assigned 
five workspaces (the default amount) and you are on the second workspace on that
monitor, running `hyprland-workspace-manager focus prev-workspace` will move focus to the 
first workspace on that monitor. When executed from the first workspace, 
the command cycles back to the fifth (last) workspace assigned to that monitor.

#### `workspace`

**Command:** `hyprland-workspace-manager focus workspace <ORDER_NUM>`

The `focus workspace` command shifts focus to a specific workspace associated with 
the currently focused monitor. This command expects an ordinal number corresponding
to the workspace index, ranging from 1 up to the total number of workspaces assigned
to that monitor (five by default). For example, running 
`hyprland-workspace-manager focus workspace 3` will move focus to the third workspace assigned 
to the active monitor.

### `move-window`

#### `monitor`

**Command:** `hyprland-workspace-manager move-window monitor <KEY>`

The `move-window monitor` command will move the currently focused window to the
active workspace on a specific monitor based off of the key assigned in your
configuration. For example, if a monitor is assigned the key `left`, running 
`hyprland-workspace-manager move-window monitor left` will move the currently focused window
to the active window on that monitor.

#### `next-workspace`

**Command:** `hyprland-workspace-manager move-window next-workspace`

The `focus move-window` command moves the currently focused window to the next
workspace on the focused monitor. For example, if each monitor is assigned 
five workspaces (the default amount) and you are on the first workspace on that
monitor, running `hyprland-workspace-manager move-window next-workspace` will move the
currently focused window to the second workspace on the current monitor. When 
executed from the fifth (last) workspace, the command cycles back and moves the
window to the first workspace assigned to that monitor.

#### `prev-workspace`

**Command:** `hyprland-workspace-manager move-window prev-workspace`

The `focus prev-window` command moves the currently focused window to the
previous workspace on the focused monitor. For example, if each monitor is assigned 
five workspaces (the default amount) and you are on the second workspace on that
monitor, running `hyprland-workspace-manager move-window next-workspace` will move the
currently focused window to the first workspace on the current monitor. When 
executed from the fifth workspace, the command cycles back and moves the
window to the first workspace assigned to that monitor.

#### `workspace`

**Command:** `hyprland-workspace-manager move-window workspace <ORDER_NUM>`

The `move-window workspace` command moves the currently to a specific workspace 
associated with the currently focused monitor. This command expects an 
ordinal number corresponding to the workspace index, ranging from 1 up to the 
total number of workspaces assigned to that monitor (five by default). For example, 
running `hyprland-workspace-manager move-window workspace 3` will move the window to the third 
workspace assigned to the active monitor.

### Scripts

All these scripts must be run from the root directory of the project. They also
all require [just](https://github.com/casey/just) to be installed.

#### `run`

**Command:** `just run <COMMAND>`

The `run` script is just an alias for `cargo run -q -- <COMMAND>`.

### `build` 

**Command:** `just build`

**Note:** This command requires [cargo-watch](https://crates.io/crates/cargo-watch) to be installed.

The `build` script is just and alias for `cargo-watch -c -x "build"` 

### `build-release` 

**Command:** `just build-release`

The `build-release` script is just alias for `cargo build --release`

### `install` 

**Command:** `just install`

The `install` script will build your project and add it to the `~/.local/bin` directory. If
this directory does not exist the directory will be created. Once this is run
you just be able to use the `hyprland-workspace-manager` command from any directory in your
terminal.

### `uninstall`

The `uninstall` script will remove the program from your `~/.local/bin`
directory.
