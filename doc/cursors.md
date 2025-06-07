# Cursors

The general idea is very simple: a cursor holds a reference to a mesh and a position in that mesh. That way, you

-   don't have to think about which mesh your ids are referring to,
-   can defer error handling using `void` cursors,
-   can use a functional style to traverse the mesh, and
-   have two less parameters to pass to basically every function.

To make this work, there are 3 \* 2 \* 2 = 12 types of cursors by combining the following options:

-   The type of position:

    1. [crate::mesh::cursor::EdgeCursor]: The cursor points to an edge.
    2. [crate::mesh::cursor::VertexCursor]: The cursor points to a vertex.
    3. [crate::mesh::cursor::FaceCursor]: The cursor points to a face.

-   The mutability of the position:

    1. [crate::mesh::cursor::ImmutableCursor]: The cursor cannot be used to modify the mesh. However, you may have multiple immutable cursors to the same mesh.
    2. [crate::mesh::cursor::MutableCursor]: The cursor can be used to modify the mesh. However, due to the borrow checker, you can only have one mutable cursor to a mesh at a time.

-   Whether the position is known to be valid in the associated mesh:
    1.  [crate::mesh::cursor::MaybeCursor]: The cursor may be `void`. This is usually the default. Traversing from a `void` cursor will return a `void` cursor or an empty iterator and most operations become no-ops. Convert it to a `ValidCursor` using [crate::mesh::cursor::CursorData::load].
    2.  [crate::mesh::cursor::ValidCursor]: The cursor is known to be valid. You can directly access additional information about the referenced mesh position without further error handling.

Many functions defined on the mesh return an appropriate cursor to continue working with the mesh, e.g., you can create a cube like this:

```rust
let p = Vec3::new(1.0, 1.0, 1.0);
let vs = [
    (-p.x(), -p.y()),
    (p.x(), -p.y()),
    (p.x(), p.y()),
    (-p.x(), p.y()),
];
let mut mesh = BevyMesh3d::new();
mesh
    // create the bottom face by inserting a polygon
    .insert_polygon(vs.iter().map(|(x, y)| vp(*x, *y, -p.z())))
    // go to the next edge
    .next()
    // The parameters 2 and 2 specify that the loft should create polygons
    // with 2 vertices at the top and 2 at the bottom, i.e., quadrilaterals.
    .loft(2, 2, vs.iter().map(|(x, y)| vp(*x, *y, p.z())))
    // close the top face
    .insert_face(Default::default());
```

Sometimes, pleasing the borrow checker might be a bit tricky. In that case, you can still fall back to referring to edges, vertices, and faces using their ids. Cursors provide their ids using the `id` method. In case of `MaybeCursor`s, the `id` method will return an `Option<IndexType>`. If you want to retrieve the id directly no matter if the cursor is `void` or not, you can use the `id_unchecked` method. To convert an id back to a cursor, you can use

-   [crate::mesh::MeshBasics::edge],
-   [crate::mesh::MeshBasics::edge_mut],
-   [crate::mesh::MeshBasics::vertex],
-   [crate::mesh::MeshBasics::vertex_mut],
-   [crate::mesh::MeshBasics::face], or
-   [crate::mesh::MeshBasics::face_mut].

## Traversing Half-Edges

Large parts of this package are based on half-edge meshes. When traversing half-edges, we use the following nomenclature:

