use std::fs::File;

/// Load an STL to a triangle buffer so that the renderer can draw it
///
/// # Arguments
/// * `filename` - The path to the file relative to the CWD
///
/// # Returns
/// A triangle buffer
pub fn load_stl_to_buffer(filename: &str) -> Result<stl_io::IndexedMesh, Box<dyn std::error::Error>> {

    let mut file = File::open(filename)?;
    Ok(stl_io::read_stl(&mut file)?)

}
