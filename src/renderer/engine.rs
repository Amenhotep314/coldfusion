//! This module is responsible for camera logic and WGPU rendering of STLs. It
//! should take a mesh from the frame loop and return a drawn GUI layer.

use eframe::egui_wgpu;
use eframe::wgpu;
use eframe::wgpu::util::DeviceExt;
use std::sync::Arc;
use crate::stl;

/// A struct containing everything that the renderer needs to draw triangles to
/// the viewport.
pub struct Renderer {
    /// A series of methods to pass the triangle buffer through to draw on the
    /// viewport. Defined in the constructor below.
    pipeline: wgpu::RenderPipeline,
    /// The triangle buffer, to which we write lists of triangles to be sent
    /// to the GPU.
    vertex_buffer: wgpu::Buffer,
    /// The index buffer, to keep track of where the triangles are
    index_buffer: wgpu::Buffer,
    /// The number of indices
    num_indices: u32,
}

/// A struct containing a single vertex of a triangle in 3D.
// This tells the struct to store its data in a simple, C-like format so that
// the GPU is not confused by any Rust compression schemes
#[repr(C)]
// The struct is safe to store as simple bytes.
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    /// A 3-tuple of float containing the position of the vertex
    position: [f32; 3],
}

/// A struct to hold all of the stuff that needs to get passed to the renderer
/// every frame.
pub struct ViewportCallback {
    /// The renderer, wrapped in an Arc to prevent it from going out of scope
    /// when the GPU multithreads
    pub renderer: Arc<Renderer>,
    // Camera and stuff that needs to get passed to the renderer eventually
    // goes here.

}

struct GPUMesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    num_indices: u32,
}

impl Renderer {
    /// The constructor, called when the window is created.
    ///
    /// # Arguments
    /// * `device` - A pointer to the GPU, so that we can push triangles to it
    /// * `format` - I have no idea what this does, something to do with the
    ///   texture?
    ///
    /// # Returns
    /// An instance of Renderer
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {

        let indexed_mesh: stl_io::IndexedMesh = stl::load_stl_to_buffer("assets/stanford_bunny.stl")
            .expect("couldn't load STL");
        let gpu_mesh: GPUMesh = GPUMesh::new(indexed_mesh);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("STL vertices"),
            contents: bytemuck::cast_slice(&gpu_mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("STL indices"),
            contents: bytemuck::cast_slice(&gpu_mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Triangle Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle pipeline"),

            layout: None,

            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::layout()],
                compilation_options: Default::default(),
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },

            depth_stencil: None,

            multisample: wgpu::MultisampleState::default(),

            multiview_mask: None,

            cache: None,
        });

        Self {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            num_indices: gpu_mesh.num_indices,
            pipeline: pipeline,
        }
    }

    fn render(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(
            self.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );

        pass.draw_indexed(
            0..self.num_indices,
            0,
            0..1,
        );
    }
}

impl Vertex {
    const ATTRIBS: &[wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
        0 => Float32x3
    ];

    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRIBS,
        }
    }
}

impl egui_wgpu::CallbackTrait for ViewportCallback {
    fn paint(
        &self,
        info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        _resources: &egui_wgpu::CallbackResources,
    ) {
        self.renderer.render(render_pass);
    }
}

impl GPUMesh {
    fn new(stl: stl_io::IndexedMesh) -> Self {

        let vertices: Vec<Vertex> = stl
            .vertices
            .iter()
            .map(|v| Vertex {
                position: [v[0]*0.01, v[1]*0.01, v[2]*0.01],
            })
            .collect();

        let indices: Vec<u32> = stl
            .faces
            .iter()
            .flat_map(|face| {
                face.vertices
                    .iter()
                    .map(|&i| i as u32)
            })
            .collect();

        let num_indices: u32 = u32::try_from(indices.len())
            .expect("Slice too large for u32");

        println!("{}", num_indices);

        Self {
            vertices: vertices,
            indices: indices,
            num_indices: num_indices
        }
    }
}
