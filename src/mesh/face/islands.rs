use crate::{
    math::IndexType,
    mesh::{cursor::*, FaceBasics, MeshBasics, MeshBuilder, MeshType, MeshTypeHalfEdge},
    util::CreateEmptyIterator,
};

// TODO: test this! Make sure functions depending on islands check first!
// TODO: make all face payloads shared! The new payload system should support some means of only having a reference to the payload and multiple faces / edges referencing the same payload.

/// A face that can have holes or sub-meshes protruding from inside of the face, i.e., "islands".
///
/// Usually, you can circumvent this by inserting an edge connecting the island to the outer edge chain of the face.
/// However, depending on how you plan to further process the mesh this may not be desirable, e.g.,
/// it can be semantically different for triangulation, non-planar faces, smoothing, etc.
///
/// There is a circular linked list based default implementation having a "wheel" of faces, each connecting either to the outside
/// or one of the islands. See [IslandCircularLinkedList] for more details.
pub trait HasIslands<T: MeshType<Face = Self>>: FaceBasics<T> {
    /// Returns the number of holes in the face.
    #[must_use]
    fn num_islands(&self, mesh: &T::Mesh) -> usize {
        self.islands(mesh).count()
    }

    /// Returns an iterator with an edge id for each edge chain adjacent of the face.
    /// The first edge id is for the outer edge chain of the face.
    #[must_use]
    fn islands<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::E> + CreateEmptyIterator
    where
        T: 'a;

    /// Adds a new island to the face.
    /// Returns the id of the island if it was added (same as the given edge id), otherwise None.
    /// The edge used to add to the island mustn't have a face.
    #[must_use]
    fn add_island(&self, mesh: &mut T::Mesh, island: T::E) -> Option<T::E>;

    /// Returns the edge id of the island that points to the given island.
    /// Returns None if the island is not part of the face.
    /// Can be used to test whether an island is part of the face.
    #[must_use]
    fn find_prev_island(&self, mesh: &T::Mesh, island: T::E) -> Option<T::E> {
        let mut prev = IndexType::max();
        let mut iter = self.islands(mesh);
        while let Some(e) = iter.next() {
            if e == island {
                if prev != IndexType::max() {
                    return Some(prev);
                }
                // island is the first one, return the last element in the wheel
                return iter.last();
            }
            prev = e;
        }
        None
    }

    /// Removes the specified island from the face by searching the island wheel.
    /// Returns the id of the island if it was removed, otherwise None.
    ///
    /// The island / hole is kept as a disconnected an open boundary.
    // TODO: don't require halfedge
    #[must_use]
    fn remove_island(&self, _mesh: &mut T::Mesh, _island: T::E) -> Option<T::E>
    where
        T: MeshTypeHalfEdge<Face = Self>,
    {
        todo!();
        // Cannot use the cursor here
        /*mesh.edge_mut(self.find_prev_island(mesh, island)?)
        .face()
        .remove_next_island()
        .id()*/
    }

    /// Removes the island this island is pointing to (the "next" one) from the island wheel.
    /// Returns the id of the island if it was removed, otherwise None.
    ///
    /// The island / hole is kept as a disconnected open boundary.
    #[must_use]
    fn remove_next_island(mesh: &mut T::Mesh, f: T::F) -> Option<T::E>;

    /// Returns true if any island of the face has this particular edge.
    #[must_use]
    #[inline]
    fn face_has_island_edge(&self, mesh: &T::Mesh, edge: T::E) -> bool {
        self.islands(mesh)
            .any(|e| mesh.edge(e).chain().any(|c| c.id() == edge))
    }

    /// Iterates the face ids of the islands of this face, i.e., the face participating to the island wheel for each representative edge from [HasIslands::islands].
    /// These ids might contain duplicates or IndexType::max() depending on the implementation of the islands.
    #[must_use]
    fn islands_faces<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::F> + 'a
    where
        T: 'a,
    {
        self.islands(mesh).map(|e| {
            mesh.edge(e)
                .faces()
                .find(|f| f.inner().face_has_island_edge(mesh, self.edge_id()))
                .map(|f| f.id())
                .unwrap_or(IndexType::max())
        })
    }

    /// Returns the representative edge of the island wheel.
    /// If the islands are represented by a circular linked list of faces, this is one of the faces in the wheel characterized by some arbitrary criterion (e.g., the face with the smallest id).
    /// If the islands share a single face, this is that face.
    /// If the face has no islands, this returns the face itself.
    #[must_use]
    fn islands_representative_face(&self, mesh: &T::Mesh) -> T::F {
        let Some(f) = self.islands_faces(mesh).min() else {
            // no islands, return the face itself
            return self.id();
        };
        if f == IndexType::max() {
            // no islands, return the face itself
            self.id()
        } else {
            f
        }
    }
}

