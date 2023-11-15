/// Utilities structure.
pub struct Utilities {}

impl Utilities {
    /// Converts a 32-bit floating point number (`f32`) to a 32-bit unsigned integer (`u32`).
    ///
    /// # Safety
    /// This function uses `unsafe` code to directly transmute the bits of the input `f32` into a `u32`.
    /// The caller must ensure that this operation is safe in the context of their application.
    ///
    /// # Arguments
    /// * `f` - The `f32` value to be converted.
    ///
    /// # Returns
    /// A `u32` value representing the bit pattern of the input `f32` value.
    pub fn f32_to_u32(f: f32) -> u32 {
        unsafe { std::mem::transmute::<f32, u32>(f) }
    }

    /// Converts a 64-bit floating point number (`f64`) to a 64-bit unsigned integer (`u64`).
    ///
    /// # Safety
    /// This function employs `unsafe` code to directly transmute the bits of the input `f64` into a `u64`.
    /// It is the responsibility of the caller to ensure this operation is safe in their specific context.
    ///
    /// # Arguments
    /// * `f` - The `f64` value to be converted.
    ///
    /// # Returns
    /// A `u64` value representing the bit pattern of the input `f64` value.
    pub fn f64_to_u64(f: f64) -> u64 {
        unsafe { std::mem::transmute::<f64, u64>(f) }
    }

    /// Converts a 32-bit unsigned integer (`u32`) to a 32-bit floating point number (`f32`).
    ///
    /// # Safety
    /// This function uses `unsafe` code to directly transmute the bits of the input `u32` into a `f32`.
    /// The caller must ensure that this operation is safe in the context of their application.
    ///
    /// # Arguments
    /// * `u` - The `u32` value to be converted.
    ///
    /// # Returns
    /// A `f32` value representing the bit pattern of the input `u32` value.
    pub fn u32_to_f32(u: u32) -> f32 {
        unsafe { std::mem::transmute::<u32, f32>(u) }
    }

    /// Converts a 64-bit unsigned integer (`u64`) to a 64-bit floating point number (`f64`).
    ///
    /// # Safety
    /// This function employs `unsafe` code to directly transmute the bits of the input `u64` into a `f64`.
    /// It is the responsibility of the caller to ensure this operation is safe in their specific context.
    ///
    /// # Arguments
    /// * `u` - The `u64` value to be converted.
    ///
    /// # Returns
    /// A `f64` value representing the bit pattern of the input `u64` value.
    pub fn u64_to_f64(u: u64) -> f64 {
        unsafe { std::mem::transmute::<u64, f64>(u) }
    }

    /// Converts a vector of 32-bit floating point numbers (`Vec<f32>`) to a vector of 32-bit unsigned integers (`Vec<u32>`).
    ///
    /// # Arguments
    /// * `f` - The vector of `f32` values to be converted.
    ///
    /// # Returns
    /// A `Vec<u32>` where each element is the converted `u32` representation of the corresponding element in the input `Vec<f32>`.
    pub fn f32vec_to_u32vec(f: Vec<f32>) -> Vec<u32> {
        f.into_iter().map(|x| Self::f32_to_u32(x)).collect()
    }

    /// Converts a vector of 64-bit floating point numbers (`Vec<f64>`) to a vector of 64-bit unsigned integers (`Vec<u64>`).
    ///
    /// # Arguments
    /// * `f` - The vector of `f64` values to be converted.
    ///
    /// # Returns
    /// A `Vec<u64>` where each element is the converted `u64` representation of the corresponding element in the input `Vec<f64>`.
    pub fn f64vec_to_u64vec(f: Vec<f64>) -> Vec<u64> {
        f.into_iter().map(|x| Self::f64_to_u64(x)).collect()
    }

    /// Converts a vector of 32-bit unsigned integers (`Vec<u32>`) to a vector of 32-bit floating point numbers (`Vec<f32>`).
    ///
    /// # Arguments
    /// * `u` - The vector of `u32` values to be converted.
    ///
    /// # Returns
    /// A `Vec<f32>` where each element is the converted `f32` representation of the corresponding element in the input `Vec<u32>`.
    pub fn u32vec_to_f32vec(u: Vec<u32>) -> Vec<f32> {
        u.into_iter().map(|x| Self::u32_to_f32(x)).collect()
    }

    /// Converts a vector of 64-bit unsigned integers (`Vec<u64>`) to a vector of 64-bit floating point numbers (`Vec<f64>`).
    ///
    /// # Arguments
    /// * `u` - The vector of `u64` values to be converted.
    ///
    /// # Returns
    /// A `Vec<f64>` where each element is the converted `f64` representation of the corresponding element in the input `Vec<u64>`.
    pub fn u64vec_to_f64vec(u: Vec<u64>) -> Vec<f64> {
        u.into_iter().map(|x| Self::u64_to_f64(x)).collect()
    }
}
