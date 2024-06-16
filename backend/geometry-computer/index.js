import * as fs from "fs";
import * as THREE from "three";
import * as BufferGeometryUtils from "three/addons/utils/BufferGeometryUtils.js";

// The first two elements of process.argv are always 'node' and the path to the script.
// We want to start reading the arguments from the third element.
const args = process.argv.slice(2);

const sourceFile = args[0];
const outputDirectory = args[1];
console.log("Source File: ", sourceFile);
console.log("Output Directory: ", outputDirectory);

// Read the JSON file
fs.readFile(sourceFile, "utf8", (err, data) => {
  if (err) {
    console.error("Error reading file:", err);
    return;
  }

  const annotationLayers = JSON.parse(data);
  draw(annotationLayers);
});

function draw(annotationLayers) {
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
      outputDirectory + "/" + annotationLayer.tag + ".json",
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
