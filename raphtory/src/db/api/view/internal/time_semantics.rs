use crate::{
    core::{
        entities::{
            edges::{edge_ref::EdgeRef, edge_store::EdgeStore},
            LayerIds, VID,
        },
        storage::timeindex::TimeIndexOps,
        Prop,
    },
    db::api::view::{
        internal::{Base, CoreGraphOps, EdgeFilter, GraphOps},
        BoxedIter,
    },
};
use std::ops::Range;

/// Methods for defining time windowing semantics for a graph
pub trait TimeSemantics: GraphOps + CoreGraphOps {
    /// Return the earliest time for a vertex
    fn vertex_earliest_time(&self, v: VID) -> Option<i64> {
        self.vertex_additions(v).first_t()
    }

    /// Return the latest time for a vertex
    fn vertex_latest_time(&self, v: VID) -> Option<i64> {
        self.vertex_additions(v).last_t()
    }

    /// Returns the default start time for perspectives over the view
    #[inline]
    fn view_start(&self) -> Option<i64> {
        self.earliest_time_global()
    }

    /// Returns the default end time for perspectives over the view
    #[inline]
    fn view_end(&self) -> Option<i64> {
        self.latest_time_global().map(|v| v.saturating_add(1))
    }

    /// Returns the timestamp for the earliest activity
    fn earliest_time_global(&self) -> Option<i64>;
    /// Returns the timestamp for the latest activity
    fn latest_time_global(&self) -> Option<i64>;
    /// Returns the timestamp for the earliest activity in the window
    fn earliest_time_window(&self, t_start: i64, t_end: i64) -> Option<i64>;

    /// Returns the timestamp for the latest activity in the window
    fn latest_time_window(&self, t_start: i64, t_end: i64) -> Option<i64>;

    /// Return the earliest time for a vertex in a window
    fn vertex_earliest_time_window(&self, v: VID, t_start: i64, t_end: i64) -> Option<i64> {
        self.vertex_additions(v).range(t_start..t_end).first_t()
    }

    /// Return the latest time for a vertex in a window
    fn vertex_latest_time_window(&self, v: VID, t_start: i64, t_end: i64) -> Option<i64> {
        self.vertex_additions(v).range(t_start..t_end).last_t()
    }
    /// check if vertex `v` should be included in window `w`
    fn include_vertex_window(
        &self,
        v: VID,
        w: Range<i64>,
        layer_ids: &LayerIds,
        edge_filter: Option<&EdgeFilter>,
    ) -> bool;

    /// check if edge `e` should be included in window `w`
    fn include_edge_window(&self, e: &EdgeStore, w: Range<i64>, layer_ids: &LayerIds) -> bool;

    /// Get the timestamps at which a vertex `v` is active (i.e has an edge addition)
    fn vertex_history(&self, v: VID) -> Vec<i64> {
        self.vertex_additions(v).iter_t().copied().collect()
    }

    /// Get the timestamps at which a vertex `v` is active in window `w` (i.e has an edge addition)
    fn vertex_history_window(&self, v: VID, w: Range<i64>) -> Vec<i64> {
        self.vertex_additions(v)
            .range(w)
            .iter_t()
            .copied()
            .collect()
    }

    /// Exploded edge iterator for edge `e`
    fn edge_exploded(&self, e: EdgeRef, layer_ids: LayerIds) -> BoxedIter<EdgeRef>;

    /// Explode edge iterator for edge `e` for every layer
    fn edge_layers(&self, e: EdgeRef, layer_ids: LayerIds) -> BoxedIter<EdgeRef>;

    /// Exploded edge iterator for edge`e` over window `w`
    fn edge_window_exploded(
        &self,
        e: EdgeRef,
        w: Range<i64>,
        layer_ids: LayerIds,
    ) -> BoxedIter<EdgeRef>;

    /// Exploded edge iterator for edge `e` over window `w` for every layer
    fn edge_window_layers(
        &self,
        e: EdgeRef,
        w: Range<i64>,
        layer_ids: LayerIds,
    ) -> BoxedIter<EdgeRef>;

