use super::*;

use std::ffi::CString;

mod native_gl {
    include!(concat!(env!("OUT_DIR"), "/opengl_bindings.rs"));
}

struct Constants {
    max_label_length: i32,
}

pub struct Context {
    raw: native_gl::Gl,
    constants: Constants,
}

impl Context {
    pub fn from_loader_function<F>(loader_function: F) -> Self
    where
        F: FnMut(&str) -> *const std::os::raw::c_void,
    {
        let raw = native_gl::Gl::load_with(loader_function);
        let constants = Constants {
            max_label_length: {
                let mut val = 0;
                unsafe {
                    raw.GetIntegerv(MAX_LABEL_LENGTH, &mut val);
                }
                val
            },
        };
        Context { raw, constants }
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO
        write!(f, "TODO")
    }
}

impl super::Context for Context {
    type Shader = native_gl::types::GLuint;
    type Program = native_gl::types::GLuint;
    type Buffer = native_gl::types::GLuint;
    type VertexArray = native_gl::types::GLuint;
    type Texture = native_gl::types::GLuint;
    type Sampler = native_gl::types::GLuint;
    type Fence = native_gl::types::GLsync;
    type Framebuffer = native_gl::types::GLuint;
    type Renderbuffer = native_gl::types::GLuint;
    type UniformLocation = native_gl::types::GLuint;

    unsafe fn create_framebuffer(&self) -> Result<Self::Framebuffer, String> {
        let gl = &self.raw;
        let mut name = 0;
        gl.GenFramebuffers(1, &mut name);
        Ok(name)
    }

    unsafe fn create_renderbuffer(&self) -> Result<Self::Renderbuffer, String> {
        let gl = &self.raw;
        let mut name = 0;
        gl.GenRenderbuffers(1, &mut name);
        Ok(name)
    }

    unsafe fn create_sampler(&self) -> Result<Self::Sampler, String> {
        let gl = &self.raw;
        let mut name = 0;
        gl.GenSamplers(1, &mut name);
        Ok(name)
    }

    unsafe fn create_shader(&self, shader_type: u32) -> Result<Self::Shader, String> {
        let gl = &self.raw;
        Ok(gl.CreateShader(shader_type as u32))
    }

    unsafe fn create_texture(&self) -> Result<Self::Texture, String> {
        let gl = &self.raw;
        let mut name = 0;
        gl.GenTextures(1, &mut name);
        Ok(name)
    }

    unsafe fn delete_shader(&self, shader: Self::Shader) {
        let gl = &self.raw;
        gl.DeleteShader(shader);
    }

    unsafe fn shader_source(&self, shader: Self::Shader, source: &str) {
        let gl = &self.raw;
        gl.ShaderSource(
            shader,
            1,
            &(source.as_ptr() as *const native_gl::types::GLchar),
            &(source.len() as native_gl::types::GLint),
        );
    }

    unsafe fn compile_shader(&self, shader: Self::Shader) {
        let gl = &self.raw;
        gl.CompileShader(shader);
    }

    unsafe fn get_shader_compile_status(&self, shader: Self::Shader) -> bool {
        let gl = &self.raw;
        let mut status = 0;
        gl.GetShaderiv(shader, COMPILE_STATUS, &mut status);
        1 == status
    }