/*
/// A FacePayload that has a circular linked list of islands.
pub trait IslandFacePayload<T: MeshType>: FacePayload + Default {
    /// Returns the next island in the circular linked list.
    /// Returns it's own id if there are no islands.
    fn next_island(&self) -> T::F;

    /// Sets the next island in the circular linked list.
    fn set_next_island(&mut self, island: T::F);
}*/

/// A Trait for a face that uses the circular linked list of islands.
pub trait IslandCircularLinkedList<T: MeshType> {
    /// Returns the next island in the circular linked list.
    /// Returns it's own id if there are no islands.
    fn next_island(&self) -> T::F;

    /// Sets the next island in the circular linked list.
    fn set_next_island(&mut self, island: T::F);
}

// TODO: requires trait inheritance which is not yet stabilized
/*
impl<T: MeshType<Face = Self>, F: FaceBasics<T>> IslandCircularLinkedList<T> for F
where
    T::FP: IslandFacePayload<T>,
{
    fn next_island(&self) -> T::F {
        self.payload().next_island()
    }

    fn set_next_island(&mut self, island: T::F) {
        self.payload_mut().set_next_island(island);
    }
}*/

/// An iterator over the islands of a face based on [IslandFacePayload].
///
/// The iterator ends without a panic if the wheel is invalid.
///
/// The iterator will not terminate if the wheel doesn't reach the first face again.
pub struct IslandListIterator<'a, T: MeshType>
where
    T::Face: IslandCircularLinkedList<T>,
{
    first: T::F,
    pos: T::F,
    mesh: Option<&'a T::Mesh>,
}

impl<'a, T: MeshType> IslandListIterator<'a, T>
where
    T::Face: IslandCircularLinkedList<T>,
{
    /// Creates a new iterator over the islands of the given face.
    pub fn new(mesh: &'a T::Mesh, start: T::F) -> Self {
        Self {
            pos: start,
            first: start,
            mesh: Some(mesh),
        }
    }
}

impl<'a, T: MeshType> CreateEmptyIterator for IslandListIterator<'a, T>
where
    T::Face: IslandCircularLinkedList<T>,
{
    fn create_empty() -> Self {
        Self {
            first: Default::default(),
            pos: Default::default(),
            mesh: None,
        }
    }
}

impl<'a, T: MeshType> Iterator for IslandListIterator<'a, T>
where
    T::Face: IslandCircularLinkedList<T>,
{
    type Item = T::E;

    fn next(&mut self) -> Option<Self::Item> {
        let mesh = self.mesh?;
        let current = self.pos;
        let c = mesh.face(current).load()?;
        self.pos = c.inner().next_island();
        if self.pos == self.first {
            // wheel done, stop iterating
            self.mesh = None;
        }
        Some(c.edge_id())
    }
}

/// A default implementation for HasIslands when the face payload implements IslandFacePayload.
impl<T: MeshType<Face = Self>, F: FaceBasics<T>> HasIslands<T> for F
where
    T::Face: IslandCircularLinkedList<T>,
{
    fn islands<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::E> + CreateEmptyIterator
    where
        T: 'a,
    {
        IslandListIterator::<'a, T>::new(mesh, self.id())
    }

    fn add_island(&self, mesh: &mut T::Mesh, island_e: T::E) -> Option<T::E> {
        let next = mesh.face(self.id()).try_inner()?.next_island();

        // TODO: check if island_e is a valid edge id
        /*if mesh.edge(island_e).load()?.face_id() != IndexType::max() {
            // island edge already has a face, cannot add it as an island
            return None;
        }*/

        // TODO: create a shared face payload for the island instead of cloning!
        let fp = mesh.face(self.id()).load()?.payload().clone();

        let island_f = mesh.insert_face(island_e, fp)?;

        mesh.face_mut(self.id())
            .load()?
            .inner_mut()
            .set_next_island(island_f);

        mesh.face_mut(island_f)
            .load()?
            .inner_mut()
            .set_next_island(next);

        Some(island_e)
    }

    fn remove_next_island(mesh: &mut T::Mesh, id: T::F) -> Option<T::E> {
        let other: T::F = mesh.face(id).load()?.inner().next_island();
        if other == id {
            // no islands, nothing to remove
            return None;
        }

        let next: T::F = mesh.face(other).load()?.inner().next_island();
        mesh.face_mut(id).load()?.inner_mut().set_next_island(next);
        mesh.face_mut(other).load()?.remove().ensure_void();
        let other_e = mesh.face(other).load()?.edge_id();
        Some(other_e)
    }
}