    /// Get the time of the earliest activity of an edge
    fn edge_earliest_time(&self, e: EdgeRef, layer_ids: LayerIds) -> Option<i64>;

    /// Get the time of the earliest activity of an edge `e` in window `w`
    fn edge_earliest_time_window(
        &self,
        e: EdgeRef,
        w: Range<i64>,
        layer_ids: LayerIds,
    ) -> Option<i64>;

    /// Get the time of the latest activity of an edge
    fn edge_latest_time(&self, e: EdgeRef, layer_ids: LayerIds) -> Option<i64>;

    /// Get the time of the latest activity of an edge `e` in window `w`
    fn edge_latest_time_window(
        &self,
        e: EdgeRef,
        w: Range<i64>,
        layer_ids: LayerIds,
    ) -> Option<i64>;

    /// Get the edge deletions for use with materialize
    fn edge_deletion_history(&self, e: EdgeRef, layer_ids: LayerIds) -> Vec<i64>;

    /// Get the edge deletions for use with materialize restricted to window `w`
    fn edge_deletion_history_window(
        &self,
        e: EdgeRef,
        w: Range<i64>,
        layer_ids: LayerIds,
    ) -> Vec<i64>;

    /// Returns a vector of all temporal values of the graph property with the given name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the property to retrieve.
    ///
    /// # Returns
    ///
    /// A vector of tuples representing the temporal values of the property
    /// that fall within the specified time window, where the first element of each tuple is the timestamp
    /// and the second element is the property value.
    fn temporal_prop_vec(&self, name: &str) -> Vec<(i64, Prop)>;

    /// Returns a vector of all temporal values of the graph property with the given name
    /// that fall within the specified time window.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the property to retrieve.
    /// * `t_start` - The start time of the window to consider.
    /// * `t_end` - The end time of the window to consider.
    ///
    /// # Returns
    ///
    /// A vector of tuples representing the temporal values of the property
    /// that fall within the specified time window, where the first element of each tuple is the timestamp
    /// and the second element is the property value.
    fn temporal_prop_vec_window(&self, name: &str, t_start: i64, t_end: i64) -> Vec<(i64, Prop)>;

    /// Returns a vector of all temporal values of the vertex property with the given name for the
    /// given vertex
    ///
    /// # Arguments
    ///
    /// * `v` - A reference to the vertex for which to retrieve the temporal property vector.
    /// * `name` - The name of the property to retrieve.
    ///
    /// # Returns
    ///
    /// A vector of tuples representing the temporal values of the property for the given vertex
    /// that fall within the specified time window, where the first element of each tuple is the timestamp
    /// and the second element is the property value.
    fn temporal_vertex_prop_vec(&self, v: VID, name: &str) -> Vec<(i64, Prop)>;

    /// Returns a vector of all temporal values of the vertex property with the given name for the given vertex
    /// that fall within the specified time window.
    ///
    /// # Arguments
    ///
    /// * `v` - A reference to the vertex for which to retrieve the temporal property vector.
    /// * `name` - The name of the property to retrieve.
    /// * `t_start` - The start time of the window to consider.
    /// * `t_end` - The end time of the window to consider.
    ///
    /// # Returns
    ///
    /// A vector of tuples representing the temporal values of the property for the given vertex
    /// that fall within the specified time window, where the first element of each tuple is the timestamp
    /// and the second element is the property value.
    fn temporal_vertex_prop_vec_window(
        &self,
        v: VID,
        name: &str,
        t_start: i64,
        t_end: i64,
    ) -> Vec<(i64, Prop)>;

