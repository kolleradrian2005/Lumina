pub unsafe fn check_gl_error(file: &str, line: u32) -> bool {
    let mut had_error = false;
    loop {
        let err = gl::GetError();
        if err == gl::NO_ERROR {
            break;
        }
        had_error = true;
        let err_str = match err {
            gl::INVALID_ENUM => "INVALID_ENUM",
            gl::INVALID_VALUE => "INVALID_VALUE",
            gl::INVALID_OPERATION => "INVALID_OPERATION",
            gl::STACK_OVERFLOW => "STACK_OVERFLOW",
            gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
            gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
            gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
            _ => "UNKNOWN",
        };
        log::error!("[GL Error] {err_str} (0x{err:04X}) at {file}:{line}");
    }
    had_error
}
