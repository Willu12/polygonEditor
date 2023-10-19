# Polygon Editor - Basic Specification

This is a simple polygon editor application built using Rust-SFML, which allows users to create, edit, and manipulate polygons. The application is designed to be user-friendly and offers a range of features for polygon manipulation.

## Features

1. **Adding, Deleting, and Editing Polygons**
   - Add a new polygon.
   - Delete an existing polygon.
   - Edit an existing polygon.

2. **Editing Polygon Properties**
   - Move individual vertices.
   - Remove vertices.
   - Add vertices in the middle of selected edges.
   - Move entire edges.
   - Move the entire polygon.

3. **Edge Constraints (Relations)**
   - Define constraints for selected edges.
   - Possible constraints: horizontal or vertical edges.
   - Ensure that two adjacent edges cannot both be horizontal or vertical.
   - Removing or adding vertices on an edge removes constraints on adjacent edges.
   - Display visible icons to indicate edge constraints.
   - Ability to remove constraints.

4. **Toggle Offset Mode**
   - Enable/disable offset mode for polygons.
   - Offset mode is applicable only for closed, non-self-intersecting polygons.

5. **Smooth Offset Adjustment**
   - Allow smooth adjustment of the offset (positive values only).

6. **Drawing Line Segments**
   - Utilize both library algorithms and a custom Bresenham's line drawing algorithm.
   - Select between these algorithms using radio buttons.

7. **Polygon Creation and Manipulation**
   - Creating a new polygon and moving it should be intuitive and user-friendly.

## Usage

To use this polygon editor, follow these steps:

1. Clone the repository to your local machine.

2. Open the application in a web browser or an appropriate development environment.

3. Begin by creating a new polygon or selecting an existing one for editing.

4. Use the various tools and options to modify the polygon as needed.

5. Toggle the offset mode if your polygon is valid and closed.

6. Save your work and enjoy editing polygons!