<svg xmlns="http://www.w3.org/2000/svg" viewBox="-40 0 350 120" xmlns:bx="https://boxy-svg.com">
  <style>
    .text{
        white-space: pre; 
        fill: rgb(51, 51, 51); 
        font-family: &quot;Arial&quot;, sans-serif; 
        font-size: 28px;
    }
    .circle {
        stroke: rgb(0, 0, 0); 
        fill: none;
    }
    .arrow {
        paint-order: stroke; 
        stroke: rgb(0, 0, 0); 
    }
  </style>
  <path class="arrow" style="transform-origin: 66.984px 103.819px 0px;" d="M 111.102 101.032 L 22.865 101.032 L 46.523 106.606 L 46.523 101.032"></path>
  <circle class="circle" cx="124.184" cy="98.79" r="10.0"></circle>
  <circle class="circle" cx="11.723" cy="97.622" r="10.0"></circle>
  <path class="arrow" style="transform-origin: 179.021px 103.288px 0px;" d="M 222.327 100.501 L 135.716 100.501 L 158.937 106.075 L 158.937 100.501"></path>
  <path class="arrow" style="transform-origin: 37.319px 92.995px 0px;" d="M 135.978 96.043 L 222.587 95.521 L 199.332 90.087 L 199.366 95.661"></path>
  <circle class="circle" cx="234.75" cy="98.195" r="10.0"></circle>
  <path class="arrow" style="transform-origin: 133.244px 56.262px 0px;" d="M 142.139 25.67 L 128.829 87.134 L 139.274 65.35 L 133.749 64.191"></path>
  <path class="arrow" style="transform-origin: 133.244px 56.262px 0px;" d="M 124.69 87.004 L 137.988 24.27 L 127.675 46.117 L 133.207 47.24"></path>
  <circle style="stroke: rgb(0, 0, 0); fill: none; transform-origin: 200.294px 96.694px 0px;" cx="244.648" cy="96.694" r="10.0" transform="matrix(0.200972, -0.979597, 0.979597, 0.200972, -66.246977, -40.233557)"></circle>
  <path class="arrow" style="transform-origin: 76.996px 53.893px 0px;" d="M 21.847 87.922 L 130.377 15.204 L 108.488 23.285 L 111.016 28.087"></path>
  <path class="arrow" style="transform-origin: -22.439px 93.526px 0px;" d="M 23.134 96.574 L 111.368 96.052 L 87.677 90.618 L 87.712 96.192"></path>
  <path class="arrow" style="transform-origin: 77.648px 53.567px 0px;" d="M 131.655 19.734 L 23.084 92.378 L 44.977 84.313 L 42.457 79.511"></path>
  <text class="text" x="90.069" y="186.482" transform="matrix(0.59291, 0, 0, 0.554809, 64.38118, -1.426582)">v</text>
  <text class="text" x="101.08" y="186.335" transform="matrix(0.37095, 0, 0, 0.30321, 87.249008, 49.146267)">1</text>
  <text class="text" x="90.069" y="186.482" transform="matrix(0.59291, 0, 0, 0.554809, -48.255749, -2.931327)">v</text>
  <text class="text" x="101.08" y="186.335" transform="matrix(0.37095, 0, 0, 0.30321, -25.387926, 47.641541)">0</text>
  <text class="text" x="90.069" y="186.482" transform="matrix(0.59291, 0, 0, 0.554809, 82.936539, -87.477455)">v</text>
  <text class="text" x="101.08" y="186.335" transform="matrix(0.37095, 0, 0, 0.30321, 105.804367, -36.904602)">2</text>
  <text class="text" x="90.069" y="186.482" transform="matrix(0.59291, 0, 0, 0.554809, 175.593277, -2.380363)">v</text>
  <text class="text" x="101.08" y="186.335" transform="matrix(0.37095, 0, 0, 0.30321, 198.461105, 48.192505)">3</text>
  <text class="text" x="90.069" y="186.482" transform="matrix(0.59291, 0, 0, 0.554809, 37.748096, -30.991905)">f</text>
  <text class="text" x="101.08" y="186.335" transform="matrix(0.37095, 0, 0, 0.30321, 58.615925, 19.580963)">0</text>
  <text class="text" x="90.069" y="186.482" transform="matrix(0.508673, 0, 0, 0.554809, 13.621807, 8.094659)">e</text>
  <text class="text" x="101.08" y="186.335" transform="matrix(0.318247, 0, 0, 0.30321, 34.956543, 58.667507)">10</text>
  <text class="text" x="90.069" y="186.482" transform="matrix(0.508673, 0, 0, 0.554809, 13.667065, -12.536216)">e</text>
  <text class="text" x="101.08" y="186.335" transform="matrix(0.318247, 0, 0, 0.30321, 35.001801, 38.036652)">01</text>
</svg>

