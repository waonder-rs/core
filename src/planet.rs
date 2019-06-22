use std::collections::HashMap;
use crate::location::{Vector3d, spherical};
use std::f32::consts::PI;

pub struct Vertices {
    data: Vec<Vector3d<f32>>
}

impl Vertices {
    pub fn new() -> Vertices {
        Vertices {
            data: Vec::new()
        }
    }

    pub fn from(vec: Vec<Vector3d<f32>>) -> Vertices {
        Vertices {
            data: vec
        }
    }

    pub fn get(&self, index: u64) -> Option<&Vector3d<f32>> {
        self.data.get(index as usize)
    }

    pub fn set(&mut self, index: u64, v: Vector3d<f32>) {
        let index = index as usize;
        if self.data.len() <= index {
            self.data.resize(index + 1, Vector3d::default())
        }
        self.data[index] = v
    }

    pub fn print_obj(&self) {
        for v in self.data.iter() {
            println!("v {} {} {}", v.x, v.y, v.z);
            println!("vn {} {} {}", v.x, v.y, v.z);
        }
    }
}

/**
 * Planet generator.
 */
pub struct Planet {
    root: Region,
    vertices: Vertices
}

impl Planet {
    /**
     * Create a new planet from a seed.
     */
    pub fn new() -> Planet {
        // we create the regions from an icosahedron.
        let t = (1.0 + 5.0f32.sqrt()) / 2.0;

        let vertices = Vertices::from(vec![
            Vector3d::new(-1.0,    t,  0.0).normal(),
            Vector3d::new( 1.0,    t,  0.0).normal(),
            Vector3d::new(-1.0,   -t,  0.0).normal(),
            Vector3d::new( 1.0,   -t,  0.0).normal(),
            Vector3d::new( 0.0, -1.0,    t).normal(),
            Vector3d::new( 0.0,  1.0,    t).normal(),
            Vector3d::new( 0.0, -1.0,   -t).normal(),
            Vector3d::new( 0.0,  1.0,   -t).normal(),
            Vector3d::new(   t,  0.0, -1.0).normal(),
            Vector3d::new(   t,  0.0,  1.0).normal(),
            Vector3d::new(  -t,  0.0, -1.0).normal(),
            Vector3d::new(  -t,  0.0,  1.0).normal()
        ]);

        let regions = [ // index [neighbors]
            Region::from_vertices(0, [0, 11, 5]), // 0 [1, 2, 3]
            Region::from_vertices(1, [0, 5, 1]), // 1 [4, 0, 5]
            Region::from_vertices(2, [0, 10, 11]), // 2 [0, 6, 7]
            Region::from_vertices(3, [5, 11, 4]), // 3 [0, 8, 9]
            Region::from_vertices(4, [0, 1, 7]), // 4 [10, 6, 1]
            Region::from_vertices(5, [1, 5, 9]), // 5 [4, 9, 11]
            Region::from_vertices(6, [0, 7, 10]), // 6 [4, 12, 2]
            Region::from_vertices(7, [11, 10, 2]), // 7 [2, 13, 8]
            Region::from_vertices(8, [2, 4, 11]), // 8 [14, 3, 7]
            Region::from_vertices(9, [4, 9, 5]), // 9 [15, 5, 3]
            Region::from_vertices(10, [7, 1, 8]), // 10 [4, 11, 16]
            Region::from_vertices(11, [9, 8, 1]), // 11 [5, 10, 17]
            Region::from_vertices(12, [10, 7, 6]), // 12 [6, 16, 13]
            Region::from_vertices(13, [6, 2, 10]), // 13 [7, 12, 18]
            Region::from_vertices(14, [3, 4, 2]), // 14 [8, 15, 18]
            Region::from_vertices(15, [3, 9, 4]), // 15 [9, 14, 17]
            Region::from_vertices(16, [8, 6, 7]), // 16 [10, 12, 19]
            Region::from_vertices(17, [3, 8, 9]), // 17 [11, 15, 19]
            Region::from_vertices(18, [3, 2, 6]), // 18 [13, 14, 19]
            Region::from_vertices(19, [3, 6, 8]), // 19 [16, 17, 18]
        ];

        Planet {
            root: Region::Root(Box::new(regions)),
            vertices: vertices
        }
    }