    /// Returns a vector of tuples containing the values of the temporal property with the given name
    /// for the given edge reference within the specified time window.
    ///
    /// # Arguments
    ///
    /// * `e` - An `EdgeRef` reference to the edge of interest.
    /// * `name` - A `String` containing the name of the temporal property.
    /// * `t_start` - An `i64` containing the start time of the time window (inclusive).
    /// * `t_end` - An `i64` containing the end time of the time window (exclusive).
    ///
    /// # Returns
    ///
    /// * A `Vec` of tuples containing the values of the temporal property with the given name for the given edge
    /// within the specified time window.
    ///
    fn temporal_edge_prop_vec_window(
        &self,
        e: EdgeRef,
        name: &str,
        t_start: i64,
        t_end: i64,
        layer_ids: LayerIds,
    ) -> Vec<(i64, Prop)>;

    /// Returns a vector of tuples containing the values of the temporal property with the given name
    /// for the given edge reference.
    ///
    /// # Arguments
    ///
    /// * `e` - An `EdgeRef` reference to the edge of interest.
    /// * `name` - A `String` containing the name of the temporal property.
    ///
    /// # Returns
    ///
    /// * A `Vec` of tuples containing the values of the temporal property with the given name for the given edge.
    fn temporal_edge_prop_vec(
        &self,
        e: EdgeRef,
        name: &str,
        layer_ids: LayerIds,
    ) -> Vec<(i64, Prop)>;
}

pub trait InheritTimeSemantics: Base + GraphOps + CoreGraphOps {}

impl<G: InheritTimeSemantics> DelegateTimeSemantics for G
where
    <G as Base>::Base: TimeSemantics,
{
    type Internal = <G as Base>::Base;

    fn graph(&self) -> &Self::Internal {
        self.base()
    }
}

pub trait DelegateTimeSemantics: GraphOps + CoreGraphOps {
    type Internal: TimeSemantics + ?Sized;

    fn graph(&self) -> &Self::Internal;
}

impl<G: DelegateTimeSemantics + ?Sized> TimeSemantics for G {
    #[inline]
    fn vertex_earliest_time(&self, v: VID) -> Option<i64> {
        self.graph().vertex_earliest_time(v)
    }

    #[inline]
    fn vertex_latest_time(&self, v: VID) -> Option<i64> {
        self.graph().vertex_latest_time(v)
    }

    #[inline]
    fn view_start(&self) -> Option<i64> {
        self.graph().view_start()
    }
    #[inline]
    fn view_end(&self) -> Option<i64> {
        self.graph().view_end()
    }
    #[inline]
    fn earliest_time_global(&self) -> Option<i64> {
        self.graph().earliest_time_global()
    }
    #[inline]
    fn latest_time_global(&self) -> Option<i64> {
        self.graph().latest_time_global()
    }
    #[inline]
    fn earliest_time_window(&self, t_start: i64, t_end: i64) -> Option<i64> {
        self.graph().earliest_time_window(t_start, t_end)
    }
    #[inline]
    fn latest_time_window(&self, t_start: i64, t_end: i64) -> Option<i64> {
        self.graph().latest_time_window(t_start, t_end)
    }
    #[inline]
    fn vertex_earliest_time_window(&self, v: VID, t_start: i64, t_end: i64) -> Option<i64> {
        self.graph().vertex_earliest_time_window(v, t_start, t_end)
    }
    #[inline]
    fn vertex_latest_time_window(&self, v: VID, t_start: i64, t_end: i64) -> Option<i64> {
        self.graph().vertex_latest_time_window(v, t_start, t_end)
    }
    #[inline]
    fn include_vertex_window(
        &self,
        v: VID,
        w: Range<i64>,
        layer_ids: &LayerIds,
        edge_filter: Option<&EdgeFilter>,
    ) -> bool {
        self.graph()
            .include_vertex_window(v, w, layer_ids, edge_filter)
    }

    #[inline]
    fn include_edge_window(&self, e: &EdgeStore, w: Range<i64>, layer_ids: &LayerIds) -> bool {
        self.graph().include_edge_window(e, w, layer_ids)
    }

    #[inline]
    fn vertex_history(&self, v: VID) -> Vec<i64> {
        self.graph().vertex_history(v)
    }

    #[inline]
    fn vertex_history_window(&self, v: VID, w: Range<i64>) -> Vec<i64> {
        self.graph().vertex_history_window(v, w)
    }

