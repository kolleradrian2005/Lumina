use std::{ptr};

use gl::types::*;

use crate::engine::model::{model::Model, sprite};

pub struct Framebuffer {
    model: Model,
    fbo: GLuint,
    post_processing_fbo: GLuint,
    texture: GLuint,
    post_processing_texture: GLuint,
    rbo: GLuint,
    width: i32,
    height: i32,
    aspect_ratio: f32,
    msaa: Option<u8>,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, msaa: Option<u8>) -> Self {
        //let msaa_samples: i32 = 16;
        let mut fbo: GLuint = 0;
        let mut rbo: GLuint = 0;
        let mut texture: GLuint = 0;
        let mut post_processing_fbo = 0;
        let mut post_processing_texture: GLuint = 0;
        let textarget = match msaa {
            Some(_) => gl::TEXTURE_2D_MULTISAMPLE,
            None => gl::TEXTURE_2D,
        };
        unsafe {
            /* Build multisampling frame buffer (using MSAA) */

            // Create a texture object
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(textarget, texture);
            if let Some(msaa_samples) = msaa {
                gl::TexImage2DMultisample(
                    textarget,
                    msaa_samples as i32,
                    gl::RGB,
                    width as GLsizei,
                    height as GLsizei,
                    gl::TRUE,
                );
            } else {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGB as GLint,
                    width,
                    height,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    ptr::null(),
                );
            }
            gl::TexParameteri(textarget, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(textarget, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(textarget, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(textarget, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::BindTexture(textarget, 0);

            // Create a renderbuffer object
            gl::GenRenderbuffers(1, &mut rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
            if let Some(msaa_samples) = msaa {
                gl::RenderbufferStorageMultisample(
                    gl::RENDERBUFFER,
                    msaa_samples as i32,
                    gl::DEPTH24_STENCIL8,
                    width as GLsizei,
                    height as GLsizei,
                );
            } else {
                gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width, height);
            }
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

            // Create a framebuffer object
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                textarget,
                texture,
                0,
            );
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                rbo,
            );
            let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            if status != gl::FRAMEBUFFER_COMPLETE {
                panic!(
                    "An error occured while creating framebuffer. Statuscode: {}",
                    status
                );
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            /* Build post-processing frame buffer */

            // Create a post-processing texture object
            gl::GenTextures(1, &mut post_processing_texture);
            gl::BindTexture(gl::TEXTURE_2D, post_processing_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as GLint,
                width,
                height,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);

            // Create a post-processing framebuffer object
            gl::GenFramebuffers(1, &mut post_processing_fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, post_processing_fbo);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                post_processing_texture,
                0,
            );
            let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            if status != gl::FRAMEBUFFER_COMPLETE {
                println!(
                    "An error occured while creating post-processing framebuffer. Statuscode: {}",
                    status
                );
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        Framebuffer {
            model: sprite::square(2.0),
            fbo,
            post_processing_fbo,
            texture,
            post_processing_texture,
            rbo,
            width,
            height,
            aspect_ratio: width as f32 / height as f32,
            msaa,
        }
    }

    fn destroy(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo);
            gl::DeleteFramebuffers(1, &self.post_processing_fbo);
            gl::DeleteRenderbuffers(1, &self.rbo);
            gl::DeleteTextures(1, &self.texture);
            gl::DeleteTextures(1, &self.post_processing_texture);
            self.fbo = 0;
            self.post_processing_fbo = 0;
            self.texture = 0;
            self.post_processing_texture = 0;
            self.rbo = 0;
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.destroy();
        *self = Framebuffer::new(width, height, self.msaa);
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub unsafe fn bind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
    }

    pub unsafe fn blit(&self) {
        gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.fbo);
        gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.post_processing_fbo);
        gl::BlitFramebuffer(
            0,
            0,
            self.width,
            self.height,
            0,
            0,
            self.width,
            self.height,
            gl::COLOR_BUFFER_BIT,
            gl::NEAREST,
        );
    }

    pub unsafe fn unbind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    pub fn get_texture(&self) -> GLuint {
        self.post_processing_texture
    }

    pub fn get_model(&self) -> &Model {
        &self.model
    }
}
