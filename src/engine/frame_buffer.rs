use std::ptr;

use gl::types::*;

use crate::{window_handler, model::Model};

pub struct Framebuffer {
    fbo: GLuint,
    pub model: Model,
    pub texture: GLuint
}

impl Framebuffer {
        pub fn new() -> Self {
            let mut fbo: GLuint = 0;
            let mut texture: GLuint = 0;
            unsafe {
                gl::GenFramebuffers(1, &mut fbo);
                gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
                gl::GenTextures(1, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGB as GLint,
                    window_handler::WINDOW_WIDTH as GLsizei,
                    window_handler::WINDOW_HEIGHT as GLsizei,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    ptr::null()
                );
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture, 0);
                let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
                if status != gl::FRAMEBUFFER_COMPLETE {
                    println!("An error occured. Statuscode: {}", status);
                }
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            }
            let vertices: &[f32] = &[
                -1.0, -1.0, 1.0,
                1.0, -1.0, 1.0,
                1.0, 1.0, 1.0,
                -1.0, 1.0, 1.0
            ];
            let indices: &[u32] = &[
                0, 1, 2,
                0, 2, 3
            ];
            let uvs: &[f32] = &[
                0.0, 0.0,
                1.0, 0.0,
                1.0, 1.0,
                0.0, 1.0,
            ];
            let model = Model::new(vertices, indices, uvs);
            Framebuffer { fbo, model, texture }
        }

        pub fn bind(&self) {
            unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo) };
        }
        pub fn unbind(&self) {
            unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0) };
        }
}
