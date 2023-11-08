use crate::error::Result;


/// Trait that defines synchronization interface.
pub trait Syncable {
    /// Type of diff. 
    type Diff;

    /// Create diff that represents changes since specified moment of time.
    /// 
    /// * `base` - moment to get diff since
    fn diff_since(&self, base: chrono::DateTime<chrono::Utc>) -> Result<Self::Diff>;

    /// Apply diffs one-by-one.
    /// 
    /// * `diffs` - container withs diffs to apple
    fn merge_diffs(&self, diffs: Vec<Self::Diff>) -> Result<()>;

    /// Serializes a diff into a writer.
    /// 
    /// * `diff` - diff to serialize
    /// * `instance` - name of instance, that the diff belongs to
    /// * `writer` - writer to store data in
    fn serialize_diff<W: std::io::Write>(&self, diff: Self::Diff, instance: &str, writer: &mut W) -> Result<()>;

    /// Deserializes a diff from a reader.
    /// 
    /// * `instance` - name of instance, that the diff belongs to
    /// * `reader` - reader to get data from
    fn deserialize_diff<R: std::io::Read>(&self, instance: &str, reader: &R) -> Result<Self::Diff>;
}
