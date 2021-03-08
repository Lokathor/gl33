use super::*;

#[repr(transparent)]
struct GlFnCell<T>(core::cell::UnsafeCell<Option<T>>);

// Note(Lokathor): _p for ptr, _t for type

// Note(Lokathor): This is a **lie**, and `GlFnCell` MUST remain private to this
// module for safety to be upheld.
unsafe impl<T> Sync for GlFnCell<T> {}

fn gl_ptr_filter(p: *const c_void) -> Option<core::ptr::NonNull<c_void>> {
  match p as usize {
    // Note(Lokathor): wgl is known to sometimes give phony non-null pointer values.
    0 | 1 | 2 | 3 | usize::MAX => None,
    _ => unsafe { core::mem::transmute(p) },
  }
}

#[cold]
#[inline(never)]
#[cfg_attr(feature = "track_caller", track_caller)]
fn gl_not_loaded(name: &str) -> ! {
  panic!("GL function not loaded: {name}", name = name);
}

/// Loads function pointer for global-style GL usage.
///
/// Note: This function is effectively just a shortcut for calling each
/// individual function pointer loader function individually. The individual
/// function pointer loaders are named `{cmd}_load_with`, though they are all
/// `doc(hidden)` to avoid polluting the module docs.
///
/// ## Safety
/// * The "Get Proc Address" function you provide will always be given a pointer
///   to the start of a null-terminated string containing the name of a GL
///   function to load.
/// * The "Get Proc Address" function given must always return accurate function
///   pointer values, or null on failure.
/// * This action alters non-atomic global memory (`static` UnsafeCell values),
///   and it is not synchronized. If your program is using GL in a
///   multi-threaded way you **must** provide external synchronization of your
///   own. The best practice is to initialize global GL *before* spawning any
///   other threads.
/// * This function is only safe to use if all GL contexts in your program will
///   be able to share the same set of function pointers. Here's some guidance,
///   depending on platform:
///   * Windows: In practice, if two GL contexts come from the same vendor, and
///     refer to the same GPU, and are for the same pixel format, then they will
///     use identical functions. See Also [the OpenGL
///     wiki](https://www.khronos.org/opengl/wiki/Load_OpenGL_Functions#Windows).
///   * Mac: While GL is increasingly less supported with each passing release
///     of Mac, starting in 10.2 all GL symbols are weak-linked. This means,
///     implicitly, that a single set of symbols is valid for your whole
///     program, regardless of a particular context. You must still only call
///     functions that are valid for your current context's API version and
///     available extensions.
///   * Linux: On Linux the GL functions are not context specific. You must
///     still only call functions that are valid for your current context's API
///     version and available extensions.
pub unsafe fn load_global_gl(f: &dyn Fn(*const u8) -> *const c_void) {
  glActiveTexture_load_with(f);
  glAttachShader_load_with(f);
  glBeginConditionalRender_load_with(f);
  glBeginQuery_load_with(f);
  glBeginTransformFeedback_load_with(f);
  glBindAttribLocation_load_with(f);
  glBindBuffer_load_with(f);
  glBindBufferBase_load_with(f);
  glBindBufferRange_load_with(f);
  glBindFragDataLocation_load_with(f);
  glBindFragDataLocationIndexed_load_with(f);
  glBindFramebuffer_load_with(f);
  glBindRenderbuffer_load_with(f);
  glBindSampler_load_with(f);
  glBindTexture_load_with(f);
  glBindVertexArray_load_with(f);
  glBlendColor_load_with(f);
  glBlendEquation_load_with(f);
  glBlendEquationSeparate_load_with(f);
  glBlendFunc_load_with(f);
  glBlendFuncSeparate_load_with(f);
  glBlitFramebuffer_load_with(f);
  glBufferData_load_with(f);
  glBufferSubData_load_with(f);
  glCheckFramebufferStatus_load_with(f);
  glClampColor_load_with(f);
  glClear_load_with(f);
  glClearBufferfi_load_with(f);
  glClearBufferfv_load_with(f);
  glClearBufferiv_load_with(f);
  glClearBufferuiv_load_with(f);
  glClearColor_load_with(f);
  glClearDepth_load_with(f);
  glClearStencil_load_with(f);
  glClientWaitSync_load_with(f);
  glColorMask_load_with(f);
  glColorMaski_load_with(f);
  glCompileShader_load_with(f);
  glCompressedTexImage1D_load_with(f);
  glCompressedTexImage2D_load_with(f);
  glCompressedTexImage3D_load_with(f);
  glCompressedTexSubImage1D_load_with(f);
  glCompressedTexSubImage2D_load_with(f);
  glCompressedTexSubImage3D_load_with(f);
  glCopyBufferSubData_load_with(f);
  glCopyTexImage1D_load_with(f);
  glCopyTexImage2D_load_with(f);
  glCopyTexSubImage1D_load_with(f);
  glCopyTexSubImage2D_load_with(f);
  glCopyTexSubImage3D_load_with(f);
  glCreateProgram_load_with(f);
  glCreateShader_load_with(f);
  glCullFace_load_with(f);
  glDebugMessageCallback_load_with(f);
  glDebugMessageControl_load_with(f);
  glDebugMessageInsert_load_with(f);
  glDeleteBuffers_load_with(f);
  glDeleteFramebuffers_load_with(f);
  glDeleteProgram_load_with(f);
  glDeleteQueries_load_with(f);
  glDeleteRenderbuffers_load_with(f);
  glDeleteSamplers_load_with(f);
  glDeleteShader_load_with(f);
  glDeleteSync_load_with(f);
  glDeleteTextures_load_with(f);
  glDeleteVertexArrays_load_with(f);
  glDepthFunc_load_with(f);
  glDepthMask_load_with(f);
  glDepthRange_load_with(f);
  glDetachShader_load_with(f);
  glDisable_load_with(f);
  glDisableVertexAttribArray_load_with(f);
  glDisablei_load_with(f);
  glDrawArrays_load_with(f);
  glDrawArraysInstanced_load_with(f);
  glDrawBuffer_load_with(f);
  glDrawBuffers_load_with(f);
  glDrawElements_load_with(f);
  glDrawElementsBaseVertex_load_with(f);
  glDrawElementsInstanced_load_with(f);
  glDrawElementsInstancedBaseVertex_load_with(f);
  glDrawRangeElements_load_with(f);
  glDrawRangeElementsBaseVertex_load_with(f);
  glEnable_load_with(f);
  glEnableVertexAttribArray_load_with(f);
  glEnablei_load_with(f);
  glEndConditionalRender_load_with(f);
  glEndQuery_load_with(f);
  glEndTransformFeedback_load_with(f);
  glFenceSync_load_with(f);
  glFinish_load_with(f);
  glFlush_load_with(f);
  glFlushMappedBufferRange_load_with(f);
  glFramebufferRenderbuffer_load_with(f);
  glFramebufferTexture_load_with(f);
  glFramebufferTexture1D_load_with(f);
  glFramebufferTexture2D_load_with(f);
  glFramebufferTexture3D_load_with(f);
  glFramebufferTextureLayer_load_with(f);
  glFrontFace_load_with(f);
  glGenBuffers_load_with(f);
  glGenFramebuffers_load_with(f);
  glGenQueries_load_with(f);
  glGenRenderbuffers_load_with(f);
  glGenSamplers_load_with(f);
  glGenTextures_load_with(f);
  glGenVertexArrays_load_with(f);
  glGenerateMipmap_load_with(f);
  glGetActiveAttrib_load_with(f);
  glGetActiveUniform_load_with(f);
  glGetActiveUniformBlockName_load_with(f);
  glGetActiveUniformBlockiv_load_with(f);
  glGetActiveUniformName_load_with(f);
  glGetActiveUniformsiv_load_with(f);
  glGetAttachedShaders_load_with(f);
  glGetAttribLocation_load_with(f);
  glGetBooleani_v_load_with(f);
  glGetBooleanv_load_with(f);
  glGetBufferParameteri64v_load_with(f);
  glGetBufferParameteriv_load_with(f);
  glGetBufferPointerv_load_with(f);
  glGetBufferSubData_load_with(f);
  glGetCompressedTexImage_load_with(f);
  glGetDebugMessageLog_load_with(f);
  glGetDoublev_load_with(f);
  glGetError_load_with(f);
  glGetFloatv_load_with(f);
  glGetFragDataIndex_load_with(f);
  glGetFragDataLocation_load_with(f);
  glGetFramebufferAttachmentParameteriv_load_with(f);
  glGetInteger64i_v_load_with(f);
  glGetInteger64v_load_with(f);
  glGetIntegeri_v_load_with(f);
  glGetIntegerv_load_with(f);
  glGetMultisamplefv_load_with(f);
  glGetObjectLabel_load_with(f);
  glGetObjectPtrLabel_load_with(f);
  glGetPointerv_load_with(f);
  glGetProgramInfoLog_load_with(f);
  glGetProgramiv_load_with(f);
  glGetQueryObjecti64v_load_with(f);
  glGetQueryObjectiv_load_with(f);
  glGetQueryObjectui64v_load_with(f);
  glGetQueryObjectuiv_load_with(f);
  glGetQueryiv_load_with(f);
  glGetRenderbufferParameteriv_load_with(f);
  glGetSamplerParameterIiv_load_with(f);
  glGetSamplerParameterIuiv_load_with(f);
  glGetSamplerParameterfv_load_with(f);
  glGetSamplerParameteriv_load_with(f);
  glGetShaderInfoLog_load_with(f);
  glGetShaderSource_load_with(f);
  glGetShaderiv_load_with(f);
  glGetString_load_with(f);
  glGetStringi_load_with(f);
  glGetSynciv_load_with(f);
  glGetTexImage_load_with(f);
  glGetTexLevelParameterfv_load_with(f);
  glGetTexLevelParameteriv_load_with(f);
  glGetTexParameterIiv_load_with(f);
  glGetTexParameterIuiv_load_with(f);
  glGetTexParameterfv_load_with(f);
  glGetTexParameteriv_load_with(f);
  glGetTransformFeedbackVarying_load_with(f);
  glGetUniformBlockIndex_load_with(f);
  glGetUniformIndices_load_with(f);
  glGetUniformLocation_load_with(f);
  glGetUniformfv_load_with(f);
  glGetUniformiv_load_with(f);
  glGetUniformuiv_load_with(f);
  glGetVertexAttribIiv_load_with(f);
  glGetVertexAttribIuiv_load_with(f);
  glGetVertexAttribPointerv_load_with(f);
  glGetVertexAttribdv_load_with(f);
  glGetVertexAttribfv_load_with(f);
  glGetVertexAttribiv_load_with(f);
  glHint_load_with(f);
  glIsBuffer_load_with(f);
  glIsEnabled_load_with(f);
  glIsEnabledi_load_with(f);
  glIsFramebuffer_load_with(f);
  glIsProgram_load_with(f);
  glIsQuery_load_with(f);
  glIsRenderbuffer_load_with(f);
  glIsSampler_load_with(f);
  glIsShader_load_with(f);
  glIsSync_load_with(f);
  glIsTexture_load_with(f);
  glIsVertexArray_load_with(f);
  glLineWidth_load_with(f);
  glLinkProgram_load_with(f);
  glLogicOp_load_with(f);
  glMapBuffer_load_with(f);
  glMapBufferRange_load_with(f);
  glMultiDrawArrays_load_with(f);
  glMultiDrawElements_load_with(f);
  glMultiDrawElementsBaseVertex_load_with(f);
  glObjectLabel_load_with(f);
  glObjectPtrLabel_load_with(f);
  glPixelStoref_load_with(f);
  glPixelStorei_load_with(f);
  glPointParameterf_load_with(f);
  glPointParameterfv_load_with(f);
  glPointParameteri_load_with(f);
  glPointParameteriv_load_with(f);
  glPointSize_load_with(f);
  glPolygonMode_load_with(f);
  glPolygonOffset_load_with(f);
  glPopDebugGroup_load_with(f);
  glPrimitiveRestartIndex_load_with(f);
  glProvokingVertex_load_with(f);
  glPushDebugGroup_load_with(f);
  glQueryCounter_load_with(f);
  glReadBuffer_load_with(f);
  glReadPixels_load_with(f);
  glRenderbufferStorage_load_with(f);
  glRenderbufferStorageMultisample_load_with(f);
  glSampleCoverage_load_with(f);
  glSampleMaski_load_with(f);
  glSamplerParameterIiv_load_with(f);
  glSamplerParameterIuiv_load_with(f);
  glSamplerParameterf_load_with(f);
  glSamplerParameterfv_load_with(f);
  glSamplerParameteri_load_with(f);
  glSamplerParameteriv_load_with(f);
  glScissor_load_with(f);
  glShaderSource_load_with(f);
  glStencilFunc_load_with(f);
  glStencilFuncSeparate_load_with(f);
  glStencilMask_load_with(f);
  glStencilMaskSeparate_load_with(f);
  glStencilOp_load_with(f);
  glStencilOpSeparate_load_with(f);
  glTexBuffer_load_with(f);
  glTexImage1D_load_with(f);
  glTexImage2D_load_with(f);
  glTexImage2DMultisample_load_with(f);
  glTexImage3D_load_with(f);
  glTexImage3DMultisample_load_with(f);
  glTexParameterIiv_load_with(f);
  glTexParameterIuiv_load_with(f);
  glTexParameterf_load_with(f);
  glTexParameterfv_load_with(f);
  glTexParameteri_load_with(f);
  glTexParameteriv_load_with(f);
  glTexSubImage1D_load_with(f);
  glTexSubImage2D_load_with(f);
  glTexSubImage3D_load_with(f);
  glTransformFeedbackVaryings_load_with(f);
  glUniform1f_load_with(f);
  glUniform1fv_load_with(f);
  glUniform1i_load_with(f);
  glUniform1iv_load_with(f);
  glUniform1ui_load_with(f);
  glUniform1uiv_load_with(f);
  glUniform2f_load_with(f);
  glUniform2fv_load_with(f);
  glUniform2i_load_with(f);
  glUniform2iv_load_with(f);
  glUniform2ui_load_with(f);
  glUniform2uiv_load_with(f);
  glUniform3f_load_with(f);
  glUniform3fv_load_with(f);
  glUniform3i_load_with(f);
  glUniform3iv_load_with(f);
  glUniform3ui_load_with(f);
  glUniform3uiv_load_with(f);
  glUniform4f_load_with(f);
  glUniform4fv_load_with(f);
  glUniform4i_load_with(f);
  glUniform4iv_load_with(f);
  glUniform4ui_load_with(f);
  glUniform4uiv_load_with(f);
  glUniformBlockBinding_load_with(f);
  glUniformMatrix2fv_load_with(f);
  glUniformMatrix2x3fv_load_with(f);
  glUniformMatrix2x4fv_load_with(f);
  glUniformMatrix3fv_load_with(f);
  glUniformMatrix3x2fv_load_with(f);
  glUniformMatrix3x4fv_load_with(f);
  glUniformMatrix4fv_load_with(f);
  glUniformMatrix4x2fv_load_with(f);
  glUniformMatrix4x3fv_load_with(f);
  glUnmapBuffer_load_with(f);
  glUseProgram_load_with(f);
  glValidateProgram_load_with(f);
  glVertexAttrib1d_load_with(f);
  glVertexAttrib1dv_load_with(f);
  glVertexAttrib1f_load_with(f);
  glVertexAttrib1fv_load_with(f);
  glVertexAttrib1s_load_with(f);
  glVertexAttrib1sv_load_with(f);
  glVertexAttrib2d_load_with(f);
  glVertexAttrib2dv_load_with(f);
  glVertexAttrib2f_load_with(f);
  glVertexAttrib2fv_load_with(f);
  glVertexAttrib2s_load_with(f);
  glVertexAttrib2sv_load_with(f);
  glVertexAttrib3d_load_with(f);
  glVertexAttrib3dv_load_with(f);
  glVertexAttrib3f_load_with(f);
  glVertexAttrib3fv_load_with(f);
  glVertexAttrib3s_load_with(f);
  glVertexAttrib3sv_load_with(f);
  glVertexAttrib4Nbv_load_with(f);
  glVertexAttrib4Niv_load_with(f);
  glVertexAttrib4Nsv_load_with(f);
  glVertexAttrib4Nub_load_with(f);
  glVertexAttrib4Nubv_load_with(f);
  glVertexAttrib4Nuiv_load_with(f);
  glVertexAttrib4Nusv_load_with(f);
  glVertexAttrib4bv_load_with(f);
  glVertexAttrib4d_load_with(f);
  glVertexAttrib4dv_load_with(f);
  glVertexAttrib4f_load_with(f);
  glVertexAttrib4fv_load_with(f);
  glVertexAttrib4iv_load_with(f);
  glVertexAttrib4s_load_with(f);
  glVertexAttrib4sv_load_with(f);
  glVertexAttrib4ubv_load_with(f);
  glVertexAttrib4uiv_load_with(f);
  glVertexAttrib4usv_load_with(f);
  glVertexAttribDivisor_load_with(f);
  glVertexAttribI1i_load_with(f);
  glVertexAttribI1iv_load_with(f);
  glVertexAttribI1ui_load_with(f);
  glVertexAttribI1uiv_load_with(f);
  glVertexAttribI2i_load_with(f);
  glVertexAttribI2iv_load_with(f);
  glVertexAttribI2ui_load_with(f);
  glVertexAttribI2uiv_load_with(f);
  glVertexAttribI3i_load_with(f);
  glVertexAttribI3iv_load_with(f);
  glVertexAttribI3ui_load_with(f);
  glVertexAttribI3uiv_load_with(f);
  glVertexAttribI4bv_load_with(f);
  glVertexAttribI4i_load_with(f);
  glVertexAttribI4iv_load_with(f);
  glVertexAttribI4sv_load_with(f);
  glVertexAttribI4ubv_load_with(f);
  glVertexAttribI4ui_load_with(f);
  glVertexAttribI4uiv_load_with(f);
  glVertexAttribI4usv_load_with(f);
  glVertexAttribIPointer_load_with(f);
  glVertexAttribP1ui_load_with(f);
  glVertexAttribP1uiv_load_with(f);
  glVertexAttribP2ui_load_with(f);
  glVertexAttribP2uiv_load_with(f);
  glVertexAttribP3ui_load_with(f);
  glVertexAttribP3uiv_load_with(f);
  glVertexAttribP4ui_load_with(f);
  glVertexAttribP4uiv_load_with(f);
  glVertexAttribPointer_load_with(f);
  glViewport_load_with(f);
  glWaitSync_load_with(f);
}