    pub fn refine(&mut self) {
        self.root.refine(&mut self.vertices)
    }

    pub fn print_obj(&self) {
        self.vertices.print_obj();
        self.root.print_obj();
    }
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    generation: u32,
    index: u64,
    vertices: (u64, u64)
} // index, vertices

impl Edge {
    fn new(generation: u32, index: u64, vertices: (u64, u64)) -> Edge {
        Edge {
            generation: generation,
            index: index,
            vertices: vertices
        }
    }

    fn index(&self) -> u64 {
        self.index
    }

    fn generation(&self) -> u32 {
        self.generation
    }

    // return the number of edges in this edge generation.
    // g(0) = 30
    // g(i+1) = g(i)*2 + 20*(3^(i+1))
    fn generation_population(&self) -> u64 {
        let mut edge_count = 30;
        for i in 0..self.generation {
            edge_count = edge_count * 2 + 20*(3u64.pow(i))
        }

        edge_count
    }

    fn start(&self) -> u64 {
        self.vertices.0
    }

    fn end(&self) -> u64 {
        self.vertices.1
    }

    fn count(generation: u32) -> u64 {
        let mut edge_count = 30;
        let mut face_count = 20;
        for i in 0..generation {
            edge_count = edge_count * 2 + face_count * 3;
            face_count *= 4;
        }

        edge_count
    }

    // Split an edge into two edges of next generation, with a new vertex.
    fn split(&self, vertices: &mut Vertices) -> (Edge, u64, Edge) {
        let mut edge_count = 30;
        let mut vertex_count = 12;
        let mut face_count = 20;
        for i in 0..self.generation {
            vertex_count += edge_count;
            edge_count = edge_count * 2 + face_count * 3;
            face_count *= 4;
        }

        let start;
        let end;
        let v = vertex_count + self.index;
        if self.start() < self.end() {
            start = Edge::new(self.generation+1, self.index*2 + 0, (self.start(), v));
            end = Edge::new(self.generation+1, self.index*2 + 1, (v, self.end()))
        } else {
            start = Edge::new(self.generation+1, self.index*2 + 1, (self.start(), v));
            end = Edge::new(self.generation+1, self.index*2 + 0, (v, self.end()))
        }

        let start_pos = vertices.get(self.start()).unwrap();
        let end_pos = vertices.get(self.end()).unwrap();
        vertices.set(v, ((*start_pos + *end_pos)/2.0).normal());

        (start, v, end)
    }
}

impl std::ops::Neg for Edge {
    type Output = Edge;

    fn neg(self) -> Edge {
        Edge {
            generation: self.generation,
            index: self.index,
            vertices: (self.vertices.1, self.vertices.0)
        }
    }
}

enum Region {
    Root(Box<[Region; 20]>),
    Child {
        generation: u32,
        index: u64,
        vertices: [u64; 3],
        edges_index: [u64; 3],
        children: Option<Box<[Region; 4]>>
    }
}

// struct BoundRegion<'a> {
//     index: u64,
//     neighbors: [&'a Region; 3],
//     region: &'a Region
// }

pub fn great_edge_index(mut v: u64, mut w: u64) -> u64 {
    if w < v {
        std::mem::swap(&mut v, &mut w)
    }

    match v {
        0 => match w {
            1 => 0,
            5 => 1,
            7 => 2,
            10 => 3,
            11 => 4,
            _ => panic!("invalid edge")
        },
        1 => match w {
            5 => 5,
            7 => 6,
            8 => 7,
            9 => 8,
            _ => panic!("invalid edge")
        },
        2 => match w {
            3 => 9,
            4 => 10,
            6 => 11,
            10 => 12,
            11 => 13,
            _ => panic!("invalid edge")
        },
        3 => match w {
            4 => 14,
            6 => 15,
            8 => 16,
            9 => 17,
            _ => panic!("invalid edge")
        },
        4 => match w {
            5 => 18,
            9 => 19,
            11 => 20,
            _ => panic!("invalid edge")
        },
        5 => match w {
            9 => 21,
            11 => 22,
            _ => panic!("invalid edge")
        },
        6 => match w {
            7 => 23,
            8 => 24,
            10 => 25,
            _ => panic!("invalid edge")
        },
        7 => match w {
            8 => 26,
            10 => 27,
            _ => panic!("invalid edge")
        },
        8 => match w {
            9 => 28,
            _ => panic!("invalid edge")
        }
        10 => match w {
            11 => 29,
            _ => panic!("invalid edge")
        },
        _ => panic!("invalid edge")
    }
}

