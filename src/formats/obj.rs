
/// Tries to load the given .obj file content
/// WIP and not really well done.
pub fn load(content: String) -> ObjFile {
    let mut obj_file = ObjFile::default();

    content.lines().enumerate().for_each(|(index, line)| {
        let mut splits = line.split_whitespace();

        match splits.next() {
            Some("v") => obj_file.add_v(parse_v(&mut splits)),
            Some("vt") => obj_file.add_vt(parse_vt(&mut splits)),
            Some("vn") => obj_file.add_vn(parse_vn(&mut splits)),
            Some("vp") => obj_file.add_vp(parse_vp(&mut splits)),
            Some("f") => obj_file.add_f(parse_f(&mut splits)),
            Some("l") => obj_file.add_l(parse_l(&mut splits)),
            Some("#") => {}
            _ => panic!("Cannot parse line {}", index),
        };
    });

    obj_file.assert_ok();
    obj_file.shrink_to_fit();

    obj_file
}

fn parse_f32(string: &mut dyn Iterator<Item=&str>, default: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    let mut f32s = default;
    string.enumerate()
        .for_each(|(i, s)| {
            match i {
                0 => f32s.0 = s.parse().unwrap(),
                1 => f32s.1 = s.parse().unwrap(),
                2 => f32s.2 = s.parse().unwrap(),
                3 => f32s.3 = s.parse().unwrap(),
                _ => panic!()
            }
        });

    f32s
}

fn parse_usize(string: &mut dyn Iterator<Item=&str>) -> (usize, usize, usize) {
    let mut usizes = (usize::MAX, usize::MAX, usize::MAX);
    string.enumerate()
        .for_each(|(i, s)| {
            match i {
                0 => usizes.0 = s.parse().unwrap(),
                1 => usizes.1 = s.parse().unwrap(),
                2 => usizes.2 = s.parse().unwrap(),
                _ => panic!()
            }
        });

    usizes
}

fn parse_v(string: &mut dyn Iterator<Item=&str>) -> ObjVertex {
    let f32s = parse_f32(string, (f32::NAN, f32::NAN, f32::NAN, 1.0));

    ObjVertex::from(f32s)
}

fn parse_vt(string: &mut dyn Iterator<Item=&str>) -> ObjTextureCoord {
    let f32s = parse_f32(string, (f32::NAN, 0.0, 0.0, f32::NAN));
    let f32s = (f32s.0, f32s.1, f32s.2);

    ObjTextureCoord::from(f32s)
}

fn parse_vn(string: &mut dyn Iterator<Item=&str>) -> ObjNormal {
    let f32s = parse_f32(string, (f32::NAN, f32::NAN, f32::NAN, f32::NAN));
    let f32s = (f32s.0, f32s.1, f32s.2);

    ObjNormal::from(f32s)
}

fn parse_vp(string: &mut dyn Iterator<Item=&str>) -> ObjParamSpaceValue {
    let f32s = parse_f32(string, (f32::NAN, f32::NAN, f32::NAN, f32::NAN));
    let f32s = (f32s.0, f32s.1, f32s.2);

    ObjParamSpaceValue::from(f32s)
}

// TODO: Parse this correctly!
fn parse_f(string: &mut dyn Iterator<Item=&str>) -> ObjFace {
    let usizes = parse_usize(string);

    ObjFace::from(usizes)
}

fn parse_l(string: &mut dyn Iterator<Item=&str>) -> ObjLine {
    let vertices: Vec<usize> = string.map(|s| s.parse::<usize>().unwrap()).collect();

    ObjLine::from(vertices)
}

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
        Self(u, 0.0, 0.0)
    }
}

impl From<(f32, f32)> for ObjParamSpaceValue {
    fn from(uv: (f32, f32)) -> Self {
        Self(uv.0, uv.1, 0.0)
    }
}

impl From<(f32, f32, f32)> for ObjParamSpaceValue {
    fn from(uvw: (f32, f32, f32)) -> Self {
        Self(uvw.0, uvw.1, uvw.2)
    }
}

