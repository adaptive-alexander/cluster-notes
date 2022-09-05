# Cluster Notes

This repo is in its early infancy. The aim is to create a CLI-app that parses note files,
finds linked tags between them and builds mind maps. It is in some sense a CLI version of
Obsidian, with intended future support of different file formats.

## Structure
The program has the following components:
- A watcher for folders set by the user which looks for changes in notes.
- A file parser looking for linked tags and changes.
- A renderer that produces initially a 2d image, but later a 3d object.
- An actions module to tie the pieces together.
- A main module that parses args and configs as well as the run loop.
