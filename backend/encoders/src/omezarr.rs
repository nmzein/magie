use crate::common::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

static GROUP_PATH: &str = "/group";

pub const NAME: &str = "OMEZarr";

pub struct Module;

impl Encoder for Module {
    fn name(&self) -> &'static str {
        NAME
    }

    fn convert<D: Decoder>(
        &self,
        input_path: &Path,
        output_path: &Path,
        decoder: D,
    ) -> Result<Vec<MetadataLayer>> {
        // One store per image.
        let store = Arc::new(FilesystemStore::new(output_path)?);
        // One group per image.
        let group = GroupBuilder::new().build(store.clone(), GROUP_PATH)?;
        // Write group metadata to store.
        group.store_metadata()?;

        let levels = decoder.get_level_count(&input_path)?;
        if levels == 0 {
            return Err(anyhow::anyhow!("Image has no levels."));
        }
        let (level_0_width, level_0_height) = decoder.get_level_dimensions(&input_path, 0)?;

        // Collect metadata in parallel.
        let metadata: Vec<MetadataLayer> = (0..levels)
            .into_par_iter()
            .map(|level| {
                // Get image dimensions.
                let (width, height) = decoder.get_level_dimensions(&input_path, level)?;

                // Calculate number of tiles per row and column.
                let cols = width.div_ceil(TILE_SIZE);
                let rows = height.div_ceil(TILE_SIZE);

                // ! Loses accuracy.
                let width_ratio = (level_0_width as f32 / width as f32) as u32;
                let height_ratio = (level_0_height as f32 / height as f32) as u32;

                // One array per image level.
                let array_path = format!("{}/{}", GROUP_PATH, level);

                let array = ArrayBuilder::new(
                    // Define image shape.
                    vec![0, RGB_CHANNELS.into(), 0, height.into(), width.into()],
                    // Define data type.
                    DataType::UInt8,
                    // Define tile size.
                    vec![1, 1, 1, TILE_SIZE.into(), TILE_SIZE.into()].try_into()?,
                    // Define initial fill value.
                    FillValue::from(41u8),
                )
                // Define compression algorithm and strength.
                .bytes_to_bytes_codecs(vec![Arc::new(GzipCodec::new(9)?)])
                // Define dimension names - time, RGB channel, z, y, x axis.
                .dimension_names(vec!["t", "c", "z", "y", "x"].into())
                .build(store.clone(), &array_path)?;

                // Write array metadata to store.
                array.store_metadata()?;

                // Parallelize the inner loop over rows.
                (0..rows).into_par_iter().try_for_each(|y| {
                    for x in 0..cols {
                        // Rearrange tile from [R,G,B,R,G,B] to [R,R,G,G,B,B].
                        let tile = decoder
                            .read_region(
                                &input_path,
                                &Region {
                                    size: Size {
                                        width: TILE_SIZE,
                                        height: TILE_SIZE,
                                    },
                                    level: level,
                                    address: Address {
                                        x: (x * TILE_SIZE * width_ratio),
                                        y: (y * TILE_SIZE * height_ratio),
                                    },
                                },
                            )?
                            .chunks(3)
                            .fold(
                                vec![Vec::new(), Vec::new(), Vec::new()],
                                |mut acc, chunk| {
                                    acc[0].push(chunk[0]);
                                    acc[1].push(chunk[1]);
                                    acc[2].push(chunk[2]);
                                    acc
                                },
                            )
                            .into_iter()
                            .flatten()
                            .collect::<Vec<u8>>();

                        array.store_chunks_elements(
                            &ArraySubset::new_with_start_end_inc(
                                vec![0, 0, 0, y.into(), x.into()],
                                vec![0, 2, 0, y.into(), x.into()],
                            )?,
                            &tile,
                        )?;
                    }

                    Ok::<(), anyhow::Error>(())
                })?;

                Ok(MetadataLayer {
                    level,
                    cols,
                    rows,
                    width,
                    height,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(metadata)
    }

    fn retrieve(&self, image_path: &Path, level: u32, x: u32, y: u32) -> Result<Vec<u8>> {
        let store = Arc::new(FilesystemStore::new(image_path)?);
        let array = Arc::new(Array::open(store, &format!("{GROUP_PATH}/{}", level))?);

        let x: u64 = x.into();
        let y: u64 = y.into();

        // Retrieve tile for each RGB channel.
        let channels = array.retrieve_chunks_elements(&ArraySubset::new_with_start_end_inc(
            vec![0, 0, 0, y, x],
            vec![0, 2, 0, y, x],
        )?)?;

        // Interleave RGB channels.
        let mut tile = Vec::with_capacity(TILE_LENGTH * 3);
        interleave(&channels, &mut tile);

        Ok(tile)
    }
}
