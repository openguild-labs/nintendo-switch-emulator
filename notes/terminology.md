# Terminology

This file contains essential terminology related to Nintendo Switch development and its surrounding ecosystem.

## Horizon OS

### Kernel

The kernel serves as the bridge between the hardware and software components of a device.

## File Formats

Reference: https://yuzu-emu.org/wiki/overview-of-switch-game-formats/#:~:text=XCI%3A%20Represents%20a%20dump%20of,a%20title%20and%20a%20game.

### NSO/NRO Homebrew

Homebrew is a term used to describe software not authorized by Nintendo. This includes custom firmwares, tools,
<br />
applications, games, emulators, etc.

### XCI - Cart Image

```
XCI might get you banned.
```

Represents a dump of a game cartridge. Contains icons, metadata, and a switch game. Sometimes contains updates to the game as well. Used For: Dumps of Gamecards you own

### NPC

An NPC file, also known as a Nintendo Partition Container, is a file format that contains multiple Nintendo Switch game card images (XCI files). It can be used to store and manage multiple games on a single file.

### NSP - Nintendo Submission Package

```
NSP will get you banned.
```

Contains all of the files and data needed to display icons, a title and a game. Used For: Dumps of SD and NAND games

### NCA

A raw format that can contain a multitude of things (most similar to a ZIP file on your computer) Used For: System Archives

### Deconstructed Rom Directory

An NCA that has been expanded into its component parts. Used For: Developers and Power Users with more advanced applications
