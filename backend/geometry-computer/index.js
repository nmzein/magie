import { spawnSync } from "node:child_process";
import * as fs from "node:fs";
import * as THREE from "three";
import { PLYExporter } from "three/addons/exporters/PLYExporter.js";
import { mergeGeometries } from "three/addons/utils/BufferGeometryUtils.js";

const exporter = new PLYExporter();

// The first two elements of process.argv are always 'node' and the path to the script.
// We want to start reading the arguments from the third element.
const args = process.argv.slice(2);

const source_file = args[0];
const output_directory = args[1];
console.log("Source File: ", source_file);
console.log("Output Directory: ", output_directory);

fs.readFile(source_file, "utf8", (err, data) => {
  if (err) throw Error("Error reading file:", err);

  const layers = JSON.parse(data);
  if (!layers || !Array.isArray(layers)) throw Error("Invalid layers.");

  for (let i = 0; i < layers.length; i++) {
    to_draco(layers[i], i);
  }
});

function to_draco(layer, index) {
  const geometries = layer.annotations.map((annotation) => {
    const shape = new THREE.Shape();
    shape.moveTo(annotation[0][0], -annotation[0][1]);
    for (let i = 1; i < annotation.length; i++) {
      shape.lineTo(annotation[i][0], -annotation[i][1]);
    }
    shape.closePath();
    return new THREE.ShapeGeometry(shape);
  });

  const merged_geometry = mergeGeometries(geometries);
  merged_geometry.computeVertexNormals();

  const mesh = new THREE.Mesh(merged_geometry);

  const ply = exporter.parse(mesh, { binary: true });
  const ply_path = `${output_directory}/tmp_${index}.ply`;

  fs.writeFileSync(ply_path, Buffer.from(ply));

  const drc_path = `${output_directory}/a${index}.drc`;
  const result = spawnSync("draco_encoder", [
    "-i",
    ply_path,
    "-o",
    drc_path,
    "-qp",
    "14", // position quantization
    "-qn",
    "10", // normal quantization
    "-qt",
    "12", // texcoord quantization
    "-cl",
    "10", // compression level
  ]);

  if (result.status !== 0) {
    console.error(result.stderr.toString());
    throw new Error("Draco encoding failed");
  }

  fs.unlinkSync(ply_path); // cleanup
}
