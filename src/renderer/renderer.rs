//! This module is responsible for camera logic and WGPU rendering of STLs. It
//! should take a mesh from the frame loop and return a drawn GUI layer.


use std::f32::consts::PI;
use std::sync::Arc;
use eframe::egui_wgpu;
use eframe::wgpu;
use eframe::wgpu::util::DeviceExt;


// TODO: Set up the camera to roll and translate, not just pitch and yaw

/// A placeholder struct to be called by the render callback in the UI logic
pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
}

/// A struct to hold all of the stuff that needs to get passed to the renderer
/// every frame.
pub struct ViewportCallback {
    /// The renderer, wrapped in an Arc to prevent it from going out of scope
    /// when the GPU multithreads
    pub renderer: Arc<Renderer>
    // Camera and stuff that needs to get passed to the renderer eventually
    // goes here.
}


const TRIANGLE: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
    },
];


impl Renderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("triangle vertices"),
                contents: bytemuck::cast_slice(TRIANGLE),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let shader = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some("Triangle Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("shader.wgsl").into(),
                ),
            },
        );

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("triangle pipeline"),

                layout: None,

                vertex: wgpu::VertexState{
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Vertex::layout()],
                    compilation_options: Default::default(),
                },

                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(
                        wgpu::ColorTargetState {
                            format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        }
                    )],
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
            }
        );

        Self { vertex_buffer: vertex_buffer, pipeline: pipeline }
    }

    pub fn render(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..3, 0..1);
    }
}

impl Vertex {
    const ATTRIBS: &[wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![
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
