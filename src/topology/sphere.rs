use glam::Vec3;
use geometer::{
	Geometry,
	geometry,
	vertex
};

const MAX_PRECISION: u32 = 8;

// The number of faces at precision n, F(n), is given by:
// F(0) = 1
// F(n+1) = F(n) * 4
// In other words, F(n) = 4^n.

// The number of edges at precision n, E(n), is given by:
// E(0) = 3
// E(n+1) = 4*E(n) - 3*2^n

/// The number of vertices at precision n, V(n), is given by:
/// V(0) = 3
/// V(n+1) = 4*E(n) - 3*(1 + 2^n)
const fn vertex_count(n: u32) -> usize {
	if n == 0 {
		3
	} else {
		4 * vertex_count(n - 1) - 3 * (1 + 2usize.pow(n - 1))
	}
}

pub type Vertex = vertex::Position3;

/// Icosahedron sphere.
pub struct Sphere {
	/// The 20 faces of the icosahedron.
	pub regions: [Geometry<Vertex>; 20]
}

impl Sphere {
	pub fn new() -> Sphere {
		let t = (1.0 + 5.0f32.sqrt()) / 2.0;

		let v = [
			Vec3::new(-1.0,  t  ,  0.0),
			Vec3::new( 1.0,  t  ,  0.0),
			Vec3::new(-1.0, -t  ,  0.0),
			Vec3::new( 1.0, -t  ,  0.0),
			Vec3::new( 0.0, -1.0,  t  ),
			Vec3::new( 0.0,  1.0,  t  ),
			Vec3::new( 0.0, -1.0, -t  ),
			Vec3::new( 0.0,  1.0, -t  ),
			Vec3::new( t  ,  0.0, -1.0),
			Vec3::new( t  ,  0.0,  1.0),
			Vec3::new(-t  ,  0.0, -1.0),
			Vec3::new(-t  ,  0.0,  1.0)
		];

		Sphere {
			regions: [
				// 5 faces around point 0
				new_region(v[0], v[11], v[5]),
				new_region(v[0], v[5], v[1]),
				new_region(v[0], v[1], v[7]),
				new_region(v[0], v[7], v[10]),
				new_region(v[0], v[10], v[11]),

				// 5 adjacent faces
				new_region(v[1], v[5], v[9]),
				new_region(v[5], v[11], v[4]),
				new_region(v[11], v[10], v[2]),
				new_region(v[10], v[7], v[6]),
				new_region(v[7], v[1], v[8]),

				// 5 faces around point 3
				new_region(v[3], v[9], v[4]),
				new_region(v[3], v[4], v[2]),
				new_region(v[3], v[2], v[6]),
				new_region(v[3], v[6], v[8]),
				new_region(v[3], v[8], v[9]),

				// 5 adjacent faces
				new_region(v[4], v[9], v[5]),
				new_region(v[2], v[4], v[11]),
				new_region(v[6], v[2], v[10]),
				new_region(v[8], v[6], v[7]),
				new_region(v[9], v[8], v[1])
			]
		}
	}
}

fn new_region(a: Vec3, b: Vec3, c: Vec3) -> Geometry<Vertex> {
	let mut vertices = Vec::new();
	vertices.resize(vertex_count(MAX_PRECISION), vertex::Position3::new(0.0, 0.0, 0.0));

	vertices[0] = vertex::Position3(a);
	vertices[1] = vertex::Position3(b);
	vertices[2] = vertex::Position3(c);

	let mut indices = Vec::new();
	indices.resize(MAX_PRECISION as usize, geometry::Precision::new());

	initialize_vertices(0, 1, 2, 0, &mut vertices, &mut indices);

	let mut geometry = Geometry::new(vertices);
	for p in indices {
		geometry.add_precision(p)
	}

	geometry
}

fn initialize_vertices(a: u32, b: u32, c: u32, depth: u32, vertices: &mut [Vertex], indices: &mut [geometry::Precision]) {
	let depth_indices = &mut indices[depth as usize];
	depth_indices.add(geometry::Face::new(a, b, c));

	if depth < MAX_PRECISION {
		//       a
		//     /   \
		//    e --- f
		//  /   \ /   \
		// c --- d --- b
		use super::utils::nrbpi2_mean;

		let a_pos = vertices[a as usize].0;
		let b_pos = vertices[b as usize].0;
		let c_pos = vertices[c as usize].0;

		let d = nrbpi2_mean(b, c);
		let e = nrbpi2_mean(c, a);
		let f = nrbpi2_mean(a, b);

		vertices[d as usize] = vertex::Position3(((b_pos + c_pos) / 2.0).normalize());
		vertices[e as usize] = vertex::Position3(((c_pos + a_pos) / 2.0).normalize());
		vertices[f as usize] = vertex::Position3(((a_pos + b_pos) / 2.0).normalize());

		initialize_vertices(a, f, e, depth + 1, vertices, indices);
		initialize_vertices(f, b, d, depth + 1, vertices, indices);
		initialize_vertices(e, d, c, depth + 1, vertices, indices);
		initialize_vertices(d, e, f, depth + 1, vertices, indices);
	}
}