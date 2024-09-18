use super::HalfEdgeMesh;
use crate::halfedge::HalfEdgeMeshType;

impl<T: HalfEdgeMeshType> std::fmt::Display for HalfEdgeMesh<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO:
        /*write!(
            f,
            "Mesh:\nvertices:\n{}\n edge --><-- twin   |  face: edge/twin \n{}\n faces: \n{}\n{} ",
            self.vertices()
                .map(|v| format!("{}", v))
                .collect::<Vec<_>>()
                .join("\n"),
            self.pair_edges()
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.faces()
                .map(|f| format!("{}", f))
                .collect::<Vec<_>>()
                .join("\n"),
            if let Err(msg) = self.check() {
                format!(
                    "⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ERROR ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️\n{}",
                    msg
                )
            } else {
                "".to_string()
            }
        );*/
        todo!("Display not implemented yet");
    }
}
