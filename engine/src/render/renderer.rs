use std::collections::VecDeque;
use std::ffi::CString;
use std::sync::Arc;

use glutin::display::{Display, GlDisplay};

use crate::model::mesh::Mesh;

use crate::render::generic_renderer::GenericRenderer;
use crate::render::render_entity::MeshLoadState;
use crate::render::render_packet::RenderPacket;
use crate::scene::scene::Scene;
use crate::scene::world::component::camera_component::CameraComponent;
use crate::scene::world::create_mesh_manager::CreateMeshManager;
use crate::scene::world::drop_mesh_request::DropMeshRequest;
use crate::texture::frame_buffer::Framebuffer;

use super::uniformbuffer::{MatrixUniformBuffer, UniformBuffer};
use super::updatable::Updatable;

pub struct Renderer {
    matrix_uniform_buffer: UniformBuffer<MatrixUniformBuffer>,
    //background_renderer: BackgroundRenderer,
    //scene_renderer: SceneRenderer,
    frame_buffer: Framebuffer,
    //postprocess_renderer: PostprocessRenderer,
    generic_renderer: GenericRenderer,
    //gui_renderer: GuiRenderer,
}

impl Renderer {
    pub fn init(gl_display: &Display, width: i32, height: i32) -> Self {
        gl::load_with(|name| {
            let symbol = CString::new(name).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });

        let mut matrix_uniform_buffer;
        let msaa = match cfg!(not(target_os = "android")) {
            true => Some(16),
            false => None,
        };

        let matrix_uniform_buffer_content = MatrixUniformBuffer {
            projection_matrix: [[0.0; 4]; 4],
            view_matrix: [[0.0; 4]; 4],
        };

        unsafe {
            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);

            if msaa.is_some() {
                gl::Enable(gl::MULTISAMPLE);
            }

            gl::DepthFunc(gl::LEQUAL);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::PatchParameteri(gl::PATCH_VERTICES, 3);

            matrix_uniform_buffer = UniformBuffer::create(0);
            matrix_uniform_buffer.set_data(matrix_uniform_buffer_content);
        };

        Renderer {
            matrix_uniform_buffer,
            //background_renderer: BackgroundRenderer::init(archive),
            //scene_renderer: SceneRenderer::init(archive),
            frame_buffer: Framebuffer::new(width, height, msaa),
            generic_renderer: GenericRenderer::init(),
            //postprocess_renderer: PostprocessRenderer::init(archive),
            //gui_renderer: GuiRenderer::init(archive),
        }
    }

    pub fn load_scene(&mut self, camera_component: &CameraComponent, aspect_ratio: f32) {
        let matrix_uniform_buffer_content = MatrixUniformBuffer {
            projection_matrix: camera_component.get_projection_matrix(aspect_ratio),
            view_matrix: camera_component.get_view_matrix(),
        };
        unsafe {
            self.matrix_uniform_buffer
                .set_data(matrix_uniform_buffer_content);
        }
        /*let foreground = scene
            .get_world()
            .get_resource::<Arc<Mutex<Foreground>>>()
            .expect("No foreground found");
        self.postprocess_renderer.load_scene(foreground);*/
    }

    unsafe fn clean_up(&self) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    pub fn update_buffers(&mut self, updatables: &mut VecDeque<Updatable>, scene: &mut Scene) {
        while let Some(updatable) = updatables.pop_front() {
            match updatable {
                Updatable::Projection { width, height } => {
                    self.frame_buffer.resize(width, height);

                    let (_camera, (camera_component,)) = scene
                        .get_world_mut()
                        .query_mut::<(&mut CameraComponent,)>()
                        .next()
                        .expect("No camera found in the scene");

                    unsafe {
                        gl::Viewport(0, 0, width as i32, height as i32);
                        self.matrix_uniform_buffer.set_projection_matrix(
                            camera_component.get_projection_matrix(width as f32 / height as f32),
                        );
                    };
                }
                Updatable::View { camera_component } => unsafe {
                    self.matrix_uniform_buffer
                        .set_view_matrix(camera_component.get_view_matrix());
                },
                Updatable::FocalRadius => {
                    /*let foreground = scene
                        .get_world()
                        .expect_resource::<Arc<Mutex<Foreground>>>()
                        .lock()
                        .unwrap();
                    self.postprocess_renderer
                        .update_focal_radius(foreground.get_focal_radius());*/ // TODO
                }
            }
        }
    }

    pub fn handle_mesh_requests(
        &mut self,
        drop_mesh_requests: &mut VecDeque<DropMeshRequest>,
        create_mesh_manager: &mut CreateMeshManager,
    ) {
        while let Some(request) = drop_mesh_requests.pop_front() {
            let mesh = &request.mesh;
            unsafe {
                gl::DeleteVertexArrays(1, &mesh.get_vao());
                gl::DeleteBuffers(1, &mesh.get_vert_vbo());
                if let Some(uvs_vbo) = mesh.get_uvs_vbo() {
                    gl::DeleteBuffers(1, &uvs_vbo);
                }
                gl::DeleteBuffers(1, &mesh.get_ebo());
            }
        }
        for (_, load_state) in create_mesh_manager.iter_mut() {
            if let MeshLoadState::CreateRequest {
                vertices,
                indices,
                uvs,
            } = load_state
            {
                let mesh = Arc::new(Mesh::new(vertices, indices, uvs));
                *load_state = MeshLoadState::Loaded(mesh);
            }
        }
    }

    pub fn render(
        &mut self,
        render_packet: RenderPacket,
        /*gui_manager: &GuiManager,
        camera_component: &CameraComponent,
        background: &Background,
        foreground: &Foreground,*/
    ) {
        unsafe {
            self.refresh_buffers(&render_packet);
            self.clean_up(); // Clean up without framebuffer
                             //self.frame_buffer.bind();
                             //self.clean_up(); // Clean up with framebuffer
            gl::Enable(gl::DEPTH_TEST);
            self.matrix_uniform_buffer.bind_base();
            self.generic_renderer.render(render_packet);
            //self.background_renderer.render(background);
            //self.scene_renderer.render(render_packet);
            //self.frame_buffer.blit();
            //self.frame_buffer.unbind();
            // Post-processing
            //self.postprocess_renderer.render(camera_component, foreground, &self.frame_buffer);
            gl::Disable(gl::DEPTH_TEST);
            self.matrix_uniform_buffer.unbind_base();
            //self.gui_renderer.render(gui_manager, self.frame_buffer.get_aspect_ratio());
        };
    }

    fn refresh_buffers(&mut self, render_packet: &RenderPacket) {
        if let Some((width, height)) = render_packet.window_resize {
            self.on_resize(
                width,
                height,
                render_packet
                    .camera_component
                    .as_ref()
                    .expect("Camera component must be present for resize"),
            );
        }
        if let Some(camera_component) = &render_packet.camera_component {
            unsafe {
                self.matrix_uniform_buffer
                    .set_view_matrix(camera_component.get_view_matrix());
            }
        }
    }

    fn on_resize(&mut self, width: i32, height: i32, camera_component: &CameraComponent) {
        self.frame_buffer.resize(width, height);
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            self.matrix_uniform_buffer.set_projection_matrix(
                camera_component.get_projection_matrix(width as f32 / height as f32),
            );
        };
    }
}
