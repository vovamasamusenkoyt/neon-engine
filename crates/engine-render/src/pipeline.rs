use std::sync::Arc;
use vulkano::device::Device;
use vulkano::pipeline::graphics::color_blend::{
    AttachmentBlend, BlendFactor, BlendOp, ColorBlendAttachmentState, ColorBlendState,
};
use vulkano::pipeline::graphics::depth_stencil::{CompareOp, DepthState, DepthStencilState};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::{CullMode, FrontFace, RasterizationState};
use vulkano::pipeline::graphics::vertex_input::{Vertex, VertexDefinition};
use vulkano::pipeline::graphics::viewport::ViewportState;
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{
    DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
};
use vulkano::render_pass::Subpass;

use crate::shaders::{fs, fs_tex, vs, vs_tex};
use crate::vertex::{Vertex2D, Vertex3D};

pub struct RenderPipelines {
    pub cube_pipeline: Arc<GraphicsPipeline>,
    pub overlay_pipeline: Arc<GraphicsPipeline>,
    pub pipeline_layout: Arc<PipelineLayout>,
    pub textured_pipeline: Arc<GraphicsPipeline>,
    pub textured_layout: Arc<PipelineLayout>,
}

impl RenderPipelines {
    pub fn new(
        device: Arc<Device>,
        subpass: Subpass,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let vs_module = vs::load(device.clone())?;
        let fs_module = fs::load(device.clone())?;
        let vs_entry = vs_module.entry_point("main").ok_or("vs main not found")?;
        let fs_entry = fs_module.entry_point("main").ok_or("fs main not found")?;

        let vertex_input_state = Vertex3D::per_vertex().definition(&vs_entry)?;
        let stages = [
            PipelineShaderStageCreateInfo::new(vs_entry),
            PipelineShaderStageCreateInfo::new(fs_entry),
        ];

        let pipeline_layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())?,
        )?;

        let cube_pipeline = GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.clone().into_iter().collect(),
                vertex_input_state: Some(vertex_input_state.clone()),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState::default()),
                rasterization_state: Some(RasterizationState {
                    cull_mode: CullMode::Back,
                    front_face: FrontFace::CounterClockwise,
                    ..Default::default()
                }),
                depth_stencil_state: Some(DepthStencilState {
                    depth: Some(DepthState {
                        write_enable: true,
                        compare_op: CompareOp::Less,
                    }),
                    ..Default::default()
                }),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                )),
                dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                subpass: Some(subpass.clone().into()),
                ..GraphicsPipelineCreateInfo::layout(pipeline_layout.clone())
            },
        )?;

        let overlay_pipeline = GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState::default()),
                rasterization_state: Some(RasterizationState {
                    cull_mode: CullMode::None,
                    ..Default::default()
                }),
                depth_stencil_state: Some(DepthStencilState {
                    depth: Some(DepthState {
                        write_enable: false,
                        compare_op: CompareOp::Always,
                    }),
                    ..Default::default()
                }),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                )),
                dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                subpass: Some(subpass.clone().into()),
                ..GraphicsPipelineCreateInfo::layout(pipeline_layout.clone())
            },
        )?;

        // 2D Textured Quad Pipeline (for authentic reVC menu rendering)
        let vs_tex_module = vs_tex::load(device.clone())?;
        let fs_tex_module = fs_tex::load(device.clone())?;
        let vs_tex_entry = vs_tex_module.entry_point("main").ok_or("vs_tex main not found")?;
        let fs_tex_entry = fs_tex_module.entry_point("main").ok_or("fs_tex main not found")?;

        let stages_tex = [
            PipelineShaderStageCreateInfo::new(vs_tex_entry.clone()),
            PipelineShaderStageCreateInfo::new(fs_tex_entry),
        ];

        let textured_layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages_tex)
                .into_pipeline_layout_create_info(device.clone())?,
        )?;

        let vertex_input_tex = Vertex2D::per_vertex().definition(&vs_tex_entry)?;

        let blend_tex = ColorBlendAttachmentState {
            blend: Some(AttachmentBlend {
                color_blend_op: BlendOp::Add,
                src_color_blend_factor: BlendFactor::SrcAlpha,
                dst_color_blend_factor: BlendFactor::OneMinusSrcAlpha,
                alpha_blend_op: BlendOp::Add,
                src_alpha_blend_factor: BlendFactor::One,
                dst_alpha_blend_factor: BlendFactor::Zero,
            }),
            ..Default::default()
        };

        let textured_pipeline = GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages_tex.into_iter().collect(),
                vertex_input_state: Some(vertex_input_tex),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState::default()),
                rasterization_state: Some(RasterizationState {
                    cull_mode: CullMode::None,
                    ..Default::default()
                }),
                depth_stencil_state: Some(DepthStencilState {
                    depth: Some(DepthState {
                        write_enable: false,
                        compare_op: CompareOp::Always,
                    }),
                    ..Default::default()
                }),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    blend_tex,
                )),
                dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(textured_layout.clone())
            },
        )?;

        Ok(Self {
            cube_pipeline,
            overlay_pipeline,
            pipeline_layout,
            textured_pipeline,
            textured_layout,
        })
    }
}
