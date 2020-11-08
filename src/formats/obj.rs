/// An .obj geometric vertex
pub struct ObjVertex(pub f32, pub f32, pub f32, pub f32);

impl ToString for ObjVertex {
    fn to_string(&self) -> String {
        format!("v {} {} {} {}", self.0, self.0, self.0, self.0)
    }
}

impl From<(f32, f32, f32)> for ObjVertex {
    fn from(xyz: (f32, f32, f32)) -> Self {
        Self(xyz.0, xyz.1, xyz.2, 1.0)
    }
}

impl From<(f32, f32, f32, f32)> for ObjVertex {
    fn from(xyzw: (f32, f32, f32, f32)) -> Self {
        Self(xyzw.0, xyzw.1, xyzw.2, xyzw.3)
    }
}

/// An .obj texture coordinate
pub struct ObjTextureCoord(pub f32, pub f32, pub f32);

impl ToString for ObjTextureCoord {
    fn to_string(&self) -> String {
        format!("vt {} {} {}", self.0, self.1, self.2)
    }
}

impl From<f32> for ObjTextureCoord {
    fn from(u: f32) -> Self {
        assert!(u >= 0.0);
        assert!(u <= 1.0);

        Self(u, 0.0, 0.0)
    }
}

impl From<(f32, f32)> for ObjTextureCoord {
    fn from(uv: (f32, f32)) -> Self {
        assert!(uv.0 >= 0.0);
        assert!(uv.0 <= 1.0);
        assert!(uv.1 >= 0.0);
        assert!(uv.1 <= 1.0);

        Self(uv.0, uv.1, 0.0)
    }
}

impl From<(f32, f32, f32)> for ObjTextureCoord {
    fn from(uvw: (f32, f32, f32)) -> Self {
        assert!(uvw.0 >= 0.0);
        assert!(uvw.0 <= 1.0);
        assert!(uvw.1 >= 0.0);
        assert!(uvw.1 <= 1.0);
        assert!(uvw.2 >= 0.0);
        assert!(uvw.2 <= 1.0);

        Self(uvw.0, uvw.1, uvw.2)
    }
}

/// An .obj vertex normal
pub struct ObjNormal(pub f32, pub f32, pub f32);

impl ToString for ObjNormal {
    fn to_string(&self) -> String {
        format!("vn {} {} {}", self.0, self.1, self.2)
    }
}

impl From<(f32, f32, f32)> for ObjNormal {
    fn from(xyz: (f32, f32, f32)) -> Self {
        Self(xyz.0, xyz.1, xyz.2)
    }
}

/// An .obj parameter space vertex
pub struct ObjParamSpaceValue(pub f32, pub f32, pub f32);

impl ToString for ObjParamSpaceValue {
    fn to_string(&self) -> String {
        format!("vp {} {} {}", self.0, self.1, self.2)
    }
}

impl From<f32> for ObjParamSpaceValue {
    fn from(u: f32) -> Self {
        assert!(u >= 0.0);
        assert!(u <= 1.0);

        Self(u, 0.0, 0.0)
    }
}

impl From<(f32, f32)> for ObjParamSpaceValue {
    fn from(uv: (f32, f32)) -> Self {
        assert!(uv.0 >= 0.0);
        assert!(uv.0 <= 1.0);
        assert!(uv.1 >= 0.0);
        assert!(uv.1 <= 1.0);

        Self(uv.0, uv.1, 0.0)
    }
}

impl From<(f32, f32, f32)> for ObjParamSpaceValue {
    fn from(uvw: (f32, f32, f32)) -> Self {
        assert!(uvw.0 >= 0.0);
        assert!(uvw.0 <= 1.0);
        assert!(uvw.1 >= 0.0);
        assert!(uvw.1 <= 1.0);
        assert!(uvw.2 >= 0.0);
        assert!(uvw.2 <= 1.0);

        Self(uvw.0, uvw.1, uvw.2)
    }
}

/// An .obj face
pub struct ObjFace {
    pub vertices: (usize, usize, usize),
    pub textures: Option<(usize, usize, usize)>,
    pub normals: Option<(usize, usize, usize)>,
}

impl ToString for ObjFace {
    fn to_string(&self) -> String {
        let v = self.vertices;

        if let Some(t) = self.textures {
            if let Some(n) = self.normals {
                return format!(
                    "f {}/{}/{} {}/{}/{} {}/{}/{}",
                    v.0, t.0, n.0, v.1, t.1, n.1, v.2, t.2, n.2,
                );
            }

            return format!("f {}/{} {}/{} {}/{}", v.0, t.0, v.1, t.1, v.2, t.2,);
        }

        if let Some(n) = self.normals {
            return format!("f {}//{} {}//{} {}//{}", v.0, n.0, v.1, n.1, v.2, n.2,);
        }

        format!("f {} {} {}", v.0, v.1, v.2)
    }
}

#[allow(clippy::type_complexity)]
impl
    From<(
        (usize, usize, usize),
        Option<(usize, usize, usize)>,
        Option<(usize, usize, usize)>,
    )> for ObjFace
{
    fn from(
        vtx: (
            (usize, usize, usize),
            Option<(usize, usize, usize)>,
            Option<(usize, usize, usize)>,
        ),
    ) -> Self {
        Self {
            vertices: vtx.0,
            textures: vtx.1,
            normals: vtx.2,
        }
    }
}

/// An .obj line
pub struct ObjLine {
    vertices: Vec<usize>,
}

impl ToString for ObjLine {
    fn to_string(&self) -> String {
        let vertices = self
            .vertices
            .iter()
            .map(|index| index.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        format!("l {}", vertices)
    }
}

impl From<Vec<usize>> for ObjLine {
    fn from(vertices: Vec<usize>) -> Self {
        Self { vertices }
    }
}

/// An .obj file
pub struct ObjFile {
    pub v: Vec<ObjVertex>,
    pub vt: Vec<ObjTextureCoord>,
    pub vn: Vec<ObjNormal>,
    pub vp: Vec<ObjParamSpaceValue>,
    pub f: Vec<ObjFace>,
    pub l: Vec<ObjLine>,
}

impl ToString for ObjFile {
    fn to_string(&self) -> String {
        let v: String = self
            .v
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let vt: String = self
            .vt
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let vn: String = self
            .vn
            .iter()
            .map(|index| index.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let vp: String = self
            .vp
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let f: String = self
            .f
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let l: String = self
            .l
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        v + "\n" + &vt + "\n" + &vn + "\n" + &vp + "\n" + &f + "\n" + &l
    }
}