/// An .obj face
pub struct ObjFace {
    pub v: (usize, usize, usize),
    pub vt: Option<(usize, usize, usize)>,
    pub vn: Option<(usize, usize, usize)>,
}

impl ToString for ObjFace {
    fn to_string(&self) -> String {
        let v = self.v;

        if let Some(t) = self.vt {
            if let Some(n) = self.vn {
                return format!(
                    "f {}/{}/{} {}/{}/{} {}/{}/{}",
                    v.0, t.0, n.0, v.1, t.1, n.1, v.2, t.2, n.2,
                );
            }

            return format!("f {}/{} {}/{} {}/{}", v.0, t.0, v.1, t.1, v.2, t.2, );
        }

        if let Some(n) = self.vn {
            return format!("f {}//{} {}//{} {}//{}", v.0, n.0, v.1, n.1, v.2, n.2, );
        }

        format!("f {} {} {}", v.0, v.1, v.2)
    }
}

impl From<(usize, usize, usize)> for ObjFace {
    fn from(v: (usize, usize, usize)) -> Self {
        Self { v, vt: None, vn: None }
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
            v: vtx.0,
            vt: vtx.1,
            vn: vtx.2,
        }
    }
}

/// An .obj line
pub struct ObjLine {
    v: Vec<usize>,
}

impl ToString for ObjLine {
    fn to_string(&self) -> String {
        let vertices = self
            .v
            .iter()
            .map(|index| index.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        format!("l {}", vertices)
    }
}

impl From<Vec<usize>> for ObjLine {
    fn from(vertices: Vec<usize>) -> Self {
        Self { v: vertices }
    }
}

/// An .obj file
#[derive(Default)]
pub struct ObjFile {
    pub v: Vec<ObjVertex>,
    pub vt: Vec<ObjTextureCoord>,
    pub vn: Vec<ObjNormal>,
    pub vp: Vec<ObjParamSpaceValue>,
    pub f: Vec<ObjFace>,
    pub l: Vec<ObjLine>,
}

impl ObjFile {
    pub fn add_v(&mut self, v: ObjVertex) {
        self.v.push(v);
    }
    pub fn add_vt(&mut self, vt: ObjTextureCoord) {
        self.vt.push(vt);
    }
    pub fn add_vn(&mut self, vn: ObjNormal) {
        self.vn.push(vn);
    }
    pub fn add_vp(&mut self, vp: ObjParamSpaceValue) {
        self.vp.push(vp);
    }
    pub fn add_f(&mut self, f: ObjFace) {
        self.f.push(f);
    }
    pub fn add_l(&mut self, l: ObjLine) {
        self.l.push(l);
    }
    
    pub fn get_v(&self, index: usize) -> &ObjVertex {
        &self.v[index - 1]
    }

    pub fn get_vt(&self, index: usize) -> &ObjTextureCoord {
        &self.vt[index - 1]
    }

    pub fn get_vn(&self, index: usize) -> &ObjNormal {
        &self.vn[index - 1]
    }

    pub fn shrink_to_fit(&mut self) {
        self.v.shrink_to_fit();
        self.vt.shrink_to_fit();
        self.vn.shrink_to_fit();
        self.vp.shrink_to_fit();
        self.f.shrink_to_fit();
        self.l.shrink_to_fit();
    }

    pub fn assert_ok(&self) {
        self.f
            .iter()
            .for_each(|f| {
                let len = self.v.len();
                assert!(f.v.0 <= len);
                assert!(f.v.1 <= len);
                assert!(f.v.2 <= len);

                if let Some(vt) = f.vt {
                    let len = self.vt.len();
                    assert!(vt.0 <= len);
                    assert!(vt.1 <= len);
                    assert!(vt.2 <= len);
                }

                if let Some(vn) = f.vn {
                    let len = self.vn.len();
                    assert!(vn.0 <= len);
                    assert!(vn.1 <= len);
                    assert!(vn.2 <= len);
                }
            });

        self.l
            .iter()
            .for_each(|l| {
                let len = self.v.len();
                l.v
                    .iter()
                    .for_each(|u| assert!(*u < len));
            })
    }
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
