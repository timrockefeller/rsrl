use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;

pub struct CuboidTiled {
    /// Half of the width, height and depth of the cuboid
    pub half_size: Vec3,
    /// The number of tiles in each direction
    pub tile_count: [usize; 2],
    /// The index of the cuboid every face that will be tiled
    /// 0: front
    /// 1: back
    /// 2: right
    /// 3: left
    /// 4: top
    /// 5: bottom
    pub face: [[usize; 2];6],
}

impl Primitive3d for CuboidTiled {}

impl Default for CuboidTiled {
    fn default() -> Self {
        Self {
            half_size: Vec3::splat(0.5),
            tile_count: [1, 1],
            face: [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]],
        }
    }
}

#[derive(Clone, Copy, Debug, Reflect)]
#[reflect(Default, Debug, Clone)]
pub struct CuboidTiledMeshBuilder {
    pub half_size: Vec3,
    pub uv: [[[f32; 2]; 2]; 6],
}

impl Default for CuboidTiledMeshBuilder {
    /// Returns the default [`CuboidMeshBuilder`] with a width, height, and depth of `1.0`.
    fn default() -> Self {
        Self {
            half_size: Vec3::splat(0.5),
            uv: [[[0.0, 0.0], [1.0, 1.0]]; 6],
        }
    }
}

impl MeshBuilder for CuboidTiledMeshBuilder {
    fn build(&self) -> Mesh {
        let min = -self.half_size;
        let max = self.half_size;

        // Suppose Y-up right hand, and camera look from +Z to -Z
        let vertices = &[
            // Front
            ([min.x, min.y, max.z], [0.0, 0.0, 1.0], [self.uv[0][0][0], self.uv[0][1][1]]),
            ([max.x, min.y, max.z], [0.0, 0.0, 1.0], [self.uv[0][1][0], self.uv[0][1][1]]),
            ([max.x, max.y, max.z], [0.0, 0.0, 1.0], [self.uv[0][1][0], self.uv[0][0][1]]),
            ([min.x, max.y, max.z], [0.0, 0.0, 1.0], [self.uv[0][0][0], self.uv[0][0][1]]),
            // Back
            ([min.x, max.y, min.z], [0.0, 0.0, -1.0], [self.uv[1][1][0], self.uv[1][0][1]]),
            ([max.x, max.y, min.z], [0.0, 0.0, -1.0], [self.uv[1][0][0], self.uv[1][0][1]]),
            ([max.x, min.y, min.z], [0.0, 0.0, -1.0], [self.uv[1][0][0], self.uv[1][1][1]]),
            ([min.x, min.y, min.z], [0.0, 0.0, -1.0], [self.uv[1][1][0], self.uv[1][1][1]]),
            // Right
            ([max.x, min.y, min.z], [1.0, 0.0, 0.0], [self.uv[2][1][0], self.uv[2][1][1]]),
            ([max.x, max.y, min.z], [1.0, 0.0, 0.0], [self.uv[2][1][0], self.uv[2][0][1]]),
            ([max.x, max.y, max.z], [1.0, 0.0, 0.0], [self.uv[2][0][0], self.uv[2][0][1]]),
            ([max.x, min.y, max.z], [1.0, 0.0, 0.0], [self.uv[2][0][0], self.uv[2][1][1]]),
            // Left
            ([min.x, min.y, max.z], [-1.0, 0.0, 0.0], [self.uv[3][1][0], self.uv[3][1][1]]),
            ([min.x, max.y, max.z], [-1.0, 0.0, 0.0], [self.uv[3][1][0], self.uv[3][0][1]]),
            ([min.x, max.y, min.z], [-1.0, 0.0, 0.0], [self.uv[3][0][0], self.uv[3][0][1]]),
            ([min.x, min.y, min.z], [-1.0, 0.0, 0.0], [self.uv[3][0][0], self.uv[3][1][1]]),
            // Top
            ([max.x, max.y, min.z], [0.0, 1.0, 0.0], [self.uv[4][1][0], self.uv[4][0][1]]),
            ([min.x, max.y, min.z], [0.0, 1.0, 0.0], [self.uv[4][0][0], self.uv[4][0][1]]),
            ([min.x, max.y, max.z], [0.0, 1.0, 0.0], [self.uv[4][0][0], self.uv[4][1][1]]),
            ([max.x, max.y, max.z], [0.0, 1.0, 0.0], [self.uv[4][1][0], self.uv[4][1][1]]),
            // Bottom
            ([max.x, min.y, max.z], [0.0, -1.0, 0.0], [self.uv[5][1][0], self.uv[5][1][1]]),
            ([min.x, min.y, max.z], [0.0, -1.0, 0.0], [self.uv[5][0][0], self.uv[5][1][1]]),
            ([min.x, min.y, min.z], [0.0, -1.0, 0.0], [self.uv[5][0][0], self.uv[5][0][1]]),
            ([max.x, min.y, min.z], [0.0, -1.0, 0.0], [self.uv[5][1][0], self.uv[5][0][1]]),
        ];

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        let indices = Indices::U32(vec![
            0, 1, 2, 2, 3, 0, // front
            4, 5, 6, 6, 7, 4, // back
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // top
            20, 21, 22, 22, 23, 20, // bottom
        ]);

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(indices)
    }
}

impl Meshable for CuboidTiled {
    type Output = CuboidTiledMeshBuilder;

    fn mesh(&self) -> Self::Output {

        CuboidTiledMeshBuilder {
            half_size: self.half_size,
            uv:[
                [
                    [self.face[0][0] as f32 / self.tile_count[0] as f32, self.face[0][1] as f32 / self.tile_count[1] as f32],
                    [(self.face[0][0]+1) as f32 / self.tile_count[0] as f32, (self.face[0][1]+1) as f32 / self.tile_count[1] as f32],
                ], // front
                [
                    [self.face[1][0] as f32 / self.tile_count[0] as f32, self.face[1][1] as f32 / self.tile_count[1] as f32],
                    [(self.face[1][0]+1) as f32 / self.tile_count[0] as f32, (self.face[1][1]+1) as f32 / self.tile_count[1] as f32],
                ], // back
                [
                    [self.face[2][0] as f32 / self.tile_count[0] as f32, self.face[2][1] as f32 / self.tile_count[1] as f32],
                    [(self.face[2][0]+1) as f32 / self.tile_count[0] as f32, (self.face[2][1]+1) as f32 / self.tile_count[1] as f32],
                ], // right
                [
                    [self.face[3][0] as f32 / self.tile_count[0] as f32, self.face[3][1] as f32 / self.tile_count[1] as f32],
                    [(self.face[3][0]+1) as f32 / self.tile_count[0] as f32, (self.face[3][1]+1) as f32 / self.tile_count[1] as f32],
                ], // left
                [
                    [self.face[4][0] as f32 / self.tile_count[0] as f32, self.face[4][1] as f32 / self.tile_count[1] as f32],
                    [(self.face[4][0]+1) as f32 / self.tile_count[0] as f32, (self.face[4][1]+1) as f32 / self.tile_count[1] as f32],
                ], // top
                [
                    [self.face[5][0] as f32 / self.tile_count[0] as f32, self.face[5][1] as f32 / self.tile_count[1] as f32],
                    [(self.face[5][0]+1) as f32 / self.tile_count[0] as f32, (self.face[5][1]+1) as f32 / self.tile_count[1] as f32],
                ], // bottom
            ]
        }
    }
}

impl From<CuboidTiled> for Mesh {
    fn from(cuboid: CuboidTiled) -> Self {
        cuboid.mesh().build()
    }
}
