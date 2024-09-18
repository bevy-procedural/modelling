/// The algorithm to use for generating normals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GenerateNormals {
    /// Do not generate normals. (no vertex duplication)
    None,

    /// Generate flat normals per face. (full vertex duplication)
    #[default]
    Flat,

    /// Generate only smooth normals. (no vertex duplication)
    Smooth,
    // Use face groups to decide whether to generate flat or smooth normals.
    //Groups,
}
