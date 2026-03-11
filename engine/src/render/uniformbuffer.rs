use gl::types::{GLintptr, GLsizeiptr, GLuint, GLvoid};

#[derive(Debug, Clone, Copy)]
pub struct MatrixUniformBuffer {
    pub projection_matrix: [[f32; 4]; 4],
    pub view_matrix: [[f32; 4]; 4],
}

#[derive(Debug, Clone, Copy)]
pub struct PostProcessUniformBuffer {
    pub saturation: f32,
    pub tint_intensity: f32,
    pub darkening_factor: f32,
    pub focal_radius: f32,
    pub tint_color: [f32; 3], // Placing is important because of padding
    pub smooth_factor: f32,
    pub vignette_intensity: f32,
}

pub struct UniformBuffer<T> {
    pub ubo: GLuint,
    binding_index: GLuint,
    allocated: GLsizeiptr,
    content: Option<T>,
}

impl<T> UniformBuffer<T> {
    pub unsafe fn create(binding_index: GLuint) -> UniformBuffer<T> {
        let mut ubo: GLuint = 0;
        gl::GenBuffers(1, &mut ubo);
        Self {
            ubo,
            binding_index,
            allocated: 0,
            content: None,
        }
    }

    pub unsafe fn set_data(&mut self, obj: T) {
        self.bind();
        let byte_size = std::mem::size_of::<T>() as GLsizeiptr;
        gl::BufferData(
            gl::UNIFORM_BUFFER,
            byte_size,
            &obj as *const _ as *const GLvoid,
            gl::DYNAMIC_DRAW,
        );
        self.allocated = byte_size;
        self.content = Some(obj);
        self.unbind();
    }

    pub unsafe fn set_sub_data<SubDataType>(&self, offset: GLintptr, obj: &SubDataType) {
        self.bind();
        let byte_size = std::mem::size_of::<SubDataType>() as GLsizeiptr;
        gl::BufferSubData(
            gl::UNIFORM_BUFFER,
            offset,
            byte_size,
            obj as *const _ as *const GLvoid,
        );
        self.unbind();
    }

    unsafe fn bind(&self) {
        gl::BindBuffer(gl::UNIFORM_BUFFER, self.ubo);
    }

    unsafe fn unbind(&self) {
        gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
    }

    pub unsafe fn bind_base(&self) {
        gl::BindBufferBase(gl::UNIFORM_BUFFER, self.binding_index, self.ubo);
    }

    pub unsafe fn unbind_base(&self) {
        gl::BindBufferBase(gl::UNIFORM_BUFFER, self.binding_index, 0);
    }
}

impl UniformBuffer<MatrixUniformBuffer> {
    pub unsafe fn set_projection_matrix(&mut self, projection_matrix: [[f32; 4]; 4]) {
        if let Some(content) = &mut self.content {
            let offset = 0;
            content.projection_matrix = projection_matrix;
            self.set_sub_data(offset, &projection_matrix);
        }
    }

    pub unsafe fn set_view_matrix(&mut self, view_matrix: [[f32; 4]; 4]) {
        if let Some(content) = &mut self.content {
            let offset = std::mem::size_of::<[[f32; 4]; 4]>() as GLintptr; // Skip projection matrix
            content.view_matrix = view_matrix;
            self.set_sub_data(offset, &view_matrix);
        }
    }
}

impl UniformBuffer<PostProcessUniformBuffer> {
    pub unsafe fn set_focal_radius(&mut self, focal_radius: f32) {
        if let Some(content) = &mut self.content {
            let offset = 3 * std::mem::size_of::<f32>() as GLintptr; // Skip 3 floats
            content.focal_radius = focal_radius;
            self.set_sub_data(offset, &focal_radius);
        }
    }
}

#[derive(Clone, Debug)]
pub struct UniformBufferData {
    pub binding_index: GLuint,
    pub data: Vec<u8>,
}

pub struct UniformBufferSource<T: Copy> {
    binding_index: GLuint,
    pub data: T,
}

impl<T: Copy> UniformBufferSource<T> {
    pub fn new(binding_index: GLuint, data: T) -> Self {
        Self {
            binding_index,
            data,
        }
    }

    pub fn update(&mut self, new_data: T) {
        self.data = new_data;
    }

    pub fn extract(&self) -> UniformBufferData {
        let data_bytes = unsafe {
            std::slice::from_raw_parts(
                &self.data as *const T as *const u8,
                std::mem::size_of::<T>(),
            )
        };
        UniformBufferData {
            binding_index: self.binding_index,
            data: data_bytes.to_vec(),
        }
    }
}

impl UniformBufferData {
    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::UNIFORM_BUFFER, self.binding_index) };
    }

    fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::UNIFORM_BUFFER, 0) };
    }

    pub fn generate_buffer(&self) -> GLuint {
        let mut ubo: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut ubo) };
        ubo
    }

    pub fn initialize_buffer_data(&self) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                self.data.len() as GLsizeiptr,
                &self.data as *const _ as *const GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
        self.unbind();
    }

    pub fn refresh_buffer_data(&self) {
        self.bind();
        unsafe {
            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                0,
                self.data.len() as GLsizeiptr,
                &self.data as *const _ as *const GLvoid,
            );
        }
        self.unbind();
    }
}
