import * as fs from "fs";
import * as THREE from "three";
import * as BufferGeometryUtils from "three/addons/utils/BufferGeometryUtils.js";

// The first two elements of process.argv are always 'node' and the path to the script.
// We want to start reading the arguments from the third element.
const args = process.argv.slice(2);

// Now, you can access the inputted variables.
// For example, if you run 'node index.js arg1 arg2', args will be ['arg1', 'arg2'].
const path = args[0];

console.log("Path: ", path);

// Read the JSON file
fs.readFile(path + "/annotations.json", "utf8", (err, data) => {
  if (err) {
    console.error("Error reading file:", err);
    return;
  }

  const annotationLayers = JSON.parse(data);
  draw(path, annotationLayers);
});

function draw(path, annotationLayers) {
  console.log(annotationLayers.length);
  for (const annotationLayer of annotationLayers) {
    console.log(annotationLayer.tag, annotationLayer.annotations.length);
    let geometries = [];

    annotationLayer.annotations.forEach((annotation, _) => {
      const shape = new THREE.Shape();

      shape.moveTo(annotation[0][0], -1 * annotation[0][1]);
      for (let i = 1; i < annotation.length; i++) {
        shape.lineTo(annotation[i][0], -1 * annotation[i][1]);
      }
      shape.closePath();

      geometries.push(new THREE.ShapeGeometry(shape));
    });

    // Merge the geometries into a single geometry to minimize draw calls.
    const mergedGeometry = BufferGeometryUtils.mergeGeometries(geometries);

    // Save geometries to JSON file
    fs.writeFileSync(
      path + "/" + annotationLayer.tag + ".json",
      JSON.stringify(mergedGeometry),
      function (err) {
        if (err) {
          console.log(err);
        }
      }
    );

    annotationLayer.annotations = [];
    mergedGeometry.dispose();
  }
}