    #[inline]
    fn edge_exploded(&self, e: EdgeRef, layer_ids: LayerIds) -> BoxedIter<EdgeRef> {
        self.graph().edge_exploded(e, layer_ids)
    }

    #[inline]
    fn edge_layers(&self, e: EdgeRef, layer_ids: LayerIds) -> BoxedIter<EdgeRef> {
        self.graph().edge_layers(e, layer_ids)
    }

    #[inline]
    fn edge_window_exploded(
        &self,
        e: EdgeRef,
        w: Range<i64>,
        layer_ids: LayerIds,
    ) -> BoxedIter<EdgeRef> {
        self.graph().edge_window_exploded(e, w, layer_ids)
    }

    #[inline]
    fn edge_window_layers(
        &self,
        e: EdgeRef,
        w: Range<i64>,
        layer_ids: LayerIds,
    ) -> BoxedIter<EdgeRef> {
        self.graph().edge_window_layers(e, w, layer_ids)
    }

    #[inline]
    fn edge_earliest_time(&self, e: EdgeRef, layer_ids: LayerIds) -> Option<i64> {
        self.graph().edge_earliest_time(e, layer_ids)
    }

    #[inline]
    fn edge_earliest_time_window(
        &self,
        e: EdgeRef,
        w: Range<i64>,
        layer_ids: LayerIds,
    ) -> Option<i64> {
        self.graph().edge_earliest_time_window(e, w, layer_ids)
    }

    #[inline]
    fn edge_latest_time(&self, e: EdgeRef, layer_ids: LayerIds) -> Option<i64> {
        self.graph().edge_latest_time(e, layer_ids)
    }

    #[inline]
    fn edge_latest_time_window(
        &self,
        e: EdgeRef,
        w: Range<i64>,
        layer_ids: LayerIds,
    ) -> Option<i64> {
        self.graph().edge_latest_time_window(e, w, layer_ids)
    }

    #[inline]
    fn edge_deletion_history(&self, e: EdgeRef, layer_ids: LayerIds) -> Vec<i64> {
        self.graph().edge_deletion_history(e, layer_ids)
    }

    #[inline]
    fn edge_deletion_history_window(
        &self,
        e: EdgeRef,
        w: Range<i64>,
        layer_ids: LayerIds,
    ) -> Vec<i64> {
        self.graph().edge_deletion_history_window(e, w, layer_ids)
    }

    #[inline]
    fn temporal_prop_vec(&self, name: &str) -> Vec<(i64, Prop)> {
        self.graph().temporal_prop_vec(name)
    }

    #[inline]
    fn temporal_prop_vec_window(&self, name: &str, t_start: i64, t_end: i64) -> Vec<(i64, Prop)> {
        self.graph().temporal_prop_vec_window(name, t_start, t_end)
    }

    #[inline]
    fn temporal_vertex_prop_vec(&self, v: VID, name: &str) -> Vec<(i64, Prop)> {
        self.graph().temporal_vertex_prop_vec(v, name)
    }

    #[inline]
    fn temporal_vertex_prop_vec_window(
        &self,
        v: VID,
        name: &str,
        t_start: i64,
        t_end: i64,
    ) -> Vec<(i64, Prop)> {
        self.graph()
            .temporal_vertex_prop_vec_window(v, name, t_start, t_end)
    }

    #[inline]
    fn temporal_edge_prop_vec_window(
        &self,
        e: EdgeRef,
        name: &str,
        t_start: i64,
        t_end: i64,
        layer_ids: LayerIds,
    ) -> Vec<(i64, Prop)> {
        self.graph()
            .temporal_edge_prop_vec_window(e, name, t_start, t_end, layer_ids)
    }

    #[inline]
    fn temporal_edge_prop_vec(
        &self,
        e: EdgeRef,
        name: &str,
        layer_ids: LayerIds,
    ) -> Vec<(i64, Prop)> {
        self.graph().temporal_edge_prop_vec(e, name, layer_ids)
    }
}
