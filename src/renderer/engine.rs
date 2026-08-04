//! This module is responsible for camera logic and WGPU rendering of STLs. It
//! should take a mesh from the frame loop and return a drawn GUI layer.

use eframe::egui_wgpu;
use eframe::wgpu;
use eframe::wgpu::util::DeviceExt;
use std::sync::Arc;
use crate::util::stl;
use crate::renderer::camera::{Camera, GPUCamera};

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
    /// The camera transform matrix
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    /// The number of indices
    num_indices: u32,
    queue: wgpu::Queue,
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
    /// The normal vector to the relevant triangle.
    normal: [f32; 3],
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
    pub fn new(device: &wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Self {

        let gpu_camera = GPUCamera{ view_proj: glam::Mat4::IDENTITY.to_cols_array_2d() };

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
        let camera_buffer = device.create_buffer_init( &wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform"),
            contents: bytemuck::bytes_of(&gpu_camera.view_proj),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera bind group layout"),

            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },

                count: None,
            }],
        });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera bind group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Triangle Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("triangle pipeline layout"),
            bind_group_layouts: &[
                Some(&camera_bind_group_layout),
            ],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle pipeline"),

            layout: Some(&pipeline_layout),

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
            camera_buffer: camera_buffer,
            camera_bind_group: camera_bind_group,
            num_indices: gpu_mesh.num_indices,
            pipeline: pipeline,
            queue: queue,
        }
    }

    fn render(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(
            0,
            &self.camera_bind_group,
            &[],
        );
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


    pub fn push_camera_to_gpu(&self, camera: &Camera, aspect: f32) {
        let gpu_camera = camera.to_gpu(aspect);

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::bytes_of(&gpu_camera),
        );

    }
}

impl Vertex {
    const ATTRIBS: &[wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
        0 => Float32x3, 1 => Float32x3,
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
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for face in &stl.faces {
            let base = vertices.len() as u32;

            for &i in &face.vertices {
                let v = stl.vertices[i];

                vertices.push(Vertex {
                    position: [v[0], v[1], v[2]],
                    normal: [
                        face.normal[0],
                        face.normal[1],
                        face.normal[2],
                    ],
                });
            }

            indices.extend([
                base,
                base + 1,
                base + 2,
            ]);
        }

        let num_indices: u32 = indices.len() as u32;

        Self {
            vertices: vertices,
            indices: indices,
            num_indices: num_indices
        }
    }
}
