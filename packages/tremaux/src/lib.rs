// tremaux - a tree
// MUST BE ACYCLIC
use graph::prelude::Idx;

// fromroot, toroot, toleaf, fromleaf
// paths, descendings only

pub trait Tremaux<NI, EV = ()>
where
    NI: Idx,
{
    /// Finds the deepest nodes
    fn leaves(&self) -> Vec<NI>;

    fn from_root(&self, node: NI) -> Vec<NI>;

    fn to_root(&self, node: NI) -> Vec<NI>;
}

pub trait TremauxValues<NI, NV = (), EV = ()>
where
    NI: Idx,
{
    fn leave_values(&self) -> &NV;

    fn from_root_values(&self, node: NI) -> Vec<&NV>;

    fn to_root_values(&self, node: NI) -> Vec<&NV>;
}
