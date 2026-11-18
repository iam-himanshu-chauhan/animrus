# Architecture
Three core pillars of the application

1. Brush Engine: Exceptional drawing feel, customizable brushes, pressure response, texture, blending, and performance.
2. Layer Engine: Sophisticated raster layer manipulation, compositing, masks, blending modes, and non-destructive operations.
3. Animation Engine: A robust timeline, frame management, exposure sheets, onion skinning, playback, and animation workflows.


## Other components

1. Presentation layer - Egui for workspace, panels, tools, timeline
2. Application layer - commands, state, undo / redo, project lifecycle
3. Brush engine - raster based brushes with full customization capabilities
4. Animation engine - easy to use one track at a time system
5. Compositor - to compose layers and drawings
6. History engine - for undo / redo
7. Asset engine - to store assets such as gifs, or compositions
8. Node based animation - simple node based editor to create visual effects, text animations, particle effects
9. Document engine - to handle animrus specific file type
10. Export engine - to export to various formats
11. Rendering - render using GPU
12. Camera system - for camera effects
13. Audio system - for audioplayback
14. film sequencer - for final arrangement of compositions and output

## Document system
* Every project consists of compositions
* composition ➡️ tracks ➡️ drawing ➡️ layers 
* ➡️ means "is/are made of"
* at a time one track will be shown in timeline with all its drawings on the number of frames in the project
* timeline will consists of frames and number of frames will be consistent throughout a composition
* tracks can have as many drawings in the frame, there will be no stacking of drawings one drawing at a time will be displayed if user needs stacking it will be achieved through tracks as there can be many stacked tracks with their respective drawings
* project will be a zip based document - with json data and pngs

----

1. A Document can contain multiple compositions.
2. Each composition contains its own independent tracks.
3. Tracks contain drawing exposures at specific frames.
4. Drawings own their respective layer stacks.
5. Empty frames are allowed.
6. Drawings can remain exposed across multiple frames.
The film sequence references compositions, allowing reuse without duplicatingartwork.


## Modules

* Document - document module will handle these things - storing data, writing data and handling data provided nothing else we will have a zip file say .anrs which will contain data document will handle that only saving, opening, loading and giving to other modules for handling