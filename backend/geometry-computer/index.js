import * as fs from "fs";
import * as THREE from "three";
import * as BufferGeometryUtils from "three/addons/utils/BufferGeometryUtils.js";
import { Document, NodeIO, Accessor } from "@gltf-transform/core";

// The first two elements of process.argv are always 'node' and the path to the script.
// We want to start reading the arguments from the third element.
const args = process.argv.slice(2);

const sourceFile = args[0];
const outputDirectory = args[1];
console.log("Source File: ", sourceFile);
console.log("Output Directory: ", outputDirectory);

// Read the JSON file
fs.readFile(sourceFile, "utf8", async (err, data) => {
  if (err) {
    console.error("Error reading file:", err);
    return;
  }

  const annotationLayers = JSON.parse(data);
  console.log(annotationLayers.length);

  for (const annotationLayer of annotationLayers) {
    console.log(annotationLayer.tag);
    const arrays = draw(annotationLayer);
    await gltf(annotationLayer.id, arrays);
  }
});

async function gltf(id, { indicesArray, positionArray, uvArray }) {
  const document = new Document();
  const buffer = document.createBuffer();

  // indices and vertex attributes
  const indices = document
    .createAccessor()
    .setArray(indicesArray.array)
    .setType(Accessor.Type.SCALAR)
    .setBuffer(buffer);

  const position = document
    .createAccessor()
    .setArray(positionArray.array)
    .setType(Accessor.Type.VEC3)
    .setBuffer(buffer);

  const uv = document
    .createAccessor()
    .setArray(uvArray.array)
    .setType(Accessor.Type.VEC3)
    .setBuffer(buffer);

  const material = document
    .createMaterial()
    .setBaseColorFactor([1, 0.5, 0.5, 1]); // RGBA

  // primitive and mesh
  const prim = document
    .createPrimitive()
    .setMaterial(material)
    .setIndices(indices)
    .setAttribute("POSITION", position)
    .setAttribute("UV", uv);

  const mesh = document.createMesh("MyMesh").addPrimitive(prim);

  const node = document
    .createNode("MyNode")
    .setMesh(mesh)
    .setTranslation([0, 0, 0]);

  const scene = document.createScene("MyScene").addChild(node);

  const io = new NodeIO();
  await io.write(`${outputDirectory}${id}.glb`, document);
}

function draw(annotationLayer) {
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

  const indicesArray = mergedGeometry.getIndex();
  const positionArray = mergedGeometry.getAttribute("position");
  const uvArray = mergedGeometry.getAttribute("uv");

  return { indicesArray, positionArray, uvArray };
}