// /**
//  * Planet region generator.
//  */
// struct Region {
//     corners: [Vector3d; 3];
// }
//
impl Region {
    pub fn from_vertices(index: u64, vertices: [u64; 3]) -> Region {
        let edges_index = [
            great_edge_index(vertices[0], vertices[1]),
            great_edge_index(vertices[1], vertices[2]),
            great_edge_index(vertices[2], vertices[0])
        ];
        Region::Child {
            generation: 0,
            index: index,
            vertices: vertices,
            edges_index: edges_index,
            children: None
        }
    }

    pub fn new(index: u64, edges: [Edge; 3]) -> Region {
        let edges_index = [
            edges[0].index(),
            edges[1].index(),
            edges[2].index()
        ];

        let vertices = [
            edges[0].start(),
            edges[1].start(),
            edges[2].start()
        ];

        Region::Child {
            generation: edges[0].generation(),
            index: index,
            vertices: vertices,
            edges_index: edges_index,
            children: None
        }
    }

    pub fn edge(&self, index: u64) -> Edge {
        match self {
            Region::Root(_) => {
                let vertices = [
                    (0, 1),
                    (0, 5),
                    (0, 7),
                    (0, 10),
                    (0, 11),
                    (1, 5),
                    (1, 7),
                    (1, 8),
                    (1, 9),
                    (2, 3),
                    (2, 4),
                    (2, 6),
                    (2, 10),
                    (2, 11),
                    (3, 4),
                    (3, 6),
                    (3, 8),
                    (3, 9),
                    (4, 5),
                    (4, 9),
                    (4, 11),
                    (5, 9),
                    (5, 11),
                    (6, 7),
                    (6, 8),
                    (6, 10),
                    (7, 8),
                    (7, 10),
                    (8, 9),
                    (10, 11)
                ][index as usize];
                Edge::new(0, index, vertices)
            },
            Region::Child{ generation, vertices, edges_index, .. } => {
                match index {
                    0 => Edge::new(*generation, edges_index[0], (vertices[0], vertices[1])),
                    1 => Edge::new(*generation, edges_index[1], (vertices[1], vertices[2])),
                    2 => Edge::new(*generation, edges_index[2], (vertices[2], vertices[0])),
                    _ => panic!("invalid edge")
                }
            }
        }
    }

