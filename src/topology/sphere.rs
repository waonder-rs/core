use std::sync::Arc;
use vulkano::{
	buffer::{
		BufferAccess,
		TypedBufferAccess
	}
};
use engine::util::Vector3d;

const MAX_PRECISION: u32 = 8;

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

pub struct Region {
	/// Region vertices.
	vertices: [Vector3d<f32>; vertex_count(MAX_PRECISION)],

	/// GPU vertex buffer.
	vertex_buffer: Arc<dyn BufferAccess + Sync + Send>,

	/// GPU index buffer.
	index_buffer: Arc<dyn TypedBufferAccess<Content = [u32]> + Sync + Send>
}

fn initialize_vertices(a: u32, b: u32, c: u32, depth: u32, vertices: &mut [Vector3d<f32>], indices: &mut [Vec<u32>]) {
	let depth_indices = &mut indices[depth as usize];
	depth_indices.push(a);
	depth_indices.push(b);
	depth_indices.push(c);

	if depth < MAX_PRECISION {
		//         a
		//       /  \
		//     e --- f
		//   /  \  /  \
		// c --- d --- b
		use super::utils::nrbpi2_mean;

		let a_pos = vertices[a as usize];
		let b_pos = vertices[b as usize];
		let c_pos = vertices[c as usize];

		let d = nrbpi2_mean(b, c);
		let e = nrbpi2_mean(c, a);
		let f = nrbpi2_mean(a, b);

		vertices[d as usize] = ((b_pos + c_pos) / 2.0).normal();
		vertices[e as usize] = ((c_pos + a_pos) / 2.0).normal();
		vertices[f as usize] = ((a_pos + b_pos) / 2.0).normal();

		initialize_vertices(a, f, e, depth + 1, vertices, indices);
		initialize_vertices(f, b, d, depth + 1, vertices, indices);
		initialize_vertices(e, d, c, depth + 1, vertices, indices);
		initialize_vertices(d, e, f, depth + 1, vertices, indices);
	}
}

impl Region {
	/// Create a new region from three points.
	pub fn new(a: Vector3d<f32>, b: Vector3d<f32>, c: Vector3d<f32>) -> Region {
		let mut vertices = [Vector3d::new(0.0, 0.0, 0.0); vertex_count(MAX_PRECISION)];

		vertices[0] = a;
		vertices[1] = b;
		vertices[2] = c;

		let mut indices = Vec::new();
		indices.resize(MAX_PRECISION as usize, Vec::new());

		initialize_vertices(0, 1, 2, 0, &mut vertices, &mut indices);

		// let (vertex_buffer, vertex_future) = ImmutableBuffer::from_data(vertices, BufferUsage::vertex_buffer(), loader.queue().clone()).unwrap();
		// let (index_buffer, index_future) = ImmutableBuffer::from_iter(indexes.into_iter(), BufferUsage::index_buffer(), loader.queue().clone()).unwrap();

		Region {
			vertices,
			vertex_buffer: panic!("TODO"),
			index_buffer: panic!("TODO")
		}
	}

	pub fn refine(&mut self) {
		// ...
	}

	/// Explode the region into sub regions.
	pub fn explode(&self) -> Vec<Region> {
		panic!("TODO")
	}
}

// /// Spherical topology
// pub struct Sphere {
// 	vertex_buffer: Arc<dyn BufferAccess + Sync + Send>,
// 	index_buffer: Arc<dyn TypedBufferAccess<Content = [u32]> + Sync + Send>
// }
//
// impl Sphere {
// 	pub fn new(&self) -> Sphere {
// 		// ...
// 	}
// }
//
// impl Geometry for Sphere {
// 	fn vertex_buffer(&self) -> &Arc<dyn BufferAccess + Sync + Send> {
// 		&self.vertex_buffer
// 	}
//
// 	fn index_buffer(&self) -> &Arc<dyn TypedBufferAccess<Content = [u32]> + Sync + Send> {
// 		&self.index_buffer
// 	}
// }