    unsafe fn get_shader_info_log(&self, shader: Self::Shader) -> String {
        let gl = &self.raw;
        let mut length = 0;
        gl.GetShaderiv(shader, INFO_LOG_LENGTH, &mut length);
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(std::iter::repeat('\0').take(length as usize));
            gl.GetShaderInfoLog(
                shader,
                length,
                &mut length,
                (&log[..]).as_ptr() as *mut native_gl::types::GLchar,
            );
            log.truncate(length as usize);
            log
        } else {
            String::from("")
        }
    }

    unsafe fn get_tex_image_u8_slice(
        &self,
        target: u32,
        level: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        let gl = &self.raw;
        gl.GetTexImage(
            target,
            level,
            format,
            ty,
            pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *mut std::ffi::c_void,
        );
    }

    unsafe fn get_tex_image_pixel_buffer_offset(
        &self,
        target: u32,
        level: i32,
        format: u32,
        ty: u32,
        pixel_buffer_offset: i32,
    ) {
        let gl = &self.raw;
        gl.GetTexImage(
            target,
            level,
            format,
            ty,
            pixel_buffer_offset as *mut std::ffi::c_void,
        );
    }

    unsafe fn create_program(&self) -> Result<Self::Program, String> {
        let gl = &self.raw;
        Ok(gl.CreateProgram())
    }

    unsafe fn delete_program(&self, program: Self::Program) {
        let gl = &self.raw;
        gl.DeleteProgram(program);
    }

    unsafe fn attach_shader(&self, program: Self::Program, shader: Self::Shader) {
        let gl = &self.raw;
        gl.AttachShader(program, shader);
    }

    unsafe fn detach_shader(&self, program: Self::Program, shader: Self::Shader) {
        let gl = &self.raw;
        gl.DetachShader(program, shader);
    }

    unsafe fn link_program(&self, program: Self::Program) {
        let gl = &self.raw;
        gl.LinkProgram(program);
    }

    unsafe fn get_program_link_status(&self, program: Self::Program) -> bool {
        let gl = &self.raw;
        let mut status = 0;
        gl.GetProgramiv(program, LINK_STATUS, &mut status);
        1 == status
    }

    unsafe fn get_program_info_log(&self, program: Self::Program) -> String {
        let gl = &self.raw;
        let mut length = 0;
        gl.GetProgramiv(program, INFO_LOG_LENGTH, &mut length);
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(std::iter::repeat('\0').take(length as usize));
            gl.GetProgramInfoLog(
                program,
                length,
                &mut length,
                (&log[..]).as_ptr() as *mut native_gl::types::GLchar,
            );
            log.truncate(length as usize);
            log
        } else {
            String::from("")
        }
    }

    unsafe fn get_active_uniforms(&self, program: Self::Program) -> u32 {
        let gl = &self.raw;
        let mut count = 0;
        gl.GetProgramiv(program, ACTIVE_UNIFORMS, &mut count);
        count as u32
    }

    unsafe fn get_active_uniform(
        &self,
        program: Self::Program,
        index: u32,
    ) -> Option<ActiveUniform> {
        let gl = &self.raw;
        let mut uniform_max_size = 0;
        gl.GetProgramiv(program, ACTIVE_UNIFORM_MAX_LENGTH, &mut uniform_max_size);

        let mut name = String::with_capacity(uniform_max_size as usize);
        name.extend(std::iter::repeat('\0').take(uniform_max_size as usize));
        let mut length = 0;
        let mut size = 0;
        let mut utype = 0;
        gl.GetActiveUniform(
            program,
            index,
            uniform_max_size,
            &mut length,
            &mut size,
            &mut utype,
            name.as_ptr() as *mut native_gl::types::GLchar,
        );
        name.truncate(length as usize);

        Some(ActiveUniform { size, utype, name })
    }

    unsafe fn use_program(&self, program: Option<Self::Program>) {
        let gl = &self.raw;
        gl.UseProgram(program.unwrap_or(0));
    }

    unsafe fn create_buffer(&self) -> Result<Self::Buffer, String> {
        let gl = &self.raw;
        let mut buffer = 0;
        gl.GenBuffers(1, &mut buffer);
        Ok(buffer)
    }

    unsafe fn bind_buffer(&self, target: u32, buffer: Option<Self::Buffer>) {
        let gl = &self.raw;
        gl.BindBuffer(target, buffer.unwrap_or(0));
    }

    unsafe fn bind_buffer_range(
        &self,
        target: u32,
        index: u32,
        buffer: Option<Self::Buffer>,
        offset: i32,
        size: i32,
    ) {
        let gl = &self.raw;
        gl.BindBufferRange(
            target,
            index,
            buffer.unwrap_or(0),
            offset as isize,
            size as isize,
        );
    }

    unsafe fn bind_framebuffer(&self, target: u32, framebuffer: Option<Self::Framebuffer>) {
        let gl = &self.raw;
        gl.BindFramebuffer(target, framebuffer.unwrap_or(0));
    }

    unsafe fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<Self::Renderbuffer>) {
        let gl = &self.raw;
        gl.BindRenderbuffer(target, renderbuffer.unwrap_or(0));
    }

    unsafe fn blit_framebuffer(
        &self,
        src_x0: i32,
        src_y0: i32,
        src_x1: i32,
        src_y1: i32,
        dst_x0: i32,
        dst_y0: i32,
        dst_x1: i32,
        dst_y1: i32,
        mask: u32,
        filter: u32,
    ) {
        let gl = &self.raw;
        gl.BlitFramebuffer(
            src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
        );
    }

    unsafe fn create_vertex_array(&self) -> Result<Self::VertexArray, String> {
        let gl = &self.raw;
        let mut vertex_array = 0;
        gl.GenVertexArrays(1, &mut vertex_array);
        Ok(vertex_array)
    }

    unsafe fn delete_vertex_array(&self, vertex_array: Self::VertexArray) {
        let gl = &self.raw;
        gl.DeleteVertexArrays(1, &vertex_array);
    }

    unsafe fn bind_vertex_array(&self, vertex_array: Option<Self::VertexArray>) {
        let gl = &self.raw;
        gl.BindVertexArray(vertex_array.unwrap_or(0));
    }

    unsafe fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        let gl = &self.raw;
        gl.ClearColor(red, green, blue, alpha);
    }

    unsafe fn supports_f64_precision() -> bool {
        // TODO: Handle OpenGL ES
        true
    }

    unsafe fn clear_depth_f64(&self, depth: f64) {
        let gl = &self.raw;
        gl.ClearDepth(depth);
    }

    unsafe fn clear_depth_f32(&self, depth: f32) {
        let gl = &self.raw;
        gl.ClearDepthf(depth);
    }

    unsafe fn clear_stencil(&self, stencil: i32) {
        let gl = &self.raw;
        gl.ClearStencil(stencil);
    }

    unsafe fn clear(&self, mask: u32) {
        let gl = &self.raw;
        gl.Clear(mask);
    }

    unsafe fn patch_parameter_i32(&self, parameter: u32, value: i32) {
        let gl = &self.raw;
        gl.PatchParameteri(parameter, value);
    }

    unsafe fn pixel_store_i32(&self, parameter: u32, value: i32) {
        let gl = &self.raw;
        gl.PixelStorei(parameter, value);
    }

    unsafe fn pixel_store_bool(&self, parameter: u32, value: bool) {
        let gl = &self.raw;
        gl.PixelStorei(parameter, value as i32);
    }

    unsafe fn bind_frag_data_location(
        &self,
        program: Self::Program,
        color_number: u32,
        name: &str,
    ) {
        let gl = &self.raw;
        gl.BindFragDataLocation(program, color_number, name.as_ptr() as *const native_gl::types::GLchar);
    }

    unsafe fn buffer_data_size(&self, target: u32, size: i32, usage: u32) {
        let gl = &self.raw;
        gl.BufferData(target, size as isize, std::ptr::null(), usage);
    }

    unsafe fn buffer_data_u8_slice(&self, target: u32, data: &[u8], usage: u32) {
        let gl = &self.raw;
        gl.BufferData(
            target,
            data.len() as isize,
            data.as_ptr() as *const std::ffi::c_void,
            usage,
        );
    }

    unsafe fn buffer_sub_data_u8_slice(&self, target: u32, offset: i32, src_data: &[u8]) {
        let gl = &self.raw;
        gl.BufferSubData(
            target,
            offset as isize,
            src_data.len() as isize,
            src_data.as_ptr() as *const std::ffi::c_void,
        );
    }

    unsafe fn get_buffer_sub_data(&self, target: u32, offset: i32, dst_data: &mut [u8]) {
        let gl = &self.raw;
        gl.GetBufferSubData(
            target,
            offset as isize,
            dst_data.len() as isize,
            dst_data.as_mut_ptr() as *mut std::ffi::c_void,
        );
    }

    unsafe fn buffer_storage(&self, target: u32, size: i32, data: Option<&mut [u8]>, flags: u32) {
        let gl = &self.raw;
        gl.BufferStorage(
            target,
            size as isize,
            data.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
            flags,
        );
    }

    unsafe fn check_framebuffer_status(&self, target: u32) -> u32 {
        let gl = &self.raw;
        gl.CheckFramebufferStatus(target)
    }

    unsafe fn clear_buffer_i32_slice(&self, target: u32, draw_buffer: u32, values: &mut [i32]) {
        let gl = &self.raw;
        gl.ClearBufferiv(target, draw_buffer as i32, values.as_ptr());
    }

    unsafe fn clear_buffer_u32_slice(&self, target: u32, draw_buffer: u32, values: &mut [u32]) {
        let gl = &self.raw;
        gl.ClearBufferuiv(target, draw_buffer as i32, values.as_ptr());
    }

    unsafe fn clear_buffer_f32_slice(&self, target: u32, draw_buffer: u32, values: &mut [f32]) {
        let gl = &self.raw;
        gl.ClearBufferfv(target, draw_buffer as i32, values.as_ptr());
    }

    unsafe fn clear_buffer_depth_stencil(
        &self,
        target: u32,
        draw_buffer: u32,
        depth: f32,
        stencil: i32,
    ) {
        let gl = &self.raw;
        gl.ClearBufferfi(target, draw_buffer as i32, depth, stencil);
    }

    unsafe fn client_wait_sync(&self, fence: Self::Fence, flags: u32, timeout: i32) -> u32 {
        let gl = &self.raw;
        gl.ClientWaitSync(fence, flags, timeout as u64)
    }

    unsafe fn copy_buffer_sub_data(
        &self,
        src_target: u32,
        dst_target: u32,
        src_offset: i32,
        dst_offset: i32,
        size: i32,
    ) {
        let gl = &self.raw;
        gl.CopyBufferSubData(
            src_target,
            dst_target,
            src_offset as isize,
            dst_offset as isize,
            size as isize,
        );
    }

    unsafe fn delete_buffer(&self, buffer: Self::Buffer) {
        let gl = &self.raw;
        gl.DeleteBuffers(1, &buffer);
    }

    unsafe fn delete_framebuffer(&self, framebuffer: Self::Framebuffer) {
        let gl = &self.raw;
        gl.DeleteFramebuffers(1, &framebuffer);
    }

    unsafe fn delete_renderbuffer(&self, renderbuffer: Self::Renderbuffer) {
        let gl = &self.raw;
        gl.DeleteRenderbuffers(1, &renderbuffer);
    }

    unsafe fn delete_sampler(&self, sampler: Self::Sampler) {
        let gl = &self.raw;
        gl.DeleteSamplers(1, &sampler);
    }

    unsafe fn delete_sync(&self, fence: Self::Fence) {
        let gl = &self.raw;
        gl.DeleteSync(fence);
    }

    unsafe fn delete_texture(&self, texture: Self::Texture) {
        let gl = &self.raw;
        gl.DeleteTextures(1, &texture);
    }

    unsafe fn disable(&self, parameter: u32) {
        let gl = &self.raw;
        gl.Disable(parameter);
    }

    unsafe fn disable_draw_buffer(&self, parameter: u32, draw_buffer: u32) {
        let gl = &self.raw;
        gl.Disablei(parameter, draw_buffer);
    }

    unsafe fn disable_vertex_attrib_array(&self, index: u32) {
        let gl = &self.raw;
        gl.DisableVertexAttribArray(index);
    }

    unsafe fn dispatch_compute(&self, groups_x: u32, groups_y: u32, groups_z: u32) {
        let gl = &self.raw;
        gl.DispatchCompute(groups_x, groups_y, groups_z);
    }

    unsafe fn dispatch_compute_indirect(&self, offset: i32) {
        let gl = &self.raw;
        gl.DispatchComputeIndirect(offset as isize);
    }

    unsafe fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        let gl = &self.raw;
        gl.DrawArrays(mode as u32, first, count);
    }

    unsafe fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, instance_count: i32) {
        let gl = &self.raw;
        gl.DrawArraysInstanced(mode as u32, first, count, instance_count);
    }

    unsafe fn draw_arrays_instanced_base_instance(
        &self,
        mode: u32,
        first: i32,
        count: i32,
        instance_count: i32,
        base_instance: u32,
    ) {
        let gl = &self.raw;
        gl.DrawArraysInstancedBaseInstance(
            mode as u32,
            first,
            count,
            instance_count,
            base_instance,
        );
    }

    unsafe fn draw_buffer(&self, draw_buffer: u32) {
        let gl = &self.raw;
        gl.DrawBuffer(draw_buffer);
    }

    unsafe fn draw_buffers(&self, buffers: &[u32]) {
        let gl = &self.raw;
        gl.DrawBuffers(buffers.len() as i32, buffers.as_ptr());
    }

    unsafe fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32) {
        let gl = &self.raw;
        gl.DrawElements(
            mode as u32,
            count,
            element_type as u32,
            offset as *const std::ffi::c_void,
        );
    }

    unsafe fn draw_elements_base_vertex(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        base_vertex: i32,
    ) {
        let gl = &self.raw;
        gl.DrawElementsBaseVertex(
            mode as u32,
            count,
            element_type as u32,
            offset as *const std::ffi::c_void,
            base_vertex,
        );
    }

    unsafe fn draw_elements_instanced(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
    ) {
        let gl = &self.raw;
        gl.DrawElementsInstanced(
            mode as u32,
            count,
            element_type as u32,
            offset as *const std::ffi::c_void,
            instance_count,
        );
    }

    unsafe fn draw_elements_instanced_base_vertex(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
        base_vertex: i32,
    ) {
        let gl = &self.raw;
        gl.DrawElementsInstancedBaseVertex(
            mode as u32,
            count,
            element_type as u32,
            offset as *const std::ffi::c_void,
            instance_count,
            base_vertex,
        );
    }

    unsafe fn draw_elements_instanced_base_vertex_base_instance(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
        base_vertex: i32,
        base_instance: u32,
    ) {
        let gl = &self.raw;
        gl.DrawElementsInstancedBaseVertexBaseInstance(
            mode as u32,
            count,
            element_type as u32,
            offset as *const std::ffi::c_void,
            instance_count,
            base_vertex,
            base_instance,
        );
    }

    unsafe fn enable(&self, parameter: u32) {
        let gl = &self.raw;
        gl.Enable(parameter);
    }

    unsafe fn enable_draw_buffer(&self, parameter: u32, draw_buffer: u32) {
        let gl = &self.raw;
        gl.Enablei(parameter, draw_buffer);
    }

    unsafe fn enable_vertex_attrib_array(&self, index: u32) {
        let gl = &self.raw;
        gl.EnableVertexAttribArray(index);
    }

    unsafe fn flush(&self) {
        let gl = &self.raw;
        gl.Flush();
    }

    unsafe fn framebuffer_renderbuffer(
        &self,
        target: u32,
        attachment: u32,
        renderbuffer_target: u32,
        renderbuffer: Option<Self::Renderbuffer>,
    ) {
        let gl = &self.raw;
        gl.FramebufferRenderbuffer(
            target,
            attachment,
            renderbuffer_target,
            renderbuffer.unwrap_or(0),
        );
    }

    unsafe fn framebuffer_texture(
        &self,
        target: u32,
        attachment: u32,
        texture: Option<Self::Texture>,
        level: i32,
    ) {
        let gl = &self.raw;
        gl.FramebufferTexture(target, attachment, texture.unwrap_or(0), level);
    }

    unsafe fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<Self::Texture>,
        level: i32,
    ) {
        let gl = &self.raw;
        gl.FramebufferTexture2D(
            target,
            attachment,
            texture_target,
            texture.unwrap_or(0),
            level,
        );
    }

    unsafe fn framebuffer_texture_3d(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<Self::Texture>,
        level: i32,
        layer: i32,
    ) {
        let gl = &self.raw;
        gl.FramebufferTexture3D(
            target,
            attachment,
            texture_target,
            texture.unwrap_or(0),
            level,
            layer,
        );
    }

    unsafe fn framebuffer_texture_layer(
        &self,
        target: u32,
        attachment: u32,
        texture: Option<Self::Texture>,
        level: i32,
        layer: i32,
    ) {
        let gl = &self.raw;
        gl.FramebufferTextureLayer(target, attachment, texture.unwrap_or(0), level, layer);
    }

    unsafe fn front_face(&self, value: u32) {
        let gl = &self.raw;
        gl.FrontFace(value as u32);
    }

    unsafe fn get_error(&self) -> u32 {
        let gl = &self.raw;
        gl.GetError()
    }

    unsafe fn get_parameter_i32(&self, parameter: u32) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetIntegerv(parameter, &mut value);
        value
    }

    unsafe fn get_parameter_indexed_i32(&self, parameter: u32, index: u32) -> i32 {
        let gl = &self.raw;
        let mut value = 0;
        gl.GetIntegeri_v(parameter, index, &mut value);
        value
    }

    unsafe fn get_parameter_indexed_string(&self, parameter: u32, index: u32) -> String {
        let gl = &self.raw;
        let raw_ptr = gl.GetStringi(parameter, index);
        std::ffi::CStr::from_ptr(raw_ptr as *const native_gl::types::GLchar)
            .to_str()
            .unwrap()
            .to_owned()
    }

    unsafe fn get_parameter_string(&self, parameter: u32) -> String {
        let gl = &self.raw;
        let raw_ptr = gl.GetString(parameter);
        std::ffi::CStr::from_ptr(raw_ptr as *const native_gl::types::GLchar)
            .to_str()
            .unwrap()
            .to_owned()
    }

    unsafe fn get_uniform_location(
        &self,
        program: Self::Program,
        name: &str,
    ) -> Option<Self::UniformLocation> {
        let gl = &self.raw;
        let name = CString::new(name).unwrap();
        let uniform_location = gl.GetUniformLocation(program, name.as_ptr() as *const native_gl::types::GLchar);
        if uniform_location < 0 {
            None
        } else {
            Some(uniform_location as u32)
        }
    }

    unsafe fn get_attrib_location(&self, program: Self::Program, name: &str) -> i32 {
        let gl = &self.raw;
        let name = CString::new(name).unwrap();
        gl.GetAttribLocation(program, name.as_ptr() as *const native_gl::types::GLchar) as i32
    }

    unsafe fn bind_attrib_location(&self, program: Self::Program, index: u32, name: &str) {
        let gl = &self.raw;
        let name = CString::new(name).unwrap();
        gl.BindAttribLocation(program, index, name.as_ptr() as *const native_gl::types::GLchar);
    }

    unsafe fn get_sync_status(&self, fence: Self::Fence) -> u32 {
        let gl = &self.raw;
        let mut len = 0;
        let mut values = [UNSIGNALED as i32];
        gl.GetSynciv(
            fence,
            SYNC_STATUS,
            values.len() as i32,
            &mut len,
            values.as_mut_ptr(),
        );
        values[0] as u32
    }

    unsafe fn is_sync(&self, fence: Self::Fence) -> bool {
        let gl = &self.raw;
        1 == gl.IsSync(fence)
    }

    unsafe fn renderbuffer_storage(
        &self,
        target: u32,
        internal_format: u32,
        width: i32,
        height: i32,
    ) {
        let gl = &self.raw;
        gl.RenderbufferStorage(target, internal_format, width, height);
    }

    unsafe fn sampler_parameter_f32(&self, sampler: Self::Sampler, name: u32, value: f32) {
        let gl = &self.raw;
        gl.SamplerParameterf(sampler, name, value);
    }

    unsafe fn sampler_parameter_f32_slice(
        &self,
        sampler: Self::Sampler,
        name: u32,
        value: &mut [f32],
    ) {
        let gl = &self.raw;
        gl.SamplerParameterfv(sampler, name, value.as_ptr());
    }

    unsafe fn sampler_parameter_i32(&self, sampler: Self::Sampler, name: u32, value: i32) {
        let gl = &self.raw;
        gl.SamplerParameteri(sampler, name, value);
    }

    unsafe fn tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        let gl = &self.raw;
        gl.TexImage2D(
            target,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            ty,
            pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
        );
    }

    unsafe fn tex_image_3d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        let gl = &self.raw;
        gl.TexImage3D(
            target,
            level,
            internal_format,
            width,
            height,
            depth,
            border,
            format,
            ty,
            pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
        );
    }

    unsafe fn tex_storage_2d(
        &self,
        target: u32,
        levels: i32,
        internal_format: u32,
        width: i32,
        height: i32,
    ) {
        let gl = &self.raw;
        gl.TexStorage2D(target, levels, internal_format, width, height);
    }

    unsafe fn tex_storage_3d(
        &self,
        target: u32,
        levels: i32,
        internal_format: u32,
        width: i32,
        height: i32,
        depth: i32,
    ) {
        let gl = &self.raw;
        gl.TexStorage3D(target, levels, internal_format, width, height, depth);
    }

    unsafe fn uniform_1_i32(&self, location: Option<Self::UniformLocation>, x: i32) {
        let gl = &self.raw;
        gl.Uniform1i(location.unwrap_or(0) as i32, x);
    }

    unsafe fn uniform_2_i32(&self, location: Option<Self::UniformLocation>, x: i32, y: i32) {
        let gl = &self.raw;
        gl.Uniform2i(location.unwrap_or(0) as i32, x, y);
    }

    unsafe fn uniform_3_i32(
        &self,
        location: Option<Self::UniformLocation>,
        x: i32,
        y: i32,
        z: i32,
    ) {
        let gl = &self.raw;
        gl.Uniform3i(location.unwrap_or(0) as i32, x, y, z);
    }

    unsafe fn uniform_4_i32(
        &self,
        location: Option<Self::UniformLocation>,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    ) {
        let gl = &self.raw;
        gl.Uniform4i(location.unwrap_or(0) as i32, x, y, z, w);
    }

    unsafe fn uniform_1_i32_slice(
        &self,
        location: Option<Self::UniformLocation>,
        v: &mut [i32; 1],
    ) {
        let gl = &self.raw;
        gl.Uniform1iv(location.unwrap_or(0) as i32, 1, v.as_ptr());
    }

    unsafe fn uniform_2_i32_slice(
        &self,
        location: Option<Self::UniformLocation>,
        v: &mut [i32; 2],
    ) {
        let gl = &self.raw;
        gl.Uniform2iv(location.unwrap_or(0) as i32, 1, v.as_ptr());
    }

    unsafe fn uniform_3_i32_slice(
        &self,
        location: Option<Self::UniformLocation>,
        v: &mut [i32; 3],
    ) {
        let gl = &self.raw;
        gl.Uniform3iv(location.unwrap_or(0) as i32, 1, v.as_ptr());
    }

    unsafe fn uniform_4_i32_slice(
        &self,
        location: Option<Self::UniformLocation>,
        v: &mut [i32; 4],
    ) {
        let gl = &self.raw;
        gl.Uniform4iv(location.unwrap_or(0) as i32, 1, v.as_ptr());
    }

    unsafe fn uniform_1_f32(&self, location: Option<Self::UniformLocation>, x: f32) {
        let gl = &self.raw;
        gl.Uniform1f(location.unwrap_or(0) as i32, x);
    }

    unsafe fn uniform_2_f32(&self, location: Option<Self::UniformLocation>, x: f32, y: f32) {
        let gl = &self.raw;
        gl.Uniform2f(location.unwrap_or(0) as i32, x, y);
    }

    unsafe fn uniform_3_f32(
        &self,
        location: Option<Self::UniformLocation>,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let gl = &self.raw;
        gl.Uniform3f(location.unwrap_or(0) as i32, x, y, z);
    }

    unsafe fn uniform_4_f32(
        &self,
        location: Option<Self::UniformLocation>,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) {
        let gl = &self.raw;
        gl.Uniform4f(location.unwrap_or(0) as i32, x, y, z, w);
    }

    unsafe fn uniform_1_f32_slice(&self, location: Option<Self::UniformLocation>, v: &[f32; 1]) {
        let gl = &self.raw;
        gl.Uniform1fv(location.unwrap_or(0) as i32, 1, v.as_ptr());
    }

    unsafe fn uniform_2_f32_slice(&self, location: Option<Self::UniformLocation>, v: &[f32; 2]) {
        let gl = &self.raw;
        gl.Uniform2fv(location.unwrap_or(0) as i32, 1, v.as_ptr());
    }

    unsafe fn uniform_3_f32_slice(&self, location: Option<Self::UniformLocation>, v: &[f32; 3]) {
        let gl = &self.raw;
        gl.Uniform3fv(location.unwrap_or(0) as i32, 1, v.as_ptr());
    }

    unsafe fn uniform_4_f32_slice(&self, location: Option<Self::UniformLocation>, v: &[f32; 4]) {
        let gl = &self.raw;
        gl.Uniform4fv(location.unwrap_or(0) as i32, 1, v.as_ptr());
    }

    unsafe fn uniform_matrix_2_f32_slice(
        &self,
        location: Option<Self::UniformLocation>,
        transpose: bool,
        v: &[f32; 4],
    ) {
        let gl = &self.raw;
        gl.UniformMatrix2fv(location.unwrap_or(0) as i32, 1, transpose as u8, v.as_ptr());
    }

    unsafe fn uniform_matrix_3_f32_slice(
        &self,
        location: Option<Self::UniformLocation>,
        transpose: bool,
        v: &[f32; 9],
    ) {
        let gl = &self.raw;
        gl.UniformMatrix3fv(location.unwrap_or(0) as i32, 1, transpose as u8, v.as_ptr());
    }

    unsafe fn uniform_matrix_4_f32_slice(
        &self,
        location: Option<Self::UniformLocation>,
        transpose: bool,
        v: &[f32; 16],
    ) {
        let gl = &self.raw;
        gl.UniformMatrix4fv(location.unwrap_or(0) as i32, 1, transpose as u8, v.as_ptr());
    }

    unsafe fn unmap_buffer(&self, target: u32) {
        let gl = &self.raw;
        gl.UnmapBuffer(target);
    }

    unsafe fn cull_face(&self, value: u32) {
        let gl = &self.raw;
        gl.CullFace(value as u32);
    }

    unsafe fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
        let gl = &self.raw;
        gl.ColorMask(red as u8, green as u8, blue as u8, alpha as u8);
    }

    unsafe fn color_mask_draw_buffer(
        &self,
        draw_buffer: u32,
        red: bool,
        green: bool,
        blue: bool,
        alpha: bool,
    ) {
        let gl = &self.raw;
        gl.ColorMaski(draw_buffer, red as u8, green as u8, blue as u8, alpha as u8);
    }

    unsafe fn depth_mask(&self, value: bool) {
        let gl = &self.raw;
        gl.DepthMask(value as u8);
    }

    unsafe fn blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        let gl = &self.raw;
        gl.BlendColor(red, green, blue, alpha);
    }

    unsafe fn line_width(&self, width: f32) {
        let gl = &self.raw;
        gl.LineWidth(width);
    }

    unsafe fn map_buffer_range(
        &self,
        target: u32,
        offset: i32,
        length: i32,
        access: u32,
    ) -> *mut u8 {
        let gl = &self.raw;
        gl.MapBufferRange(target, offset as isize, length as isize, access) as *mut u8
    }

    unsafe fn flush_mapped_buffer_range(
        &self,
        target: u32,
        offset: i32,
        length: i32,
    ) {
        let gl = &self.raw;
        gl.FlushMappedBufferRange(target, offset as isize, length as isize)
    }

    unsafe fn invalidate_buffer_sub_data(
        &self,
        target: u32,
        offset: i32,
        length: i32,
    ) {
        let gl = &self.raw;
        gl.InvalidateBufferSubData(target, offset as isize, length as isize)
    }

    unsafe fn polygon_offset(&self, factor: f32, units: f32) {
        let gl = &self.raw;
        gl.PolygonOffset(factor, units);
    }

    unsafe fn polygon_mode(&self, face: u32, mode: u32) {
        let gl = &self.raw;
        gl.PolygonMode(face as u32, mode as u32);
    }

    unsafe fn finish(&self) {
        let gl = &self.raw;
        gl.Finish();
    }

    unsafe fn bind_texture(&self, target: u32, texture: Option<Self::Texture>) {
        let gl = &self.raw;
        gl.BindTexture(target, texture.unwrap_or(0));
    }

    unsafe fn bind_sampler(&self, unit: u32, sampler: Option<Self::Sampler>) {
        let gl = &self.raw;
        gl.BindSampler(unit, sampler.unwrap_or(0));
    }

    unsafe fn active_texture(&self, unit: u32) {
        let gl = &self.raw;
        gl.ActiveTexture(unit);
    }

    unsafe fn fence_sync(&self, condition: u32, flags: u32) -> Result<Self::Fence, String> {
        let gl = &self.raw;
        Ok(gl.FenceSync(condition as u32, flags))
    }

    unsafe fn tex_parameter_f32(&self, target: u32, parameter: u32, value: f32) {
        let gl = &self.raw;
        gl.TexParameterf(target, parameter, value);
    }

    unsafe fn tex_parameter_i32(&self, target: u32, parameter: u32, value: i32) {
        let gl = &self.raw;
        gl.TexParameteri(target, parameter, value);
    }

    unsafe fn tex_parameter_f32_slice(&self, target: u32, parameter: u32, values: &[f32]) {
        let gl = &self.raw;
        gl.TexParameterfv(target, parameter, values.as_ptr());
    }

    unsafe fn tex_parameter_i32_slice(&self, target: u32, parameter: u32, values: &[i32]) {
        let gl = &self.raw;
        gl.TexParameteriv(target, parameter, values.as_ptr());
    }

    unsafe fn tex_sub_image_2d_u8_slice(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        let gl = &self.raw;
        gl.TexSubImage2D(
            target,
            level,
            x_offset,
            y_offset,
            width,
            height,
            format,
            ty,
            pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
        );
    }

    unsafe fn tex_sub_image_2d_pixel_buffer_offset(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: u32,
        ty: u32,
        pixel_buffer_offset: i32,
    ) {
        let gl = &self.raw;
        gl.TexSubImage2D(
            target,
            level,
            x_offset,
            y_offset,
            width,
            height,
            format,
            ty,
            pixel_buffer_offset as *const std::ffi::c_void,
        );
    }

    unsafe fn tex_sub_image_3d_u8_slice(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        z_offset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        let gl = &self.raw;
        gl.TexSubImage3D(
            target,
            level,
            x_offset,
            y_offset,
            z_offset,
            width,
            height,
            depth,
            format,
            ty,
            pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
        );
    }

    unsafe fn tex_sub_image_3d_pixel_buffer_offset(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        z_offset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        ty: u32,
        pixel_buffer_offset: i32,
    ) {
        let gl = &self.raw;
        gl.TexSubImage3D(
            target,
            level,
            x_offset,
            y_offset,
            z_offset,
            width,
            height,
            depth,
            format,
            ty,
            pixel_buffer_offset as *const std::ffi::c_void,
        );
    }

    unsafe fn depth_func(&self, func: u32) {
        let gl = &self.raw;
        gl.DepthFunc(func as u32);
    }

    unsafe fn depth_range_f32(&self, near: f32, far: f32) {
        let gl = &self.raw;
        gl.DepthRangef(near, far);
    }

    unsafe fn depth_range_f64(&self, near: f64, far: f64) {
        let gl = &self.raw;
        gl.DepthRange(near, far);
    }

    unsafe fn depth_range_f64_slice(&self, first: u32, count: i32, values: &[[f64; 2]]) {
        let gl = &self.raw;
        gl.DepthRangeArrayv(first, count, values.as_ptr() as *const f64);
    }

    unsafe fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        let gl = &self.raw;
        gl.Scissor(x, y, width, height);
    }

    unsafe fn scissor_slice(&self, first: u32, count: i32, scissors: &[[i32; 4]]) {
        let gl = &self.raw;
        gl.ScissorArrayv(first, count, scissors.as_ptr() as *const i32);
    }

    unsafe fn vertex_attrib_divisor(&self, index: u32, divisor: u32) {
        let gl = &self.raw;
        gl.VertexAttribDivisor(index, divisor);
    }

    unsafe fn vertex_attrib_pointer_f32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        let gl = &self.raw;
        gl.VertexAttribPointer(
            index,
            size,
            data_type,
            normalized as u8,
            stride,
            offset as *const std::ffi::c_void,
        );
    }

    unsafe fn vertex_attrib_pointer_i32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    ) {
        let gl = &self.raw;
        gl.VertexAttribIPointer(
            index,
            size,
            data_type,
            stride,
            offset as *const std::ffi::c_void,
        );
    }

    unsafe fn vertex_attrib_pointer_f64(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    ) {
        let gl = &self.raw;
        gl.VertexAttribLPointer(
            index,
            size,
            data_type,
            stride,
            offset as *const std::ffi::c_void,
        );
    }

    unsafe fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        let gl = &self.raw;
        gl.Viewport(x, y, width, height);
    }

    unsafe fn viewport_f32_slice(&self, first: u32, count: i32, values: &[[f32; 4]]) {
        let gl = &self.raw;
        gl.ViewportArrayv(first, count, values.as_ptr() as *const f32);
    }

    unsafe fn blend_equation(&self, mode: u32) {
        let gl = &self.raw;
        gl.BlendEquation(mode as u32);
    }

    unsafe fn blend_equation_draw_buffer(&self, draw_buffer: u32, mode: u32) {
        let gl = &self.raw;
        gl.BlendEquationi(draw_buffer, mode as u32);
    }

    unsafe fn blend_equation_separate(&self, mode_rgb: u32, mode_alpha: u32) {
        let gl = &self.raw;
        gl.BlendEquationSeparate(mode_rgb as u32, mode_alpha as u32);
    }

    unsafe fn blend_equation_separate_draw_buffer(
        &self,
        draw_buffer: u32,
        mode_rgb: u32,
        mode_alpha: u32,
    ) {
        let gl = &self.raw;
        gl.BlendEquationSeparatei(draw_buffer, mode_rgb as u32, mode_alpha as u32);
    }

    unsafe fn blend_func(&self, src: u32, dst: u32) {
        let gl = &self.raw;
        gl.BlendFunc(src as u32, dst as u32);
    }

    unsafe fn blend_func_draw_buffer(&self, draw_buffer: u32, src: u32, dst: u32) {
        let gl = &self.raw;
        gl.BlendFunci(draw_buffer, src as u32, dst as u32);
    }

    unsafe fn blend_func_separate(
        &self,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    ) {
        let gl = &self.raw;
        gl.BlendFuncSeparate(
            src_rgb as u32,
            dst_rgb as u32,
            src_alpha as u32,
            dst_alpha as u32,
        );
    }

    unsafe fn blend_func_separate_draw_buffer(
        &self,
        draw_buffer: u32,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    ) {
        let gl = &self.raw;
        gl.BlendFuncSeparatei(
            draw_buffer,
            src_rgb as u32,
            dst_rgb as u32,
            src_alpha as u32,
            dst_alpha as u32,
        );
    }

    unsafe fn stencil_func(&self, func: u32, reference: i32, mask: u32) {
        let gl = &self.raw;
        gl.StencilFunc(func as u32, reference, mask);
    }

    unsafe fn stencil_func_separate(&self, face: u32, func: u32, reference: i32, mask: u32) {
        let gl = &self.raw;
        gl.StencilFuncSeparate(face as u32, func as u32, reference, mask);
    }

    unsafe fn stencil_mask(&self, mask: u32) {
        let gl = &self.raw;
        gl.StencilMask(mask);
    }

    unsafe fn stencil_mask_separate(&self, face: u32, mask: u32) {
        let gl = &self.raw;
        gl.StencilMaskSeparate(face as u32, mask);
    }

    unsafe fn stencil_op(&self, stencil_fail: u32, depth_fail: u32, pass: u32) {
        let gl = &self.raw;
        gl.StencilOp(stencil_fail as u32, depth_fail as u32, pass as u32);
    }

    unsafe fn stencil_op_separate(&self, face: u32, stencil_fail: u32, depth_fail: u32, pass: u32) {
        let gl = &self.raw;
        gl.StencilOpSeparate(
            face as u32,
            stencil_fail as u32,
            depth_fail as u32,
            pass as u32,
        );
    }

    unsafe fn debug_message_control(
        &self,
        source: u32,
        msg_type: u32,
        severity: u32,
        ids: &[u32],
        enabled: bool,
    ) {
        let gl = &self.raw;

        let ids_ptr = if ids.is_empty() {
            std::ptr::null()
        } else {
            ids.as_ptr()
        };

        gl.DebugMessageControl(
            source,
            msg_type,
            severity,
            ids.len() as i32,
            ids_ptr,
            enabled as u8,
        );
    }

    unsafe fn debug_message_insert<S>(
        &self,
        source: u32,
        msg_type: u32,
        id: u32,
        severity: u32,
        msg: S,
    ) where
        S: AsRef<str>,
    {
        let gl = &self.raw;
        let message = msg.as_ref().as_bytes();
        let length = message.len() as i32;
        gl.DebugMessageInsert(
            source,
            msg_type,
            id,
            severity,
            length,
            message.as_ptr() as *const native_gl::types::GLchar,
        );
    }

    unsafe fn debug_message_callback<F>(&self, mut callback: F)
    where
        F: FnMut(u32, u32, u32, u32, &str)
    {
        let gl = &self.raw;
        gl.DebugMessageCallback(
            raw_debug_message_callback::<F>,
            &mut callback as *mut _ as *mut std::ffi::c_void
        );
    }

    unsafe fn get_debug_message_log(&self, count: u32) -> Vec<DebugMessageLogEntry> {
        let ct = count as usize;
        let mut sources = Vec::with_capacity(ct);
        let mut types = Vec::with_capacity(ct);
        let mut ids = Vec::with_capacity(ct);
        let mut severities = Vec::with_capacity(ct);
        let mut lengths = Vec::with_capacity(ct);
        let buf_size = (count * MAX_DEBUG_MESSAGE_LENGTH) as i32;
        let mut message_log = Vec::with_capacity(buf_size as usize);

        let gl = &self.raw;
        let received = gl.GetDebugMessageLog(
            count,
            buf_size,
            sources.as_mut_ptr(),
            types.as_mut_ptr(),
            ids.as_mut_ptr(),
            severities.as_mut_ptr(),
            lengths.as_mut_ptr(),
            message_log.as_mut_ptr(),
        ) as usize;

        sources.set_len(received);
        types.set_len(received);
        ids.set_len(received);
        severities.set_len(received);
        lengths.set_len(received);
        message_log.set_len(buf_size as usize);

        let mut entries = Vec::new();
        let mut offset = 0;
        for i in 0..received {
            let message =
                std::ffi::CStr::from_ptr(message_log[offset..].as_ptr()).to_string_lossy();
            offset += lengths[i] as usize;
            entries.push(DebugMessageLogEntry {
                source: sources[i],
                msg_type: types[i],
                id: ids[i],
                severity: severities[i],
                message: message.to_string(),
            });
        }

        entries
    }

    unsafe fn push_debug_group<S>(&self, source: u32, id: u32, message: S)
    where
        S: AsRef<str>,
    {
        let gl = &self.raw;
        let msg = message.as_ref().as_bytes();
        let length = msg.len() as i32;
        gl.PushDebugGroup(source, id, length, msg.as_ptr() as *const native_gl::types::GLchar);
    }

    unsafe fn pop_debug_group(&self) {
        let gl = &self.raw;
        gl.PopDebugGroup();
    }

    unsafe fn object_label<S>(&self, identifier: u32, name: u32, label: Option<S>)
    where
        S: AsRef<str>,
    {
        let gl = &self.raw;

        match label {
            Some(l) => {
                let lbl = l.as_ref().as_bytes();
                let length = lbl.len() as i32;
                gl.ObjectLabel(identifier, name, length, lbl.as_ptr() as *const native_gl::types::GLchar);
            }
            None => gl.ObjectLabel(identifier, name, 0, std::ptr::null()),
        }
    }

    unsafe fn get_object_label(&self, identifier: u32, name: u32) -> String {
        let gl = &self.raw;
        let mut len = 0;
        let mut label_buf = Vec::with_capacity(self.constants.max_label_length as usize);
        gl.GetObjectLabel(
            identifier,
            name,
            self.constants.max_label_length,
            &mut len,
            label_buf.as_mut_ptr(),
        );
        label_buf.set_len(len as usize);
        std::ffi::CStr::from_ptr(label_buf.as_ptr())
            .to_str()
            .unwrap()
            .to_owned()
    }

    unsafe fn object_ptr_label<S>(&self, sync: Self::Fence, label: Option<S>)
    where
        S: AsRef<str>,
    {
        let gl = &self.raw;

        match label {
            Some(l) => {
                let lbl = l.as_ref().as_bytes();
                let length = lbl.len() as i32;
                gl.ObjectPtrLabel(
                    sync as *mut std::ffi::c_void,
                    length,
                    lbl.as_ptr() as *const native_gl::types::GLchar
                );
            }
            None => gl.ObjectPtrLabel(sync as *mut std::ffi::c_void, 0, std::ptr::null()),
        }
    }

    unsafe fn get_object_ptr_label(&self, sync: Self::Fence) -> String {
        let gl = &self.raw;
        let mut len = 0;
        let mut label_buf = Vec::with_capacity(self.constants.max_label_length as usize);
        gl.GetObjectPtrLabel(
            sync as *mut std::ffi::c_void,
            self.constants.max_label_length,
            &mut len,
            label_buf.as_mut_ptr(),
        );
        label_buf.set_len(len as usize);
        std::ffi::CStr::from_ptr(label_buf.as_ptr())
            .to_str()
            .unwrap()
            .to_owned()
    }
}