-   **half-edge**: an arc in an otherwise undirected graph.
-   **edge**: an undirected edge in a graph, often identified by a single half-edge (the one with the lower vertex id as source) representing the pair of half-edges.
-   **vertex**: a point in the graph
-   **face**: a polygon in the graph that is part of the surface of the mesh. Each half-edge is associated with at most one face (if the mesh is manifold), e.g., `e01`s face is `f0` while `e10`s does not have a face. On the contrary, each face is associated with exactly one incident half-edge, e.g., `f0`'s incident half-edge could either be `e01`, `e12`, or `e21`. The (half-)edges of a face are always ordered in a counter-clockwise manner.
-   **next**: the clockwise next half-edge at the target, e.g., `e01`'s next half-edge is `e12`. Notice that this definition is independent of the actual vertex positions of the mesh -- if you assign the counter-clockwise next half-edge to `next`, all adjacent faces are implicitly flipped, restoring the original clockwise order.
-   **prev**: the counter-clockwise next half-edge at the source, e.g., `e01`'s previous half-edge is `e20`.
-   **twin**: the half-edge in the opposite direction, e.g., `e01`'s twin is `e10`.
-   **source**: the vertex at the start of the half-edge, e.g., `e01`'s source is `v0`.
-   **target**: the vertex at the end of the half-edge, e.g., `e01`'s target is `v1`.
-   **next_sibling**: the counter-clockwise next half-edge with the same source vertex, e.g., `e01`'s next sibling is `e02` and `e10`'s next sibling is `e12`.
-   **boundary**: a chain of half-edges that is connected in a circle and might or might not be part of a face. The above example has only two boundaries, namely `[e01, e12, e10]` and `[e10, e02, e21, e13, e31]`. TODO: make boundary separate from chains. Current term boundary should be chain.
-   **manifold**: a mesh is manifold if it is _edge-manifold_, i.e., each half-edge pair has exactly two faces, and it is _vertex-manifold_, i.e., each vertex' 1-ring neighborhood is homeomorphic to a disk. In simpler terms, this means that the mesh is the surface of a closed volume without any holes or "thin bridges".
-   **open-manifold**: a mesh is open-manifold if each half-edge pair has one or two faces and it is _vertex-manifold_. In simpler terms, this means, that the mesh may have holes, but there are still no "thin bridges".

## Useful Patterns

When mutating the mesh using cursor, you can't create other cursors referencing the same mesh. Though, sometimes you need to refer to another position in the mesh, e.g., when closing a face. The solution to this is to create a local variable holding the id of the other position before creating the mutable cursor. To streamline this process, you can use the [crate::mesh::cursor::CursorData::with_id] method, e.g.,

```rust
 some_edge_cursor.with_id(|cursor, e0| {
        cursor.insert_vertex(..., ...)
              .close_face(e0, ..., ...)
    })
```

Some other functions you should know:

-   [crate::mesh::cursor::CursorData::stay]: Run the given closure on the cursor and return the un-moved cursor afterwards.
-   [crate::mesh::cursor::CursorData::load_or_else]: Run the first closure if the cursor is void, run the second closure on the `ValidCursor` otherwise.
-   [crate::mesh::cursor::CursorData::load_or_void]: Run the closure on the loaded `ValidCursor` if the cursor is valid, or return `void` without doing anything else.
-   [crate::mesh::cursor::CursorData::ensure]: Panics if the cursor is void. This is useful to assert that some chain of operation was successful.

## Performance

You should prefer using Cursors over direct access to the mesh data structures whenever possible.
You don't have to worry about performance, as the rust compiler will completely optimize them away. Cloning immutable cursors is also optimized away, so feel free to clone them as much as you like.

For example, when compiling `cursor.next().next().next().next()`, all function
calls will be inlined leading to the same 8 commands for each call to `next`:

```ir
getelementptr + load    ; compute address of and load the `id` in the `HalfEdgeImpl` in the `Vec`
icmp + br               ; if the `id` is `IndexType::max()`, skip all further blocks (since it is deleted, and the cursor, hence, void)
getelementptr + load    ; compute address of and load the `next_id` in the `HalfEdgeImpl`
icmp + br               ; if the `next_id` exceeds the length of the `Vec` or is `IndexType::max()`, skip all further blocks
```

(using `cargo rustc -- --emit=llvm-ir -O -C debuginfo=2`)

## Payloads

TODO
