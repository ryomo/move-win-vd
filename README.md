# move-win-vd

A minimal Windows utility that moves the active window to an adjacent virtual desktop.

## Requirements

- Windows 11 24H2 (26100.2605) or later

<br>

## Quick Start

### Download

Download `move-win-vd.exe` from the [Releases](https://github.com/ryomo/move-win-vd/releases) page and place it anywhere you like (e.g. `C:\Tools\move-win-vd.exe`).

#### SmartScreen warning

Because this binary is not code-signed, Windows SmartScreen may block it from running.
When launched via PowerToys Keyboard Manager, it can fail **silently** with no error message.

To unblock the file:

1. Right-click `move-win-vd.exe` → **Properties**.
2. At the bottom of the **General** tab, check **Unblock**.
3. Click **OK**.

### Recommended Setup: PowerToys Keyboard Manager

The most convenient way to use this tool is to bind it to keyboard shortcuts via [PowerToys Keyboard Manager](https://learn.microsoft.com/en-us/windows/powertoys/keyboard-manager).

1. Install [PowerToys](https://github.com/microsoft/PowerToys).
2. Open PowerToys and go to **Keyboard Manager** → **Remap a shortcut**.
3. Add the following shortcuts:

| Shortcut | Program | Arguments | Effect |
|----------|----------------------|--------|--------|
| `Win + Ctrl + Alt + Right` | `move-win-vd.exe` | `r /s` | Move window to next desktop and follow |
| `Win + Ctrl + Alt + Left`  | `move-win-vd.exe` | `l /s` | Move window to previous desktop and follow |

**Tips:**
- Use the full path to `move-win-vd.exe` in the Keyboard Manager configuration (e.g. `C:\Tools\move-win-vd.exe`).
- Add `/w` if you want wrap-around behavior at the edges.
- If you only want to move the window without switching desktops yourself, omit `/s`.
- First run is slow? Register it in startup to cache and speed up.
  1. Right-click `move-win-vd.exe` and create a shortcut
  2. Win+R → `shell:startup` -> paste the shortcut

<br>

## Command Line

```
move-win-vd.exe r|l [/w] [/s]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `r` | Move the active window to the **next** (right) virtual desktop |
| `l` | Move the active window to the **previous** (left) virtual desktop |

The `r` or `l` argument is required.

### Options

| Option | Description |
|--------|-------------|
| `/w` | **Wrap around** — if at the last desktop, move to the first (and vice versa) |
| `/s` | **Switch** — also move your view to the destination desktop |

Options can be combined freely.

### Examples

```
# Move window to the next desktop
move-win-vd.exe r

# Move window to the previous desktop, wrapping around if at the edge
move-win-vd.exe l /w

# Move window to the next desktop and follow it there
move-win-vd.exe r /s

# Move window to the previous desktop, follow it, and wrap around
move-win-vd.exe l /s /w
```

<br>

## Troubleshooting

### Shortcuts not working with PowerToys FancyZones

If you have FancyZones enabled with **"Override Windows Snap"** turned on, your custom shortcuts may stop working — even if those key combinations are not used by FancyZones at all. This appears to be a side effect of how FancyZones hooks into keyboard input globally when that option is enabled.

**Fix:** Disable **"Override Windows Snap"** in FancyZones settings, or reassign your shortcuts to key combinations that do not involve arrow keys.

<br>

## Development

### Building

```
cargo build --release
```

The binary will be at `target/release/move-win-vd.exe`.

### Release Process

Below is the process for releasing a new version on GitHub.\
Just a reminder for myself :smile:

1. Go to **Actions** → **release** → **Run workflow**, enter the version number (e.g. `1.2.3`), and run it.
   - This automatically updates `Cargo.toml` and `Cargo.lock`, commits the changes, and creates + pushes the `v*.*.*` tag.
   - Then, the app is built and a draft release is created.
2. Once the workflow completes, go to the GitHub releases page, find the draft release, review the content, and publish it.
3. Don't forget `git pull` to update your local repository with the new tag.

<br>

## License

MIT
