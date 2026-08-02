//! Generic serde helpers, unlike `crate::galaxy::map::serde_helpers` which is
//! specific to the galaxy's ordered cache maps.

/// Serde adapter for `f32` fields that may hold a non-finite sentinel value
/// (e.g. [`Orbit::day_length`](crate::system::orbital_point::types::Orbit),
/// which uses [`f32::INFINITY`] for tide-locked bodies and empty orbit slots).
///
/// `serde_json` already writes non-finite floats as `null`, but plain `f32`
/// deserialization rejects `null` on the way back, which broke JSON
/// round-trips for every generated system. This adapter keeps the exact same
/// wire representation (finite -> number, non-finite -> `null`, so serialized
/// output is byte-identical) and makes the read path work by mapping `null`
/// back to [`f32::INFINITY`].
pub mod non_finite_f32 {
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serializes a finite float as a number and a non-finite one as `null`
    /// (exactly what `serde_json` does with raw non-finite floats).
    // serde's `with` attribute mandates the `&f32` reference signature.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(value: &f32, serializer: S) -> Result<S::Ok, S::Error> {
        if value.is_finite() {
            serializer.serialize_f32(*value)
        } else {
            serializer.serialize_none()
        }
    }

    /// Deserializes a number as-is and `null` as [`f32::INFINITY`], the
    /// non-finite sentinel used by the generator.
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f32, D::Error> {
        Ok(Option::<f32>::deserialize(deserializer)?.unwrap_or(f32::INFINITY))
    }
}
