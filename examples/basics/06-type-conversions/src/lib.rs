    /// Sum of the two inputs as `i128`.
    pub fn sum_different_types(_env: Env, input_u32: u32, input_i64: i64) -> i128 {
        let a: i128 = input_u32.into(); // From<u32> for i128
        let b: i128 = input_i64.into(); // From<i64> for i128
        a + b
    }

    /// Demonstrates a full `u32` → `Val` → `u32` roundtrip.
    ///
    /// `IntoVal` converts a native type to the host `Val` representation;
    /// `TryFromVal` converts it back. This roundtrip is the foundation of
    /// all cross-boundary data passing in Soroban.
    ///
    /// # Returns
    /// The original value after the roundtrip, or 0 on failure.
    pub fn val_roundtrip(env: Env, input: u32) -> u32 {
        let val: Val = input.into_val(&env);
        u32::try_from_val(&env, &val).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
>>>>>>> 99867cb (feat:implement Show Type Conversions)
