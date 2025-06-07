use std::collections::HashSet;

use crate::{
    math::{Scalar, Vector, VectorIteratorExt},
    mesh::{
        cursor::*, DefaultEdgePayload, EuclideanMeshType, FaceBasics, HasIslands, MeshBasics,
        VertexBasics,
    },
};

use super::{MeshBuilder, MeshTypeHalfEdge};

// TODO: Instead, implement a incidence list graph implementation for meshes and define Kruskal on that.
#[derive(Clone)]
struct Bridge<E, S> {
    u: usize,
    v: usize,
    a: E,
    b: E,
    w: S,
}

fn kruskal<E: Copy, S: Scalar>(n: usize, e: &[Bridge<E, S>]) -> Vec<&Bridge<E, S>> {
    fn find(p: &mut [usize], x: usize) -> usize {
        if p[x] == x {
            x
        } else {
            let r = find(p, p[x]);
            p[x] = r;
            r
        }
    }
    fn unite(p: &mut [usize], a: usize, b: usize) -> bool {
        let (ra, rb) = (find(p, a), find(p, b));
        if ra == rb {
            false
        } else {
            p[ra] = rb;
            true
        }
    }

    let mut parent: Vec<usize> = (0..n).collect();
    let mut idx: Vec<usize> = (0..e.len()).collect();
    idx.sort_by(|&i, &j| {
        e[i].w
            .partial_cmp(&e[j].w)
            .unwrap_or(std::cmp::Ordering::Equal)
    }); // e[i].w.total_cmp(&e[j].w)

    let mut mst = Vec::with_capacity(n - 1);
    for i in idx {
        let br = &e[i];
        if unite(&mut parent, br.u, br.v) {
            mst.push(br);
            if mst.len() == n - 1 {
                break;
            }
        }
    }
    mst
}

/// Methods for transforming meshes.
pub trait MeshPosition<const D: usize, T: EuclideanMeshType<D, Mesh = Self>>:
    MeshBasics<T>
{
    /// Returns the mean of all vertex positions.
    fn centroid(&self) -> T::Vec {
        self.vertices().map(|v| v.pos()).stable_mean()
    }

    /// Returns the closest vertex to a given position.
    /// Without a spatial data structure, this takes O(n) time.
    fn closest_vertex<'a>(&'a self, pos: T::Vec) -> Option<&'a T::Vertex>
    where
        T: 'a,
    {
        self.vertex_refs()
            .map(|v| (v, v.pos().distance_squared(&pos)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(v, _)| v)
    }

    /// Calls [MeshPosition::simplify_islands] for each non-trivial group of islands in the mesh.
    fn simplify_all_islands(&mut self) -> Option<&mut Self>
    where
        T::Face: HasIslands<T>,
        Self: MeshBuilder<T>,
        T::EP: DefaultEdgePayload,
        T: MeshTypeHalfEdge,
    {
        let mut faces_with_islands: HashSet<T::F> = HashSet::new();
        for face in self.faces() {
            if face.inner().has_islands() {
                faces_with_islands
                    .insert(HasIslands::islands_representative_face(face.inner(), self));
            }
        }
        for f in faces_with_islands {
            self.simplify_islands(f)?;
        }
        Some(self)
    }

    /// Join all islands by adding the shortest edge between them.
    /// This is not very optimized and can even insert invalid edges.
    ///
    /// TODO: Make this more efficient and robust.
    ///
    /// TODO: This implementation only works for HalfEdge meshes.
    fn simplify_islands(&mut self, f: T::F) -> Option<&mut Self>
    where
        T::Face: HasIslands<T>,
        Self: MeshBuilder<T>,
        T::EP: DefaultEdgePayload,
        T: MeshTypeHalfEdge,
    {
        let islands: Vec<T::E> = self.face(f).islands().map(|v| v.id()).collect();
        let n = islands.len();
        if n <= 1 {
            return Some(self);
        }

        let mut bridges = Vec::<Bridge<T::E, T::S>>::new();

        for (i, &ei0) in islands.iter().enumerate() {
            for (j, &ej0) in islands.iter().enumerate().skip(i + 1) {
                // scan both edge chains and keep the minimum
                let mut best = (ei0, ej0, T::S::INFINITY);

                for e1 in self.edge(ei0).chain() {
                    let p1 = e1.fork().origin().load()?.pos();
                    for e2 in self.edge(ej0).chain() {
                        let p2 = e2.fork().origin().load()?.pos();
                        let d = p1.distance_squared(&p2);
                        if d < best.2 {
                            best = (e1.id(), e2.id(), d)
                        }
                    }
                }

                bridges.push(Bridge {
                    u: i,
                    v: j,
                    a: best.0,
                    b: best.1,
                    w: best.2,
                });
            }
        }

        let mst = kruskal(n, &bridges);

        while self.face(f).try_inner()?.has_islands() {
            self.face_mut(f).remove_next_island().ensure(); // TODO: Use `?`
        }

        let e0 = self.face(f).edge().id()?;
        let fp = self.try_remove_face(f)?;

        for br in mst {
            let e1 = self.edge(br.a).prev().id()?;
            let e2 = self.edge(br.b).id()?;
            self.insert_edge_ee(e1, e2, Default::default())?;
        }

        // Reinsert the face payload
        self.insert_face(e0, fp)?;

        Some(self)
    }
}
