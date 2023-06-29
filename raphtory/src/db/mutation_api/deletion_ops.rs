use crate::core::errors::GraphError;
use crate::core::time::{IntoTimeWithFormat, TryIntoTime};
use crate::core::vertex::InputVertex;
use crate::db::mutation_api::internal::InternalDeletionOps;

pub trait DeletionOps {
    fn delete_edge<V: InputVertex, T: TryIntoTime>(
        &self,
        t: T,
        src: V,
        dst: V,
        layer: Option<&str>,
    ) -> Result<(), GraphError>;

    fn delete_edge_with_custom_time_format<V: InputVertex>(
        &self,
        t: &str,
        fmt: &str,
        src: V,
        dst: V,
        layer: Option<&str>,
    ) -> Result<(), GraphError> {
        let time: i64 = t.parse_time(fmt)?;
        self.delete_edge(time, src, dst, layer)
    }
}

impl<G: InternalDeletionOps> DeletionOps for G {
    fn delete_edge<V: InputVertex, T: TryIntoTime>(
        &self,
        t: T,
        src: V,
        dst: V,
        layer: Option<&str>,
    ) -> Result<(), GraphError> {
        let t = t.try_into_time()?;
        self.internal_delete_edge(t, src.id(), dst.id(), layer)
    }
}