/// glActiveTexture
/// * `texture` group: TextureUnit
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glActiveTexture(texture: TextureUnit) {
  #[allow(unused_unsafe)]
  match unsafe { *glActiveTexture_p.0.get() } {
    Some(glActiveTexture_inner) => glActiveTexture_inner(texture),
    None => gl_not_loaded("glActiveTexture"),
  }
}
static glActiveTexture_p: GlFnCell<glActiveTexture_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glActiveTexture_is_loaded() -> bool {
  unsafe { *glActiveTexture_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glActiveTexture_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glActiveTexture_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glActiveTexture_t>>(gl_ptr_filter(f(b"glActiveTexture\0".as_ptr())));
}
/// [glAttachShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachShader.xhtml)
///
/// Attaches the given shader object to the given program object. You can attach
/// more than one shader of the same type to a program.
///
/// * `program` is the program you're attaching the shader object to.
/// * `shader` is the shader you're attaching.
#[cfg_attr(feature = "track_caller", track_caller)]
pub fn glAttachShader(program: GLuint, shader: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glAttachShader_p.0.get() } {
    Some(glAttachShader_inner) => glAttachShader_inner(program, shader),
    None => gl_not_loaded("glAttachShader"),
  }
}
static glAttachShader_p: GlFnCell<glAttachShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glAttachShader_is_loaded() -> bool {
  unsafe { *glAttachShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glAttachShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glAttachShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glAttachShader_t>>(gl_ptr_filter(f(b"glAttachShader\0".as_ptr())));
}
/// glBeginConditionalRender
/// * `mode` group: ConditionalRenderMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBeginConditionalRender(id: GLuint, mode: ConditionalRenderMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glBeginConditionalRender_p.0.get() } {
    Some(glBeginConditionalRender_inner) => glBeginConditionalRender_inner(id, mode),
    None => gl_not_loaded("glBeginConditionalRender"),
  }
}
static glBeginConditionalRender_p: GlFnCell<glBeginConditionalRender_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBeginConditionalRender_is_loaded() -> bool {
  unsafe { *glBeginConditionalRender_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBeginConditionalRender_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBeginConditionalRender_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBeginConditionalRender_t>>(gl_ptr_filter(f(b"glBeginConditionalRender\0".as_ptr())));
}
/// glBeginQuery
/// * `target` group: QueryTarget
/// * `id` class: query
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBeginQuery(target: QueryTarget, id: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBeginQuery_p.0.get() } {
    Some(glBeginQuery_inner) => glBeginQuery_inner(target, id),
    None => gl_not_loaded("glBeginQuery"),
  }
}
static glBeginQuery_p: GlFnCell<glBeginQuery_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBeginQuery_is_loaded() -> bool {
  unsafe { *glBeginQuery_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBeginQuery_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBeginQuery_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBeginQuery_t>>(gl_ptr_filter(f(b"glBeginQuery\0".as_ptr())));
}
/// glBeginTransformFeedback
/// * `primitiveMode` group: PrimitiveType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBeginTransformFeedback(primitiveMode: PrimitiveType) {
  #[allow(unused_unsafe)]
  match unsafe { *glBeginTransformFeedback_p.0.get() } {
    Some(glBeginTransformFeedback_inner) => glBeginTransformFeedback_inner(primitiveMode),
    None => gl_not_loaded("glBeginTransformFeedback"),
  }
}
static glBeginTransformFeedback_p: GlFnCell<glBeginTransformFeedback_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBeginTransformFeedback_is_loaded() -> bool {
  unsafe { *glBeginTransformFeedback_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBeginTransformFeedback_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBeginTransformFeedback_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBeginTransformFeedback_t>>(gl_ptr_filter(f(b"glBeginTransformFeedback\0".as_ptr())));
}
/// glBindAttribLocation
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindAttribLocation_p.0.get() } {
    Some(glBindAttribLocation_inner) => glBindAttribLocation_inner(program, index, name),
    None => gl_not_loaded("glBindAttribLocation"),
  }
}
static glBindAttribLocation_p: GlFnCell<glBindAttribLocation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindAttribLocation_is_loaded() -> bool {
  unsafe { *glBindAttribLocation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindAttribLocation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindAttribLocation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindAttribLocation_t>>(gl_ptr_filter(f(b"glBindAttribLocation\0".as_ptr())));
}
/// glBindBuffer
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindBuffer(target: BufferTargetARB, buffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindBuffer_p.0.get() } {
    Some(glBindBuffer_inner) => glBindBuffer_inner(target, buffer),
    None => gl_not_loaded("glBindBuffer"),
  }
}
static glBindBuffer_p: GlFnCell<glBindBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindBuffer_is_loaded() -> bool {
  unsafe { *glBindBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindBuffer_t>>(gl_ptr_filter(f(b"glBindBuffer\0".as_ptr())));
}
/// glBindBufferBase
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindBufferBase(target: BufferTargetARB, index: GLuint, buffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindBufferBase_p.0.get() } {
    Some(glBindBufferBase_inner) => glBindBufferBase_inner(target, index, buffer),
    None => gl_not_loaded("glBindBufferBase"),
  }
}
static glBindBufferBase_p: GlFnCell<glBindBufferBase_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindBufferBase_is_loaded() -> bool {
  unsafe { *glBindBufferBase_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindBufferBase_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindBufferBase_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindBufferBase_t>>(gl_ptr_filter(f(b"glBindBufferBase\0".as_ptr())));
}
/// glBindBufferRange
/// * `target` group: BufferTargetARB
/// * `buffer` class: buffer
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindBufferRange(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindBufferRange_p.0.get() } {
    Some(glBindBufferRange_inner) => glBindBufferRange_inner(target, index, buffer, offset, size),
    None => gl_not_loaded("glBindBufferRange"),
  }
}
static glBindBufferRange_p: GlFnCell<glBindBufferRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindBufferRange_is_loaded() -> bool {
  unsafe { *glBindBufferRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindBufferRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindBufferRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindBufferRange_t>>(gl_ptr_filter(f(b"glBindBufferRange\0".as_ptr())));
}
/// glBindFragDataLocation
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindFragDataLocation_p.0.get() } {
    Some(glBindFragDataLocation_inner) => glBindFragDataLocation_inner(program, color, name),
    None => gl_not_loaded("glBindFragDataLocation"),
  }
}
static glBindFragDataLocation_p: GlFnCell<glBindFragDataLocation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindFragDataLocation_is_loaded() -> bool {
  unsafe { *glBindFragDataLocation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindFragDataLocation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindFragDataLocation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindFragDataLocation_t>>(gl_ptr_filter(f(b"glBindFragDataLocation\0".as_ptr())));
}
/// glBindFragDataLocationIndexed
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindFragDataLocationIndexed(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindFragDataLocationIndexed_p.0.get() } {
    Some(glBindFragDataLocationIndexed_inner) => glBindFragDataLocationIndexed_inner(program, colorNumber, index, name),
    None => gl_not_loaded("glBindFragDataLocationIndexed"),
  }
}
static glBindFragDataLocationIndexed_p: GlFnCell<glBindFragDataLocationIndexed_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindFragDataLocationIndexed_is_loaded() -> bool {
  unsafe { *glBindFragDataLocationIndexed_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindFragDataLocationIndexed_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindFragDataLocationIndexed_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindFragDataLocationIndexed_t>>(gl_ptr_filter(f(b"glBindFragDataLocationIndexed\0".as_ptr())));
}
/// glBindFramebuffer
/// * `target` group: FramebufferTarget
/// * `framebuffer` class: framebuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindFramebuffer(target: FramebufferTarget, framebuffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindFramebuffer_p.0.get() } {
    Some(glBindFramebuffer_inner) => glBindFramebuffer_inner(target, framebuffer),
    None => gl_not_loaded("glBindFramebuffer"),
  }
}
static glBindFramebuffer_p: GlFnCell<glBindFramebuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindFramebuffer_is_loaded() -> bool {
  unsafe { *glBindFramebuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindFramebuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindFramebuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindFramebuffer_t>>(gl_ptr_filter(f(b"glBindFramebuffer\0".as_ptr())));
}
/// glBindRenderbuffer
/// * `target` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindRenderbuffer(target: RenderbufferTarget, renderbuffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindRenderbuffer_p.0.get() } {
    Some(glBindRenderbuffer_inner) => glBindRenderbuffer_inner(target, renderbuffer),
    None => gl_not_loaded("glBindRenderbuffer"),
  }
}
static glBindRenderbuffer_p: GlFnCell<glBindRenderbuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindRenderbuffer_is_loaded() -> bool {
  unsafe { *glBindRenderbuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindRenderbuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindRenderbuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindRenderbuffer_t>>(gl_ptr_filter(f(b"glBindRenderbuffer\0".as_ptr())));
}
/// glBindSampler
/// * `sampler` class: sampler
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindSampler(unit: GLuint, sampler: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindSampler_p.0.get() } {
    Some(glBindSampler_inner) => glBindSampler_inner(unit, sampler),
    None => gl_not_loaded("glBindSampler"),
  }
}
static glBindSampler_p: GlFnCell<glBindSampler_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindSampler_is_loaded() -> bool {
  unsafe { *glBindSampler_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindSampler_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindSampler_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindSampler_t>>(gl_ptr_filter(f(b"glBindSampler\0".as_ptr())));
}
/// glBindTexture
/// * `target` group: TextureTarget
/// * `texture` group: Texture
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBindTexture(target: TextureTarget, texture: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindTexture_p.0.get() } {
    Some(glBindTexture_inner) => glBindTexture_inner(target, texture),
    None => gl_not_loaded("glBindTexture"),
  }
}
static glBindTexture_p: GlFnCell<glBindTexture_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindTexture_is_loaded() -> bool {
  unsafe { *glBindTexture_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindTexture_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindTexture_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindTexture_t>>(gl_ptr_filter(f(b"glBindTexture\0".as_ptr())));
}
/// [glBindVertexArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml)
///
/// Binds a given vertex array object as the active vertex array object. Passing
/// 0 will make it so that no vertex array object is bound.
///
/// * `array` names the vertex array object to bind.
#[cfg_attr(feature = "track_caller", track_caller)]
pub fn glBindVertexArray(array: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glBindVertexArray_p.0.get() } {
    Some(glBindVertexArray_inner) => glBindVertexArray_inner(array),
    None => gl_not_loaded("glBindVertexArray"),
  }
}
static glBindVertexArray_p: GlFnCell<glBindVertexArray_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBindVertexArray_is_loaded() -> bool {
  unsafe { *glBindVertexArray_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBindVertexArray_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBindVertexArray_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBindVertexArray_t>>(gl_ptr_filter(f(b"glBindVertexArray\0".as_ptr())));
}
/// glBlendColor
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
/// * `alpha` group: ColorF
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendColor_p.0.get() } {
    Some(glBlendColor_inner) => glBlendColor_inner(red, green, blue, alpha),
    None => gl_not_loaded("glBlendColor"),
  }
}
static glBlendColor_p: GlFnCell<glBlendColor_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendColor_is_loaded() -> bool {
  unsafe { *glBlendColor_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendColor_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendColor_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendColor_t>>(gl_ptr_filter(f(b"glBlendColor\0".as_ptr())));
}
/// glBlendEquation
/// * `mode` group: BlendEquationModeEXT
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendEquation(mode: BlendEquationModeEXT) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendEquation_p.0.get() } {
    Some(glBlendEquation_inner) => glBlendEquation_inner(mode),
    None => gl_not_loaded("glBlendEquation"),
  }
}
static glBlendEquation_p: GlFnCell<glBlendEquation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendEquation_is_loaded() -> bool {
  unsafe { *glBlendEquation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendEquation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendEquation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendEquation_t>>(gl_ptr_filter(f(b"glBlendEquation\0".as_ptr())));
}
/// glBlendEquationSeparate
/// * `modeRGB` group: BlendEquationModeEXT
/// * `modeAlpha` group: BlendEquationModeEXT
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendEquationSeparate(modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendEquationSeparate_p.0.get() } {
    Some(glBlendEquationSeparate_inner) => glBlendEquationSeparate_inner(modeRGB, modeAlpha),
    None => gl_not_loaded("glBlendEquationSeparate"),
  }
}
static glBlendEquationSeparate_p: GlFnCell<glBlendEquationSeparate_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendEquationSeparate_is_loaded() -> bool {
  unsafe { *glBlendEquationSeparate_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendEquationSeparate_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendEquationSeparate_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendEquationSeparate_t>>(gl_ptr_filter(f(b"glBlendEquationSeparate\0".as_ptr())));
}
/// glBlendFunc
/// * `sfactor` group: BlendingFactor
/// * `dfactor` group: BlendingFactor
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendFunc(sfactor: BlendingFactor, dfactor: BlendingFactor) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendFunc_p.0.get() } {
    Some(glBlendFunc_inner) => glBlendFunc_inner(sfactor, dfactor),
    None => gl_not_loaded("glBlendFunc"),
  }
}
static glBlendFunc_p: GlFnCell<glBlendFunc_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendFunc_is_loaded() -> bool {
  unsafe { *glBlendFunc_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendFunc_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendFunc_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendFunc_t>>(gl_ptr_filter(f(b"glBlendFunc\0".as_ptr())));
}
/// glBlendFuncSeparate
/// * `sfactorRGB` group: BlendingFactor
/// * `dfactorRGB` group: BlendingFactor
/// * `sfactorAlpha` group: BlendingFactor
/// * `dfactorAlpha` group: BlendingFactor
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlendFuncSeparate(sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlendFuncSeparate_p.0.get() } {
    Some(glBlendFuncSeparate_inner) => glBlendFuncSeparate_inner(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha),
    None => gl_not_loaded("glBlendFuncSeparate"),
  }
}
static glBlendFuncSeparate_p: GlFnCell<glBlendFuncSeparate_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlendFuncSeparate_is_loaded() -> bool {
  unsafe { *glBlendFuncSeparate_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlendFuncSeparate_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlendFuncSeparate_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlendFuncSeparate_t>>(gl_ptr_filter(f(b"glBlendFuncSeparate\0".as_ptr())));
}
/// glBlitFramebuffer
/// * `mask` group: ClearBufferMask
/// * `filter` group: BlitFramebufferFilter
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter) {
  #[allow(unused_unsafe)]
  match unsafe { *glBlitFramebuffer_p.0.get() } {
    Some(glBlitFramebuffer_inner) => glBlitFramebuffer_inner(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter),
    None => gl_not_loaded("glBlitFramebuffer"),
  }
}
static glBlitFramebuffer_p: GlFnCell<glBlitFramebuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBlitFramebuffer_is_loaded() -> bool {
  unsafe { *glBlitFramebuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBlitFramebuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBlitFramebuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBlitFramebuffer_t>>(gl_ptr_filter(f(b"glBlitFramebuffer\0".as_ptr())));
}
/// glBufferData
/// * `target` group: BufferTargetARB
/// * `size` group: BufferSize
/// * `data` len: size
/// * `usage` group: BufferUsageARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBufferData(target: BufferTargetARB, size: GLsizeiptr, data: *const void, usage: BufferUsageARB) {
  #[allow(unused_unsafe)]
  match unsafe { *glBufferData_p.0.get() } {
    Some(glBufferData_inner) => glBufferData_inner(target, size, data, usage),
    None => gl_not_loaded("glBufferData"),
  }
}
static glBufferData_p: GlFnCell<glBufferData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBufferData_is_loaded() -> bool {
  unsafe { *glBufferData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBufferData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBufferData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBufferData_t>>(gl_ptr_filter(f(b"glBufferData\0".as_ptr())));
}
/// glBufferSubData
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
/// * `data` len: size
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glBufferSubData(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glBufferSubData_p.0.get() } {
    Some(glBufferSubData_inner) => glBufferSubData_inner(target, offset, size, data),
    None => gl_not_loaded("glBufferSubData"),
  }
}
static glBufferSubData_p: GlFnCell<glBufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glBufferSubData_is_loaded() -> bool {
  unsafe { *glBufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glBufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glBufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glBufferSubData_t>>(gl_ptr_filter(f(b"glBufferSubData\0".as_ptr())));
}
/// glCheckFramebufferStatus
/// * `target` group: FramebufferTarget
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCheckFramebufferStatus(target: FramebufferTarget) -> FramebufferStatus {
  #[allow(unused_unsafe)]
  match unsafe { *glCheckFramebufferStatus_p.0.get() } {
    Some(glCheckFramebufferStatus_inner) => glCheckFramebufferStatus_inner(target),
    None => gl_not_loaded("glCheckFramebufferStatus"),
  }
}
static glCheckFramebufferStatus_p: GlFnCell<glCheckFramebufferStatus_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCheckFramebufferStatus_is_loaded() -> bool {
  unsafe { *glCheckFramebufferStatus_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCheckFramebufferStatus_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCheckFramebufferStatus_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCheckFramebufferStatus_t>>(gl_ptr_filter(f(b"glCheckFramebufferStatus\0".as_ptr())));
}
/// glClampColor
/// * `target` group: ClampColorTargetARB
/// * `clamp` group: ClampColorModeARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClampColor(target: ClampColorTargetARB, clamp: ClampColorModeARB) {
  #[allow(unused_unsafe)]
  match unsafe { *glClampColor_p.0.get() } {
    Some(glClampColor_inner) => glClampColor_inner(target, clamp),
    None => gl_not_loaded("glClampColor"),
  }
}
static glClampColor_p: GlFnCell<glClampColor_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClampColor_is_loaded() -> bool {
  unsafe { *glClampColor_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClampColor_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClampColor_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClampColor_t>>(gl_ptr_filter(f(b"glClampColor\0".as_ptr())));
}
/// glClear
/// * `mask` group: ClearBufferMask
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClear(mask: GLbitfield) {
  #[allow(unused_unsafe)]
  match unsafe { *glClear_p.0.get() } {
    Some(glClear_inner) => glClear_inner(mask),
    None => gl_not_loaded("glClear"),
  }
}
static glClear_p: GlFnCell<glClear_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClear_is_loaded() -> bool {
  unsafe { *glClear_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClear_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClear_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClear_t>>(gl_ptr_filter(f(b"glClear\0".as_ptr())));
}
/// glClearBufferfi
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearBufferfi(buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearBufferfi_p.0.get() } {
    Some(glClearBufferfi_inner) => glClearBufferfi_inner(buffer, drawbuffer, depth, stencil),
    None => gl_not_loaded("glClearBufferfi"),
  }
}
static glClearBufferfi_p: GlFnCell<glClearBufferfi_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearBufferfi_is_loaded() -> bool {
  unsafe { *glClearBufferfi_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearBufferfi_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearBufferfi_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearBufferfi_t>>(gl_ptr_filter(f(b"glClearBufferfi\0".as_ptr())));
}
/// [glClearBufferfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBuffer.xhtml)
///
/// Clears a specified buffer of the currently bound draw framebuffer object to
/// a specified value.
///
/// * If `buffer` is `GL_COLOR`, a particular draw buffer `GL_DRAW_BUFFERi` is
///   specified by passing `i` as `drawbuffer` (eg: to affect `GL_DRAW_BUFFER0`
///   you'd pass 0). In this case, `value` points to a four-element vector
///   specifying the R, G, B, and A color to clear that draw buffer to. If the
///   value of `GL_DRAW_BUFFERi` is `GL_NONE`, the command has no effect.
///   Otherwise, the value of `GL_DRAW_BUFFERi` identifies one or more color
///   buffers, each of which is cleared to the same value. Clamping and type
///   conversion for fixed-point color buffers are performed in the same fashion
///   as for `glClearColor`.
/// * If `buffer` is `GL_DEPTH`, `drawbuffer` must be zero, and `value` points
///   to a single value to clear the depth buffer to. Clamping and type
///   conversion for fixed-point depth buffers are performed in the same fashion
///   as for `glClearDepth`.
///
/// ## Errors
/// * `GL_INVALID_ENUM` is generated if `buffer` is not `GL_COLOR` or
///   `GL_DEPTH`.
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearBufferfv(buffer: Buffer, drawbuffer: GLint, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearBufferfv_p.0.get() } {
    Some(glClearBufferfv_inner) => glClearBufferfv_inner(buffer, drawbuffer, value),
    None => gl_not_loaded("glClearBufferfv"),
  }
}
static glClearBufferfv_p: GlFnCell<glClearBufferfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearBufferfv_is_loaded() -> bool {
  unsafe { *glClearBufferfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearBufferfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearBufferfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearBufferfv_t>>(gl_ptr_filter(f(b"glClearBufferfv\0".as_ptr())));
}
/// glClearBufferiv
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
/// * `value` len: COMPSIZE(buffer)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearBufferiv(buffer: Buffer, drawbuffer: GLint, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearBufferiv_p.0.get() } {
    Some(glClearBufferiv_inner) => glClearBufferiv_inner(buffer, drawbuffer, value),
    None => gl_not_loaded("glClearBufferiv"),
  }
}
static glClearBufferiv_p: GlFnCell<glClearBufferiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearBufferiv_is_loaded() -> bool {
  unsafe { *glClearBufferiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearBufferiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearBufferiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearBufferiv_t>>(gl_ptr_filter(f(b"glClearBufferiv\0".as_ptr())));
}
/// glClearBufferuiv
/// * `buffer` group: Buffer
/// * `drawbuffer` group: DrawBufferName
/// * `value` len: COMPSIZE(buffer)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearBufferuiv(buffer: Buffer, drawbuffer: GLint, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearBufferuiv_p.0.get() } {
    Some(glClearBufferuiv_inner) => glClearBufferuiv_inner(buffer, drawbuffer, value),
    None => gl_not_loaded("glClearBufferuiv"),
  }
}
static glClearBufferuiv_p: GlFnCell<glClearBufferuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearBufferuiv_is_loaded() -> bool {
  unsafe { *glClearBufferuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearBufferuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearBufferuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearBufferuiv_t>>(gl_ptr_filter(f(b"glClearBufferuiv\0".as_ptr())));
}
/// glClearColor
/// * `red` group: ColorF
/// * `green` group: ColorF
/// * `blue` group: ColorF
/// * `alpha` group: ColorF
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearColor_p.0.get() } {
    Some(glClearColor_inner) => glClearColor_inner(red, green, blue, alpha),
    None => gl_not_loaded("glClearColor"),
  }
}
static glClearColor_p: GlFnCell<glClearColor_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearColor_is_loaded() -> bool {
  unsafe { *glClearColor_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearColor_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearColor_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearColor_t>>(gl_ptr_filter(f(b"glClearColor\0".as_ptr())));
}
/// glClearDepth
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearDepth(depth: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearDepth_p.0.get() } {
    Some(glClearDepth_inner) => glClearDepth_inner(depth),
    None => gl_not_loaded("glClearDepth"),
  }
}
static glClearDepth_p: GlFnCell<glClearDepth_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearDepth_is_loaded() -> bool {
  unsafe { *glClearDepth_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearDepth_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearDepth_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearDepth_t>>(gl_ptr_filter(f(b"glClearDepth\0".as_ptr())));
}
/// glClearStencil
/// * `s` group: StencilValue
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClearStencil(s: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glClearStencil_p.0.get() } {
    Some(glClearStencil_inner) => glClearStencil_inner(s),
    None => gl_not_loaded("glClearStencil"),
  }
}
static glClearStencil_p: GlFnCell<glClearStencil_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClearStencil_is_loaded() -> bool {
  unsafe { *glClearStencil_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClearStencil_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClearStencil_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClearStencil_t>>(gl_ptr_filter(f(b"glClearStencil\0".as_ptr())));
}
/// glClientWaitSync
/// * `sync` group: sync
/// * `sync` class: sync
/// * `flags` group: SyncObjectMask
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> SyncStatus {
  #[allow(unused_unsafe)]
  match unsafe { *glClientWaitSync_p.0.get() } {
    Some(glClientWaitSync_inner) => glClientWaitSync_inner(sync, flags, timeout),
    None => gl_not_loaded("glClientWaitSync"),
  }
}
static glClientWaitSync_p: GlFnCell<glClientWaitSync_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glClientWaitSync_is_loaded() -> bool {
  unsafe { *glClientWaitSync_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glClientWaitSync_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glClientWaitSync_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glClientWaitSync_t>>(gl_ptr_filter(f(b"glClientWaitSync\0".as_ptr())));
}
/// glColorMask
/// * `red` group: Boolean
/// * `green` group: Boolean
/// * `blue` group: Boolean
/// * `alpha` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glColorMask_p.0.get() } {
    Some(glColorMask_inner) => glColorMask_inner(red, green, blue, alpha),
    None => gl_not_loaded("glColorMask"),
  }
}
static glColorMask_p: GlFnCell<glColorMask_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glColorMask_is_loaded() -> bool {
  unsafe { *glColorMask_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glColorMask_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glColorMask_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glColorMask_t>>(gl_ptr_filter(f(b"glColorMask\0".as_ptr())));
}
/// glColorMaski
/// * `r` group: Boolean
/// * `g` group: Boolean
/// * `b` group: Boolean
/// * `a` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glColorMaski_p.0.get() } {
    Some(glColorMaski_inner) => glColorMaski_inner(index, r, g, b, a),
    None => gl_not_loaded("glColorMaski"),
  }
}
static glColorMaski_p: GlFnCell<glColorMaski_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glColorMaski_is_loaded() -> bool {
  unsafe { *glColorMaski_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glColorMaski_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glColorMaski_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glColorMaski_t>>(gl_ptr_filter(f(b"glColorMaski\0".as_ptr())));
}
/// [glCompileShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileShader.xhtml)
///
/// Compiles the source code assigned to the shader. The compilation status is
/// stored as part of the shader object's state, check it with `glGetShader` and
/// `glGetShaderInfoLog`.
///
/// * `shader` names the shader to compile.
#[cfg_attr(feature = "track_caller", track_caller)]
pub fn glCompileShader(shader: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompileShader_p.0.get() } {
    Some(glCompileShader_inner) => glCompileShader_inner(shader),
    None => gl_not_loaded("glCompileShader"),
  }
}
static glCompileShader_p: GlFnCell<glCompileShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompileShader_is_loaded() -> bool {
  unsafe { *glCompileShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompileShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompileShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompileShader_t>>(gl_ptr_filter(f(b"glCompileShader\0".as_ptr())));
}
/// glCompressedTexImage1D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTexImage1D(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTexImage1D_p.0.get() } {
    Some(glCompressedTexImage1D_inner) => glCompressedTexImage1D_inner(target, level, internalformat, width, border, imageSize, data),
    None => gl_not_loaded("glCompressedTexImage1D"),
  }
}
static glCompressedTexImage1D_p: GlFnCell<glCompressedTexImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTexImage1D_is_loaded() -> bool {
  unsafe { *glCompressedTexImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTexImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTexImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTexImage1D_t>>(gl_ptr_filter(f(b"glCompressedTexImage1D\0".as_ptr())));
}
/// glCompressedTexImage2D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTexImage2D(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTexImage2D_p.0.get() } {
    Some(glCompressedTexImage2D_inner) => glCompressedTexImage2D_inner(target, level, internalformat, width, height, border, imageSize, data),
    None => gl_not_loaded("glCompressedTexImage2D"),
  }
}
static glCompressedTexImage2D_p: GlFnCell<glCompressedTexImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTexImage2D_is_loaded() -> bool {
  unsafe { *glCompressedTexImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTexImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTexImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTexImage2D_t>>(gl_ptr_filter(f(b"glCompressedTexImage2D\0".as_ptr())));
}
/// glCompressedTexImage3D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTexImage3D(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTexImage3D_p.0.get() } {
    Some(glCompressedTexImage3D_inner) => glCompressedTexImage3D_inner(target, level, internalformat, width, height, depth, border, imageSize, data),
    None => gl_not_loaded("glCompressedTexImage3D"),
  }
}
static glCompressedTexImage3D_p: GlFnCell<glCompressedTexImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTexImage3D_is_loaded() -> bool {
  unsafe { *glCompressedTexImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTexImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTexImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTexImage3D_t>>(gl_ptr_filter(f(b"glCompressedTexImage3D\0".as_ptr())));
}
/// glCompressedTexSubImage1D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTexSubImage1D(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTexSubImage1D_p.0.get() } {
    Some(glCompressedTexSubImage1D_inner) => glCompressedTexSubImage1D_inner(target, level, xoffset, width, format, imageSize, data),
    None => gl_not_loaded("glCompressedTexSubImage1D"),
  }
}
static glCompressedTexSubImage1D_p: GlFnCell<glCompressedTexSubImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTexSubImage1D_is_loaded() -> bool {
  unsafe { *glCompressedTexSubImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTexSubImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTexSubImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTexSubImage1D_t>>(gl_ptr_filter(f(b"glCompressedTexSubImage1D\0".as_ptr())));
}
/// glCompressedTexSubImage2D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTexSubImage2D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTexSubImage2D_p.0.get() } {
    Some(glCompressedTexSubImage2D_inner) => glCompressedTexSubImage2D_inner(target, level, xoffset, yoffset, width, height, format, imageSize, data),
    None => gl_not_loaded("glCompressedTexSubImage2D"),
  }
}
static glCompressedTexSubImage2D_p: GlFnCell<glCompressedTexSubImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTexSubImage2D_is_loaded() -> bool {
  unsafe { *glCompressedTexSubImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTexSubImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTexSubImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTexSubImage2D_t>>(gl_ptr_filter(f(b"glCompressedTexSubImage2D\0".as_ptr())));
}
/// glCompressedTexSubImage3D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `data` group: CompressedTextureARB
/// * `data` len: imageSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCompressedTexSubImage3D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glCompressedTexSubImage3D_p.0.get() } {
    Some(glCompressedTexSubImage3D_inner) => glCompressedTexSubImage3D_inner(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data),
    None => gl_not_loaded("glCompressedTexSubImage3D"),
  }
}
static glCompressedTexSubImage3D_p: GlFnCell<glCompressedTexSubImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCompressedTexSubImage3D_is_loaded() -> bool {
  unsafe { *glCompressedTexSubImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCompressedTexSubImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCompressedTexSubImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCompressedTexSubImage3D_t>>(gl_ptr_filter(f(b"glCompressedTexSubImage3D\0".as_ptr())));
}
/// glCopyBufferSubData
/// * `readTarget` group: CopyBufferSubDataTarget
/// * `writeTarget` group: CopyBufferSubDataTarget
/// * `readOffset` group: BufferOffset
/// * `writeOffset` group: BufferOffset
/// * `size` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyBufferSubData(readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyBufferSubData_p.0.get() } {
    Some(glCopyBufferSubData_inner) => glCopyBufferSubData_inner(readTarget, writeTarget, readOffset, writeOffset, size),
    None => gl_not_loaded("glCopyBufferSubData"),
  }
}
static glCopyBufferSubData_p: GlFnCell<glCopyBufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyBufferSubData_is_loaded() -> bool {
  unsafe { *glCopyBufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyBufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyBufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyBufferSubData_t>>(gl_ptr_filter(f(b"glCopyBufferSubData\0".as_ptr())));
}
/// glCopyTexImage1D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTexImage1D(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTexImage1D_p.0.get() } {
    Some(glCopyTexImage1D_inner) => glCopyTexImage1D_inner(target, level, internalformat, x, y, width, border),
    None => gl_not_loaded("glCopyTexImage1D"),
  }
}
static glCopyTexImage1D_p: GlFnCell<glCopyTexImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTexImage1D_is_loaded() -> bool {
  unsafe { *glCopyTexImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTexImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTexImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTexImage1D_t>>(gl_ptr_filter(f(b"glCopyTexImage1D\0".as_ptr())));
}
/// glCopyTexImage2D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `border` group: CheckedInt32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTexImage2D(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTexImage2D_p.0.get() } {
    Some(glCopyTexImage2D_inner) => glCopyTexImage2D_inner(target, level, internalformat, x, y, width, height, border),
    None => gl_not_loaded("glCopyTexImage2D"),
  }
}
static glCopyTexImage2D_p: GlFnCell<glCopyTexImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTexImage2D_is_loaded() -> bool {
  unsafe { *glCopyTexImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTexImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTexImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTexImage2D_t>>(gl_ptr_filter(f(b"glCopyTexImage2D\0".as_ptr())));
}
/// glCopyTexSubImage1D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTexSubImage1D(target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTexSubImage1D_p.0.get() } {
    Some(glCopyTexSubImage1D_inner) => glCopyTexSubImage1D_inner(target, level, xoffset, x, y, width),
    None => gl_not_loaded("glCopyTexSubImage1D"),
  }
}
static glCopyTexSubImage1D_p: GlFnCell<glCopyTexSubImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTexSubImage1D_is_loaded() -> bool {
  unsafe { *glCopyTexSubImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTexSubImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTexSubImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTexSubImage1D_t>>(gl_ptr_filter(f(b"glCopyTexSubImage1D\0".as_ptr())));
}
/// glCopyTexSubImage2D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTexSubImage2D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTexSubImage2D_p.0.get() } {
    Some(glCopyTexSubImage2D_inner) => glCopyTexSubImage2D_inner(target, level, xoffset, yoffset, x, y, width, height),
    None => gl_not_loaded("glCopyTexSubImage2D"),
  }
}
static glCopyTexSubImage2D_p: GlFnCell<glCopyTexSubImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTexSubImage2D_is_loaded() -> bool {
  unsafe { *glCopyTexSubImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTexSubImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTexSubImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTexSubImage2D_t>>(gl_ptr_filter(f(b"glCopyTexSubImage2D\0".as_ptr())));
}
/// glCopyTexSubImage3D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `x` group: WinCoord
/// * `y` group: WinCoord
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCopyTexSubImage3D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glCopyTexSubImage3D_p.0.get() } {
    Some(glCopyTexSubImage3D_inner) => glCopyTexSubImage3D_inner(target, level, xoffset, yoffset, zoffset, x, y, width, height),
    None => gl_not_loaded("glCopyTexSubImage3D"),
  }
}
static glCopyTexSubImage3D_p: GlFnCell<glCopyTexSubImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCopyTexSubImage3D_is_loaded() -> bool {
  unsafe { *glCopyTexSubImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCopyTexSubImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCopyTexSubImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCopyTexSubImage3D_t>>(gl_ptr_filter(f(b"glCopyTexSubImage3D\0".as_ptr())));
}
/// [glCreateProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgram.xhtml)
///
/// Creates an empty program object, returning its name (a non-zero ID value).
///
/// ## Failure
/// * If this fails, 0 is returned.
#[cfg_attr(feature = "track_caller", track_caller)]
pub fn glCreateProgram() -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateProgram_p.0.get() } {
    Some(glCreateProgram_inner) => glCreateProgram_inner(),
    None => gl_not_loaded("glCreateProgram"),
  }
}
static glCreateProgram_p: GlFnCell<glCreateProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateProgram_is_loaded() -> bool {
  unsafe { *glCreateProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateProgram_t>>(gl_ptr_filter(f(b"glCreateProgram\0".as_ptr())));
}
/// [glCreateShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShader.xhtml)
///
/// Creates a new empty shader object of the given type, returning its name (a
/// non-zero ID value).
///
/// * `type` group: ShaderType
///
/// ## Failure
/// * If an error occurs the function returns 0.
#[cfg_attr(feature = "track_caller", track_caller)]
pub fn glCreateShader(type_: ShaderType) -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glCreateShader_p.0.get() } {
    Some(glCreateShader_inner) => glCreateShader_inner(type_),
    None => gl_not_loaded("glCreateShader"),
  }
}
static glCreateShader_p: GlFnCell<glCreateShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCreateShader_is_loaded() -> bool {
  unsafe { *glCreateShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCreateShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCreateShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCreateShader_t>>(gl_ptr_filter(f(b"glCreateShader\0".as_ptr())));
}
/// glCullFace
/// * `mode` group: CullFaceMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glCullFace(mode: CullFaceMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glCullFace_p.0.get() } {
    Some(glCullFace_inner) => glCullFace_inner(mode),
    None => gl_not_loaded("glCullFace"),
  }
}
static glCullFace_p: GlFnCell<glCullFace_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glCullFace_is_loaded() -> bool {
  unsafe { *glCullFace_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glCullFace_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glCullFace_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glCullFace_t>>(gl_ptr_filter(f(b"glCullFace\0".as_ptr())));
}
/// glDebugMessageCallback
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDebugMessageCallback(callback: GLDEBUGPROC, userParam: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glDebugMessageCallback_p.0.get() } {
    Some(glDebugMessageCallback_inner) => glDebugMessageCallback_inner(callback, userParam),
    None => gl_not_loaded("glDebugMessageCallback"),
  }
}
static glDebugMessageCallback_p: GlFnCell<glDebugMessageCallback_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDebugMessageCallback_is_loaded() -> bool {
  unsafe { *glDebugMessageCallback_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDebugMessageCallback_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDebugMessageCallback_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDebugMessageCallback_t>>(gl_ptr_filter(f(b"glDebugMessageCallback\0".as_ptr())));
}
/// glDebugMessageControl
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `ids` len: count
/// * `enabled` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDebugMessageControl(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glDebugMessageControl_p.0.get() } {
    Some(glDebugMessageControl_inner) => glDebugMessageControl_inner(source, type_, severity, count, ids, enabled),
    None => gl_not_loaded("glDebugMessageControl"),
  }
}
static glDebugMessageControl_p: GlFnCell<glDebugMessageControl_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDebugMessageControl_is_loaded() -> bool {
  unsafe { *glDebugMessageControl_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDebugMessageControl_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDebugMessageControl_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDebugMessageControl_t>>(gl_ptr_filter(f(b"glDebugMessageControl\0".as_ptr())));
}
/// glDebugMessageInsert
/// * `source` group: DebugSource
/// * `type` group: DebugType
/// * `severity` group: DebugSeverity
/// * `buf` len: COMPSIZE(buf,length)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDebugMessageInsert(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glDebugMessageInsert_p.0.get() } {
    Some(glDebugMessageInsert_inner) => glDebugMessageInsert_inner(source, type_, id, severity, length, buf),
    None => gl_not_loaded("glDebugMessageInsert"),
  }
}
static glDebugMessageInsert_p: GlFnCell<glDebugMessageInsert_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDebugMessageInsert_is_loaded() -> bool {
  unsafe { *glDebugMessageInsert_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDebugMessageInsert_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDebugMessageInsert_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDebugMessageInsert_t>>(gl_ptr_filter(f(b"glDebugMessageInsert\0".as_ptr())));
}
/// glDeleteBuffers
/// * `buffers` class: buffer
/// * `buffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteBuffers_p.0.get() } {
    Some(glDeleteBuffers_inner) => glDeleteBuffers_inner(n, buffers),
    None => gl_not_loaded("glDeleteBuffers"),
  }
}
static glDeleteBuffers_p: GlFnCell<glDeleteBuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteBuffers_is_loaded() -> bool {
  unsafe { *glDeleteBuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteBuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteBuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteBuffers_t>>(gl_ptr_filter(f(b"glDeleteBuffers\0".as_ptr())));
}
/// glDeleteFramebuffers
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteFramebuffers_p.0.get() } {
    Some(glDeleteFramebuffers_inner) => glDeleteFramebuffers_inner(n, framebuffers),
    None => gl_not_loaded("glDeleteFramebuffers"),
  }
}
static glDeleteFramebuffers_p: GlFnCell<glDeleteFramebuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteFramebuffers_is_loaded() -> bool {
  unsafe { *glDeleteFramebuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteFramebuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteFramebuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteFramebuffers_t>>(gl_ptr_filter(f(b"glDeleteFramebuffers\0".as_ptr())));
}
/// [glDeleteProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgram.xhtml)
///
/// Marks a program object for deletion. If the shader program is not in use it
/// will be immediately deleted, otherwise it will be deleted once it's no
/// longer in use. When a program object is deleted any shaders attached to it
/// are automatically unattached from it.
///
/// * `program` names the program to mark for deletion.
#[cfg_attr(feature = "track_caller", track_caller)]
pub fn glDeleteProgram(program: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteProgram_p.0.get() } {
    Some(glDeleteProgram_inner) => glDeleteProgram_inner(program),
    None => gl_not_loaded("glDeleteProgram"),
  }
}
static glDeleteProgram_p: GlFnCell<glDeleteProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteProgram_is_loaded() -> bool {
  unsafe { *glDeleteProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteProgram_t>>(gl_ptr_filter(f(b"glDeleteProgram\0".as_ptr())));
}
/// glDeleteQueries
/// * `ids` class: query
/// * `ids` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteQueries(n: GLsizei, ids: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteQueries_p.0.get() } {
    Some(glDeleteQueries_inner) => glDeleteQueries_inner(n, ids),
    None => gl_not_loaded("glDeleteQueries"),
  }
}
static glDeleteQueries_p: GlFnCell<glDeleteQueries_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteQueries_is_loaded() -> bool {
  unsafe { *glDeleteQueries_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteQueries_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteQueries_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteQueries_t>>(gl_ptr_filter(f(b"glDeleteQueries\0".as_ptr())));
}
/// glDeleteRenderbuffers
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteRenderbuffers_p.0.get() } {
    Some(glDeleteRenderbuffers_inner) => glDeleteRenderbuffers_inner(n, renderbuffers),
    None => gl_not_loaded("glDeleteRenderbuffers"),
  }
}
static glDeleteRenderbuffers_p: GlFnCell<glDeleteRenderbuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteRenderbuffers_is_loaded() -> bool {
  unsafe { *glDeleteRenderbuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteRenderbuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteRenderbuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteRenderbuffers_t>>(gl_ptr_filter(f(b"glDeleteRenderbuffers\0".as_ptr())));
}
/// glDeleteSamplers
/// * `samplers` class: sampler
/// * `samplers` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteSamplers(count: GLsizei, samplers: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteSamplers_p.0.get() } {
    Some(glDeleteSamplers_inner) => glDeleteSamplers_inner(count, samplers),
    None => gl_not_loaded("glDeleteSamplers"),
  }
}
static glDeleteSamplers_p: GlFnCell<glDeleteSamplers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteSamplers_is_loaded() -> bool {
  unsafe { *glDeleteSamplers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteSamplers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteSamplers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteSamplers_t>>(gl_ptr_filter(f(b"glDeleteSamplers\0".as_ptr())));
}
/// [glDeleteShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteShader.xhtml)
///
/// Marks a shader to be deleted. If it's not attached to a program it will be
/// deleted immediately, otherwise it won't be deleted until it's unattached.
///
/// * `shader` names the shader to mark for deletion.
#[cfg_attr(feature = "track_caller", track_caller)]
pub fn glDeleteShader(shader: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteShader_p.0.get() } {
    Some(glDeleteShader_inner) => glDeleteShader_inner(shader),
    None => gl_not_loaded("glDeleteShader"),
  }
}
static glDeleteShader_p: GlFnCell<glDeleteShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteShader_is_loaded() -> bool {
  unsafe { *glDeleteShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteShader_t>>(gl_ptr_filter(f(b"glDeleteShader\0".as_ptr())));
}
/// glDeleteSync
/// * `sync` group: sync
/// * `sync` class: sync
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteSync(sync: GLsync) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteSync_p.0.get() } {
    Some(glDeleteSync_inner) => glDeleteSync_inner(sync),
    None => gl_not_loaded("glDeleteSync"),
  }
}
static glDeleteSync_p: GlFnCell<glDeleteSync_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteSync_is_loaded() -> bool {
  unsafe { *glDeleteSync_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteSync_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteSync_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteSync_t>>(gl_ptr_filter(f(b"glDeleteSync\0".as_ptr())));
}
/// glDeleteTextures
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteTextures_p.0.get() } {
    Some(glDeleteTextures_inner) => glDeleteTextures_inner(n, textures),
    None => gl_not_loaded("glDeleteTextures"),
  }
}
static glDeleteTextures_p: GlFnCell<glDeleteTextures_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteTextures_is_loaded() -> bool {
  unsafe { *glDeleteTextures_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteTextures_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteTextures_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteTextures_t>>(gl_ptr_filter(f(b"glDeleteTextures\0".as_ptr())));
}
/// [glDeleteVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteVertexArrays.xhtml)
///
/// Deletes a list of vertex array objects. If a vertex array object that is
/// bound is deleted then the binding reverts to 0 and the default vertex array
/// becomes current. Passing any vertex array object IDs not currently in use,
/// or passing 0, is silently ignored.
///
/// * `n` the size of the list
/// * `arrays` the vertex array objects to delete.
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDeleteVertexArrays_p.0.get() } {
    Some(glDeleteVertexArrays_inner) => glDeleteVertexArrays_inner(n, arrays),
    None => gl_not_loaded("glDeleteVertexArrays"),
  }
}
static glDeleteVertexArrays_p: GlFnCell<glDeleteVertexArrays_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDeleteVertexArrays_is_loaded() -> bool {
  unsafe { *glDeleteVertexArrays_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDeleteVertexArrays_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDeleteVertexArrays_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDeleteVertexArrays_t>>(gl_ptr_filter(f(b"glDeleteVertexArrays\0".as_ptr())));
}
/// glDepthFunc
/// * `func` group: DepthFunction
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDepthFunc(func: DepthFunction) {
  #[allow(unused_unsafe)]
  match unsafe { *glDepthFunc_p.0.get() } {
    Some(glDepthFunc_inner) => glDepthFunc_inner(func),
    None => gl_not_loaded("glDepthFunc"),
  }
}
static glDepthFunc_p: GlFnCell<glDepthFunc_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDepthFunc_is_loaded() -> bool {
  unsafe { *glDepthFunc_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDepthFunc_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDepthFunc_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDepthFunc_t>>(gl_ptr_filter(f(b"glDepthFunc\0".as_ptr())));
}
/// glDepthMask
/// * `flag` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDepthMask(flag: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glDepthMask_p.0.get() } {
    Some(glDepthMask_inner) => glDepthMask_inner(flag),
    None => gl_not_loaded("glDepthMask"),
  }
}
static glDepthMask_p: GlFnCell<glDepthMask_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDepthMask_is_loaded() -> bool {
  unsafe { *glDepthMask_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDepthMask_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDepthMask_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDepthMask_t>>(gl_ptr_filter(f(b"glDepthMask\0".as_ptr())));
}
/// glDepthRange
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDepthRange(n: GLdouble, f: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glDepthRange_p.0.get() } {
    Some(glDepthRange_inner) => glDepthRange_inner(n, f),
    None => gl_not_loaded("glDepthRange"),
  }
}
static glDepthRange_p: GlFnCell<glDepthRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDepthRange_is_loaded() -> bool {
  unsafe { *glDepthRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDepthRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDepthRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDepthRange_t>>(gl_ptr_filter(f(b"glDepthRange\0".as_ptr())));
}
/// glDetachShader
/// * `program` class: program
/// * `shader` class: shader
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDetachShader(program: GLuint, shader: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDetachShader_p.0.get() } {
    Some(glDetachShader_inner) => glDetachShader_inner(program, shader),
    None => gl_not_loaded("glDetachShader"),
  }
}
static glDetachShader_p: GlFnCell<glDetachShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDetachShader_is_loaded() -> bool {
  unsafe { *glDetachShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDetachShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDetachShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDetachShader_t>>(gl_ptr_filter(f(b"glDetachShader\0".as_ptr())));
}
/// glDisable
/// * `cap` group: EnableCap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDisable(cap: EnableCap) {
  #[allow(unused_unsafe)]
  match unsafe { *glDisable_p.0.get() } {
    Some(glDisable_inner) => glDisable_inner(cap),
    None => gl_not_loaded("glDisable"),
  }
}
static glDisable_p: GlFnCell<glDisable_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDisable_is_loaded() -> bool {
  unsafe { *glDisable_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDisable_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDisable_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDisable_t>>(gl_ptr_filter(f(b"glDisable\0".as_ptr())));
}
/// glDisableVertexAttribArray
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDisableVertexAttribArray(index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDisableVertexAttribArray_p.0.get() } {
    Some(glDisableVertexAttribArray_inner) => glDisableVertexAttribArray_inner(index),
    None => gl_not_loaded("glDisableVertexAttribArray"),
  }
}
static glDisableVertexAttribArray_p: GlFnCell<glDisableVertexAttribArray_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDisableVertexAttribArray_is_loaded() -> bool {
  unsafe { *glDisableVertexAttribArray_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDisableVertexAttribArray_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDisableVertexAttribArray_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDisableVertexAttribArray_t>>(gl_ptr_filter(f(b"glDisableVertexAttribArray\0".as_ptr())));
}
/// glDisablei
/// * `target` group: EnableCap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDisablei(target: EnableCap, index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDisablei_p.0.get() } {
    Some(glDisablei_inner) => glDisablei_inner(target, index),
    None => gl_not_loaded("glDisablei"),
  }
}
static glDisablei_p: GlFnCell<glDisablei_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDisablei_is_loaded() -> bool {
  unsafe { *glDisablei_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDisablei_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDisablei_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDisablei_t>>(gl_ptr_filter(f(b"glDisablei\0".as_ptr())));
}
/// [glDrawArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArrays.xhtml)
///
/// Draws `count` sequential indices using each enabled array to construct a
/// sequence of primitives, starting from `first`. Depending on the `mode`
/// specified, the number of indices required per primitive can vary.
///
/// If vertex attributes are modified by `glDrawArrays` the values are
/// unspecified after the call returns. Any other attributes remain well
/// defined.
///
/// * `mode` is the type of primitive to render.
/// * `first` is the starting index to use within the enabled arrays.
/// * `count` is the number of **indices** to be rendered.
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawArrays(mode: PrimitiveType, first: GLint, count: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawArrays_p.0.get() } {
    Some(glDrawArrays_inner) => glDrawArrays_inner(mode, first, count),
    None => gl_not_loaded("glDrawArrays"),
  }
}
static glDrawArrays_p: GlFnCell<glDrawArrays_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawArrays_is_loaded() -> bool {
  unsafe { *glDrawArrays_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawArrays_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawArrays_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawArrays_t>>(gl_ptr_filter(f(b"glDrawArrays\0".as_ptr())));
}
/// glDrawArraysInstanced
/// * `mode` group: PrimitiveType
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawArraysInstanced(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawArraysInstanced_p.0.get() } {
    Some(glDrawArraysInstanced_inner) => glDrawArraysInstanced_inner(mode, first, count, instancecount),
    None => gl_not_loaded("glDrawArraysInstanced"),
  }
}
static glDrawArraysInstanced_p: GlFnCell<glDrawArraysInstanced_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawArraysInstanced_is_loaded() -> bool {
  unsafe { *glDrawArraysInstanced_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawArraysInstanced_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawArraysInstanced_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawArraysInstanced_t>>(gl_ptr_filter(f(b"glDrawArraysInstanced\0".as_ptr())));
}
/// glDrawBuffer
/// * `buf` group: DrawBufferMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawBuffer(buf: DrawBufferMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawBuffer_p.0.get() } {
    Some(glDrawBuffer_inner) => glDrawBuffer_inner(buf),
    None => gl_not_loaded("glDrawBuffer"),
  }
}
static glDrawBuffer_p: GlFnCell<glDrawBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawBuffer_is_loaded() -> bool {
  unsafe { *glDrawBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawBuffer_t>>(gl_ptr_filter(f(b"glDrawBuffer\0".as_ptr())));
}
/// glDrawBuffers
/// * `bufs` group: DrawBufferMode
/// * `bufs` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawBuffers(n: GLsizei, bufs: *const DrawBufferMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawBuffers_p.0.get() } {
    Some(glDrawBuffers_inner) => glDrawBuffers_inner(n, bufs),
    None => gl_not_loaded("glDrawBuffers"),
  }
}
static glDrawBuffers_p: GlFnCell<glDrawBuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawBuffers_is_loaded() -> bool {
  unsafe { *glDrawBuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawBuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawBuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawBuffers_t>>(gl_ptr_filter(f(b"glDrawBuffers\0".as_ptr())));
}
/// glDrawElements
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawElements(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawElements_p.0.get() } {
    Some(glDrawElements_inner) => glDrawElements_inner(mode, count, type_, indices),
    None => gl_not_loaded("glDrawElements"),
  }
}
static glDrawElements_p: GlFnCell<glDrawElements_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawElements_is_loaded() -> bool {
  unsafe { *glDrawElements_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawElements_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawElements_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawElements_t>>(gl_ptr_filter(f(b"glDrawElements\0".as_ptr())));
}
/// glDrawElementsBaseVertex
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawElementsBaseVertex(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawElementsBaseVertex_p.0.get() } {
    Some(glDrawElementsBaseVertex_inner) => glDrawElementsBaseVertex_inner(mode, count, type_, indices, basevertex),
    None => gl_not_loaded("glDrawElementsBaseVertex"),
  }
}
static glDrawElementsBaseVertex_p: GlFnCell<glDrawElementsBaseVertex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawElementsBaseVertex_is_loaded() -> bool {
  unsafe { *glDrawElementsBaseVertex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawElementsBaseVertex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawElementsBaseVertex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawElementsBaseVertex_t>>(gl_ptr_filter(f(b"glDrawElementsBaseVertex\0".as_ptr())));
}
/// glDrawElementsInstanced
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawElementsInstanced(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawElementsInstanced_p.0.get() } {
    Some(glDrawElementsInstanced_inner) => glDrawElementsInstanced_inner(mode, count, type_, indices, instancecount),
    None => gl_not_loaded("glDrawElementsInstanced"),
  }
}
static glDrawElementsInstanced_p: GlFnCell<glDrawElementsInstanced_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawElementsInstanced_is_loaded() -> bool {
  unsafe { *glDrawElementsInstanced_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawElementsInstanced_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawElementsInstanced_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawElementsInstanced_t>>(gl_ptr_filter(f(b"glDrawElementsInstanced\0".as_ptr())));
}
/// glDrawElementsInstancedBaseVertex
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawElementsInstancedBaseVertex(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawElementsInstancedBaseVertex_p.0.get() } {
    Some(glDrawElementsInstancedBaseVertex_inner) => glDrawElementsInstancedBaseVertex_inner(mode, count, type_, indices, instancecount, basevertex),
    None => gl_not_loaded("glDrawElementsInstancedBaseVertex"),
  }
}
static glDrawElementsInstancedBaseVertex_p: GlFnCell<glDrawElementsInstancedBaseVertex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawElementsInstancedBaseVertex_is_loaded() -> bool {
  unsafe { *glDrawElementsInstancedBaseVertex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawElementsInstancedBaseVertex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawElementsInstancedBaseVertex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawElementsInstancedBaseVertex_t>>(gl_ptr_filter(f(b"glDrawElementsInstancedBaseVertex\0".as_ptr())));
}
/// glDrawRangeElements
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawRangeElements(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawRangeElements_p.0.get() } {
    Some(glDrawRangeElements_inner) => glDrawRangeElements_inner(mode, start, end, count, type_, indices),
    None => gl_not_loaded("glDrawRangeElements"),
  }
}
static glDrawRangeElements_p: GlFnCell<glDrawRangeElements_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawRangeElements_is_loaded() -> bool {
  unsafe { *glDrawRangeElements_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawRangeElements_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawRangeElements_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawRangeElements_t>>(gl_ptr_filter(f(b"glDrawRangeElements\0".as_ptr())));
}
/// glDrawRangeElementsBaseVertex
/// * `mode` group: PrimitiveType
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(count,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glDrawRangeElementsBaseVertex(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glDrawRangeElementsBaseVertex_p.0.get() } {
    Some(glDrawRangeElementsBaseVertex_inner) => glDrawRangeElementsBaseVertex_inner(mode, start, end, count, type_, indices, basevertex),
    None => gl_not_loaded("glDrawRangeElementsBaseVertex"),
  }
}
static glDrawRangeElementsBaseVertex_p: GlFnCell<glDrawRangeElementsBaseVertex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glDrawRangeElementsBaseVertex_is_loaded() -> bool {
  unsafe { *glDrawRangeElementsBaseVertex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glDrawRangeElementsBaseVertex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glDrawRangeElementsBaseVertex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glDrawRangeElementsBaseVertex_t>>(gl_ptr_filter(f(b"glDrawRangeElementsBaseVertex\0".as_ptr())));
}
/// glEnable
/// * `cap` group: EnableCap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEnable(cap: EnableCap) {
  #[allow(unused_unsafe)]
  match unsafe { *glEnable_p.0.get() } {
    Some(glEnable_inner) => glEnable_inner(cap),
    None => gl_not_loaded("glEnable"),
  }
}
static glEnable_p: GlFnCell<glEnable_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEnable_is_loaded() -> bool {
  unsafe { *glEnable_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEnable_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEnable_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEnable_t>>(gl_ptr_filter(f(b"glEnable\0".as_ptr())));
}
/// glEnableVertexAttribArray
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEnableVertexAttribArray(index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glEnableVertexAttribArray_p.0.get() } {
    Some(glEnableVertexAttribArray_inner) => glEnableVertexAttribArray_inner(index),
    None => gl_not_loaded("glEnableVertexAttribArray"),
  }
}
static glEnableVertexAttribArray_p: GlFnCell<glEnableVertexAttribArray_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEnableVertexAttribArray_is_loaded() -> bool {
  unsafe { *glEnableVertexAttribArray_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEnableVertexAttribArray_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEnableVertexAttribArray_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEnableVertexAttribArray_t>>(gl_ptr_filter(f(b"glEnableVertexAttribArray\0".as_ptr())));
}
/// glEnablei
/// * `target` group: EnableCap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEnablei(target: EnableCap, index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glEnablei_p.0.get() } {
    Some(glEnablei_inner) => glEnablei_inner(target, index),
    None => gl_not_loaded("glEnablei"),
  }
}
static glEnablei_p: GlFnCell<glEnablei_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEnablei_is_loaded() -> bool {
  unsafe { *glEnablei_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEnablei_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEnablei_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEnablei_t>>(gl_ptr_filter(f(b"glEnablei\0".as_ptr())));
}
/// glEndConditionalRender
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEndConditionalRender() {
  #[allow(unused_unsafe)]
  match unsafe { *glEndConditionalRender_p.0.get() } {
    Some(glEndConditionalRender_inner) => glEndConditionalRender_inner(),
    None => gl_not_loaded("glEndConditionalRender"),
  }
}
static glEndConditionalRender_p: GlFnCell<glEndConditionalRender_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEndConditionalRender_is_loaded() -> bool {
  unsafe { *glEndConditionalRender_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEndConditionalRender_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEndConditionalRender_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEndConditionalRender_t>>(gl_ptr_filter(f(b"glEndConditionalRender\0".as_ptr())));
}
/// glEndQuery
/// * `target` group: QueryTarget
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEndQuery(target: QueryTarget) {
  #[allow(unused_unsafe)]
  match unsafe { *glEndQuery_p.0.get() } {
    Some(glEndQuery_inner) => glEndQuery_inner(target),
    None => gl_not_loaded("glEndQuery"),
  }
}
static glEndQuery_p: GlFnCell<glEndQuery_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEndQuery_is_loaded() -> bool {
  unsafe { *glEndQuery_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEndQuery_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEndQuery_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEndQuery_t>>(gl_ptr_filter(f(b"glEndQuery\0".as_ptr())));
}
/// glEndTransformFeedback
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glEndTransformFeedback() {
  #[allow(unused_unsafe)]
  match unsafe { *glEndTransformFeedback_p.0.get() } {
    Some(glEndTransformFeedback_inner) => glEndTransformFeedback_inner(),
    None => gl_not_loaded("glEndTransformFeedback"),
  }
}
static glEndTransformFeedback_p: GlFnCell<glEndTransformFeedback_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glEndTransformFeedback_is_loaded() -> bool {
  unsafe { *glEndTransformFeedback_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glEndTransformFeedback_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glEndTransformFeedback_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glEndTransformFeedback_t>>(gl_ptr_filter(f(b"glEndTransformFeedback\0".as_ptr())));
}
/// glFenceSync
/// * `condition` group: SyncCondition
/// * `flags` group: SyncBehaviorFlags
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFenceSync(condition: SyncCondition, flags: GLbitfield) -> GLsync {
  #[allow(unused_unsafe)]
  match unsafe { *glFenceSync_p.0.get() } {
    Some(glFenceSync_inner) => glFenceSync_inner(condition, flags),
    None => gl_not_loaded("glFenceSync"),
  }
}
static glFenceSync_p: GlFnCell<glFenceSync_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFenceSync_is_loaded() -> bool {
  unsafe { *glFenceSync_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFenceSync_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFenceSync_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFenceSync_t>>(gl_ptr_filter(f(b"glFenceSync\0".as_ptr())));
}
/// glFinish
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFinish() {
  #[allow(unused_unsafe)]
  match unsafe { *glFinish_p.0.get() } {
    Some(glFinish_inner) => glFinish_inner(),
    None => gl_not_loaded("glFinish"),
  }
}
static glFinish_p: GlFnCell<glFinish_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFinish_is_loaded() -> bool {
  unsafe { *glFinish_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFinish_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFinish_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFinish_t>>(gl_ptr_filter(f(b"glFinish\0".as_ptr())));
}
/// glFlush
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFlush() {
  #[allow(unused_unsafe)]
  match unsafe { *glFlush_p.0.get() } {
    Some(glFlush_inner) => glFlush_inner(),
    None => gl_not_loaded("glFlush"),
  }
}
static glFlush_p: GlFnCell<glFlush_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFlush_is_loaded() -> bool {
  unsafe { *glFlush_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFlush_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFlush_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFlush_t>>(gl_ptr_filter(f(b"glFlush\0".as_ptr())));
}
/// glFlushMappedBufferRange
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `length` group: BufferSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFlushMappedBufferRange(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr) {
  #[allow(unused_unsafe)]
  match unsafe { *glFlushMappedBufferRange_p.0.get() } {
    Some(glFlushMappedBufferRange_inner) => glFlushMappedBufferRange_inner(target, offset, length),
    None => gl_not_loaded("glFlushMappedBufferRange"),
  }
}
static glFlushMappedBufferRange_p: GlFnCell<glFlushMappedBufferRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFlushMappedBufferRange_is_loaded() -> bool {
  unsafe { *glFlushMappedBufferRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFlushMappedBufferRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFlushMappedBufferRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFlushMappedBufferRange_t>>(gl_ptr_filter(f(b"glFlushMappedBufferRange\0".as_ptr())));
}
/// glFramebufferRenderbuffer
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `renderbuffertarget` group: RenderbufferTarget
/// * `renderbuffer` class: renderbuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferRenderbuffer(target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferRenderbuffer_p.0.get() } {
    Some(glFramebufferRenderbuffer_inner) => glFramebufferRenderbuffer_inner(target, attachment, renderbuffertarget, renderbuffer),
    None => gl_not_loaded("glFramebufferRenderbuffer"),
  }
}
static glFramebufferRenderbuffer_p: GlFnCell<glFramebufferRenderbuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferRenderbuffer_is_loaded() -> bool {
  unsafe { *glFramebufferRenderbuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferRenderbuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferRenderbuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferRenderbuffer_t>>(gl_ptr_filter(f(b"glFramebufferRenderbuffer\0".as_ptr())));
}
/// glFramebufferTexture
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferTexture(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferTexture_p.0.get() } {
    Some(glFramebufferTexture_inner) => glFramebufferTexture_inner(target, attachment, texture, level),
    None => gl_not_loaded("glFramebufferTexture"),
  }
}
static glFramebufferTexture_p: GlFnCell<glFramebufferTexture_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferTexture_is_loaded() -> bool {
  unsafe { *glFramebufferTexture_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferTexture_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferTexture_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferTexture_t>>(gl_ptr_filter(f(b"glFramebufferTexture\0".as_ptr())));
}
/// glFramebufferTexture1D
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferTexture1D(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferTexture1D_p.0.get() } {
    Some(glFramebufferTexture1D_inner) => glFramebufferTexture1D_inner(target, attachment, textarget, texture, level),
    None => gl_not_loaded("glFramebufferTexture1D"),
  }
}
static glFramebufferTexture1D_p: GlFnCell<glFramebufferTexture1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferTexture1D_is_loaded() -> bool {
  unsafe { *glFramebufferTexture1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferTexture1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferTexture1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferTexture1D_t>>(gl_ptr_filter(f(b"glFramebufferTexture1D\0".as_ptr())));
}
/// glFramebufferTexture2D
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferTexture2D(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferTexture2D_p.0.get() } {
    Some(glFramebufferTexture2D_inner) => glFramebufferTexture2D_inner(target, attachment, textarget, texture, level),
    None => gl_not_loaded("glFramebufferTexture2D"),
  }
}
static glFramebufferTexture2D_p: GlFnCell<glFramebufferTexture2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferTexture2D_is_loaded() -> bool {
  unsafe { *glFramebufferTexture2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferTexture2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferTexture2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferTexture2D_t>>(gl_ptr_filter(f(b"glFramebufferTexture2D\0".as_ptr())));
}
/// glFramebufferTexture3D
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `textarget` group: TextureTarget
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferTexture3D(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferTexture3D_p.0.get() } {
    Some(glFramebufferTexture3D_inner) => glFramebufferTexture3D_inner(target, attachment, textarget, texture, level, zoffset),
    None => gl_not_loaded("glFramebufferTexture3D"),
  }
}
static glFramebufferTexture3D_p: GlFnCell<glFramebufferTexture3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferTexture3D_is_loaded() -> bool {
  unsafe { *glFramebufferTexture3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferTexture3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferTexture3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferTexture3D_t>>(gl_ptr_filter(f(b"glFramebufferTexture3D\0".as_ptr())));
}
/// glFramebufferTextureLayer
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `texture` group: Texture
/// * `texture` class: texture
/// * `level` group: CheckedInt32
/// * `layer` group: CheckedInt32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFramebufferTextureLayer(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glFramebufferTextureLayer_p.0.get() } {
    Some(glFramebufferTextureLayer_inner) => glFramebufferTextureLayer_inner(target, attachment, texture, level, layer),
    None => gl_not_loaded("glFramebufferTextureLayer"),
  }
}
static glFramebufferTextureLayer_p: GlFnCell<glFramebufferTextureLayer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFramebufferTextureLayer_is_loaded() -> bool {
  unsafe { *glFramebufferTextureLayer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFramebufferTextureLayer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFramebufferTextureLayer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFramebufferTextureLayer_t>>(gl_ptr_filter(f(b"glFramebufferTextureLayer\0".as_ptr())));
}
/// glFrontFace
/// * `mode` group: FrontFaceDirection
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glFrontFace(mode: FrontFaceDirection) {
  #[allow(unused_unsafe)]
  match unsafe { *glFrontFace_p.0.get() } {
    Some(glFrontFace_inner) => glFrontFace_inner(mode),
    None => gl_not_loaded("glFrontFace"),
  }
}
static glFrontFace_p: GlFnCell<glFrontFace_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glFrontFace_is_loaded() -> bool {
  unsafe { *glFrontFace_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glFrontFace_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glFrontFace_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glFrontFace_t>>(gl_ptr_filter(f(b"glFrontFace\0".as_ptr())));
}
/// glGenBuffers
/// * `buffers` class: buffer
/// * `buffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenBuffers_p.0.get() } {
    Some(glGenBuffers_inner) => glGenBuffers_inner(n, buffers),
    None => gl_not_loaded("glGenBuffers"),
  }
}
static glGenBuffers_p: GlFnCell<glGenBuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenBuffers_is_loaded() -> bool {
  unsafe { *glGenBuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenBuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenBuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenBuffers_t>>(gl_ptr_filter(f(b"glGenBuffers\0".as_ptr())));
}
/// glGenFramebuffers
/// * `framebuffers` class: framebuffer
/// * `framebuffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenFramebuffers_p.0.get() } {
    Some(glGenFramebuffers_inner) => glGenFramebuffers_inner(n, framebuffers),
    None => gl_not_loaded("glGenFramebuffers"),
  }
}
static glGenFramebuffers_p: GlFnCell<glGenFramebuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenFramebuffers_is_loaded() -> bool {
  unsafe { *glGenFramebuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenFramebuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenFramebuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenFramebuffers_t>>(gl_ptr_filter(f(b"glGenFramebuffers\0".as_ptr())));
}
/// glGenQueries
/// * `ids` class: query
/// * `ids` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenQueries(n: GLsizei, ids: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenQueries_p.0.get() } {
    Some(glGenQueries_inner) => glGenQueries_inner(n, ids),
    None => gl_not_loaded("glGenQueries"),
  }
}
static glGenQueries_p: GlFnCell<glGenQueries_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenQueries_is_loaded() -> bool {
  unsafe { *glGenQueries_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenQueries_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenQueries_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenQueries_t>>(gl_ptr_filter(f(b"glGenQueries\0".as_ptr())));
}
/// glGenRenderbuffers
/// * `renderbuffers` class: renderbuffer
/// * `renderbuffers` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenRenderbuffers_p.0.get() } {
    Some(glGenRenderbuffers_inner) => glGenRenderbuffers_inner(n, renderbuffers),
    None => gl_not_loaded("glGenRenderbuffers"),
  }
}
static glGenRenderbuffers_p: GlFnCell<glGenRenderbuffers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenRenderbuffers_is_loaded() -> bool {
  unsafe { *glGenRenderbuffers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenRenderbuffers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenRenderbuffers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenRenderbuffers_t>>(gl_ptr_filter(f(b"glGenRenderbuffers\0".as_ptr())));
}
/// glGenSamplers
/// * `samplers` class: sampler
/// * `samplers` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenSamplers(count: GLsizei, samplers: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenSamplers_p.0.get() } {
    Some(glGenSamplers_inner) => glGenSamplers_inner(count, samplers),
    None => gl_not_loaded("glGenSamplers"),
  }
}
static glGenSamplers_p: GlFnCell<glGenSamplers_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenSamplers_is_loaded() -> bool {
  unsafe { *glGenSamplers_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenSamplers_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenSamplers_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenSamplers_t>>(gl_ptr_filter(f(b"glGenSamplers\0".as_ptr())));
}
/// glGenTextures
/// * `textures` group: Texture
/// * `textures` class: texture
/// * `textures` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenTextures(n: GLsizei, textures: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenTextures_p.0.get() } {
    Some(glGenTextures_inner) => glGenTextures_inner(n, textures),
    None => gl_not_loaded("glGenTextures"),
  }
}
static glGenTextures_p: GlFnCell<glGenTextures_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenTextures_is_loaded() -> bool {
  unsafe { *glGenTextures_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenTextures_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenTextures_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenTextures_t>>(gl_ptr_filter(f(b"glGenTextures\0".as_ptr())));
}
/// glGenVertexArrays
/// * `arrays` class: vertex array
/// * `arrays` len: n
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenVertexArrays_p.0.get() } {
    Some(glGenVertexArrays_inner) => glGenVertexArrays_inner(n, arrays),
    None => gl_not_loaded("glGenVertexArrays"),
  }
}
static glGenVertexArrays_p: GlFnCell<glGenVertexArrays_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenVertexArrays_is_loaded() -> bool {
  unsafe { *glGenVertexArrays_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenVertexArrays_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenVertexArrays_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenVertexArrays_t>>(gl_ptr_filter(f(b"glGenVertexArrays\0".as_ptr())));
}
/// glGenerateMipmap
/// * `target` group: TextureTarget
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGenerateMipmap(target: TextureTarget) {
  #[allow(unused_unsafe)]
  match unsafe { *glGenerateMipmap_p.0.get() } {
    Some(glGenerateMipmap_inner) => glGenerateMipmap_inner(target),
    None => gl_not_loaded("glGenerateMipmap"),
  }
}
static glGenerateMipmap_p: GlFnCell<glGenerateMipmap_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGenerateMipmap_is_loaded() -> bool {
  unsafe { *glGenerateMipmap_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGenerateMipmap_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGenerateMipmap_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGenerateMipmap_t>>(gl_ptr_filter(f(b"glGenerateMipmap\0".as_ptr())));
}
/// glGetActiveAttrib
/// * `program` class: program
/// * `type` group: AttributeType
/// * `name` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut AttributeType, name: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveAttrib_p.0.get() } {
    Some(glGetActiveAttrib_inner) => glGetActiveAttrib_inner(program, index, bufSize, length, size, type_, name),
    None => gl_not_loaded("glGetActiveAttrib"),
  }
}
static glGetActiveAttrib_p: GlFnCell<glGetActiveAttrib_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveAttrib_is_loaded() -> bool {
  unsafe { *glGetActiveAttrib_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveAttrib_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveAttrib_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveAttrib_t>>(gl_ptr_filter(f(b"glGetActiveAttrib\0".as_ptr())));
}
/// glGetActiveUniform
/// * `program` class: program
/// * `type` group: UniformType
/// * `name` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut UniformType, name: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveUniform_p.0.get() } {
    Some(glGetActiveUniform_inner) => glGetActiveUniform_inner(program, index, bufSize, length, size, type_, name),
    None => gl_not_loaded("glGetActiveUniform"),
  }
}
static glGetActiveUniform_p: GlFnCell<glGetActiveUniform_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveUniform_is_loaded() -> bool {
  unsafe { *glGetActiveUniform_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniform_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveUniform_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveUniform_t>>(gl_ptr_filter(f(b"glGetActiveUniform\0".as_ptr())));
}
/// glGetActiveUniformBlockName
/// * `program` class: program
/// * `uniformBlockName` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveUniformBlockName_p.0.get() } {
    Some(glGetActiveUniformBlockName_inner) => glGetActiveUniformBlockName_inner(program, uniformBlockIndex, bufSize, length, uniformBlockName),
    None => gl_not_loaded("glGetActiveUniformBlockName"),
  }
}
static glGetActiveUniformBlockName_p: GlFnCell<glGetActiveUniformBlockName_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveUniformBlockName_is_loaded() -> bool {
  unsafe { *glGetActiveUniformBlockName_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniformBlockName_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveUniformBlockName_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveUniformBlockName_t>>(gl_ptr_filter(f(b"glGetActiveUniformBlockName\0".as_ptr())));
}
/// glGetActiveUniformBlockiv
/// * `program` class: program
/// * `pname` group: UniformBlockPName
/// * `params` len: COMPSIZE(program,uniformBlockIndex,pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: UniformBlockPName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveUniformBlockiv_p.0.get() } {
    Some(glGetActiveUniformBlockiv_inner) => glGetActiveUniformBlockiv_inner(program, uniformBlockIndex, pname, params),
    None => gl_not_loaded("glGetActiveUniformBlockiv"),
  }
}
static glGetActiveUniformBlockiv_p: GlFnCell<glGetActiveUniformBlockiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveUniformBlockiv_is_loaded() -> bool {
  unsafe { *glGetActiveUniformBlockiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniformBlockiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveUniformBlockiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveUniformBlockiv_t>>(gl_ptr_filter(f(b"glGetActiveUniformBlockiv\0".as_ptr())));
}
/// glGetActiveUniformName
/// * `program` class: program
/// * `uniformName` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveUniformName_p.0.get() } {
    Some(glGetActiveUniformName_inner) => glGetActiveUniformName_inner(program, uniformIndex, bufSize, length, uniformName),
    None => gl_not_loaded("glGetActiveUniformName"),
  }
}
static glGetActiveUniformName_p: GlFnCell<glGetActiveUniformName_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveUniformName_is_loaded() -> bool {
  unsafe { *glGetActiveUniformName_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniformName_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveUniformName_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveUniformName_t>>(gl_ptr_filter(f(b"glGetActiveUniformName\0".as_ptr())));
}
/// glGetActiveUniformsiv
/// * `program` class: program
/// * `uniformIndices` len: uniformCount
/// * `pname` group: UniformPName
/// * `params` len: COMPSIZE(uniformCount,pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: UniformPName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetActiveUniformsiv_p.0.get() } {
    Some(glGetActiveUniformsiv_inner) => glGetActiveUniformsiv_inner(program, uniformCount, uniformIndices, pname, params),
    None => gl_not_loaded("glGetActiveUniformsiv"),
  }
}
static glGetActiveUniformsiv_p: GlFnCell<glGetActiveUniformsiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetActiveUniformsiv_is_loaded() -> bool {
  unsafe { *glGetActiveUniformsiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetActiveUniformsiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetActiveUniformsiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetActiveUniformsiv_t>>(gl_ptr_filter(f(b"glGetActiveUniformsiv\0".as_ptr())));
}
/// glGetAttachedShaders
/// * `program` class: program
/// * `shaders` class: shader
/// * `shaders` len: maxCount
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetAttachedShaders_p.0.get() } {
    Some(glGetAttachedShaders_inner) => glGetAttachedShaders_inner(program, maxCount, count, shaders),
    None => gl_not_loaded("glGetAttachedShaders"),
  }
}
static glGetAttachedShaders_p: GlFnCell<glGetAttachedShaders_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetAttachedShaders_is_loaded() -> bool {
  unsafe { *glGetAttachedShaders_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetAttachedShaders_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetAttachedShaders_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetAttachedShaders_t>>(gl_ptr_filter(f(b"glGetAttachedShaders\0".as_ptr())));
}
/// glGetAttribLocation
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetAttribLocation_p.0.get() } {
    Some(glGetAttribLocation_inner) => glGetAttribLocation_inner(program, name),
    None => gl_not_loaded("glGetAttribLocation"),
  }
}
static glGetAttribLocation_p: GlFnCell<glGetAttribLocation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetAttribLocation_is_loaded() -> bool {
  unsafe { *glGetAttribLocation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetAttribLocation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetAttribLocation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetAttribLocation_t>>(gl_ptr_filter(f(b"glGetAttribLocation\0".as_ptr())));
}
/// glGetBooleani_v
/// * `target` group: BufferTargetARB
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(target)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetBooleani_v(target: BufferTargetARB, index: GLuint, data: *mut GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetBooleani_v_p.0.get() } {
    Some(glGetBooleani_v_inner) => glGetBooleani_v_inner(target, index, data),
    None => gl_not_loaded("glGetBooleani_v"),
  }
}
static glGetBooleani_v_p: GlFnCell<glGetBooleani_v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetBooleani_v_is_loaded() -> bool {
  unsafe { *glGetBooleani_v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetBooleani_v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetBooleani_v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetBooleani_v_t>>(gl_ptr_filter(f(b"glGetBooleani_v\0".as_ptr())));
}
/// glGetBooleanv
/// * `pname` group: GetPName
/// * `data` group: Boolean
/// * `data` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetBooleanv(pname: GetPName, data: *mut GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetBooleanv_p.0.get() } {
    Some(glGetBooleanv_inner) => glGetBooleanv_inner(pname, data),
    None => gl_not_loaded("glGetBooleanv"),
  }
}
static glGetBooleanv_p: GlFnCell<glGetBooleanv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetBooleanv_is_loaded() -> bool {
  unsafe { *glGetBooleanv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetBooleanv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetBooleanv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetBooleanv_t>>(gl_ptr_filter(f(b"glGetBooleanv\0".as_ptr())));
}
/// glGetBufferParameteri64v
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetBufferParameteri64v(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetBufferParameteri64v_p.0.get() } {
    Some(glGetBufferParameteri64v_inner) => glGetBufferParameteri64v_inner(target, pname, params),
    None => gl_not_loaded("glGetBufferParameteri64v"),
  }
}
static glGetBufferParameteri64v_p: GlFnCell<glGetBufferParameteri64v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetBufferParameteri64v_is_loaded() -> bool {
  unsafe { *glGetBufferParameteri64v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetBufferParameteri64v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetBufferParameteri64v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetBufferParameteri64v_t>>(gl_ptr_filter(f(b"glGetBufferParameteri64v\0".as_ptr())));
}
/// glGetBufferParameteriv
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPNameARB
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetBufferParameteriv(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetBufferParameteriv_p.0.get() } {
    Some(glGetBufferParameteriv_inner) => glGetBufferParameteriv_inner(target, pname, params),
    None => gl_not_loaded("glGetBufferParameteriv"),
  }
}
static glGetBufferParameteriv_p: GlFnCell<glGetBufferParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetBufferParameteriv_is_loaded() -> bool {
  unsafe { *glGetBufferParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetBufferParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetBufferParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetBufferParameteriv_t>>(gl_ptr_filter(f(b"glGetBufferParameteriv\0".as_ptr())));
}
/// glGetBufferPointerv
/// * `target` group: BufferTargetARB
/// * `pname` group: BufferPointerNameARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetBufferPointerv(target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetBufferPointerv_p.0.get() } {
    Some(glGetBufferPointerv_inner) => glGetBufferPointerv_inner(target, pname, params),
    None => gl_not_loaded("glGetBufferPointerv"),
  }
}
static glGetBufferPointerv_p: GlFnCell<glGetBufferPointerv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetBufferPointerv_is_loaded() -> bool {
  unsafe { *glGetBufferPointerv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetBufferPointerv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetBufferPointerv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetBufferPointerv_t>>(gl_ptr_filter(f(b"glGetBufferPointerv\0".as_ptr())));
}
/// glGetBufferSubData
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `size` group: BufferSize
/// * `data` len: size
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetBufferSubData(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetBufferSubData_p.0.get() } {
    Some(glGetBufferSubData_inner) => glGetBufferSubData_inner(target, offset, size, data),
    None => gl_not_loaded("glGetBufferSubData"),
  }
}
static glGetBufferSubData_p: GlFnCell<glGetBufferSubData_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetBufferSubData_is_loaded() -> bool {
  unsafe { *glGetBufferSubData_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetBufferSubData_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetBufferSubData_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetBufferSubData_t>>(gl_ptr_filter(f(b"glGetBufferSubData\0".as_ptr())));
}
/// glGetCompressedTexImage
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `img` group: CompressedTextureARB
/// * `img` len: COMPSIZE(target,level)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetCompressedTexImage(target: TextureTarget, level: GLint, img: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetCompressedTexImage_p.0.get() } {
    Some(glGetCompressedTexImage_inner) => glGetCompressedTexImage_inner(target, level, img),
    None => gl_not_loaded("glGetCompressedTexImage"),
  }
}
static glGetCompressedTexImage_p: GlFnCell<glGetCompressedTexImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetCompressedTexImage_is_loaded() -> bool {
  unsafe { *glGetCompressedTexImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetCompressedTexImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetCompressedTexImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetCompressedTexImage_t>>(gl_ptr_filter(f(b"glGetCompressedTexImage\0".as_ptr())));
}
/// glGetDebugMessageLog
/// * `sources` group: DebugSource
/// * `sources` len: count
/// * `types` group: DebugType
/// * `types` len: count
/// * `ids` len: count
/// * `severities` group: DebugSeverity
/// * `severities` len: count
/// * `lengths` len: count
/// * `messageLog` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetDebugMessageLog(count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetDebugMessageLog_p.0.get() } {
    Some(glGetDebugMessageLog_inner) => glGetDebugMessageLog_inner(count, bufSize, sources, types, ids, severities, lengths, messageLog),
    None => gl_not_loaded("glGetDebugMessageLog"),
  }
}
static glGetDebugMessageLog_p: GlFnCell<glGetDebugMessageLog_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetDebugMessageLog_is_loaded() -> bool {
  unsafe { *glGetDebugMessageLog_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetDebugMessageLog_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetDebugMessageLog_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetDebugMessageLog_t>>(gl_ptr_filter(f(b"glGetDebugMessageLog\0".as_ptr())));
}
/// glGetDoublev
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetDoublev(pname: GetPName, data: *mut GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetDoublev_p.0.get() } {
    Some(glGetDoublev_inner) => glGetDoublev_inner(pname, data),
    None => gl_not_loaded("glGetDoublev"),
  }
}
static glGetDoublev_p: GlFnCell<glGetDoublev_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetDoublev_is_loaded() -> bool {
  unsafe { *glGetDoublev_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetDoublev_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetDoublev_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetDoublev_t>>(gl_ptr_filter(f(b"glGetDoublev\0".as_ptr())));
}
/// glGetError
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetError() -> ErrorCode {
  #[allow(unused_unsafe)]
  match unsafe { *glGetError_p.0.get() } {
    Some(glGetError_inner) => glGetError_inner(),
    None => gl_not_loaded("glGetError"),
  }
}
static glGetError_p: GlFnCell<glGetError_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetError_is_loaded() -> bool {
  unsafe { *glGetError_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetError_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetError_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetError_t>>(gl_ptr_filter(f(b"glGetError\0".as_ptr())));
}
/// glGetFloatv
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetFloatv(pname: GetPName, data: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetFloatv_p.0.get() } {
    Some(glGetFloatv_inner) => glGetFloatv_inner(pname, data),
    None => gl_not_loaded("glGetFloatv"),
  }
}
static glGetFloatv_p: GlFnCell<glGetFloatv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetFloatv_is_loaded() -> bool {
  unsafe { *glGetFloatv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetFloatv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetFloatv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetFloatv_t>>(gl_ptr_filter(f(b"glGetFloatv\0".as_ptr())));
}
/// glGetFragDataIndex
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetFragDataIndex_p.0.get() } {
    Some(glGetFragDataIndex_inner) => glGetFragDataIndex_inner(program, name),
    None => gl_not_loaded("glGetFragDataIndex"),
  }
}
static glGetFragDataIndex_p: GlFnCell<glGetFragDataIndex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetFragDataIndex_is_loaded() -> bool {
  unsafe { *glGetFragDataIndex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetFragDataIndex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetFragDataIndex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetFragDataIndex_t>>(gl_ptr_filter(f(b"glGetFragDataIndex\0".as_ptr())));
}
/// glGetFragDataLocation
/// * `program` class: program
/// * `name` len: COMPSIZE(name)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetFragDataLocation_p.0.get() } {
    Some(glGetFragDataLocation_inner) => glGetFragDataLocation_inner(program, name),
    None => gl_not_loaded("glGetFragDataLocation"),
  }
}
static glGetFragDataLocation_p: GlFnCell<glGetFragDataLocation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetFragDataLocation_is_loaded() -> bool {
  unsafe { *glGetFragDataLocation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetFragDataLocation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetFragDataLocation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetFragDataLocation_t>>(gl_ptr_filter(f(b"glGetFragDataLocation\0".as_ptr())));
}
/// glGetFramebufferAttachmentParameteriv
/// * `target` group: FramebufferTarget
/// * `attachment` group: FramebufferAttachment
/// * `pname` group: FramebufferAttachmentParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetFramebufferAttachmentParameteriv(target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetFramebufferAttachmentParameteriv_p.0.get() } {
    Some(glGetFramebufferAttachmentParameteriv_inner) => glGetFramebufferAttachmentParameteriv_inner(target, attachment, pname, params),
    None => gl_not_loaded("glGetFramebufferAttachmentParameteriv"),
  }
}
static glGetFramebufferAttachmentParameteriv_p: GlFnCell<glGetFramebufferAttachmentParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetFramebufferAttachmentParameteriv_is_loaded() -> bool {
  unsafe { *glGetFramebufferAttachmentParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetFramebufferAttachmentParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetFramebufferAttachmentParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetFramebufferAttachmentParameteriv_t>>(gl_ptr_filter(f(b"glGetFramebufferAttachmentParameteriv\0".as_ptr())));
}
/// glGetInteger64i_v
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetInteger64i_v(target: GetPName, index: GLuint, data: *mut GLint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetInteger64i_v_p.0.get() } {
    Some(glGetInteger64i_v_inner) => glGetInteger64i_v_inner(target, index, data),
    None => gl_not_loaded("glGetInteger64i_v"),
  }
}
static glGetInteger64i_v_p: GlFnCell<glGetInteger64i_v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetInteger64i_v_is_loaded() -> bool {
  unsafe { *glGetInteger64i_v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetInteger64i_v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetInteger64i_v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetInteger64i_v_t>>(gl_ptr_filter(f(b"glGetInteger64i_v\0".as_ptr())));
}
/// glGetInteger64v
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetInteger64v(pname: GetPName, data: *mut GLint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetInteger64v_p.0.get() } {
    Some(glGetInteger64v_inner) => glGetInteger64v_inner(pname, data),
    None => gl_not_loaded("glGetInteger64v"),
  }
}
static glGetInteger64v_p: GlFnCell<glGetInteger64v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetInteger64v_is_loaded() -> bool {
  unsafe { *glGetInteger64v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetInteger64v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetInteger64v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetInteger64v_t>>(gl_ptr_filter(f(b"glGetInteger64v\0".as_ptr())));
}
/// glGetIntegeri_v
/// * `target` group: GetPName
/// * `data` len: COMPSIZE(target)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetIntegeri_v(target: GetPName, index: GLuint, data: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetIntegeri_v_p.0.get() } {
    Some(glGetIntegeri_v_inner) => glGetIntegeri_v_inner(target, index, data),
    None => gl_not_loaded("glGetIntegeri_v"),
  }
}
static glGetIntegeri_v_p: GlFnCell<glGetIntegeri_v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetIntegeri_v_is_loaded() -> bool {
  unsafe { *glGetIntegeri_v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetIntegeri_v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetIntegeri_v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetIntegeri_v_t>>(gl_ptr_filter(f(b"glGetIntegeri_v\0".as_ptr())));
}
/// glGetIntegerv
/// * `pname` group: GetPName
/// * `data` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetIntegerv(pname: GetPName, data: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetIntegerv_p.0.get() } {
    Some(glGetIntegerv_inner) => glGetIntegerv_inner(pname, data),
    None => gl_not_loaded("glGetIntegerv"),
  }
}
static glGetIntegerv_p: GlFnCell<glGetIntegerv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetIntegerv_is_loaded() -> bool {
  unsafe { *glGetIntegerv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetIntegerv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetIntegerv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetIntegerv_t>>(gl_ptr_filter(f(b"glGetIntegerv\0".as_ptr())));
}
/// glGetMultisamplefv
/// * `pname` group: GetMultisamplePNameNV
/// * `val` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetMultisamplefv(pname: GetMultisamplePNameNV, index: GLuint, val: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetMultisamplefv_p.0.get() } {
    Some(glGetMultisamplefv_inner) => glGetMultisamplefv_inner(pname, index, val),
    None => gl_not_loaded("glGetMultisamplefv"),
  }
}
static glGetMultisamplefv_p: GlFnCell<glGetMultisamplefv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetMultisamplefv_is_loaded() -> bool {
  unsafe { *glGetMultisamplefv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetMultisamplefv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetMultisamplefv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetMultisamplefv_t>>(gl_ptr_filter(f(b"glGetMultisamplefv\0".as_ptr())));
}
/// glGetObjectLabel
/// * `identifier` group: ObjectIdentifier
/// * `label` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetObjectLabel(identifier: ObjectIdentifier, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetObjectLabel_p.0.get() } {
    Some(glGetObjectLabel_inner) => glGetObjectLabel_inner(identifier, name, bufSize, length, label),
    None => gl_not_loaded("glGetObjectLabel"),
  }
}
static glGetObjectLabel_p: GlFnCell<glGetObjectLabel_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetObjectLabel_is_loaded() -> bool {
  unsafe { *glGetObjectLabel_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetObjectLabel_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetObjectLabel_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetObjectLabel_t>>(gl_ptr_filter(f(b"glGetObjectLabel\0".as_ptr())));
}
/// glGetObjectPtrLabel
/// * `label` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetObjectPtrLabel(ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetObjectPtrLabel_p.0.get() } {
    Some(glGetObjectPtrLabel_inner) => glGetObjectPtrLabel_inner(ptr, bufSize, length, label),
    None => gl_not_loaded("glGetObjectPtrLabel"),
  }
}
static glGetObjectPtrLabel_p: GlFnCell<glGetObjectPtrLabel_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetObjectPtrLabel_is_loaded() -> bool {
  unsafe { *glGetObjectPtrLabel_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetObjectPtrLabel_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetObjectPtrLabel_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetObjectPtrLabel_t>>(gl_ptr_filter(f(b"glGetObjectPtrLabel\0".as_ptr())));
}
/// glGetPointerv
/// * `pname` group: GetPointervPName
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetPointerv(pname: GetPointervPName, params: *mut *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetPointerv_p.0.get() } {
    Some(glGetPointerv_inner) => glGetPointerv_inner(pname, params),
    None => gl_not_loaded("glGetPointerv"),
  }
}
static glGetPointerv_p: GlFnCell<glGetPointerv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetPointerv_is_loaded() -> bool {
  unsafe { *glGetPointerv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetPointerv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetPointerv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetPointerv_t>>(gl_ptr_filter(f(b"glGetPointerv\0".as_ptr())));
}
/// glGetProgramInfoLog
/// * `program` class: program
/// * `infoLog` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramInfoLog_p.0.get() } {
    Some(glGetProgramInfoLog_inner) => glGetProgramInfoLog_inner(program, bufSize, length, infoLog),
    None => gl_not_loaded("glGetProgramInfoLog"),
  }
}
static glGetProgramInfoLog_p: GlFnCell<glGetProgramInfoLog_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramInfoLog_is_loaded() -> bool {
  unsafe { *glGetProgramInfoLog_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramInfoLog_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramInfoLog_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramInfoLog_t>>(gl_ptr_filter(f(b"glGetProgramInfoLog\0".as_ptr())));
}
/// glGetProgramiv
/// * `program` class: program
/// * `pname` group: ProgramPropertyARB
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetProgramiv(program: GLuint, pname: ProgramPropertyARB, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetProgramiv_p.0.get() } {
    Some(glGetProgramiv_inner) => glGetProgramiv_inner(program, pname, params),
    None => gl_not_loaded("glGetProgramiv"),
  }
}
static glGetProgramiv_p: GlFnCell<glGetProgramiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetProgramiv_is_loaded() -> bool {
  unsafe { *glGetProgramiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetProgramiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetProgramiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetProgramiv_t>>(gl_ptr_filter(f(b"glGetProgramiv\0".as_ptr())));
}
/// glGetQueryObjecti64v
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryObjecti64v(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryObjecti64v_p.0.get() } {
    Some(glGetQueryObjecti64v_inner) => glGetQueryObjecti64v_inner(id, pname, params),
    None => gl_not_loaded("glGetQueryObjecti64v"),
  }
}
static glGetQueryObjecti64v_p: GlFnCell<glGetQueryObjecti64v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryObjecti64v_is_loaded() -> bool {
  unsafe { *glGetQueryObjecti64v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryObjecti64v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryObjecti64v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryObjecti64v_t>>(gl_ptr_filter(f(b"glGetQueryObjecti64v\0".as_ptr())));
}
/// glGetQueryObjectiv
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryObjectiv(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryObjectiv_p.0.get() } {
    Some(glGetQueryObjectiv_inner) => glGetQueryObjectiv_inner(id, pname, params),
    None => gl_not_loaded("glGetQueryObjectiv"),
  }
}
static glGetQueryObjectiv_p: GlFnCell<glGetQueryObjectiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryObjectiv_is_loaded() -> bool {
  unsafe { *glGetQueryObjectiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryObjectiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryObjectiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryObjectiv_t>>(gl_ptr_filter(f(b"glGetQueryObjectiv\0".as_ptr())));
}
/// glGetQueryObjectui64v
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryObjectui64v(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryObjectui64v_p.0.get() } {
    Some(glGetQueryObjectui64v_inner) => glGetQueryObjectui64v_inner(id, pname, params),
    None => gl_not_loaded("glGetQueryObjectui64v"),
  }
}
static glGetQueryObjectui64v_p: GlFnCell<glGetQueryObjectui64v_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryObjectui64v_is_loaded() -> bool {
  unsafe { *glGetQueryObjectui64v_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryObjectui64v_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryObjectui64v_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryObjectui64v_t>>(gl_ptr_filter(f(b"glGetQueryObjectui64v\0".as_ptr())));
}
/// glGetQueryObjectuiv
/// * `id` class: query
/// * `pname` group: QueryObjectParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryObjectuiv(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryObjectuiv_p.0.get() } {
    Some(glGetQueryObjectuiv_inner) => glGetQueryObjectuiv_inner(id, pname, params),
    None => gl_not_loaded("glGetQueryObjectuiv"),
  }
}
static glGetQueryObjectuiv_p: GlFnCell<glGetQueryObjectuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryObjectuiv_is_loaded() -> bool {
  unsafe { *glGetQueryObjectuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryObjectuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryObjectuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryObjectuiv_t>>(gl_ptr_filter(f(b"glGetQueryObjectuiv\0".as_ptr())));
}
/// glGetQueryiv
/// * `target` group: QueryTarget
/// * `pname` group: QueryParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetQueryiv(target: QueryTarget, pname: QueryParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetQueryiv_p.0.get() } {
    Some(glGetQueryiv_inner) => glGetQueryiv_inner(target, pname, params),
    None => gl_not_loaded("glGetQueryiv"),
  }
}
static glGetQueryiv_p: GlFnCell<glGetQueryiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetQueryiv_is_loaded() -> bool {
  unsafe { *glGetQueryiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetQueryiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetQueryiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetQueryiv_t>>(gl_ptr_filter(f(b"glGetQueryiv\0".as_ptr())));
}
/// glGetRenderbufferParameteriv
/// * `target` group: RenderbufferTarget
/// * `pname` group: RenderbufferParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetRenderbufferParameteriv(target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetRenderbufferParameteriv_p.0.get() } {
    Some(glGetRenderbufferParameteriv_inner) => glGetRenderbufferParameteriv_inner(target, pname, params),
    None => gl_not_loaded("glGetRenderbufferParameteriv"),
  }
}
static glGetRenderbufferParameteriv_p: GlFnCell<glGetRenderbufferParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetRenderbufferParameteriv_is_loaded() -> bool {
  unsafe { *glGetRenderbufferParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetRenderbufferParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetRenderbufferParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetRenderbufferParameteriv_t>>(gl_ptr_filter(f(b"glGetRenderbufferParameteriv\0".as_ptr())));
}
/// glGetSamplerParameterIiv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetSamplerParameterIiv(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetSamplerParameterIiv_p.0.get() } {
    Some(glGetSamplerParameterIiv_inner) => glGetSamplerParameterIiv_inner(sampler, pname, params),
    None => gl_not_loaded("glGetSamplerParameterIiv"),
  }
}
static glGetSamplerParameterIiv_p: GlFnCell<glGetSamplerParameterIiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetSamplerParameterIiv_is_loaded() -> bool {
  unsafe { *glGetSamplerParameterIiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetSamplerParameterIiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetSamplerParameterIiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetSamplerParameterIiv_t>>(gl_ptr_filter(f(b"glGetSamplerParameterIiv\0".as_ptr())));
}
/// glGetSamplerParameterIuiv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetSamplerParameterIuiv(sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetSamplerParameterIuiv_p.0.get() } {
    Some(glGetSamplerParameterIuiv_inner) => glGetSamplerParameterIuiv_inner(sampler, pname, params),
    None => gl_not_loaded("glGetSamplerParameterIuiv"),
  }
}
static glGetSamplerParameterIuiv_p: GlFnCell<glGetSamplerParameterIuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetSamplerParameterIuiv_is_loaded() -> bool {
  unsafe { *glGetSamplerParameterIuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetSamplerParameterIuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetSamplerParameterIuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetSamplerParameterIuiv_t>>(gl_ptr_filter(f(b"glGetSamplerParameterIuiv\0".as_ptr())));
}
/// glGetSamplerParameterfv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterF
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetSamplerParameterfv(sampler: GLuint, pname: SamplerParameterF, params: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetSamplerParameterfv_p.0.get() } {
    Some(glGetSamplerParameterfv_inner) => glGetSamplerParameterfv_inner(sampler, pname, params),
    None => gl_not_loaded("glGetSamplerParameterfv"),
  }
}
static glGetSamplerParameterfv_p: GlFnCell<glGetSamplerParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetSamplerParameterfv_is_loaded() -> bool {
  unsafe { *glGetSamplerParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetSamplerParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetSamplerParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetSamplerParameterfv_t>>(gl_ptr_filter(f(b"glGetSamplerParameterfv\0".as_ptr())));
}
/// glGetSamplerParameteriv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetSamplerParameteriv(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetSamplerParameteriv_p.0.get() } {
    Some(glGetSamplerParameteriv_inner) => glGetSamplerParameteriv_inner(sampler, pname, params),
    None => gl_not_loaded("glGetSamplerParameteriv"),
  }
}
static glGetSamplerParameteriv_p: GlFnCell<glGetSamplerParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetSamplerParameteriv_is_loaded() -> bool {
  unsafe { *glGetSamplerParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetSamplerParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetSamplerParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetSamplerParameteriv_t>>(gl_ptr_filter(f(b"glGetSamplerParameteriv\0".as_ptr())));
}
/// glGetShaderInfoLog
/// * `shader` class: shader
/// * `infoLog` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetShaderInfoLog_p.0.get() } {
    Some(glGetShaderInfoLog_inner) => glGetShaderInfoLog_inner(shader, bufSize, length, infoLog),
    None => gl_not_loaded("glGetShaderInfoLog"),
  }
}
static glGetShaderInfoLog_p: GlFnCell<glGetShaderInfoLog_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetShaderInfoLog_is_loaded() -> bool {
  unsafe { *glGetShaderInfoLog_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetShaderInfoLog_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetShaderInfoLog_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetShaderInfoLog_t>>(gl_ptr_filter(f(b"glGetShaderInfoLog\0".as_ptr())));
}
/// glGetShaderSource
/// * `shader` class: shader
/// * `source` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetShaderSource_p.0.get() } {
    Some(glGetShaderSource_inner) => glGetShaderSource_inner(shader, bufSize, length, source),
    None => gl_not_loaded("glGetShaderSource"),
  }
}
static glGetShaderSource_p: GlFnCell<glGetShaderSource_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetShaderSource_is_loaded() -> bool {
  unsafe { *glGetShaderSource_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetShaderSource_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetShaderSource_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetShaderSource_t>>(gl_ptr_filter(f(b"glGetShaderSource\0".as_ptr())));
}
/// glGetShaderiv
/// * `shader` class: shader
/// * `pname` group: ShaderParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetShaderiv(shader: GLuint, pname: ShaderParameterName, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetShaderiv_p.0.get() } {
    Some(glGetShaderiv_inner) => glGetShaderiv_inner(shader, pname, params),
    None => gl_not_loaded("glGetShaderiv"),
  }
}
static glGetShaderiv_p: GlFnCell<glGetShaderiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetShaderiv_is_loaded() -> bool {
  unsafe { *glGetShaderiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetShaderiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetShaderiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetShaderiv_t>>(gl_ptr_filter(f(b"glGetShaderiv\0".as_ptr())));
}
/// [glGetString](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetString.xhtml)
///
/// Gets various string info about the GL implementation.
///
/// * `name`:
///   * `GL_VENDOR`: The name of the company that made this GL implementation.
///   * `GL_RENDERER`: The name of the renderer.
///   * `GL_VERSION`: A version or release number.
///   * `GL_SHADING_LANGUAGE_VERSION`: The version/release for the shading
///     language.
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetString(name: StringName) -> *const GLubyte {
  #[allow(unused_unsafe)]
  match unsafe { *glGetString_p.0.get() } {
    Some(glGetString_inner) => glGetString_inner(name),
    None => gl_not_loaded("glGetString"),
  }
}
static glGetString_p: GlFnCell<glGetString_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetString_is_loaded() -> bool {
  unsafe { *glGetString_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetString_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetString_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetString_t>>(gl_ptr_filter(f(b"glGetString\0".as_ptr())));
}
/// [glGetStringi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetString.xhtml)
///
/// Gets indexed string info about the GL implementation.
///
/// * `name`: One of:
///   * `GL_EXTENSIONS`: Returns the name of the extension specified by `index`.
///     Extensions are indexed in the range `0 .. GL_NUM_EXTENSIONS`. Use
///     `glGetIntegerv` to find the current implementation's number of
///     extensions.
/// * `index`: The index of the string to return.
///
/// See Also:
/// [glGetIntegerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGet.xhtml)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetStringi(name: StringName, index: GLuint) -> *const GLubyte {
  #[allow(unused_unsafe)]
  match unsafe { *glGetStringi_p.0.get() } {
    Some(glGetStringi_inner) => glGetStringi_inner(name, index),
    None => gl_not_loaded("glGetStringi"),
  }
}
static glGetStringi_p: GlFnCell<glGetStringi_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetStringi_is_loaded() -> bool {
  unsafe { *glGetStringi_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetStringi_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetStringi_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetStringi_t>>(gl_ptr_filter(f(b"glGetStringi\0".as_ptr())));
}
/// glGetSynciv
/// * `sync` group: sync
/// * `sync` class: sync
/// * `pname` group: SyncParameterName
/// * `values` len: count
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetSynciv(sync: GLsync, pname: SyncParameterName, count: GLsizei, length: *mut GLsizei, values: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetSynciv_p.0.get() } {
    Some(glGetSynciv_inner) => glGetSynciv_inner(sync, pname, count, length, values),
    None => gl_not_loaded("glGetSynciv"),
  }
}
static glGetSynciv_p: GlFnCell<glGetSynciv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetSynciv_is_loaded() -> bool {
  unsafe { *glGetSynciv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetSynciv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetSynciv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetSynciv_t>>(gl_ptr_filter(f(b"glGetSynciv\0".as_ptr())));
}
/// glGetTexImage
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(target,level,format,type)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexImage(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexImage_p.0.get() } {
    Some(glGetTexImage_inner) => glGetTexImage_inner(target, level, format, type_, pixels),
    None => gl_not_loaded("glGetTexImage"),
  }
}
static glGetTexImage_p: GlFnCell<glGetTexImage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexImage_is_loaded() -> bool {
  unsafe { *glGetTexImage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexImage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexImage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexImage_t>>(gl_ptr_filter(f(b"glGetTexImage\0".as_ptr())));
}
/// glGetTexLevelParameterfv
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexLevelParameterfv(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexLevelParameterfv_p.0.get() } {
    Some(glGetTexLevelParameterfv_inner) => glGetTexLevelParameterfv_inner(target, level, pname, params),
    None => gl_not_loaded("glGetTexLevelParameterfv"),
  }
}
static glGetTexLevelParameterfv_p: GlFnCell<glGetTexLevelParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexLevelParameterfv_is_loaded() -> bool {
  unsafe { *glGetTexLevelParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexLevelParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexLevelParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexLevelParameterfv_t>>(gl_ptr_filter(f(b"glGetTexLevelParameterfv\0".as_ptr())));
}
/// glGetTexLevelParameteriv
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexLevelParameteriv(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexLevelParameteriv_p.0.get() } {
    Some(glGetTexLevelParameteriv_inner) => glGetTexLevelParameteriv_inner(target, level, pname, params),
    None => gl_not_loaded("glGetTexLevelParameteriv"),
  }
}
static glGetTexLevelParameteriv_p: GlFnCell<glGetTexLevelParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexLevelParameteriv_is_loaded() -> bool {
  unsafe { *glGetTexLevelParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexLevelParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexLevelParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexLevelParameteriv_t>>(gl_ptr_filter(f(b"glGetTexLevelParameteriv\0".as_ptr())));
}
/// glGetTexParameterIiv
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexParameterIiv(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexParameterIiv_p.0.get() } {
    Some(glGetTexParameterIiv_inner) => glGetTexParameterIiv_inner(target, pname, params),
    None => gl_not_loaded("glGetTexParameterIiv"),
  }
}
static glGetTexParameterIiv_p: GlFnCell<glGetTexParameterIiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexParameterIiv_is_loaded() -> bool {
  unsafe { *glGetTexParameterIiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexParameterIiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexParameterIiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexParameterIiv_t>>(gl_ptr_filter(f(b"glGetTexParameterIiv\0".as_ptr())));
}
/// glGetTexParameterIuiv
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexParameterIuiv(target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexParameterIuiv_p.0.get() } {
    Some(glGetTexParameterIuiv_inner) => glGetTexParameterIuiv_inner(target, pname, params),
    None => gl_not_loaded("glGetTexParameterIuiv"),
  }
}
static glGetTexParameterIuiv_p: GlFnCell<glGetTexParameterIuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexParameterIuiv_is_loaded() -> bool {
  unsafe { *glGetTexParameterIuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexParameterIuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexParameterIuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexParameterIuiv_t>>(gl_ptr_filter(f(b"glGetTexParameterIuiv\0".as_ptr())));
}
/// glGetTexParameterfv
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexParameterfv(target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexParameterfv_p.0.get() } {
    Some(glGetTexParameterfv_inner) => glGetTexParameterfv_inner(target, pname, params),
    None => gl_not_loaded("glGetTexParameterfv"),
  }
}
static glGetTexParameterfv_p: GlFnCell<glGetTexParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexParameterfv_is_loaded() -> bool {
  unsafe { *glGetTexParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexParameterfv_t>>(gl_ptr_filter(f(b"glGetTexParameterfv\0".as_ptr())));
}
/// glGetTexParameteriv
/// * `target` group: TextureTarget
/// * `pname` group: GetTextureParameter
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTexParameteriv(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTexParameteriv_p.0.get() } {
    Some(glGetTexParameteriv_inner) => glGetTexParameteriv_inner(target, pname, params),
    None => gl_not_loaded("glGetTexParameteriv"),
  }
}
static glGetTexParameteriv_p: GlFnCell<glGetTexParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTexParameteriv_is_loaded() -> bool {
  unsafe { *glGetTexParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTexParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTexParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTexParameteriv_t>>(gl_ptr_filter(f(b"glGetTexParameteriv\0".as_ptr())));
}
/// glGetTransformFeedbackVarying
/// * `program` class: program
/// * `type` group: AttributeType
/// * `name` len: bufSize
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut AttributeType, name: *mut GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetTransformFeedbackVarying_p.0.get() } {
    Some(glGetTransformFeedbackVarying_inner) => glGetTransformFeedbackVarying_inner(program, index, bufSize, length, size, type_, name),
    None => gl_not_loaded("glGetTransformFeedbackVarying"),
  }
}
static glGetTransformFeedbackVarying_p: GlFnCell<glGetTransformFeedbackVarying_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetTransformFeedbackVarying_is_loaded() -> bool {
  unsafe { *glGetTransformFeedbackVarying_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetTransformFeedbackVarying_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetTransformFeedbackVarying_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetTransformFeedbackVarying_t>>(gl_ptr_filter(f(b"glGetTransformFeedbackVarying\0".as_ptr())));
}
/// glGetUniformBlockIndex
/// * `program` class: program
/// * `uniformBlockName` len: COMPSIZE()
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformBlockIndex_p.0.get() } {
    Some(glGetUniformBlockIndex_inner) => glGetUniformBlockIndex_inner(program, uniformBlockName),
    None => gl_not_loaded("glGetUniformBlockIndex"),
  }
}
static glGetUniformBlockIndex_p: GlFnCell<glGetUniformBlockIndex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformBlockIndex_is_loaded() -> bool {
  unsafe { *glGetUniformBlockIndex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformBlockIndex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformBlockIndex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformBlockIndex_t>>(gl_ptr_filter(f(b"glGetUniformBlockIndex\0".as_ptr())));
}
/// glGetUniformIndices
/// * `program` class: program
/// * `uniformNames` len: COMPSIZE(uniformCount)
/// * `uniformIndices` len: COMPSIZE(uniformCount)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformIndices_p.0.get() } {
    Some(glGetUniformIndices_inner) => glGetUniformIndices_inner(program, uniformCount, uniformNames, uniformIndices),
    None => gl_not_loaded("glGetUniformIndices"),
  }
}
static glGetUniformIndices_p: GlFnCell<glGetUniformIndices_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformIndices_is_loaded() -> bool {
  unsafe { *glGetUniformIndices_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformIndices_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformIndices_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformIndices_t>>(gl_ptr_filter(f(b"glGetUniformIndices\0".as_ptr())));
}
/// glGetUniformLocation
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformLocation_p.0.get() } {
    Some(glGetUniformLocation_inner) => glGetUniformLocation_inner(program, name),
    None => gl_not_loaded("glGetUniformLocation"),
  }
}
static glGetUniformLocation_p: GlFnCell<glGetUniformLocation_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformLocation_is_loaded() -> bool {
  unsafe { *glGetUniformLocation_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformLocation_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformLocation_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformLocation_t>>(gl_ptr_filter(f(b"glGetUniformLocation\0".as_ptr())));
}
/// glGetUniformfv
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformfv_p.0.get() } {
    Some(glGetUniformfv_inner) => glGetUniformfv_inner(program, location, params),
    None => gl_not_loaded("glGetUniformfv"),
  }
}
static glGetUniformfv_p: GlFnCell<glGetUniformfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformfv_is_loaded() -> bool {
  unsafe { *glGetUniformfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformfv_t>>(gl_ptr_filter(f(b"glGetUniformfv\0".as_ptr())));
}
/// glGetUniformiv
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformiv_p.0.get() } {
    Some(glGetUniformiv_inner) => glGetUniformiv_inner(program, location, params),
    None => gl_not_loaded("glGetUniformiv"),
  }
}
static glGetUniformiv_p: GlFnCell<glGetUniformiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformiv_is_loaded() -> bool {
  unsafe { *glGetUniformiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformiv_t>>(gl_ptr_filter(f(b"glGetUniformiv\0".as_ptr())));
}
/// glGetUniformuiv
/// * `program` class: program
/// * `params` len: COMPSIZE(program,location)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetUniformuiv_p.0.get() } {
    Some(glGetUniformuiv_inner) => glGetUniformuiv_inner(program, location, params),
    None => gl_not_loaded("glGetUniformuiv"),
  }
}
static glGetUniformuiv_p: GlFnCell<glGetUniformuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetUniformuiv_is_loaded() -> bool {
  unsafe { *glGetUniformuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetUniformuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetUniformuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetUniformuiv_t>>(gl_ptr_filter(f(b"glGetUniformuiv\0".as_ptr())));
}
/// glGetVertexAttribIiv
/// * `pname` group: VertexAttribEnum
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribIiv(index: GLuint, pname: VertexAttribEnum, params: *mut GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribIiv_p.0.get() } {
    Some(glGetVertexAttribIiv_inner) => glGetVertexAttribIiv_inner(index, pname, params),
    None => gl_not_loaded("glGetVertexAttribIiv"),
  }
}
static glGetVertexAttribIiv_p: GlFnCell<glGetVertexAttribIiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribIiv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribIiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribIiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribIiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribIiv_t>>(gl_ptr_filter(f(b"glGetVertexAttribIiv\0".as_ptr())));
}
/// glGetVertexAttribIuiv
/// * `pname` group: VertexAttribEnum
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribIuiv(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribIuiv_p.0.get() } {
    Some(glGetVertexAttribIuiv_inner) => glGetVertexAttribIuiv_inner(index, pname, params),
    None => gl_not_loaded("glGetVertexAttribIuiv"),
  }
}
static glGetVertexAttribIuiv_p: GlFnCell<glGetVertexAttribIuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribIuiv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribIuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribIuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribIuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribIuiv_t>>(gl_ptr_filter(f(b"glGetVertexAttribIuiv\0".as_ptr())));
}
/// glGetVertexAttribPointerv
/// * `pname` group: VertexAttribPointerPropertyARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribPointerv(index: GLuint, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribPointerv_p.0.get() } {
    Some(glGetVertexAttribPointerv_inner) => glGetVertexAttribPointerv_inner(index, pname, pointer),
    None => gl_not_loaded("glGetVertexAttribPointerv"),
  }
}
static glGetVertexAttribPointerv_p: GlFnCell<glGetVertexAttribPointerv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribPointerv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribPointerv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribPointerv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribPointerv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribPointerv_t>>(gl_ptr_filter(f(b"glGetVertexAttribPointerv\0".as_ptr())));
}
/// glGetVertexAttribdv
/// * `pname` group: VertexAttribPropertyARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribdv(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLdouble; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribdv_p.0.get() } {
    Some(glGetVertexAttribdv_inner) => glGetVertexAttribdv_inner(index, pname, params),
    None => gl_not_loaded("glGetVertexAttribdv"),
  }
}
static glGetVertexAttribdv_p: GlFnCell<glGetVertexAttribdv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribdv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribdv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribdv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribdv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribdv_t>>(gl_ptr_filter(f(b"glGetVertexAttribdv\0".as_ptr())));
}
/// glGetVertexAttribfv
/// * `pname` group: VertexAttribPropertyARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribfv(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLfloat; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribfv_p.0.get() } {
    Some(glGetVertexAttribfv_inner) => glGetVertexAttribfv_inner(index, pname, params),
    None => gl_not_loaded("glGetVertexAttribfv"),
  }
}
static glGetVertexAttribfv_p: GlFnCell<glGetVertexAttribfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribfv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribfv_t>>(gl_ptr_filter(f(b"glGetVertexAttribfv\0".as_ptr())));
}
/// glGetVertexAttribiv
/// * `pname` group: VertexAttribPropertyARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glGetVertexAttribiv(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glGetVertexAttribiv_p.0.get() } {
    Some(glGetVertexAttribiv_inner) => glGetVertexAttribiv_inner(index, pname, params),
    None => gl_not_loaded("glGetVertexAttribiv"),
  }
}
static glGetVertexAttribiv_p: GlFnCell<glGetVertexAttribiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glGetVertexAttribiv_is_loaded() -> bool {
  unsafe { *glGetVertexAttribiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glGetVertexAttribiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glGetVertexAttribiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glGetVertexAttribiv_t>>(gl_ptr_filter(f(b"glGetVertexAttribiv\0".as_ptr())));
}
/// glHint
/// * `target` group: HintTarget
/// * `mode` group: HintMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glHint(target: HintTarget, mode: HintMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glHint_p.0.get() } {
    Some(glHint_inner) => glHint_inner(target, mode),
    None => gl_not_loaded("glHint"),
  }
}
static glHint_p: GlFnCell<glHint_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glHint_is_loaded() -> bool {
  unsafe { *glHint_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glHint_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glHint_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glHint_t>>(gl_ptr_filter(f(b"glHint\0".as_ptr())));
}
/// glIsBuffer
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsBuffer(buffer: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsBuffer_p.0.get() } {
    Some(glIsBuffer_inner) => glIsBuffer_inner(buffer),
    None => gl_not_loaded("glIsBuffer"),
  }
}
static glIsBuffer_p: GlFnCell<glIsBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsBuffer_is_loaded() -> bool {
  unsafe { *glIsBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsBuffer_t>>(gl_ptr_filter(f(b"glIsBuffer\0".as_ptr())));
}
/// glIsEnabled
/// * `cap` group: EnableCap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsEnabled(cap: EnableCap) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsEnabled_p.0.get() } {
    Some(glIsEnabled_inner) => glIsEnabled_inner(cap),
    None => gl_not_loaded("glIsEnabled"),
  }
}
static glIsEnabled_p: GlFnCell<glIsEnabled_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsEnabled_is_loaded() -> bool {
  unsafe { *glIsEnabled_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsEnabled_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsEnabled_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsEnabled_t>>(gl_ptr_filter(f(b"glIsEnabled\0".as_ptr())));
}
/// glIsEnabledi
/// * `target` group: EnableCap
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsEnabledi(target: EnableCap, index: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsEnabledi_p.0.get() } {
    Some(glIsEnabledi_inner) => glIsEnabledi_inner(target, index),
    None => gl_not_loaded("glIsEnabledi"),
  }
}
static glIsEnabledi_p: GlFnCell<glIsEnabledi_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsEnabledi_is_loaded() -> bool {
  unsafe { *glIsEnabledi_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsEnabledi_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsEnabledi_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsEnabledi_t>>(gl_ptr_filter(f(b"glIsEnabledi\0".as_ptr())));
}
/// glIsFramebuffer
/// * `framebuffer` class: framebuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsFramebuffer_p.0.get() } {
    Some(glIsFramebuffer_inner) => glIsFramebuffer_inner(framebuffer),
    None => gl_not_loaded("glIsFramebuffer"),
  }
}
static glIsFramebuffer_p: GlFnCell<glIsFramebuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsFramebuffer_is_loaded() -> bool {
  unsafe { *glIsFramebuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsFramebuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsFramebuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsFramebuffer_t>>(gl_ptr_filter(f(b"glIsFramebuffer\0".as_ptr())));
}
/// glIsProgram
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsProgram(program: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsProgram_p.0.get() } {
    Some(glIsProgram_inner) => glIsProgram_inner(program),
    None => gl_not_loaded("glIsProgram"),
  }
}
static glIsProgram_p: GlFnCell<glIsProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsProgram_is_loaded() -> bool {
  unsafe { *glIsProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsProgram_t>>(gl_ptr_filter(f(b"glIsProgram\0".as_ptr())));
}
/// glIsQuery
/// * `id` class: query
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsQuery(id: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsQuery_p.0.get() } {
    Some(glIsQuery_inner) => glIsQuery_inner(id),
    None => gl_not_loaded("glIsQuery"),
  }
}
static glIsQuery_p: GlFnCell<glIsQuery_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsQuery_is_loaded() -> bool {
  unsafe { *glIsQuery_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsQuery_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsQuery_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsQuery_t>>(gl_ptr_filter(f(b"glIsQuery\0".as_ptr())));
}
/// glIsRenderbuffer
/// * `renderbuffer` class: renderbuffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsRenderbuffer_p.0.get() } {
    Some(glIsRenderbuffer_inner) => glIsRenderbuffer_inner(renderbuffer),
    None => gl_not_loaded("glIsRenderbuffer"),
  }
}
static glIsRenderbuffer_p: GlFnCell<glIsRenderbuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsRenderbuffer_is_loaded() -> bool {
  unsafe { *glIsRenderbuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsRenderbuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsRenderbuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsRenderbuffer_t>>(gl_ptr_filter(f(b"glIsRenderbuffer\0".as_ptr())));
}
/// glIsSampler
/// * `sampler` class: sampler
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsSampler(sampler: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsSampler_p.0.get() } {
    Some(glIsSampler_inner) => glIsSampler_inner(sampler),
    None => gl_not_loaded("glIsSampler"),
  }
}
static glIsSampler_p: GlFnCell<glIsSampler_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsSampler_is_loaded() -> bool {
  unsafe { *glIsSampler_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsSampler_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsSampler_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsSampler_t>>(gl_ptr_filter(f(b"glIsSampler\0".as_ptr())));
}
/// glIsShader
/// * `shader` class: shader
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsShader(shader: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsShader_p.0.get() } {
    Some(glIsShader_inner) => glIsShader_inner(shader),
    None => gl_not_loaded("glIsShader"),
  }
}
static glIsShader_p: GlFnCell<glIsShader_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsShader_is_loaded() -> bool {
  unsafe { *glIsShader_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsShader_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsShader_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsShader_t>>(gl_ptr_filter(f(b"glIsShader\0".as_ptr())));
}
/// glIsSync
/// * `sync` group: sync
/// * `sync` class: sync
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsSync(sync: GLsync) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsSync_p.0.get() } {
    Some(glIsSync_inner) => glIsSync_inner(sync),
    None => gl_not_loaded("glIsSync"),
  }
}
static glIsSync_p: GlFnCell<glIsSync_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsSync_is_loaded() -> bool {
  unsafe { *glIsSync_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsSync_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsSync_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsSync_t>>(gl_ptr_filter(f(b"glIsSync\0".as_ptr())));
}
/// glIsTexture
/// * `texture` group: Texture
/// * `texture` class: texture
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsTexture(texture: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsTexture_p.0.get() } {
    Some(glIsTexture_inner) => glIsTexture_inner(texture),
    None => gl_not_loaded("glIsTexture"),
  }
}
static glIsTexture_p: GlFnCell<glIsTexture_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsTexture_is_loaded() -> bool {
  unsafe { *glIsTexture_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsTexture_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsTexture_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsTexture_t>>(gl_ptr_filter(f(b"glIsTexture\0".as_ptr())));
}
/// glIsVertexArray
/// * `array` class: vertex array
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glIsVertexArray(array: GLuint) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glIsVertexArray_p.0.get() } {
    Some(glIsVertexArray_inner) => glIsVertexArray_inner(array),
    None => gl_not_loaded("glIsVertexArray"),
  }
}
static glIsVertexArray_p: GlFnCell<glIsVertexArray_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glIsVertexArray_is_loaded() -> bool {
  unsafe { *glIsVertexArray_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glIsVertexArray_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glIsVertexArray_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glIsVertexArray_t>>(gl_ptr_filter(f(b"glIsVertexArray\0".as_ptr())));
}
/// glLineWidth
/// * `width` group: CheckedFloat32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glLineWidth(width: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glLineWidth_p.0.get() } {
    Some(glLineWidth_inner) => glLineWidth_inner(width),
    None => gl_not_loaded("glLineWidth"),
  }
}
static glLineWidth_p: GlFnCell<glLineWidth_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glLineWidth_is_loaded() -> bool {
  unsafe { *glLineWidth_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glLineWidth_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glLineWidth_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glLineWidth_t>>(gl_ptr_filter(f(b"glLineWidth\0".as_ptr())));
}
/// [glLinkProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLinkProgram.xhtml)
///
/// Performs linking on a program object. The link status of the program will be
/// stored in its object state, you can check it with `glGetProgram` and/or
/// `glGetProgramInfoLog`.
///
/// * `program` the name of the program to link
#[cfg_attr(feature = "track_caller", track_caller)]
pub fn glLinkProgram(program: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glLinkProgram_p.0.get() } {
    Some(glLinkProgram_inner) => glLinkProgram_inner(program),
    None => gl_not_loaded("glLinkProgram"),
  }
}
static glLinkProgram_p: GlFnCell<glLinkProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glLinkProgram_is_loaded() -> bool {
  unsafe { *glLinkProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glLinkProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glLinkProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glLinkProgram_t>>(gl_ptr_filter(f(b"glLinkProgram\0".as_ptr())));
}
/// glLogicOp
/// * `opcode` group: LogicOp
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glLogicOp(opcode: LogicOp) {
  #[allow(unused_unsafe)]
  match unsafe { *glLogicOp_p.0.get() } {
    Some(glLogicOp_inner) => glLogicOp_inner(opcode),
    None => gl_not_loaded("glLogicOp"),
  }
}
static glLogicOp_p: GlFnCell<glLogicOp_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glLogicOp_is_loaded() -> bool {
  unsafe { *glLogicOp_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glLogicOp_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glLogicOp_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glLogicOp_t>>(gl_ptr_filter(f(b"glLogicOp\0".as_ptr())));
}
/// glMapBuffer
/// * `target` group: BufferTargetARB
/// * `access` group: BufferAccessARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMapBuffer(target: BufferTargetARB, access: BufferAccessARB) -> *mut void {
  #[allow(unused_unsafe)]
  match unsafe { *glMapBuffer_p.0.get() } {
    Some(glMapBuffer_inner) => glMapBuffer_inner(target, access),
    None => gl_not_loaded("glMapBuffer"),
  }
}
static glMapBuffer_p: GlFnCell<glMapBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMapBuffer_is_loaded() -> bool {
  unsafe { *glMapBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMapBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMapBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMapBuffer_t>>(gl_ptr_filter(f(b"glMapBuffer\0".as_ptr())));
}
/// glMapBufferRange
/// * `target` group: BufferTargetARB
/// * `offset` group: BufferOffset
/// * `length` group: BufferSize
/// * `access` group: MapBufferAccessMask
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMapBufferRange(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void {
  #[allow(unused_unsafe)]
  match unsafe { *glMapBufferRange_p.0.get() } {
    Some(glMapBufferRange_inner) => glMapBufferRange_inner(target, offset, length, access),
    None => gl_not_loaded("glMapBufferRange"),
  }
}
static glMapBufferRange_p: GlFnCell<glMapBufferRange_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMapBufferRange_is_loaded() -> bool {
  unsafe { *glMapBufferRange_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMapBufferRange_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMapBufferRange_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMapBufferRange_t>>(gl_ptr_filter(f(b"glMapBufferRange\0".as_ptr())));
}
/// glMultiDrawArrays
/// * `mode` group: PrimitiveType
/// * `first` len: COMPSIZE(drawcount)
/// * `count` len: COMPSIZE(drawcount)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiDrawArrays(mode: PrimitiveType, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiDrawArrays_p.0.get() } {
    Some(glMultiDrawArrays_inner) => glMultiDrawArrays_inner(mode, first, count, drawcount),
    None => gl_not_loaded("glMultiDrawArrays"),
  }
}
static glMultiDrawArrays_p: GlFnCell<glMultiDrawArrays_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiDrawArrays_is_loaded() -> bool {
  unsafe { *glMultiDrawArrays_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiDrawArrays_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiDrawArrays_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiDrawArrays_t>>(gl_ptr_filter(f(b"glMultiDrawArrays\0".as_ptr())));
}
/// glMultiDrawElements
/// * `mode` group: PrimitiveType
/// * `count` len: COMPSIZE(drawcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(drawcount)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiDrawElements(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiDrawElements_p.0.get() } {
    Some(glMultiDrawElements_inner) => glMultiDrawElements_inner(mode, count, type_, indices, drawcount),
    None => gl_not_loaded("glMultiDrawElements"),
  }
}
static glMultiDrawElements_p: GlFnCell<glMultiDrawElements_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiDrawElements_is_loaded() -> bool {
  unsafe { *glMultiDrawElements_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiDrawElements_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiDrawElements_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiDrawElements_t>>(gl_ptr_filter(f(b"glMultiDrawElements\0".as_ptr())));
}
/// glMultiDrawElementsBaseVertex
/// * `mode` group: PrimitiveType
/// * `count` len: COMPSIZE(drawcount)
/// * `type` group: DrawElementsType
/// * `indices` len: COMPSIZE(drawcount)
/// * `basevertex` len: COMPSIZE(drawcount)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glMultiDrawElementsBaseVertex(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glMultiDrawElementsBaseVertex_p.0.get() } {
    Some(glMultiDrawElementsBaseVertex_inner) => glMultiDrawElementsBaseVertex_inner(mode, count, type_, indices, drawcount, basevertex),
    None => gl_not_loaded("glMultiDrawElementsBaseVertex"),
  }
}
static glMultiDrawElementsBaseVertex_p: GlFnCell<glMultiDrawElementsBaseVertex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glMultiDrawElementsBaseVertex_is_loaded() -> bool {
  unsafe { *glMultiDrawElementsBaseVertex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glMultiDrawElementsBaseVertex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glMultiDrawElementsBaseVertex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glMultiDrawElementsBaseVertex_t>>(gl_ptr_filter(f(b"glMultiDrawElementsBaseVertex\0".as_ptr())));
}
/// glObjectLabel
/// * `identifier` group: ObjectIdentifier
/// * `label` len: COMPSIZE(label,length)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glObjectLabel(identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glObjectLabel_p.0.get() } {
    Some(glObjectLabel_inner) => glObjectLabel_inner(identifier, name, length, label),
    None => gl_not_loaded("glObjectLabel"),
  }
}
static glObjectLabel_p: GlFnCell<glObjectLabel_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glObjectLabel_is_loaded() -> bool {
  unsafe { *glObjectLabel_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glObjectLabel_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glObjectLabel_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glObjectLabel_t>>(gl_ptr_filter(f(b"glObjectLabel\0".as_ptr())));
}
/// glObjectPtrLabel
/// * `label` len: COMPSIZE(label,length)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glObjectPtrLabel(ptr: *const void, length: GLsizei, label: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glObjectPtrLabel_p.0.get() } {
    Some(glObjectPtrLabel_inner) => glObjectPtrLabel_inner(ptr, length, label),
    None => gl_not_loaded("glObjectPtrLabel"),
  }
}
static glObjectPtrLabel_p: GlFnCell<glObjectPtrLabel_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glObjectPtrLabel_is_loaded() -> bool {
  unsafe { *glObjectPtrLabel_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glObjectPtrLabel_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glObjectPtrLabel_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glObjectPtrLabel_t>>(gl_ptr_filter(f(b"glObjectPtrLabel\0".as_ptr())));
}
/// glPixelStoref
/// * `pname` group: PixelStoreParameter
/// * `param` group: CheckedFloat32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPixelStoref(pname: PixelStoreParameter, param: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glPixelStoref_p.0.get() } {
    Some(glPixelStoref_inner) => glPixelStoref_inner(pname, param),
    None => gl_not_loaded("glPixelStoref"),
  }
}
static glPixelStoref_p: GlFnCell<glPixelStoref_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPixelStoref_is_loaded() -> bool {
  unsafe { *glPixelStoref_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPixelStoref_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPixelStoref_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPixelStoref_t>>(gl_ptr_filter(f(b"glPixelStoref\0".as_ptr())));
}
/// glPixelStorei
/// * `pname` group: PixelStoreParameter
/// * `param` group: CheckedInt32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPixelStorei(pname: PixelStoreParameter, param: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glPixelStorei_p.0.get() } {
    Some(glPixelStorei_inner) => glPixelStorei_inner(pname, param),
    None => gl_not_loaded("glPixelStorei"),
  }
}
static glPixelStorei_p: GlFnCell<glPixelStorei_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPixelStorei_is_loaded() -> bool {
  unsafe { *glPixelStorei_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPixelStorei_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPixelStorei_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPixelStorei_t>>(gl_ptr_filter(f(b"glPixelStorei\0".as_ptr())));
}
/// glPointParameterf
/// * `pname` group: PointParameterNameARB
/// * `param` group: CheckedFloat32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPointParameterf(pname: PointParameterNameARB, param: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glPointParameterf_p.0.get() } {
    Some(glPointParameterf_inner) => glPointParameterf_inner(pname, param),
    None => gl_not_loaded("glPointParameterf"),
  }
}
static glPointParameterf_p: GlFnCell<glPointParameterf_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPointParameterf_is_loaded() -> bool {
  unsafe { *glPointParameterf_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPointParameterf_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPointParameterf_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPointParameterf_t>>(gl_ptr_filter(f(b"glPointParameterf\0".as_ptr())));
}
/// glPointParameterfv
/// * `pname` group: PointParameterNameARB
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPointParameterfv(pname: PointParameterNameARB, params: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glPointParameterfv_p.0.get() } {
    Some(glPointParameterfv_inner) => glPointParameterfv_inner(pname, params),
    None => gl_not_loaded("glPointParameterfv"),
  }
}
static glPointParameterfv_p: GlFnCell<glPointParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPointParameterfv_is_loaded() -> bool {
  unsafe { *glPointParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPointParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPointParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPointParameterfv_t>>(gl_ptr_filter(f(b"glPointParameterfv\0".as_ptr())));
}
/// glPointParameteri
/// * `pname` group: PointParameterNameARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPointParameteri(pname: PointParameterNameARB, param: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glPointParameteri_p.0.get() } {
    Some(glPointParameteri_inner) => glPointParameteri_inner(pname, param),
    None => gl_not_loaded("glPointParameteri"),
  }
}
static glPointParameteri_p: GlFnCell<glPointParameteri_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPointParameteri_is_loaded() -> bool {
  unsafe { *glPointParameteri_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPointParameteri_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPointParameteri_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPointParameteri_t>>(gl_ptr_filter(f(b"glPointParameteri\0".as_ptr())));
}
/// glPointParameteriv
/// * `pname` group: PointParameterNameARB
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPointParameteriv(pname: PointParameterNameARB, params: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glPointParameteriv_p.0.get() } {
    Some(glPointParameteriv_inner) => glPointParameteriv_inner(pname, params),
    None => gl_not_loaded("glPointParameteriv"),
  }
}
static glPointParameteriv_p: GlFnCell<glPointParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPointParameteriv_is_loaded() -> bool {
  unsafe { *glPointParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPointParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPointParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPointParameteriv_t>>(gl_ptr_filter(f(b"glPointParameteriv\0".as_ptr())));
}
/// [glPointSize](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointSize.xhtml)
///
/// Sets the diameter of rasterized points if `GL_PROGRAM_POINT_SIZE` is
/// *disabled*. (Otherwise, this setting is ignored and you must modify
/// `gl_PointSize` from within a shader to change point size.)
///
/// The default point size is 1.0, and it cannot be set to less than 0.0.
#[cfg_attr(feature = "track_caller", track_caller)]
pub fn glPointSize(size: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glPointSize_p.0.get() } {
    Some(glPointSize_inner) => glPointSize_inner(size),
    None => gl_not_loaded("glPointSize"),
  }
}
static glPointSize_p: GlFnCell<glPointSize_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPointSize_is_loaded() -> bool {
  unsafe { *glPointSize_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPointSize_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPointSize_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPointSize_t>>(gl_ptr_filter(f(b"glPointSize\0".as_ptr())));
}
/// glPolygonMode
/// * `face` group: MaterialFace
/// * `mode` group: PolygonMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPolygonMode(face: MaterialFace, mode: PolygonMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glPolygonMode_p.0.get() } {
    Some(glPolygonMode_inner) => glPolygonMode_inner(face, mode),
    None => gl_not_loaded("glPolygonMode"),
  }
}
static glPolygonMode_p: GlFnCell<glPolygonMode_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPolygonMode_is_loaded() -> bool {
  unsafe { *glPolygonMode_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPolygonMode_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPolygonMode_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPolygonMode_t>>(gl_ptr_filter(f(b"glPolygonMode\0".as_ptr())));
}
/// glPolygonOffset
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPolygonOffset(factor: GLfloat, units: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glPolygonOffset_p.0.get() } {
    Some(glPolygonOffset_inner) => glPolygonOffset_inner(factor, units),
    None => gl_not_loaded("glPolygonOffset"),
  }
}
static glPolygonOffset_p: GlFnCell<glPolygonOffset_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPolygonOffset_is_loaded() -> bool {
  unsafe { *glPolygonOffset_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPolygonOffset_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPolygonOffset_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPolygonOffset_t>>(gl_ptr_filter(f(b"glPolygonOffset\0".as_ptr())));
}
/// glPopDebugGroup
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPopDebugGroup() {
  #[allow(unused_unsafe)]
  match unsafe { *glPopDebugGroup_p.0.get() } {
    Some(glPopDebugGroup_inner) => glPopDebugGroup_inner(),
    None => gl_not_loaded("glPopDebugGroup"),
  }
}
static glPopDebugGroup_p: GlFnCell<glPopDebugGroup_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPopDebugGroup_is_loaded() -> bool {
  unsafe { *glPopDebugGroup_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPopDebugGroup_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPopDebugGroup_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPopDebugGroup_t>>(gl_ptr_filter(f(b"glPopDebugGroup\0".as_ptr())));
}
/// glPrimitiveRestartIndex
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPrimitiveRestartIndex(index: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glPrimitiveRestartIndex_p.0.get() } {
    Some(glPrimitiveRestartIndex_inner) => glPrimitiveRestartIndex_inner(index),
    None => gl_not_loaded("glPrimitiveRestartIndex"),
  }
}
static glPrimitiveRestartIndex_p: GlFnCell<glPrimitiveRestartIndex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPrimitiveRestartIndex_is_loaded() -> bool {
  unsafe { *glPrimitiveRestartIndex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPrimitiveRestartIndex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPrimitiveRestartIndex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPrimitiveRestartIndex_t>>(gl_ptr_filter(f(b"glPrimitiveRestartIndex\0".as_ptr())));
}
/// glProvokingVertex
/// * `mode` group: VertexProvokingMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glProvokingVertex(mode: VertexProvokingMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glProvokingVertex_p.0.get() } {
    Some(glProvokingVertex_inner) => glProvokingVertex_inner(mode),
    None => gl_not_loaded("glProvokingVertex"),
  }
}
static glProvokingVertex_p: GlFnCell<glProvokingVertex_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glProvokingVertex_is_loaded() -> bool {
  unsafe { *glProvokingVertex_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glProvokingVertex_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glProvokingVertex_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glProvokingVertex_t>>(gl_ptr_filter(f(b"glProvokingVertex\0".as_ptr())));
}
/// glPushDebugGroup
/// * `source` group: DebugSource
/// * `message` len: COMPSIZE(message,length)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glPushDebugGroup(source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar) {
  #[allow(unused_unsafe)]
  match unsafe { *glPushDebugGroup_p.0.get() } {
    Some(glPushDebugGroup_inner) => glPushDebugGroup_inner(source, id, length, message),
    None => gl_not_loaded("glPushDebugGroup"),
  }
}
static glPushDebugGroup_p: GlFnCell<glPushDebugGroup_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glPushDebugGroup_is_loaded() -> bool {
  unsafe { *glPushDebugGroup_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glPushDebugGroup_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glPushDebugGroup_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glPushDebugGroup_t>>(gl_ptr_filter(f(b"glPushDebugGroup\0".as_ptr())));
}
/// glQueryCounter
/// * `id` class: query
/// * `target` group: QueryCounterTarget
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glQueryCounter(id: GLuint, target: QueryCounterTarget) {
  #[allow(unused_unsafe)]
  match unsafe { *glQueryCounter_p.0.get() } {
    Some(glQueryCounter_inner) => glQueryCounter_inner(id, target),
    None => gl_not_loaded("glQueryCounter"),
  }
}
static glQueryCounter_p: GlFnCell<glQueryCounter_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glQueryCounter_is_loaded() -> bool {
  unsafe { *glQueryCounter_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glQueryCounter_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glQueryCounter_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glQueryCounter_t>>(gl_ptr_filter(f(b"glQueryCounter\0".as_ptr())));
}
/// glReadBuffer
/// * `src` group: ReadBufferMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glReadBuffer(src: ReadBufferMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glReadBuffer_p.0.get() } {
    Some(glReadBuffer_inner) => glReadBuffer_inner(src),
    None => gl_not_loaded("glReadBuffer"),
  }
}
static glReadBuffer_p: GlFnCell<glReadBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glReadBuffer_is_loaded() -> bool {
  unsafe { *glReadBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glReadBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glReadBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glReadBuffer_t>>(gl_ptr_filter(f(b"glReadBuffer\0".as_ptr())));
}
/// glReadPixels
/// * `x` group: WinCoord
/// * `y` group: WinCoord
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *mut void) {
  #[allow(unused_unsafe)]
  match unsafe { *glReadPixels_p.0.get() } {
    Some(glReadPixels_inner) => glReadPixels_inner(x, y, width, height, format, type_, pixels),
    None => gl_not_loaded("glReadPixels"),
  }
}
static glReadPixels_p: GlFnCell<glReadPixels_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glReadPixels_is_loaded() -> bool {
  unsafe { *glReadPixels_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glReadPixels_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glReadPixels_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glReadPixels_t>>(gl_ptr_filter(f(b"glReadPixels\0".as_ptr())));
}
/// glRenderbufferStorage
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glRenderbufferStorage(target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glRenderbufferStorage_p.0.get() } {
    Some(glRenderbufferStorage_inner) => glRenderbufferStorage_inner(target, internalformat, width, height),
    None => gl_not_loaded("glRenderbufferStorage"),
  }
}
static glRenderbufferStorage_p: GlFnCell<glRenderbufferStorage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glRenderbufferStorage_is_loaded() -> bool {
  unsafe { *glRenderbufferStorage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glRenderbufferStorage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glRenderbufferStorage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glRenderbufferStorage_t>>(gl_ptr_filter(f(b"glRenderbufferStorage\0".as_ptr())));
}
/// glRenderbufferStorageMultisample
/// * `target` group: RenderbufferTarget
/// * `internalformat` group: InternalFormat
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glRenderbufferStorageMultisample(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glRenderbufferStorageMultisample_p.0.get() } {
    Some(glRenderbufferStorageMultisample_inner) => glRenderbufferStorageMultisample_inner(target, samples, internalformat, width, height),
    None => gl_not_loaded("glRenderbufferStorageMultisample"),
  }
}
static glRenderbufferStorageMultisample_p: GlFnCell<glRenderbufferStorageMultisample_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glRenderbufferStorageMultisample_is_loaded() -> bool {
  unsafe { *glRenderbufferStorageMultisample_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glRenderbufferStorageMultisample_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glRenderbufferStorageMultisample_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glRenderbufferStorageMultisample_t>>(gl_ptr_filter(f(b"glRenderbufferStorageMultisample\0".as_ptr())));
}
/// glSampleCoverage
/// * `invert` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSampleCoverage(value: GLfloat, invert: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glSampleCoverage_p.0.get() } {
    Some(glSampleCoverage_inner) => glSampleCoverage_inner(value, invert),
    None => gl_not_loaded("glSampleCoverage"),
  }
}
static glSampleCoverage_p: GlFnCell<glSampleCoverage_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSampleCoverage_is_loaded() -> bool {
  unsafe { *glSampleCoverage_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSampleCoverage_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSampleCoverage_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSampleCoverage_t>>(gl_ptr_filter(f(b"glSampleCoverage\0".as_ptr())));
}
/// glSampleMaski
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSampleMaski(maskNumber: GLuint, mask: GLbitfield) {
  #[allow(unused_unsafe)]
  match unsafe { *glSampleMaski_p.0.get() } {
    Some(glSampleMaski_inner) => glSampleMaski_inner(maskNumber, mask),
    None => gl_not_loaded("glSampleMaski"),
  }
}
static glSampleMaski_p: GlFnCell<glSampleMaski_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSampleMaski_is_loaded() -> bool {
  unsafe { *glSampleMaski_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSampleMaski_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSampleMaski_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSampleMaski_t>>(gl_ptr_filter(f(b"glSampleMaski\0".as_ptr())));
}
/// glSamplerParameterIiv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSamplerParameterIiv(sampler: GLuint, pname: SamplerParameterI, param: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glSamplerParameterIiv_p.0.get() } {
    Some(glSamplerParameterIiv_inner) => glSamplerParameterIiv_inner(sampler, pname, param),
    None => gl_not_loaded("glSamplerParameterIiv"),
  }
}
static glSamplerParameterIiv_p: GlFnCell<glSamplerParameterIiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSamplerParameterIiv_is_loaded() -> bool {
  unsafe { *glSamplerParameterIiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSamplerParameterIiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSamplerParameterIiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSamplerParameterIiv_t>>(gl_ptr_filter(f(b"glSamplerParameterIiv\0".as_ptr())));
}
/// glSamplerParameterIuiv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSamplerParameterIuiv(sampler: GLuint, pname: SamplerParameterI, param: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glSamplerParameterIuiv_p.0.get() } {
    Some(glSamplerParameterIuiv_inner) => glSamplerParameterIuiv_inner(sampler, pname, param),
    None => gl_not_loaded("glSamplerParameterIuiv"),
  }
}
static glSamplerParameterIuiv_p: GlFnCell<glSamplerParameterIuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSamplerParameterIuiv_is_loaded() -> bool {
  unsafe { *glSamplerParameterIuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSamplerParameterIuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSamplerParameterIuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSamplerParameterIuiv_t>>(gl_ptr_filter(f(b"glSamplerParameterIuiv\0".as_ptr())));
}
/// glSamplerParameterf
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterF
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSamplerParameterf(sampler: GLuint, pname: SamplerParameterF, param: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glSamplerParameterf_p.0.get() } {
    Some(glSamplerParameterf_inner) => glSamplerParameterf_inner(sampler, pname, param),
    None => gl_not_loaded("glSamplerParameterf"),
  }
}
static glSamplerParameterf_p: GlFnCell<glSamplerParameterf_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSamplerParameterf_is_loaded() -> bool {
  unsafe { *glSamplerParameterf_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSamplerParameterf_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSamplerParameterf_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSamplerParameterf_t>>(gl_ptr_filter(f(b"glSamplerParameterf\0".as_ptr())));
}
/// glSamplerParameterfv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterF
/// * `param` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSamplerParameterfv(sampler: GLuint, pname: SamplerParameterF, param: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glSamplerParameterfv_p.0.get() } {
    Some(glSamplerParameterfv_inner) => glSamplerParameterfv_inner(sampler, pname, param),
    None => gl_not_loaded("glSamplerParameterfv"),
  }
}
static glSamplerParameterfv_p: GlFnCell<glSamplerParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSamplerParameterfv_is_loaded() -> bool {
  unsafe { *glSamplerParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSamplerParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSamplerParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSamplerParameterfv_t>>(gl_ptr_filter(f(b"glSamplerParameterfv\0".as_ptr())));
}
/// glSamplerParameteri
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSamplerParameteri(sampler: GLuint, pname: SamplerParameterI, param: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glSamplerParameteri_p.0.get() } {
    Some(glSamplerParameteri_inner) => glSamplerParameteri_inner(sampler, pname, param),
    None => gl_not_loaded("glSamplerParameteri"),
  }
}
static glSamplerParameteri_p: GlFnCell<glSamplerParameteri_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSamplerParameteri_is_loaded() -> bool {
  unsafe { *glSamplerParameteri_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSamplerParameteri_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSamplerParameteri_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSamplerParameteri_t>>(gl_ptr_filter(f(b"glSamplerParameteri\0".as_ptr())));
}
/// glSamplerParameteriv
/// * `sampler` class: sampler
/// * `pname` group: SamplerParameterI
/// * `param` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glSamplerParameteriv(sampler: GLuint, pname: SamplerParameterI, param: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glSamplerParameteriv_p.0.get() } {
    Some(glSamplerParameteriv_inner) => glSamplerParameteriv_inner(sampler, pname, param),
    None => gl_not_loaded("glSamplerParameteriv"),
  }
}
static glSamplerParameteriv_p: GlFnCell<glSamplerParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glSamplerParameteriv_is_loaded() -> bool {
  unsafe { *glSamplerParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glSamplerParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glSamplerParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glSamplerParameteriv_t>>(gl_ptr_filter(f(b"glSamplerParameteriv\0".as_ptr())));
}
/// glScissor
/// * `x` group: WinCoord
/// * `y` group: WinCoord
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glScissor_p.0.get() } {
    Some(glScissor_inner) => glScissor_inner(x, y, width, height),
    None => gl_not_loaded("glScissor"),
  }
}
static glScissor_p: GlFnCell<glScissor_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glScissor_is_loaded() -> bool {
  unsafe { *glScissor_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glScissor_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glScissor_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glScissor_t>>(gl_ptr_filter(f(b"glScissor\0".as_ptr())));
}
/// [glShaderSource](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSource.xhtml)
///
/// Sets the source string of the named shader. This replaces any previously set
/// source. OpenGL copies the data into its own memory, so you can free your
/// instance of the source string after the call (if necessary).
///
/// * `shader` the shader ID to attach the source to.
/// * `count` the length of the `string` and `length` arrays.
/// * `string` an array of pointers to the start of shader source code strings.
/// * `length` if non-null, this is an array of lengths for each pointer in
///   `string` (or negative if that entry is null-termiated). if `length` is
///   null then *all* strings in `string` must each be null-termianted.
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glShaderSource_p.0.get() } {
    Some(glShaderSource_inner) => glShaderSource_inner(shader, count, string, length),
    None => gl_not_loaded("glShaderSource"),
  }
}
static glShaderSource_p: GlFnCell<glShaderSource_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glShaderSource_is_loaded() -> bool {
  unsafe { *glShaderSource_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glShaderSource_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glShaderSource_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glShaderSource_t>>(gl_ptr_filter(f(b"glShaderSource\0".as_ptr())));
}
/// glStencilFunc
/// * `func` group: StencilFunction
/// * `ref` group: StencilValue
/// * `mask` group: MaskedStencilValue
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glStencilFunc(func: StencilFunction, ref_: GLint, mask: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glStencilFunc_p.0.get() } {
    Some(glStencilFunc_inner) => glStencilFunc_inner(func, ref_, mask),
    None => gl_not_loaded("glStencilFunc"),
  }
}
static glStencilFunc_p: GlFnCell<glStencilFunc_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glStencilFunc_is_loaded() -> bool {
  unsafe { *glStencilFunc_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glStencilFunc_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glStencilFunc_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glStencilFunc_t>>(gl_ptr_filter(f(b"glStencilFunc\0".as_ptr())));
}
/// glStencilFuncSeparate
/// * `face` group: StencilFaceDirection
/// * `func` group: StencilFunction
/// * `ref` group: StencilValue
/// * `mask` group: MaskedStencilValue
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glStencilFuncSeparate(face: StencilFaceDirection, func: StencilFunction, ref_: GLint, mask: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glStencilFuncSeparate_p.0.get() } {
    Some(glStencilFuncSeparate_inner) => glStencilFuncSeparate_inner(face, func, ref_, mask),
    None => gl_not_loaded("glStencilFuncSeparate"),
  }
}
static glStencilFuncSeparate_p: GlFnCell<glStencilFuncSeparate_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glStencilFuncSeparate_is_loaded() -> bool {
  unsafe { *glStencilFuncSeparate_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glStencilFuncSeparate_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glStencilFuncSeparate_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glStencilFuncSeparate_t>>(gl_ptr_filter(f(b"glStencilFuncSeparate\0".as_ptr())));
}
/// glStencilMask
/// * `mask` group: MaskedStencilValue
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glStencilMask(mask: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glStencilMask_p.0.get() } {
    Some(glStencilMask_inner) => glStencilMask_inner(mask),
    None => gl_not_loaded("glStencilMask"),
  }
}
static glStencilMask_p: GlFnCell<glStencilMask_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glStencilMask_is_loaded() -> bool {
  unsafe { *glStencilMask_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glStencilMask_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glStencilMask_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glStencilMask_t>>(gl_ptr_filter(f(b"glStencilMask\0".as_ptr())));
}
/// glStencilMaskSeparate
/// * `face` group: StencilFaceDirection
/// * `mask` group: MaskedStencilValue
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glStencilMaskSeparate(face: StencilFaceDirection, mask: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glStencilMaskSeparate_p.0.get() } {
    Some(glStencilMaskSeparate_inner) => glStencilMaskSeparate_inner(face, mask),
    None => gl_not_loaded("glStencilMaskSeparate"),
  }
}
static glStencilMaskSeparate_p: GlFnCell<glStencilMaskSeparate_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glStencilMaskSeparate_is_loaded() -> bool {
  unsafe { *glStencilMaskSeparate_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glStencilMaskSeparate_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glStencilMaskSeparate_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glStencilMaskSeparate_t>>(gl_ptr_filter(f(b"glStencilMaskSeparate\0".as_ptr())));
}
/// glStencilOp
/// * `fail` group: StencilOp
/// * `zfail` group: StencilOp
/// * `zpass` group: StencilOp
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glStencilOp(fail: StencilOp, zfail: StencilOp, zpass: StencilOp) {
  #[allow(unused_unsafe)]
  match unsafe { *glStencilOp_p.0.get() } {
    Some(glStencilOp_inner) => glStencilOp_inner(fail, zfail, zpass),
    None => gl_not_loaded("glStencilOp"),
  }
}
static glStencilOp_p: GlFnCell<glStencilOp_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glStencilOp_is_loaded() -> bool {
  unsafe { *glStencilOp_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glStencilOp_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glStencilOp_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glStencilOp_t>>(gl_ptr_filter(f(b"glStencilOp\0".as_ptr())));
}
/// glStencilOpSeparate
/// * `face` group: StencilFaceDirection
/// * `sfail` group: StencilOp
/// * `dpfail` group: StencilOp
/// * `dppass` group: StencilOp
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glStencilOpSeparate(face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp) {
  #[allow(unused_unsafe)]
  match unsafe { *glStencilOpSeparate_p.0.get() } {
    Some(glStencilOpSeparate_inner) => glStencilOpSeparate_inner(face, sfail, dpfail, dppass),
    None => gl_not_loaded("glStencilOpSeparate"),
  }
}
static glStencilOpSeparate_p: GlFnCell<glStencilOpSeparate_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glStencilOpSeparate_is_loaded() -> bool {
  unsafe { *glStencilOpSeparate_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glStencilOpSeparate_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glStencilOpSeparate_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glStencilOpSeparate_t>>(gl_ptr_filter(f(b"glStencilOpSeparate\0".as_ptr())));
}
/// glTexBuffer
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `buffer` class: buffer
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexBuffer(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexBuffer_p.0.get() } {
    Some(glTexBuffer_inner) => glTexBuffer_inner(target, internalformat, buffer),
    None => gl_not_loaded("glTexBuffer"),
  }
}
static glTexBuffer_p: GlFnCell<glTexBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexBuffer_is_loaded() -> bool {
  unsafe { *glTexBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexBuffer_t>>(gl_ptr_filter(f(b"glTexBuffer\0".as_ptr())));
}
/// glTexImage1D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexImage1D(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexImage1D_p.0.get() } {
    Some(glTexImage1D_inner) => glTexImage1D_inner(target, level, internalformat, width, border, format, type_, pixels),
    None => gl_not_loaded("glTexImage1D"),
  }
}
static glTexImage1D_p: GlFnCell<glTexImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexImage1D_is_loaded() -> bool {
  unsafe { *glTexImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexImage1D_t>>(gl_ptr_filter(f(b"glTexImage1D\0".as_ptr())));
}
/// glTexImage2D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexImage2D(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexImage2D_p.0.get() } {
    Some(glTexImage2D_inner) => glTexImage2D_inner(target, level, internalformat, width, height, border, format, type_, pixels),
    None => gl_not_loaded("glTexImage2D"),
  }
}
static glTexImage2D_p: GlFnCell<glTexImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexImage2D_is_loaded() -> bool {
  unsafe { *glTexImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexImage2D_t>>(gl_ptr_filter(f(b"glTexImage2D\0".as_ptr())));
}
/// glTexImage2DMultisample
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexImage2DMultisample(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexImage2DMultisample_p.0.get() } {
    Some(glTexImage2DMultisample_inner) => glTexImage2DMultisample_inner(target, samples, internalformat, width, height, fixedsamplelocations),
    None => gl_not_loaded("glTexImage2DMultisample"),
  }
}
static glTexImage2DMultisample_p: GlFnCell<glTexImage2DMultisample_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexImage2DMultisample_is_loaded() -> bool {
  unsafe { *glTexImage2DMultisample_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexImage2DMultisample_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexImage2DMultisample_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexImage2DMultisample_t>>(gl_ptr_filter(f(b"glTexImage2DMultisample\0".as_ptr())));
}
/// glTexImage3D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `internalformat` group: InternalFormat
/// * `border` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexImage3D(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexImage3D_p.0.get() } {
    Some(glTexImage3D_inner) => glTexImage3D_inner(target, level, internalformat, width, height, depth, border, format, type_, pixels),
    None => gl_not_loaded("glTexImage3D"),
  }
}
static glTexImage3D_p: GlFnCell<glTexImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexImage3D_is_loaded() -> bool {
  unsafe { *glTexImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexImage3D_t>>(gl_ptr_filter(f(b"glTexImage3D\0".as_ptr())));
}
/// glTexImage3DMultisample
/// * `target` group: TextureTarget
/// * `internalformat` group: InternalFormat
/// * `fixedsamplelocations` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexImage3DMultisample(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexImage3DMultisample_p.0.get() } {
    Some(glTexImage3DMultisample_inner) => glTexImage3DMultisample_inner(target, samples, internalformat, width, height, depth, fixedsamplelocations),
    None => gl_not_loaded("glTexImage3DMultisample"),
  }
}
static glTexImage3DMultisample_p: GlFnCell<glTexImage3DMultisample_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexImage3DMultisample_is_loaded() -> bool {
  unsafe { *glTexImage3DMultisample_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexImage3DMultisample_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexImage3DMultisample_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexImage3DMultisample_t>>(gl_ptr_filter(f(b"glTexImage3DMultisample\0".as_ptr())));
}
/// glTexParameterIiv
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexParameterIiv(target: TextureTarget, pname: TextureParameterName, params: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexParameterIiv_p.0.get() } {
    Some(glTexParameterIiv_inner) => glTexParameterIiv_inner(target, pname, params),
    None => gl_not_loaded("glTexParameterIiv"),
  }
}
static glTexParameterIiv_p: GlFnCell<glTexParameterIiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexParameterIiv_is_loaded() -> bool {
  unsafe { *glTexParameterIiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexParameterIiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexParameterIiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexParameterIiv_t>>(gl_ptr_filter(f(b"glTexParameterIiv\0".as_ptr())));
}
/// glTexParameterIuiv
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexParameterIuiv(target: TextureTarget, pname: TextureParameterName, params: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexParameterIuiv_p.0.get() } {
    Some(glTexParameterIuiv_inner) => glTexParameterIuiv_inner(target, pname, params),
    None => gl_not_loaded("glTexParameterIuiv"),
  }
}
static glTexParameterIuiv_p: GlFnCell<glTexParameterIuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexParameterIuiv_is_loaded() -> bool {
  unsafe { *glTexParameterIuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexParameterIuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexParameterIuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexParameterIuiv_t>>(gl_ptr_filter(f(b"glTexParameterIuiv\0".as_ptr())));
}
/// glTexParameterf
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedFloat32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexParameterf(target: TextureTarget, pname: TextureParameterName, param: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexParameterf_p.0.get() } {
    Some(glTexParameterf_inner) => glTexParameterf_inner(target, pname, param),
    None => gl_not_loaded("glTexParameterf"),
  }
}
static glTexParameterf_p: GlFnCell<glTexParameterf_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexParameterf_is_loaded() -> bool {
  unsafe { *glTexParameterf_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexParameterf_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexParameterf_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexParameterf_t>>(gl_ptr_filter(f(b"glTexParameterf\0".as_ptr())));
}
/// glTexParameterfv
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedFloat32
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexParameterfv(target: TextureTarget, pname: TextureParameterName, params: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexParameterfv_p.0.get() } {
    Some(glTexParameterfv_inner) => glTexParameterfv_inner(target, pname, params),
    None => gl_not_loaded("glTexParameterfv"),
  }
}
static glTexParameterfv_p: GlFnCell<glTexParameterfv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexParameterfv_is_loaded() -> bool {
  unsafe { *glTexParameterfv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexParameterfv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexParameterfv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexParameterfv_t>>(gl_ptr_filter(f(b"glTexParameterfv\0".as_ptr())));
}
/// glTexParameteri
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `param` group: CheckedInt32
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexParameteri(target: TextureTarget, pname: TextureParameterName, param: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexParameteri_p.0.get() } {
    Some(glTexParameteri_inner) => glTexParameteri_inner(target, pname, param),
    None => gl_not_loaded("glTexParameteri"),
  }
}
static glTexParameteri_p: GlFnCell<glTexParameteri_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexParameteri_is_loaded() -> bool {
  unsafe { *glTexParameteri_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexParameteri_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexParameteri_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexParameteri_t>>(gl_ptr_filter(f(b"glTexParameteri\0".as_ptr())));
}
/// glTexParameteriv
/// * `target` group: TextureTarget
/// * `pname` group: TextureParameterName
/// * `params` group: CheckedInt32
/// * `params` len: COMPSIZE(pname)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexParameteriv(target: TextureTarget, pname: TextureParameterName, params: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexParameteriv_p.0.get() } {
    Some(glTexParameteriv_inner) => glTexParameteriv_inner(target, pname, params),
    None => gl_not_loaded("glTexParameteriv"),
  }
}
static glTexParameteriv_p: GlFnCell<glTexParameteriv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexParameteriv_is_loaded() -> bool {
  unsafe { *glTexParameteriv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexParameteriv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexParameteriv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexParameteriv_t>>(gl_ptr_filter(f(b"glTexParameteriv\0".as_ptr())));
}
/// glTexSubImage1D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexSubImage1D(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexSubImage1D_p.0.get() } {
    Some(glTexSubImage1D_inner) => glTexSubImage1D_inner(target, level, xoffset, width, format, type_, pixels),
    None => gl_not_loaded("glTexSubImage1D"),
  }
}
static glTexSubImage1D_p: GlFnCell<glTexSubImage1D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexSubImage1D_is_loaded() -> bool {
  unsafe { *glTexSubImage1D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexSubImage1D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexSubImage1D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexSubImage1D_t>>(gl_ptr_filter(f(b"glTexSubImage1D\0".as_ptr())));
}
/// glTexSubImage2D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexSubImage2D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexSubImage2D_p.0.get() } {
    Some(glTexSubImage2D_inner) => glTexSubImage2D_inner(target, level, xoffset, yoffset, width, height, format, type_, pixels),
    None => gl_not_loaded("glTexSubImage2D"),
  }
}
static glTexSubImage2D_p: GlFnCell<glTexSubImage2D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexSubImage2D_is_loaded() -> bool {
  unsafe { *glTexSubImage2D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexSubImage2D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexSubImage2D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexSubImage2D_t>>(gl_ptr_filter(f(b"glTexSubImage2D\0".as_ptr())));
}
/// glTexSubImage3D
/// * `target` group: TextureTarget
/// * `level` group: CheckedInt32
/// * `xoffset` group: CheckedInt32
/// * `yoffset` group: CheckedInt32
/// * `zoffset` group: CheckedInt32
/// * `format` group: PixelFormat
/// * `type` group: PixelType
/// * `pixels` len: COMPSIZE(format,type,width,height,depth)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTexSubImage3D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glTexSubImage3D_p.0.get() } {
    Some(glTexSubImage3D_inner) => glTexSubImage3D_inner(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels),
    None => gl_not_loaded("glTexSubImage3D"),
  }
}
static glTexSubImage3D_p: GlFnCell<glTexSubImage3D_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTexSubImage3D_is_loaded() -> bool {
  unsafe { *glTexSubImage3D_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTexSubImage3D_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTexSubImage3D_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTexSubImage3D_t>>(gl_ptr_filter(f(b"glTexSubImage3D\0".as_ptr())));
}
/// glTransformFeedbackVaryings
/// * `program` class: program
/// * `varyings` len: count
/// * `bufferMode` group: TransformFeedbackBufferMode
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glTransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: TransformFeedbackBufferMode) {
  #[allow(unused_unsafe)]
  match unsafe { *glTransformFeedbackVaryings_p.0.get() } {
    Some(glTransformFeedbackVaryings_inner) => glTransformFeedbackVaryings_inner(program, count, varyings, bufferMode),
    None => gl_not_loaded("glTransformFeedbackVaryings"),
  }
}
static glTransformFeedbackVaryings_p: GlFnCell<glTransformFeedbackVaryings_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glTransformFeedbackVaryings_is_loaded() -> bool {
  unsafe { *glTransformFeedbackVaryings_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glTransformFeedbackVaryings_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glTransformFeedbackVaryings_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glTransformFeedbackVaryings_t>>(gl_ptr_filter(f(b"glTransformFeedbackVaryings\0".as_ptr())));
}
/// glUniform1f
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1f(location: GLint, v0: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1f_p.0.get() } {
    Some(glUniform1f_inner) => glUniform1f_inner(location, v0),
    None => gl_not_loaded("glUniform1f"),
  }
}
static glUniform1f_p: GlFnCell<glUniform1f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1f_is_loaded() -> bool {
  unsafe { *glUniform1f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1f_t>>(gl_ptr_filter(f(b"glUniform1f\0".as_ptr())));
}
/// glUniform1fv
/// * `value` len: count*1
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1fv_p.0.get() } {
    Some(glUniform1fv_inner) => glUniform1fv_inner(location, count, value),
    None => gl_not_loaded("glUniform1fv"),
  }
}
static glUniform1fv_p: GlFnCell<glUniform1fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1fv_is_loaded() -> bool {
  unsafe { *glUniform1fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1fv_t>>(gl_ptr_filter(f(b"glUniform1fv\0".as_ptr())));
}
/// glUniform1i
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1i(location: GLint, v0: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1i_p.0.get() } {
    Some(glUniform1i_inner) => glUniform1i_inner(location, v0),
    None => gl_not_loaded("glUniform1i"),
  }
}
static glUniform1i_p: GlFnCell<glUniform1i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1i_is_loaded() -> bool {
  unsafe { *glUniform1i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1i_t>>(gl_ptr_filter(f(b"glUniform1i\0".as_ptr())));
}
/// glUniform1iv
/// * `value` len: count*1
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1iv(location: GLint, count: GLsizei, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1iv_p.0.get() } {
    Some(glUniform1iv_inner) => glUniform1iv_inner(location, count, value),
    None => gl_not_loaded("glUniform1iv"),
  }
}
static glUniform1iv_p: GlFnCell<glUniform1iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1iv_is_loaded() -> bool {
  unsafe { *glUniform1iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1iv_t>>(gl_ptr_filter(f(b"glUniform1iv\0".as_ptr())));
}
/// glUniform1ui
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1ui(location: GLint, v0: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1ui_p.0.get() } {
    Some(glUniform1ui_inner) => glUniform1ui_inner(location, v0),
    None => gl_not_loaded("glUniform1ui"),
  }
}
static glUniform1ui_p: GlFnCell<glUniform1ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1ui_is_loaded() -> bool {
  unsafe { *glUniform1ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1ui_t>>(gl_ptr_filter(f(b"glUniform1ui\0".as_ptr())));
}
/// glUniform1uiv
/// * `value` len: count*1
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform1uiv_p.0.get() } {
    Some(glUniform1uiv_inner) => glUniform1uiv_inner(location, count, value),
    None => gl_not_loaded("glUniform1uiv"),
  }
}
static glUniform1uiv_p: GlFnCell<glUniform1uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform1uiv_is_loaded() -> bool {
  unsafe { *glUniform1uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform1uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform1uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform1uiv_t>>(gl_ptr_filter(f(b"glUniform1uiv\0".as_ptr())));
}
/// glUniform2f
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2f_p.0.get() } {
    Some(glUniform2f_inner) => glUniform2f_inner(location, v0, v1),
    None => gl_not_loaded("glUniform2f"),
  }
}
static glUniform2f_p: GlFnCell<glUniform2f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2f_is_loaded() -> bool {
  unsafe { *glUniform2f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2f_t>>(gl_ptr_filter(f(b"glUniform2f\0".as_ptr())));
}
/// glUniform2fv
/// * `value` len: count*2
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2fv_p.0.get() } {
    Some(glUniform2fv_inner) => glUniform2fv_inner(location, count, value),
    None => gl_not_loaded("glUniform2fv"),
  }
}
static glUniform2fv_p: GlFnCell<glUniform2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2fv_is_loaded() -> bool {
  unsafe { *glUniform2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2fv_t>>(gl_ptr_filter(f(b"glUniform2fv\0".as_ptr())));
}
/// glUniform2i
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2i(location: GLint, v0: GLint, v1: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2i_p.0.get() } {
    Some(glUniform2i_inner) => glUniform2i_inner(location, v0, v1),
    None => gl_not_loaded("glUniform2i"),
  }
}
static glUniform2i_p: GlFnCell<glUniform2i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2i_is_loaded() -> bool {
  unsafe { *glUniform2i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2i_t>>(gl_ptr_filter(f(b"glUniform2i\0".as_ptr())));
}
/// glUniform2iv
/// * `value` len: count*2
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2iv(location: GLint, count: GLsizei, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2iv_p.0.get() } {
    Some(glUniform2iv_inner) => glUniform2iv_inner(location, count, value),
    None => gl_not_loaded("glUniform2iv"),
  }
}
static glUniform2iv_p: GlFnCell<glUniform2iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2iv_is_loaded() -> bool {
  unsafe { *glUniform2iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2iv_t>>(gl_ptr_filter(f(b"glUniform2iv\0".as_ptr())));
}
/// glUniform2ui
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2ui(location: GLint, v0: GLuint, v1: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2ui_p.0.get() } {
    Some(glUniform2ui_inner) => glUniform2ui_inner(location, v0, v1),
    None => gl_not_loaded("glUniform2ui"),
  }
}
static glUniform2ui_p: GlFnCell<glUniform2ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2ui_is_loaded() -> bool {
  unsafe { *glUniform2ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2ui_t>>(gl_ptr_filter(f(b"glUniform2ui\0".as_ptr())));
}
/// glUniform2uiv
/// * `value` len: count*2
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform2uiv_p.0.get() } {
    Some(glUniform2uiv_inner) => glUniform2uiv_inner(location, count, value),
    None => gl_not_loaded("glUniform2uiv"),
  }
}
static glUniform2uiv_p: GlFnCell<glUniform2uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform2uiv_is_loaded() -> bool {
  unsafe { *glUniform2uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform2uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform2uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform2uiv_t>>(gl_ptr_filter(f(b"glUniform2uiv\0".as_ptr())));
}
/// glUniform3f
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3f_p.0.get() } {
    Some(glUniform3f_inner) => glUniform3f_inner(location, v0, v1, v2),
    None => gl_not_loaded("glUniform3f"),
  }
}
static glUniform3f_p: GlFnCell<glUniform3f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3f_is_loaded() -> bool {
  unsafe { *glUniform3f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3f_t>>(gl_ptr_filter(f(b"glUniform3f\0".as_ptr())));
}
/// glUniform3fv
/// * `value` len: count*3
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3fv_p.0.get() } {
    Some(glUniform3fv_inner) => glUniform3fv_inner(location, count, value),
    None => gl_not_loaded("glUniform3fv"),
  }
}
static glUniform3fv_p: GlFnCell<glUniform3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3fv_is_loaded() -> bool {
  unsafe { *glUniform3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3fv_t>>(gl_ptr_filter(f(b"glUniform3fv\0".as_ptr())));
}
/// glUniform3i
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3i_p.0.get() } {
    Some(glUniform3i_inner) => glUniform3i_inner(location, v0, v1, v2),
    None => gl_not_loaded("glUniform3i"),
  }
}
static glUniform3i_p: GlFnCell<glUniform3i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3i_is_loaded() -> bool {
  unsafe { *glUniform3i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3i_t>>(gl_ptr_filter(f(b"glUniform3i\0".as_ptr())));
}
/// glUniform3iv
/// * `value` len: count*3
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3iv(location: GLint, count: GLsizei, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3iv_p.0.get() } {
    Some(glUniform3iv_inner) => glUniform3iv_inner(location, count, value),
    None => gl_not_loaded("glUniform3iv"),
  }
}
static glUniform3iv_p: GlFnCell<glUniform3iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3iv_is_loaded() -> bool {
  unsafe { *glUniform3iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3iv_t>>(gl_ptr_filter(f(b"glUniform3iv\0".as_ptr())));
}
/// glUniform3ui
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3ui_p.0.get() } {
    Some(glUniform3ui_inner) => glUniform3ui_inner(location, v0, v1, v2),
    None => gl_not_loaded("glUniform3ui"),
  }
}
static glUniform3ui_p: GlFnCell<glUniform3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3ui_is_loaded() -> bool {
  unsafe { *glUniform3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3ui_t>>(gl_ptr_filter(f(b"glUniform3ui\0".as_ptr())));
}
/// glUniform3uiv
/// * `value` len: count*3
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform3uiv_p.0.get() } {
    Some(glUniform3uiv_inner) => glUniform3uiv_inner(location, count, value),
    None => gl_not_loaded("glUniform3uiv"),
  }
}
static glUniform3uiv_p: GlFnCell<glUniform3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform3uiv_is_loaded() -> bool {
  unsafe { *glUniform3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform3uiv_t>>(gl_ptr_filter(f(b"glUniform3uiv\0".as_ptr())));
}
/// glUniform4f
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4f_p.0.get() } {
    Some(glUniform4f_inner) => glUniform4f_inner(location, v0, v1, v2, v3),
    None => gl_not_loaded("glUniform4f"),
  }
}
static glUniform4f_p: GlFnCell<glUniform4f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4f_is_loaded() -> bool {
  unsafe { *glUniform4f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4f_t>>(gl_ptr_filter(f(b"glUniform4f\0".as_ptr())));
}
/// glUniform4fv
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4fv_p.0.get() } {
    Some(glUniform4fv_inner) => glUniform4fv_inner(location, count, value),
    None => gl_not_loaded("glUniform4fv"),
  }
}
static glUniform4fv_p: GlFnCell<glUniform4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4fv_is_loaded() -> bool {
  unsafe { *glUniform4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4fv_t>>(gl_ptr_filter(f(b"glUniform4fv\0".as_ptr())));
}
/// glUniform4i
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4i_p.0.get() } {
    Some(glUniform4i_inner) => glUniform4i_inner(location, v0, v1, v2, v3),
    None => gl_not_loaded("glUniform4i"),
  }
}
static glUniform4i_p: GlFnCell<glUniform4i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4i_is_loaded() -> bool {
  unsafe { *glUniform4i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4i_t>>(gl_ptr_filter(f(b"glUniform4i\0".as_ptr())));
}
/// glUniform4iv
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4iv(location: GLint, count: GLsizei, value: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4iv_p.0.get() } {
    Some(glUniform4iv_inner) => glUniform4iv_inner(location, count, value),
    None => gl_not_loaded("glUniform4iv"),
  }
}
static glUniform4iv_p: GlFnCell<glUniform4iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4iv_is_loaded() -> bool {
  unsafe { *glUniform4iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4iv_t>>(gl_ptr_filter(f(b"glUniform4iv\0".as_ptr())));
}
/// glUniform4ui
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4ui_p.0.get() } {
    Some(glUniform4ui_inner) => glUniform4ui_inner(location, v0, v1, v2, v3),
    None => gl_not_loaded("glUniform4ui"),
  }
}
static glUniform4ui_p: GlFnCell<glUniform4ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4ui_is_loaded() -> bool {
  unsafe { *glUniform4ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4ui_t>>(gl_ptr_filter(f(b"glUniform4ui\0".as_ptr())));
}
/// glUniform4uiv
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniform4uiv_p.0.get() } {
    Some(glUniform4uiv_inner) => glUniform4uiv_inner(location, count, value),
    None => gl_not_loaded("glUniform4uiv"),
  }
}
static glUniform4uiv_p: GlFnCell<glUniform4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniform4uiv_is_loaded() -> bool {
  unsafe { *glUniform4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniform4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniform4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniform4uiv_t>>(gl_ptr_filter(f(b"glUniform4uiv\0".as_ptr())));
}
/// glUniformBlockBinding
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformBlockBinding_p.0.get() } {
    Some(glUniformBlockBinding_inner) => glUniformBlockBinding_inner(program, uniformBlockIndex, uniformBlockBinding),
    None => gl_not_loaded("glUniformBlockBinding"),
  }
}
static glUniformBlockBinding_p: GlFnCell<glUniformBlockBinding_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformBlockBinding_is_loaded() -> bool {
  unsafe { *glUniformBlockBinding_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformBlockBinding_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformBlockBinding_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformBlockBinding_t>>(gl_ptr_filter(f(b"glUniformBlockBinding\0".as_ptr())));
}
/// glUniformMatrix2fv
/// * `transpose` group: Boolean
/// * `value` len: count*4
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix2fv_p.0.get() } {
    Some(glUniformMatrix2fv_inner) => glUniformMatrix2fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix2fv"),
  }
}
static glUniformMatrix2fv_p: GlFnCell<glUniformMatrix2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix2fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix2fv_t>>(gl_ptr_filter(f(b"glUniformMatrix2fv\0".as_ptr())));
}
/// glUniformMatrix2x3fv
/// * `transpose` group: Boolean
/// * `value` len: count*6
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix2x3fv_p.0.get() } {
    Some(glUniformMatrix2x3fv_inner) => glUniformMatrix2x3fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix2x3fv"),
  }
}
static glUniformMatrix2x3fv_p: GlFnCell<glUniformMatrix2x3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix2x3fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix2x3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2x3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix2x3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix2x3fv_t>>(gl_ptr_filter(f(b"glUniformMatrix2x3fv\0".as_ptr())));
}
/// glUniformMatrix2x4fv
/// * `transpose` group: Boolean
/// * `value` len: count*8
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix2x4fv_p.0.get() } {
    Some(glUniformMatrix2x4fv_inner) => glUniformMatrix2x4fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix2x4fv"),
  }
}
static glUniformMatrix2x4fv_p: GlFnCell<glUniformMatrix2x4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix2x4fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix2x4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix2x4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix2x4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix2x4fv_t>>(gl_ptr_filter(f(b"glUniformMatrix2x4fv\0".as_ptr())));
}
/// glUniformMatrix3fv
/// * `transpose` group: Boolean
/// * `value` len: count*9
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix3fv_p.0.get() } {
    Some(glUniformMatrix3fv_inner) => glUniformMatrix3fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix3fv"),
  }
}
static glUniformMatrix3fv_p: GlFnCell<glUniformMatrix3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix3fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix3fv_t>>(gl_ptr_filter(f(b"glUniformMatrix3fv\0".as_ptr())));
}
/// glUniformMatrix3x2fv
/// * `transpose` group: Boolean
/// * `value` len: count*6
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix3x2fv_p.0.get() } {
    Some(glUniformMatrix3x2fv_inner) => glUniformMatrix3x2fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix3x2fv"),
  }
}
static glUniformMatrix3x2fv_p: GlFnCell<glUniformMatrix3x2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix3x2fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix3x2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3x2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix3x2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix3x2fv_t>>(gl_ptr_filter(f(b"glUniformMatrix3x2fv\0".as_ptr())));
}
/// glUniformMatrix3x4fv
/// * `transpose` group: Boolean
/// * `value` len: count*12
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix3x4fv_p.0.get() } {
    Some(glUniformMatrix3x4fv_inner) => glUniformMatrix3x4fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix3x4fv"),
  }
}
static glUniformMatrix3x4fv_p: GlFnCell<glUniformMatrix3x4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix3x4fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix3x4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix3x4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix3x4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix3x4fv_t>>(gl_ptr_filter(f(b"glUniformMatrix3x4fv\0".as_ptr())));
}
/// glUniformMatrix4fv
/// * `transpose` group: Boolean
/// * `value` len: count*16
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix4fv_p.0.get() } {
    Some(glUniformMatrix4fv_inner) => glUniformMatrix4fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix4fv"),
  }
}
static glUniformMatrix4fv_p: GlFnCell<glUniformMatrix4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix4fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix4fv_t>>(gl_ptr_filter(f(b"glUniformMatrix4fv\0".as_ptr())));
}
/// glUniformMatrix4x2fv
/// * `transpose` group: Boolean
/// * `value` len: count*8
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix4x2fv_p.0.get() } {
    Some(glUniformMatrix4x2fv_inner) => glUniformMatrix4x2fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix4x2fv"),
  }
}
static glUniformMatrix4x2fv_p: GlFnCell<glUniformMatrix4x2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix4x2fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix4x2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4x2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix4x2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix4x2fv_t>>(gl_ptr_filter(f(b"glUniformMatrix4x2fv\0".as_ptr())));
}
/// glUniformMatrix4x3fv
/// * `transpose` group: Boolean
/// * `value` len: count*12
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glUniformMatrix4x3fv_p.0.get() } {
    Some(glUniformMatrix4x3fv_inner) => glUniformMatrix4x3fv_inner(location, count, transpose, value),
    None => gl_not_loaded("glUniformMatrix4x3fv"),
  }
}
static glUniformMatrix4x3fv_p: GlFnCell<glUniformMatrix4x3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUniformMatrix4x3fv_is_loaded() -> bool {
  unsafe { *glUniformMatrix4x3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUniformMatrix4x3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUniformMatrix4x3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUniformMatrix4x3fv_t>>(gl_ptr_filter(f(b"glUniformMatrix4x3fv\0".as_ptr())));
}
/// glUnmapBuffer
/// * `target` group: BufferTargetARB
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glUnmapBuffer(target: BufferTargetARB) -> GLboolean {
  #[allow(unused_unsafe)]
  match unsafe { *glUnmapBuffer_p.0.get() } {
    Some(glUnmapBuffer_inner) => glUnmapBuffer_inner(target),
    None => gl_not_loaded("glUnmapBuffer"),
  }
}
static glUnmapBuffer_p: GlFnCell<glUnmapBuffer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUnmapBuffer_is_loaded() -> bool {
  unsafe { *glUnmapBuffer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUnmapBuffer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUnmapBuffer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUnmapBuffer_t>>(gl_ptr_filter(f(b"glUnmapBuffer\0".as_ptr())));
}
/// [glUseProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgram.xhtml)
///
/// Sets a given shader program for use during rendering.
///
/// Setting 0 as the program object makes the output of all rendering actions
/// undefined, but this is not an error.
///
/// * `program` names the program to set for use.
#[cfg_attr(feature = "track_caller", track_caller)]
pub fn glUseProgram(program: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glUseProgram_p.0.get() } {
    Some(glUseProgram_inner) => glUseProgram_inner(program),
    None => gl_not_loaded("glUseProgram"),
  }
}
static glUseProgram_p: GlFnCell<glUseProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glUseProgram_is_loaded() -> bool {
  unsafe { *glUseProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glUseProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glUseProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glUseProgram_t>>(gl_ptr_filter(f(b"glUseProgram\0".as_ptr())));
}
/// glValidateProgram
/// * `program` class: program
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glValidateProgram(program: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glValidateProgram_p.0.get() } {
    Some(glValidateProgram_inner) => glValidateProgram_inner(program),
    None => gl_not_loaded("glValidateProgram"),
  }
}
static glValidateProgram_p: GlFnCell<glValidateProgram_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glValidateProgram_is_loaded() -> bool {
  unsafe { *glValidateProgram_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glValidateProgram_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glValidateProgram_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glValidateProgram_t>>(gl_ptr_filter(f(b"glValidateProgram\0".as_ptr())));
}
/// glVertexAttrib1d
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib1d(index: GLuint, x: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib1d_p.0.get() } {
    Some(glVertexAttrib1d_inner) => glVertexAttrib1d_inner(index, x),
    None => gl_not_loaded("glVertexAttrib1d"),
  }
}
static glVertexAttrib1d_p: GlFnCell<glVertexAttrib1d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib1d_is_loaded() -> bool {
  unsafe { *glVertexAttrib1d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib1d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib1d_t>>(gl_ptr_filter(f(b"glVertexAttrib1d\0".as_ptr())));
}
/// glVertexAttrib1dv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib1dv(index: GLuint, v: *const GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib1dv_p.0.get() } {
    Some(glVertexAttrib1dv_inner) => glVertexAttrib1dv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib1dv"),
  }
}
static glVertexAttrib1dv_p: GlFnCell<glVertexAttrib1dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib1dv_is_loaded() -> bool {
  unsafe { *glVertexAttrib1dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib1dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib1dv_t>>(gl_ptr_filter(f(b"glVertexAttrib1dv\0".as_ptr())));
}
/// glVertexAttrib1f
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib1f(index: GLuint, x: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib1f_p.0.get() } {
    Some(glVertexAttrib1f_inner) => glVertexAttrib1f_inner(index, x),
    None => gl_not_loaded("glVertexAttrib1f"),
  }
}
static glVertexAttrib1f_p: GlFnCell<glVertexAttrib1f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib1f_is_loaded() -> bool {
  unsafe { *glVertexAttrib1f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib1f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib1f_t>>(gl_ptr_filter(f(b"glVertexAttrib1f\0".as_ptr())));
}
/// glVertexAttrib1fv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib1fv(index: GLuint, v: *const GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib1fv_p.0.get() } {
    Some(glVertexAttrib1fv_inner) => glVertexAttrib1fv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib1fv"),
  }
}
static glVertexAttrib1fv_p: GlFnCell<glVertexAttrib1fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib1fv_is_loaded() -> bool {
  unsafe { *glVertexAttrib1fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib1fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib1fv_t>>(gl_ptr_filter(f(b"glVertexAttrib1fv\0".as_ptr())));
}
/// glVertexAttrib1s
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib1s(index: GLuint, x: GLshort) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib1s_p.0.get() } {
    Some(glVertexAttrib1s_inner) => glVertexAttrib1s_inner(index, x),
    None => gl_not_loaded("glVertexAttrib1s"),
  }
}
static glVertexAttrib1s_p: GlFnCell<glVertexAttrib1s_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib1s_is_loaded() -> bool {
  unsafe { *glVertexAttrib1s_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1s_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib1s_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib1s_t>>(gl_ptr_filter(f(b"glVertexAttrib1s\0".as_ptr())));
}
/// glVertexAttrib1sv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib1sv(index: GLuint, v: *const GLshort) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib1sv_p.0.get() } {
    Some(glVertexAttrib1sv_inner) => glVertexAttrib1sv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib1sv"),
  }
}
static glVertexAttrib1sv_p: GlFnCell<glVertexAttrib1sv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib1sv_is_loaded() -> bool {
  unsafe { *glVertexAttrib1sv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib1sv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib1sv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib1sv_t>>(gl_ptr_filter(f(b"glVertexAttrib1sv\0".as_ptr())));
}
/// glVertexAttrib2d
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib2d_p.0.get() } {
    Some(glVertexAttrib2d_inner) => glVertexAttrib2d_inner(index, x, y),
    None => gl_not_loaded("glVertexAttrib2d"),
  }
}
static glVertexAttrib2d_p: GlFnCell<glVertexAttrib2d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib2d_is_loaded() -> bool {
  unsafe { *glVertexAttrib2d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib2d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib2d_t>>(gl_ptr_filter(f(b"glVertexAttrib2d\0".as_ptr())));
}
/// glVertexAttrib2dv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib2dv(index: GLuint, v: *const [GLdouble; 2]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib2dv_p.0.get() } {
    Some(glVertexAttrib2dv_inner) => glVertexAttrib2dv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib2dv"),
  }
}
static glVertexAttrib2dv_p: GlFnCell<glVertexAttrib2dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib2dv_is_loaded() -> bool {
  unsafe { *glVertexAttrib2dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib2dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib2dv_t>>(gl_ptr_filter(f(b"glVertexAttrib2dv\0".as_ptr())));
}
/// glVertexAttrib2f
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib2f_p.0.get() } {
    Some(glVertexAttrib2f_inner) => glVertexAttrib2f_inner(index, x, y),
    None => gl_not_loaded("glVertexAttrib2f"),
  }
}
static glVertexAttrib2f_p: GlFnCell<glVertexAttrib2f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib2f_is_loaded() -> bool {
  unsafe { *glVertexAttrib2f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib2f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib2f_t>>(gl_ptr_filter(f(b"glVertexAttrib2f\0".as_ptr())));
}
/// glVertexAttrib2fv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib2fv(index: GLuint, v: *const [GLfloat; 2]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib2fv_p.0.get() } {
    Some(glVertexAttrib2fv_inner) => glVertexAttrib2fv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib2fv"),
  }
}
static glVertexAttrib2fv_p: GlFnCell<glVertexAttrib2fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib2fv_is_loaded() -> bool {
  unsafe { *glVertexAttrib2fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib2fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib2fv_t>>(gl_ptr_filter(f(b"glVertexAttrib2fv\0".as_ptr())));
}
/// glVertexAttrib2s
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib2s_p.0.get() } {
    Some(glVertexAttrib2s_inner) => glVertexAttrib2s_inner(index, x, y),
    None => gl_not_loaded("glVertexAttrib2s"),
  }
}
static glVertexAttrib2s_p: GlFnCell<glVertexAttrib2s_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib2s_is_loaded() -> bool {
  unsafe { *glVertexAttrib2s_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2s_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib2s_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib2s_t>>(gl_ptr_filter(f(b"glVertexAttrib2s\0".as_ptr())));
}
/// glVertexAttrib2sv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib2sv(index: GLuint, v: *const [GLshort; 2]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib2sv_p.0.get() } {
    Some(glVertexAttrib2sv_inner) => glVertexAttrib2sv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib2sv"),
  }
}
static glVertexAttrib2sv_p: GlFnCell<glVertexAttrib2sv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib2sv_is_loaded() -> bool {
  unsafe { *glVertexAttrib2sv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib2sv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib2sv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib2sv_t>>(gl_ptr_filter(f(b"glVertexAttrib2sv\0".as_ptr())));
}
/// glVertexAttrib3d
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib3d_p.0.get() } {
    Some(glVertexAttrib3d_inner) => glVertexAttrib3d_inner(index, x, y, z),
    None => gl_not_loaded("glVertexAttrib3d"),
  }
}
static glVertexAttrib3d_p: GlFnCell<glVertexAttrib3d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib3d_is_loaded() -> bool {
  unsafe { *glVertexAttrib3d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib3d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib3d_t>>(gl_ptr_filter(f(b"glVertexAttrib3d\0".as_ptr())));
}
/// glVertexAttrib3dv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib3dv(index: GLuint, v: *const [GLdouble; 3]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib3dv_p.0.get() } {
    Some(glVertexAttrib3dv_inner) => glVertexAttrib3dv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib3dv"),
  }
}
static glVertexAttrib3dv_p: GlFnCell<glVertexAttrib3dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib3dv_is_loaded() -> bool {
  unsafe { *glVertexAttrib3dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib3dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib3dv_t>>(gl_ptr_filter(f(b"glVertexAttrib3dv\0".as_ptr())));
}
/// glVertexAttrib3f
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib3f_p.0.get() } {
    Some(glVertexAttrib3f_inner) => glVertexAttrib3f_inner(index, x, y, z),
    None => gl_not_loaded("glVertexAttrib3f"),
  }
}
static glVertexAttrib3f_p: GlFnCell<glVertexAttrib3f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib3f_is_loaded() -> bool {
  unsafe { *glVertexAttrib3f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib3f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib3f_t>>(gl_ptr_filter(f(b"glVertexAttrib3f\0".as_ptr())));
}
/// glVertexAttrib3fv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib3fv(index: GLuint, v: *const [GLfloat; 3]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib3fv_p.0.get() } {
    Some(glVertexAttrib3fv_inner) => glVertexAttrib3fv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib3fv"),
  }
}
static glVertexAttrib3fv_p: GlFnCell<glVertexAttrib3fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib3fv_is_loaded() -> bool {
  unsafe { *glVertexAttrib3fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib3fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib3fv_t>>(gl_ptr_filter(f(b"glVertexAttrib3fv\0".as_ptr())));
}
/// glVertexAttrib3s
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib3s_p.0.get() } {
    Some(glVertexAttrib3s_inner) => glVertexAttrib3s_inner(index, x, y, z),
    None => gl_not_loaded("glVertexAttrib3s"),
  }
}
static glVertexAttrib3s_p: GlFnCell<glVertexAttrib3s_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib3s_is_loaded() -> bool {
  unsafe { *glVertexAttrib3s_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3s_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib3s_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib3s_t>>(gl_ptr_filter(f(b"glVertexAttrib3s\0".as_ptr())));
}
/// glVertexAttrib3sv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib3sv(index: GLuint, v: *const [GLshort; 3]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib3sv_p.0.get() } {
    Some(glVertexAttrib3sv_inner) => glVertexAttrib3sv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib3sv"),
  }
}
static glVertexAttrib3sv_p: GlFnCell<glVertexAttrib3sv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib3sv_is_loaded() -> bool {
  unsafe { *glVertexAttrib3sv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib3sv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib3sv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib3sv_t>>(gl_ptr_filter(f(b"glVertexAttrib3sv\0".as_ptr())));
}
/// glVertexAttrib4Nbv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Nbv(index: GLuint, v: *const [GLbyte; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Nbv_p.0.get() } {
    Some(glVertexAttrib4Nbv_inner) => glVertexAttrib4Nbv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4Nbv"),
  }
}
static glVertexAttrib4Nbv_p: GlFnCell<glVertexAttrib4Nbv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Nbv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Nbv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nbv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Nbv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Nbv_t>>(gl_ptr_filter(f(b"glVertexAttrib4Nbv\0".as_ptr())));
}
/// glVertexAttrib4Niv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Niv(index: GLuint, v: *const [GLint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Niv_p.0.get() } {
    Some(glVertexAttrib4Niv_inner) => glVertexAttrib4Niv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4Niv"),
  }
}
static glVertexAttrib4Niv_p: GlFnCell<glVertexAttrib4Niv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Niv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Niv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Niv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Niv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Niv_t>>(gl_ptr_filter(f(b"glVertexAttrib4Niv\0".as_ptr())));
}
/// glVertexAttrib4Nsv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Nsv(index: GLuint, v: *const [GLshort; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Nsv_p.0.get() } {
    Some(glVertexAttrib4Nsv_inner) => glVertexAttrib4Nsv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4Nsv"),
  }
}
static glVertexAttrib4Nsv_p: GlFnCell<glVertexAttrib4Nsv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Nsv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Nsv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nsv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Nsv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Nsv_t>>(gl_ptr_filter(f(b"glVertexAttrib4Nsv\0".as_ptr())));
}
/// glVertexAttrib4Nub
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Nub_p.0.get() } {
    Some(glVertexAttrib4Nub_inner) => glVertexAttrib4Nub_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttrib4Nub"),
  }
}
static glVertexAttrib4Nub_p: GlFnCell<glVertexAttrib4Nub_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Nub_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Nub_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nub_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Nub_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Nub_t>>(gl_ptr_filter(f(b"glVertexAttrib4Nub\0".as_ptr())));
}
/// glVertexAttrib4Nubv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Nubv(index: GLuint, v: *const [GLubyte; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Nubv_p.0.get() } {
    Some(glVertexAttrib4Nubv_inner) => glVertexAttrib4Nubv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4Nubv"),
  }
}
static glVertexAttrib4Nubv_p: GlFnCell<glVertexAttrib4Nubv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Nubv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Nubv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nubv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Nubv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Nubv_t>>(gl_ptr_filter(f(b"glVertexAttrib4Nubv\0".as_ptr())));
}
/// glVertexAttrib4Nuiv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Nuiv(index: GLuint, v: *const [GLuint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Nuiv_p.0.get() } {
    Some(glVertexAttrib4Nuiv_inner) => glVertexAttrib4Nuiv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4Nuiv"),
  }
}
static glVertexAttrib4Nuiv_p: GlFnCell<glVertexAttrib4Nuiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Nuiv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Nuiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nuiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Nuiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Nuiv_t>>(gl_ptr_filter(f(b"glVertexAttrib4Nuiv\0".as_ptr())));
}
/// glVertexAttrib4Nusv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4Nusv(index: GLuint, v: *const [GLushort; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4Nusv_p.0.get() } {
    Some(glVertexAttrib4Nusv_inner) => glVertexAttrib4Nusv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4Nusv"),
  }
}
static glVertexAttrib4Nusv_p: GlFnCell<glVertexAttrib4Nusv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4Nusv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4Nusv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4Nusv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4Nusv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4Nusv_t>>(gl_ptr_filter(f(b"glVertexAttrib4Nusv\0".as_ptr())));
}
/// glVertexAttrib4bv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4bv(index: GLuint, v: *const [GLbyte; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4bv_p.0.get() } {
    Some(glVertexAttrib4bv_inner) => glVertexAttrib4bv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4bv"),
  }
}
static glVertexAttrib4bv_p: GlFnCell<glVertexAttrib4bv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4bv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4bv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4bv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4bv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4bv_t>>(gl_ptr_filter(f(b"glVertexAttrib4bv\0".as_ptr())));
}
/// glVertexAttrib4d
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4d_p.0.get() } {
    Some(glVertexAttrib4d_inner) => glVertexAttrib4d_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttrib4d"),
  }
}
static glVertexAttrib4d_p: GlFnCell<glVertexAttrib4d_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4d_is_loaded() -> bool {
  unsafe { *glVertexAttrib4d_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4d_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4d_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4d_t>>(gl_ptr_filter(f(b"glVertexAttrib4d\0".as_ptr())));
}
/// glVertexAttrib4dv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4dv(index: GLuint, v: *const [GLdouble; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4dv_p.0.get() } {
    Some(glVertexAttrib4dv_inner) => glVertexAttrib4dv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4dv"),
  }
}
static glVertexAttrib4dv_p: GlFnCell<glVertexAttrib4dv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4dv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4dv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4dv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4dv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4dv_t>>(gl_ptr_filter(f(b"glVertexAttrib4dv\0".as_ptr())));
}
/// glVertexAttrib4f
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4f_p.0.get() } {
    Some(glVertexAttrib4f_inner) => glVertexAttrib4f_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttrib4f"),
  }
}
static glVertexAttrib4f_p: GlFnCell<glVertexAttrib4f_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4f_is_loaded() -> bool {
  unsafe { *glVertexAttrib4f_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4f_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4f_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4f_t>>(gl_ptr_filter(f(b"glVertexAttrib4f\0".as_ptr())));
}
/// glVertexAttrib4fv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4fv(index: GLuint, v: *const [GLfloat; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4fv_p.0.get() } {
    Some(glVertexAttrib4fv_inner) => glVertexAttrib4fv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4fv"),
  }
}
static glVertexAttrib4fv_p: GlFnCell<glVertexAttrib4fv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4fv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4fv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4fv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4fv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4fv_t>>(gl_ptr_filter(f(b"glVertexAttrib4fv\0".as_ptr())));
}
/// glVertexAttrib4iv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4iv(index: GLuint, v: *const [GLint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4iv_p.0.get() } {
    Some(glVertexAttrib4iv_inner) => glVertexAttrib4iv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4iv"),
  }
}
static glVertexAttrib4iv_p: GlFnCell<glVertexAttrib4iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4iv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4iv_t>>(gl_ptr_filter(f(b"glVertexAttrib4iv\0".as_ptr())));
}
/// glVertexAttrib4s
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4s_p.0.get() } {
    Some(glVertexAttrib4s_inner) => glVertexAttrib4s_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttrib4s"),
  }
}
static glVertexAttrib4s_p: GlFnCell<glVertexAttrib4s_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4s_is_loaded() -> bool {
  unsafe { *glVertexAttrib4s_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4s_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4s_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4s_t>>(gl_ptr_filter(f(b"glVertexAttrib4s\0".as_ptr())));
}
/// glVertexAttrib4sv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4sv(index: GLuint, v: *const [GLshort; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4sv_p.0.get() } {
    Some(glVertexAttrib4sv_inner) => glVertexAttrib4sv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4sv"),
  }
}
static glVertexAttrib4sv_p: GlFnCell<glVertexAttrib4sv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4sv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4sv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4sv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4sv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4sv_t>>(gl_ptr_filter(f(b"glVertexAttrib4sv\0".as_ptr())));
}
/// glVertexAttrib4ubv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4ubv(index: GLuint, v: *const [GLubyte; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4ubv_p.0.get() } {
    Some(glVertexAttrib4ubv_inner) => glVertexAttrib4ubv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4ubv"),
  }
}
static glVertexAttrib4ubv_p: GlFnCell<glVertexAttrib4ubv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4ubv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4ubv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4ubv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4ubv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4ubv_t>>(gl_ptr_filter(f(b"glVertexAttrib4ubv\0".as_ptr())));
}
/// glVertexAttrib4uiv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4uiv(index: GLuint, v: *const [GLuint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4uiv_p.0.get() } {
    Some(glVertexAttrib4uiv_inner) => glVertexAttrib4uiv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4uiv"),
  }
}
static glVertexAttrib4uiv_p: GlFnCell<glVertexAttrib4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4uiv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4uiv_t>>(gl_ptr_filter(f(b"glVertexAttrib4uiv\0".as_ptr())));
}
/// glVertexAttrib4usv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttrib4usv(index: GLuint, v: *const [GLushort; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttrib4usv_p.0.get() } {
    Some(glVertexAttrib4usv_inner) => glVertexAttrib4usv_inner(index, v),
    None => gl_not_loaded("glVertexAttrib4usv"),
  }
}
static glVertexAttrib4usv_p: GlFnCell<glVertexAttrib4usv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttrib4usv_is_loaded() -> bool {
  unsafe { *glVertexAttrib4usv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttrib4usv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttrib4usv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttrib4usv_t>>(gl_ptr_filter(f(b"glVertexAttrib4usv\0".as_ptr())));
}
/// glVertexAttribDivisor
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribDivisor(index: GLuint, divisor: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribDivisor_p.0.get() } {
    Some(glVertexAttribDivisor_inner) => glVertexAttribDivisor_inner(index, divisor),
    None => gl_not_loaded("glVertexAttribDivisor"),
  }
}
static glVertexAttribDivisor_p: GlFnCell<glVertexAttribDivisor_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribDivisor_is_loaded() -> bool {
  unsafe { *glVertexAttribDivisor_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribDivisor_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribDivisor_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribDivisor_t>>(gl_ptr_filter(f(b"glVertexAttribDivisor\0".as_ptr())));
}
/// glVertexAttribI1i
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI1i(index: GLuint, x: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI1i_p.0.get() } {
    Some(glVertexAttribI1i_inner) => glVertexAttribI1i_inner(index, x),
    None => gl_not_loaded("glVertexAttribI1i"),
  }
}
static glVertexAttribI1i_p: GlFnCell<glVertexAttribI1i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI1i_is_loaded() -> bool {
  unsafe { *glVertexAttribI1i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI1i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI1i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI1i_t>>(gl_ptr_filter(f(b"glVertexAttribI1i\0".as_ptr())));
}
/// glVertexAttribI1iv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI1iv(index: GLuint, v: *const GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI1iv_p.0.get() } {
    Some(glVertexAttribI1iv_inner) => glVertexAttribI1iv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI1iv"),
  }
}
static glVertexAttribI1iv_p: GlFnCell<glVertexAttribI1iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI1iv_is_loaded() -> bool {
  unsafe { *glVertexAttribI1iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI1iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI1iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI1iv_t>>(gl_ptr_filter(f(b"glVertexAttribI1iv\0".as_ptr())));
}
/// glVertexAttribI1ui
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI1ui(index: GLuint, x: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI1ui_p.0.get() } {
    Some(glVertexAttribI1ui_inner) => glVertexAttribI1ui_inner(index, x),
    None => gl_not_loaded("glVertexAttribI1ui"),
  }
}
static glVertexAttribI1ui_p: GlFnCell<glVertexAttribI1ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI1ui_is_loaded() -> bool {
  unsafe { *glVertexAttribI1ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI1ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI1ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI1ui_t>>(gl_ptr_filter(f(b"glVertexAttribI1ui\0".as_ptr())));
}
/// glVertexAttribI1uiv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI1uiv(index: GLuint, v: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI1uiv_p.0.get() } {
    Some(glVertexAttribI1uiv_inner) => glVertexAttribI1uiv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI1uiv"),
  }
}
static glVertexAttribI1uiv_p: GlFnCell<glVertexAttribI1uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI1uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribI1uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI1uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI1uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI1uiv_t>>(gl_ptr_filter(f(b"glVertexAttribI1uiv\0".as_ptr())));
}
/// glVertexAttribI2i
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI2i(index: GLuint, x: GLint, y: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI2i_p.0.get() } {
    Some(glVertexAttribI2i_inner) => glVertexAttribI2i_inner(index, x, y),
    None => gl_not_loaded("glVertexAttribI2i"),
  }
}
static glVertexAttribI2i_p: GlFnCell<glVertexAttribI2i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI2i_is_loaded() -> bool {
  unsafe { *glVertexAttribI2i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI2i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI2i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI2i_t>>(gl_ptr_filter(f(b"glVertexAttribI2i\0".as_ptr())));
}
/// glVertexAttribI2iv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI2iv(index: GLuint, v: *const [GLint; 2]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI2iv_p.0.get() } {
    Some(glVertexAttribI2iv_inner) => glVertexAttribI2iv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI2iv"),
  }
}
static glVertexAttribI2iv_p: GlFnCell<glVertexAttribI2iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI2iv_is_loaded() -> bool {
  unsafe { *glVertexAttribI2iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI2iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI2iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI2iv_t>>(gl_ptr_filter(f(b"glVertexAttribI2iv\0".as_ptr())));
}
/// glVertexAttribI2ui
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI2ui_p.0.get() } {
    Some(glVertexAttribI2ui_inner) => glVertexAttribI2ui_inner(index, x, y),
    None => gl_not_loaded("glVertexAttribI2ui"),
  }
}
static glVertexAttribI2ui_p: GlFnCell<glVertexAttribI2ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI2ui_is_loaded() -> bool {
  unsafe { *glVertexAttribI2ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI2ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI2ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI2ui_t>>(gl_ptr_filter(f(b"glVertexAttribI2ui\0".as_ptr())));
}
/// glVertexAttribI2uiv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI2uiv(index: GLuint, v: *const [GLuint; 2]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI2uiv_p.0.get() } {
    Some(glVertexAttribI2uiv_inner) => glVertexAttribI2uiv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI2uiv"),
  }
}
static glVertexAttribI2uiv_p: GlFnCell<glVertexAttribI2uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI2uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribI2uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI2uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI2uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI2uiv_t>>(gl_ptr_filter(f(b"glVertexAttribI2uiv\0".as_ptr())));
}
/// glVertexAttribI3i
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI3i_p.0.get() } {
    Some(glVertexAttribI3i_inner) => glVertexAttribI3i_inner(index, x, y, z),
    None => gl_not_loaded("glVertexAttribI3i"),
  }
}
static glVertexAttribI3i_p: GlFnCell<glVertexAttribI3i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI3i_is_loaded() -> bool {
  unsafe { *glVertexAttribI3i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI3i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI3i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI3i_t>>(gl_ptr_filter(f(b"glVertexAttribI3i\0".as_ptr())));
}
/// glVertexAttribI3iv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI3iv(index: GLuint, v: *const [GLint; 3]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI3iv_p.0.get() } {
    Some(glVertexAttribI3iv_inner) => glVertexAttribI3iv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI3iv"),
  }
}
static glVertexAttribI3iv_p: GlFnCell<glVertexAttribI3iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI3iv_is_loaded() -> bool {
  unsafe { *glVertexAttribI3iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI3iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI3iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI3iv_t>>(gl_ptr_filter(f(b"glVertexAttribI3iv\0".as_ptr())));
}
/// glVertexAttribI3ui
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI3ui_p.0.get() } {
    Some(glVertexAttribI3ui_inner) => glVertexAttribI3ui_inner(index, x, y, z),
    None => gl_not_loaded("glVertexAttribI3ui"),
  }
}
static glVertexAttribI3ui_p: GlFnCell<glVertexAttribI3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI3ui_is_loaded() -> bool {
  unsafe { *glVertexAttribI3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI3ui_t>>(gl_ptr_filter(f(b"glVertexAttribI3ui\0".as_ptr())));
}
/// glVertexAttribI3uiv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI3uiv(index: GLuint, v: *const [GLuint; 3]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI3uiv_p.0.get() } {
    Some(glVertexAttribI3uiv_inner) => glVertexAttribI3uiv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI3uiv"),
  }
}
static glVertexAttribI3uiv_p: GlFnCell<glVertexAttribI3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI3uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribI3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI3uiv_t>>(gl_ptr_filter(f(b"glVertexAttribI3uiv\0".as_ptr())));
}
/// glVertexAttribI4bv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4bv(index: GLuint, v: *const [GLbyte; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4bv_p.0.get() } {
    Some(glVertexAttribI4bv_inner) => glVertexAttribI4bv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI4bv"),
  }
}
static glVertexAttribI4bv_p: GlFnCell<glVertexAttribI4bv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4bv_is_loaded() -> bool {
  unsafe { *glVertexAttribI4bv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4bv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4bv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4bv_t>>(gl_ptr_filter(f(b"glVertexAttribI4bv\0".as_ptr())));
}
/// glVertexAttribI4i
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4i_p.0.get() } {
    Some(glVertexAttribI4i_inner) => glVertexAttribI4i_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttribI4i"),
  }
}
static glVertexAttribI4i_p: GlFnCell<glVertexAttribI4i_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4i_is_loaded() -> bool {
  unsafe { *glVertexAttribI4i_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4i_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4i_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4i_t>>(gl_ptr_filter(f(b"glVertexAttribI4i\0".as_ptr())));
}
/// glVertexAttribI4iv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4iv(index: GLuint, v: *const [GLint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4iv_p.0.get() } {
    Some(glVertexAttribI4iv_inner) => glVertexAttribI4iv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI4iv"),
  }
}
static glVertexAttribI4iv_p: GlFnCell<glVertexAttribI4iv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4iv_is_loaded() -> bool {
  unsafe { *glVertexAttribI4iv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4iv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4iv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4iv_t>>(gl_ptr_filter(f(b"glVertexAttribI4iv\0".as_ptr())));
}
/// glVertexAttribI4sv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4sv(index: GLuint, v: *const [GLshort; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4sv_p.0.get() } {
    Some(glVertexAttribI4sv_inner) => glVertexAttribI4sv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI4sv"),
  }
}
static glVertexAttribI4sv_p: GlFnCell<glVertexAttribI4sv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4sv_is_loaded() -> bool {
  unsafe { *glVertexAttribI4sv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4sv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4sv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4sv_t>>(gl_ptr_filter(f(b"glVertexAttribI4sv\0".as_ptr())));
}
/// glVertexAttribI4ubv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4ubv(index: GLuint, v: *const [GLubyte; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4ubv_p.0.get() } {
    Some(glVertexAttribI4ubv_inner) => glVertexAttribI4ubv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI4ubv"),
  }
}
static glVertexAttribI4ubv_p: GlFnCell<glVertexAttribI4ubv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4ubv_is_loaded() -> bool {
  unsafe { *glVertexAttribI4ubv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4ubv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4ubv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4ubv_t>>(gl_ptr_filter(f(b"glVertexAttribI4ubv\0".as_ptr())));
}
/// glVertexAttribI4ui
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4ui_p.0.get() } {
    Some(glVertexAttribI4ui_inner) => glVertexAttribI4ui_inner(index, x, y, z, w),
    None => gl_not_loaded("glVertexAttribI4ui"),
  }
}
static glVertexAttribI4ui_p: GlFnCell<glVertexAttribI4ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4ui_is_loaded() -> bool {
  unsafe { *glVertexAttribI4ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4ui_t>>(gl_ptr_filter(f(b"glVertexAttribI4ui\0".as_ptr())));
}
/// glVertexAttribI4uiv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4uiv(index: GLuint, v: *const [GLuint; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4uiv_p.0.get() } {
    Some(glVertexAttribI4uiv_inner) => glVertexAttribI4uiv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI4uiv"),
  }
}
static glVertexAttribI4uiv_p: GlFnCell<glVertexAttribI4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribI4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4uiv_t>>(gl_ptr_filter(f(b"glVertexAttribI4uiv\0".as_ptr())));
}
/// glVertexAttribI4usv
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribI4usv(index: GLuint, v: *const [GLushort; 4]) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribI4usv_p.0.get() } {
    Some(glVertexAttribI4usv_inner) => glVertexAttribI4usv_inner(index, v),
    None => gl_not_loaded("glVertexAttribI4usv"),
  }
}
static glVertexAttribI4usv_p: GlFnCell<glVertexAttribI4usv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribI4usv_is_loaded() -> bool {
  unsafe { *glVertexAttribI4usv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribI4usv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribI4usv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribI4usv_t>>(gl_ptr_filter(f(b"glVertexAttribI4usv\0".as_ptr())));
}
/// glVertexAttribIPointer
/// * `type` group: VertexAttribIType
/// * `pointer` len: COMPSIZE(size,type,stride)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribIPointer(index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei, pointer: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribIPointer_p.0.get() } {
    Some(glVertexAttribIPointer_inner) => glVertexAttribIPointer_inner(index, size, type_, stride, pointer),
    None => gl_not_loaded("glVertexAttribIPointer"),
  }
}
static glVertexAttribIPointer_p: GlFnCell<glVertexAttribIPointer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribIPointer_is_loaded() -> bool {
  unsafe { *glVertexAttribIPointer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribIPointer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribIPointer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribIPointer_t>>(gl_ptr_filter(f(b"glVertexAttribIPointer\0".as_ptr())));
}
/// glVertexAttribP1ui
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP1ui(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP1ui_p.0.get() } {
    Some(glVertexAttribP1ui_inner) => glVertexAttribP1ui_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP1ui"),
  }
}
static glVertexAttribP1ui_p: GlFnCell<glVertexAttribP1ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP1ui_is_loaded() -> bool {
  unsafe { *glVertexAttribP1ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP1ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP1ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP1ui_t>>(gl_ptr_filter(f(b"glVertexAttribP1ui\0".as_ptr())));
}
/// glVertexAttribP1uiv
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP1uiv(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP1uiv_p.0.get() } {
    Some(glVertexAttribP1uiv_inner) => glVertexAttribP1uiv_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP1uiv"),
  }
}
static glVertexAttribP1uiv_p: GlFnCell<glVertexAttribP1uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP1uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribP1uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP1uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP1uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP1uiv_t>>(gl_ptr_filter(f(b"glVertexAttribP1uiv\0".as_ptr())));
}
/// glVertexAttribP2ui
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP2ui(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP2ui_p.0.get() } {
    Some(glVertexAttribP2ui_inner) => glVertexAttribP2ui_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP2ui"),
  }
}
static glVertexAttribP2ui_p: GlFnCell<glVertexAttribP2ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP2ui_is_loaded() -> bool {
  unsafe { *glVertexAttribP2ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP2ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP2ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP2ui_t>>(gl_ptr_filter(f(b"glVertexAttribP2ui\0".as_ptr())));
}
/// glVertexAttribP2uiv
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP2uiv(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP2uiv_p.0.get() } {
    Some(glVertexAttribP2uiv_inner) => glVertexAttribP2uiv_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP2uiv"),
  }
}
static glVertexAttribP2uiv_p: GlFnCell<glVertexAttribP2uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP2uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribP2uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP2uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP2uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP2uiv_t>>(gl_ptr_filter(f(b"glVertexAttribP2uiv\0".as_ptr())));
}
/// glVertexAttribP3ui
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP3ui(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP3ui_p.0.get() } {
    Some(glVertexAttribP3ui_inner) => glVertexAttribP3ui_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP3ui"),
  }
}
static glVertexAttribP3ui_p: GlFnCell<glVertexAttribP3ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP3ui_is_loaded() -> bool {
  unsafe { *glVertexAttribP3ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP3ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP3ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP3ui_t>>(gl_ptr_filter(f(b"glVertexAttribP3ui\0".as_ptr())));
}
/// glVertexAttribP3uiv
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP3uiv(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP3uiv_p.0.get() } {
    Some(glVertexAttribP3uiv_inner) => glVertexAttribP3uiv_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP3uiv"),
  }
}
static glVertexAttribP3uiv_p: GlFnCell<glVertexAttribP3uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP3uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribP3uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP3uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP3uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP3uiv_t>>(gl_ptr_filter(f(b"glVertexAttribP3uiv\0".as_ptr())));
}
/// glVertexAttribP4ui
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP4ui(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP4ui_p.0.get() } {
    Some(glVertexAttribP4ui_inner) => glVertexAttribP4ui_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP4ui"),
  }
}
static glVertexAttribP4ui_p: GlFnCell<glVertexAttribP4ui_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP4ui_is_loaded() -> bool {
  unsafe { *glVertexAttribP4ui_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP4ui_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP4ui_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP4ui_t>>(gl_ptr_filter(f(b"glVertexAttribP4ui\0".as_ptr())));
}
/// glVertexAttribP4uiv
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribP4uiv(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribP4uiv_p.0.get() } {
    Some(glVertexAttribP4uiv_inner) => glVertexAttribP4uiv_inner(index, type_, normalized, value),
    None => gl_not_loaded("glVertexAttribP4uiv"),
  }
}
static glVertexAttribP4uiv_p: GlFnCell<glVertexAttribP4uiv_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribP4uiv_is_loaded() -> bool {
  unsafe { *glVertexAttribP4uiv_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribP4uiv_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribP4uiv_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribP4uiv_t>>(gl_ptr_filter(f(b"glVertexAttribP4uiv\0".as_ptr())));
}
/// glVertexAttribPointer
/// * `type` group: VertexAttribPointerType
/// * `normalized` group: Boolean
/// * `pointer` len: COMPSIZE(size,type,stride)
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glVertexAttribPointer(index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, pointer: *const void) {
  #[allow(unused_unsafe)]
  match unsafe { *glVertexAttribPointer_p.0.get() } {
    Some(glVertexAttribPointer_inner) => glVertexAttribPointer_inner(index, size, type_, normalized, stride, pointer),
    None => gl_not_loaded("glVertexAttribPointer"),
  }
}
static glVertexAttribPointer_p: GlFnCell<glVertexAttribPointer_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glVertexAttribPointer_is_loaded() -> bool {
  unsafe { *glVertexAttribPointer_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glVertexAttribPointer_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glVertexAttribPointer_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glVertexAttribPointer_t>>(gl_ptr_filter(f(b"glVertexAttribPointer\0".as_ptr())));
}
/// glViewport
/// * `x` group: WinCoord
/// * `y` group: WinCoord
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
  #[allow(unused_unsafe)]
  match unsafe { *glViewport_p.0.get() } {
    Some(glViewport_inner) => glViewport_inner(x, y, width, height),
    None => gl_not_loaded("glViewport"),
  }
}
static glViewport_p: GlFnCell<glViewport_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glViewport_is_loaded() -> bool {
  unsafe { *glViewport_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glViewport_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glViewport_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glViewport_t>>(gl_ptr_filter(f(b"glViewport\0".as_ptr())));
}
/// glWaitSync
/// * `sync` group: sync
/// * `sync` class: sync
/// * `flags` group: SyncBehaviorFlags
#[cfg_attr(feature = "track_caller", track_caller)]
pub unsafe fn glWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
  #[allow(unused_unsafe)]
  match unsafe { *glWaitSync_p.0.get() } {
    Some(glWaitSync_inner) => glWaitSync_inner(sync, flags, timeout),
    None => gl_not_loaded("glWaitSync"),
  }
}
static glWaitSync_p: GlFnCell<glWaitSync_t> = GlFnCell(core::cell::UnsafeCell::new(None));
#[doc(hidden)]
pub fn glWaitSync_is_loaded() -> bool {
  unsafe { *glWaitSync_p.0.get() }.is_some()
}
#[doc(hidden)]
pub unsafe fn glWaitSync_load_with(f: &dyn Fn(*const u8) -> *const c_void) {
  *glWaitSync_p.0.get() = core::mem::transmute::<Option<core::ptr::NonNull<c_void>>, Option<glWaitSync_t>>(gl_ptr_filter(f(b"glWaitSync\0".as_ptr())));
}