extern "system" fn raw_debug_message_callback<F>(
    source: u32,
    gltype: u32,
    id: u32,
    severity: u32,
    length: i32,
    message: *const native_gl::types::GLchar,
    user_param: *mut std::ffi::c_void,
)
where
    F: FnMut(u32, u32, u32, u32, &str)
{
    match std::panic::catch_unwind(move || {
        unsafe {
            let callback: &mut F = &mut *(user_param as *mut _);
            let slice = std::slice::from_raw_parts(message as *const u8, length as usize);
            let msg = std::str::from_utf8(slice).unwrap();
            (callback)(source, gltype, id, severity, msg);
        }
    }) {
        Ok(_) => (),
        Err(_) => (),
    };
}

pub struct RenderLoop<W> {
    window: W,
}

#[cfg(feature = "glutin")]
impl RenderLoop<glutin::GlWindow> {
    pub fn from_glutin_window(window: glutin::GlWindow) -> Self {
        RenderLoop { window }
    }
}

#[cfg(feature = "glutin")]
impl super::RenderLoop for RenderLoop<glutin::GlWindow> {
    type Window = glutin::GlWindow;

    fn run<F: FnMut(&mut bool) + 'static>(&self, mut callback: F) {
        let mut running = true;
        while running {
            callback(&mut running);
            self.window.swap_buffers().unwrap();
        }
    }
}

#[cfg(feature = "sdl2")]
impl RenderLoop<sdl2::video::Window> {
    pub fn from_sdl_window(window: sdl2::video::Window) -> Self {
        RenderLoop { window }
    }
}

#[cfg(feature = "sdl2")]
impl super::RenderLoop for RenderLoop<sdl2::video::Window> {
    type Window = sdl2::video::Window;

    fn run<F: FnMut(&mut bool) + 'static>(&self, mut callback: F) {
        let mut running = true;
        while running {
            callback(&mut running);
            self.window.gl_swap_window();
        }
    }
}