    ///                     vertex[0]
    ///                      / \
    ///                  ^  /   \
    ///                 e5 /     \ e0
    ///            ^      /       \ v   \
    ///           /      /   e6>   \     \
    ///  edge[2] /     v2----------v0     \ edge[0]
    ///         /      / \        / \      \
    ///        /   ^  / ^ \      /   \      v
    ///           e4 /  e8 \    / e7  \ e1
    ///             /       \  /  v    \ v
    ///            /         \/         \
    ///           -----------v1----------
    ///   vertex[2]   <e3         <e2      vertex[1]
    ///
    ///               <--- edge[1] ----
    ///
    pub fn refine(&mut self, vertices_data: &mut Vertices) {
        let new_children = match &*self {
            Region::Child { generation, index, vertices, children, .. } => {
                match children {
                    Some(_) => None,
                    None => {
                        let next_gen = *generation+1;

                        let (e0, v0, e1) = self.edge(0).split(vertices_data);
                        let (e2, v1, e3) = self.edge(1).split(vertices_data);
                        let (e4, v2, e5) = self.edge(2).split(vertices_data);

                        let edge_offset = Edge::count(*generation)*2 + *index*3;
                        let e6 = Edge::new(next_gen, edge_offset+0, (v2, v0));
                        let e7 = Edge::new(next_gen, edge_offset+1, (v0, v1));
                        let e8 = Edge::new(next_gen, edge_offset+2, (v1, v2));

                        let face_offset = *index*4;
                        Some(Box::new([
                            Region::new(face_offset+0, [e0, -e6, e5]),
                            Region::new(face_offset+1, [e1, e2, -e7]),
                            Region::new(face_offset+2, [e3, e4, -e8]),
                            Region::new(face_offset+3, [e6, e7, e8])
                        ]))
                    }
                }
            },
            _ => None
        };

        if let Some(new_children) = new_children {
            match self {
                Region::Child { ref mut children, .. } => {
                    *children = Some(new_children)
                },
                _ => ()
            }
        } else {
            match self {
                Region::Root(ref mut children) => {
                    for mut child in children.iter_mut() {
                        child.refine(vertices_data)
                    }
                },
                Region::Child { ref mut children, .. } => {
                    match children {
                        Some(ref mut children) => {
                            for mut child in children.iter_mut() {
                                child.refine(vertices_data)
                            }
                        },
                        None => ()
                    }
                }
            }
        }
    }

    pub fn print_obj(&self) {
        match self {
            Region::Root(regions) => {
                for region in regions.iter() {
                    region.print_obj()
                }
            },
            Region::Child { vertices, children, .. } => {
                match children {
                    Some(children) => {
                        for child in children.iter() {
                            child.print_obj()
                        }
                    },
                    None => {
                        println!("f {} {} {}", vertices[0]+1, vertices[1]+1, vertices[2]+1); // .obj starts indexes at 1.
                    }
                }

            }
        }
    }
}

// impl<'a> BoundRegion<'a> {
//     // pub fn neighbors_of(&self, index: u64) -> [&Region; 3] {
//     //     match self.region {
//     //         Region::Root(regions) => {
//     //             match index {
//     //                 0 => [&regions[1], &regions[2], &regions[3]],
//     //                 1 => [&regions[4], &regions[0], &regions[5]],
//     //                 2 => [&regions[0], &regions[6], &regions[7]],
//     //                 3 => [&regions[0], &regions[8], &regions[9]],
//     //                 4 => [&regions[10], &regions[6], &regions[1]],
//     //                 5 => [&regions[4], &regions[9], &regions[11]],
//     //                 6 => [&regions[4], &regions[12], &regions[2]],
//     //                 7 => [&regions[2], &regions[13], &regions[8]],
//     //                 8 => [&regions[14], &regions[3], &regions[7]],
//     //                 9 => [&regions[15], &regions[5], &regions[3]],
//     //                 10 => [&regions[4], &regions[11], &regions[16]],
//     //                 11 => [&regions[5], &regions[10], &regions[17]],
//     //                 12 => [&regions[6], &regions[16], &regions[13]],
//     //                 13 => [&regions[7], &regions[12], &regions[18]],
//     //                 14 => [&regions[8], &regions[15], &regions[18]],
//     //                 15 => [&regions[9], &regions[14], &regions[17]],
//     //                 16 => [&regions[10], &regions[12], &regions[19]],
//     //                 17 => [&regions[11], &regions[15], &regions[19]],
//     //                 18 => [&regions[13], &regions[14], &regions[19]],
//     //                 _ => [&regions[16], &regions[17], &regions[18]]
//     //             }
//     //         },
//     //         Region::Child{ .. } => {
//     //             panic!("no children")
//     //         }
//     //     }
//     // }
//
//     // pub fn print_obj_vertices(&self, index: u64) {
//     //     match self.region {
//     //         Region::Root(regions) => {
//     //             for (i, region) in regions.iter().enumerate() {
//     //                 region.bind(i, self.neighbors_of(i)).print_obj_vertices(i)
//     //             }
//     //         },
//     //         Region::Child { vertices } => {
//     //             for v in vertices {
//     //                 println!("v {} {} {}", v.x, v.y, v.z);
//     //             }
//     //         }
//     //     }
//     // }
//
//
// }
