use super::*;

// Note(Lokathor): _p for ptr, _t for type

/// A struct that holds all the functions to use OpenGL 3.3 Core.
///
/// Each supported GL command has a method named after the command, with the
/// `gl` prefix removed. The expectation is that you'll name the struct's
/// variable `gl`, and then calling a command such as `glFlush()` would be
/// written as `gl.Flush()`.
///
/// Optionally, the `GL_KHR_debug` extension functions might be available. To
/// see if an optional function is available, use the function's base name with
/// `_is_loaded` at the end. The `_is_loaded` functions are hidden in the
/// generated documentation, but they are there. Calling an optional function
/// that is not loaded will panic. Note: To get high quality debug output, you
/// generally have to ask for a debug context during context creation.
#[repr(C)]
pub struct GlFns {
  glActiveTexture_p: glActiveTexture_t,
  glAttachShader_p: glAttachShader_t,
  glBeginConditionalRender_p: glBeginConditionalRender_t,
  glBeginQuery_p: glBeginQuery_t,
  glBeginTransformFeedback_p: glBeginTransformFeedback_t,
  glBindAttribLocation_p: glBindAttribLocation_t,
  glBindBuffer_p: glBindBuffer_t,
  glBindBufferBase_p: glBindBufferBase_t,
  glBindBufferRange_p: glBindBufferRange_t,
  glBindFragDataLocation_p: glBindFragDataLocation_t,
  glBindFragDataLocationIndexed_p: glBindFragDataLocationIndexed_t,
  glBindFramebuffer_p: glBindFramebuffer_t,
  glBindRenderbuffer_p: glBindRenderbuffer_t,
  glBindSampler_p: glBindSampler_t,
  glBindTexture_p: glBindTexture_t,
  glBindVertexArray_p: glBindVertexArray_t,
  glBlendColor_p: glBlendColor_t,
  glBlendEquation_p: glBlendEquation_t,
  glBlendEquationSeparate_p: glBlendEquationSeparate_t,
  glBlendFunc_p: glBlendFunc_t,
  glBlendFuncSeparate_p: glBlendFuncSeparate_t,
  glBlitFramebuffer_p: glBlitFramebuffer_t,
  glBufferData_p: glBufferData_t,
  glBufferSubData_p: glBufferSubData_t,
  glCheckFramebufferStatus_p: glCheckFramebufferStatus_t,
  glClampColor_p: glClampColor_t,
  glClear_p: glClear_t,
  glClearBufferfi_p: glClearBufferfi_t,
  glClearBufferfv_p: glClearBufferfv_t,
  glClearBufferiv_p: glClearBufferiv_t,
  glClearBufferuiv_p: glClearBufferuiv_t,
  glClearColor_p: glClearColor_t,
  glClearDepth_p: glClearDepth_t,
  glClearStencil_p: glClearStencil_t,
  glClientWaitSync_p: glClientWaitSync_t,
  glColorMask_p: glColorMask_t,
  glColorMaski_p: glColorMaski_t,
  glCompileShader_p: glCompileShader_t,
  glCompressedTexImage1D_p: glCompressedTexImage1D_t,
  glCompressedTexImage2D_p: glCompressedTexImage2D_t,
  glCompressedTexImage3D_p: glCompressedTexImage3D_t,
  glCompressedTexSubImage1D_p: glCompressedTexSubImage1D_t,
  glCompressedTexSubImage2D_p: glCompressedTexSubImage2D_t,
  glCompressedTexSubImage3D_p: glCompressedTexSubImage3D_t,
  glCopyBufferSubData_p: glCopyBufferSubData_t,
  glCopyTexImage1D_p: glCopyTexImage1D_t,
  glCopyTexImage2D_p: glCopyTexImage2D_t,
  glCopyTexSubImage1D_p: glCopyTexSubImage1D_t,
  glCopyTexSubImage2D_p: glCopyTexSubImage2D_t,
  glCopyTexSubImage3D_p: glCopyTexSubImage3D_t,
  glCreateProgram_p: glCreateProgram_t,
  glCreateShader_p: glCreateShader_t,
  glCullFace_p: glCullFace_t,
  glDeleteBuffers_p: glDeleteBuffers_t,
  glDeleteFramebuffers_p: glDeleteFramebuffers_t,
  glDeleteProgram_p: glDeleteProgram_t,
  glDeleteQueries_p: glDeleteQueries_t,
  glDeleteRenderbuffers_p: glDeleteRenderbuffers_t,
  glDeleteSamplers_p: glDeleteSamplers_t,
  glDeleteShader_p: glDeleteShader_t,
  glDeleteSync_p: glDeleteSync_t,
  glDeleteTextures_p: glDeleteTextures_t,
  glDeleteVertexArrays_p: glDeleteVertexArrays_t,
  glDepthFunc_p: glDepthFunc_t,
  glDepthMask_p: glDepthMask_t,
  glDepthRange_p: glDepthRange_t,
  glDetachShader_p: glDetachShader_t,
  glDisable_p: glDisable_t,
  glDisableVertexAttribArray_p: glDisableVertexAttribArray_t,
  glDisablei_p: glDisablei_t,
  glDrawArrays_p: glDrawArrays_t,
  glDrawArraysInstanced_p: glDrawArraysInstanced_t,
  glDrawBuffer_p: glDrawBuffer_t,
  glDrawBuffers_p: glDrawBuffers_t,
  glDrawElements_p: glDrawElements_t,
  glDrawElementsBaseVertex_p: glDrawElementsBaseVertex_t,
  glDrawElementsInstanced_p: glDrawElementsInstanced_t,
  glDrawElementsInstancedBaseVertex_p: glDrawElementsInstancedBaseVertex_t,
  glDrawRangeElements_p: glDrawRangeElements_t,
  glDrawRangeElementsBaseVertex_p: glDrawRangeElementsBaseVertex_t,
  glEnable_p: glEnable_t,
  glEnableVertexAttribArray_p: glEnableVertexAttribArray_t,
  glEnablei_p: glEnablei_t,
  glEndConditionalRender_p: glEndConditionalRender_t,
  glEndQuery_p: glEndQuery_t,
  glEndTransformFeedback_p: glEndTransformFeedback_t,
  glFenceSync_p: glFenceSync_t,
  glFinish_p: glFinish_t,
  glFlush_p: glFlush_t,
  glFlushMappedBufferRange_p: glFlushMappedBufferRange_t,
  glFramebufferRenderbuffer_p: glFramebufferRenderbuffer_t,
  glFramebufferTexture_p: glFramebufferTexture_t,
  glFramebufferTexture1D_p: glFramebufferTexture1D_t,
  glFramebufferTexture2D_p: glFramebufferTexture2D_t,
  glFramebufferTexture3D_p: glFramebufferTexture3D_t,
  glFramebufferTextureLayer_p: glFramebufferTextureLayer_t,
  glFrontFace_p: glFrontFace_t,
  glGenBuffers_p: glGenBuffers_t,
  glGenFramebuffers_p: glGenFramebuffers_t,
  glGenQueries_p: glGenQueries_t,
  glGenRenderbuffers_p: glGenRenderbuffers_t,
  glGenSamplers_p: glGenSamplers_t,
  glGenTextures_p: glGenTextures_t,
  glGenVertexArrays_p: glGenVertexArrays_t,
  glGenerateMipmap_p: glGenerateMipmap_t,
  glGetActiveAttrib_p: glGetActiveAttrib_t,
  glGetActiveUniform_p: glGetActiveUniform_t,
  glGetActiveUniformBlockName_p: glGetActiveUniformBlockName_t,
  glGetActiveUniformBlockiv_p: glGetActiveUniformBlockiv_t,
  glGetActiveUniformName_p: glGetActiveUniformName_t,
  glGetActiveUniformsiv_p: glGetActiveUniformsiv_t,
  glGetAttachedShaders_p: glGetAttachedShaders_t,
  glGetAttribLocation_p: glGetAttribLocation_t,
  glGetBooleani_v_p: glGetBooleani_v_t,
  glGetBooleanv_p: glGetBooleanv_t,
  glGetBufferParameteri64v_p: glGetBufferParameteri64v_t,
  glGetBufferParameteriv_p: glGetBufferParameteriv_t,
  glGetBufferPointerv_p: glGetBufferPointerv_t,
  glGetBufferSubData_p: glGetBufferSubData_t,
  glGetCompressedTexImage_p: glGetCompressedTexImage_t,
  glGetDoublev_p: glGetDoublev_t,
  glGetError_p: glGetError_t,
  glGetFloatv_p: glGetFloatv_t,
  glGetFragDataIndex_p: glGetFragDataIndex_t,
  glGetFragDataLocation_p: glGetFragDataLocation_t,
  glGetFramebufferAttachmentParameteriv_p: glGetFramebufferAttachmentParameteriv_t,
  glGetInteger64i_v_p: glGetInteger64i_v_t,
  glGetInteger64v_p: glGetInteger64v_t,
  glGetIntegeri_v_p: glGetIntegeri_v_t,
  glGetIntegerv_p: glGetIntegerv_t,
  glGetMultisamplefv_p: glGetMultisamplefv_t,
  glGetProgramInfoLog_p: glGetProgramInfoLog_t,
  glGetProgramiv_p: glGetProgramiv_t,
  glGetQueryObjecti64v_p: glGetQueryObjecti64v_t,
  glGetQueryObjectiv_p: glGetQueryObjectiv_t,
  glGetQueryObjectui64v_p: glGetQueryObjectui64v_t,
  glGetQueryObjectuiv_p: glGetQueryObjectuiv_t,
  glGetQueryiv_p: glGetQueryiv_t,
  glGetRenderbufferParameteriv_p: glGetRenderbufferParameteriv_t,
  glGetSamplerParameterIiv_p: glGetSamplerParameterIiv_t,
  glGetSamplerParameterIuiv_p: glGetSamplerParameterIuiv_t,
  glGetSamplerParameterfv_p: glGetSamplerParameterfv_t,
  glGetSamplerParameteriv_p: glGetSamplerParameteriv_t,
  glGetShaderInfoLog_p: glGetShaderInfoLog_t,
  glGetShaderSource_p: glGetShaderSource_t,
  glGetShaderiv_p: glGetShaderiv_t,
  glGetString_p: glGetString_t,
  glGetStringi_p: glGetStringi_t,
  glGetSynciv_p: glGetSynciv_t,
  glGetTexImage_p: glGetTexImage_t,
  glGetTexLevelParameterfv_p: glGetTexLevelParameterfv_t,
  glGetTexLevelParameteriv_p: glGetTexLevelParameteriv_t,
  glGetTexParameterIiv_p: glGetTexParameterIiv_t,
  glGetTexParameterIuiv_p: glGetTexParameterIuiv_t,
  glGetTexParameterfv_p: glGetTexParameterfv_t,
  glGetTexParameteriv_p: glGetTexParameteriv_t,
  glGetTransformFeedbackVarying_p: glGetTransformFeedbackVarying_t,
  glGetUniformBlockIndex_p: glGetUniformBlockIndex_t,
  glGetUniformIndices_p: glGetUniformIndices_t,
  glGetUniformLocation_p: glGetUniformLocation_t,
  glGetUniformfv_p: glGetUniformfv_t,
  glGetUniformiv_p: glGetUniformiv_t,
  glGetUniformuiv_p: glGetUniformuiv_t,
  glGetVertexAttribIiv_p: glGetVertexAttribIiv_t,
  glGetVertexAttribIuiv_p: glGetVertexAttribIuiv_t,
  glGetVertexAttribPointerv_p: glGetVertexAttribPointerv_t,
  glGetVertexAttribdv_p: glGetVertexAttribdv_t,
  glGetVertexAttribfv_p: glGetVertexAttribfv_t,
  glGetVertexAttribiv_p: glGetVertexAttribiv_t,
  glHint_p: glHint_t,
  glIsBuffer_p: glIsBuffer_t,
  glIsEnabled_p: glIsEnabled_t,
  glIsEnabledi_p: glIsEnabledi_t,
  glIsFramebuffer_p: glIsFramebuffer_t,
  glIsProgram_p: glIsProgram_t,
  glIsQuery_p: glIsQuery_t,
  glIsRenderbuffer_p: glIsRenderbuffer_t,
  glIsSampler_p: glIsSampler_t,
  glIsShader_p: glIsShader_t,
  glIsSync_p: glIsSync_t,
  glIsTexture_p: glIsTexture_t,
  glIsVertexArray_p: glIsVertexArray_t,
  glLineWidth_p: glLineWidth_t,
  glLinkProgram_p: glLinkProgram_t,
  glLogicOp_p: glLogicOp_t,
  glMapBuffer_p: glMapBuffer_t,
  glMapBufferRange_p: glMapBufferRange_t,
  glMultiDrawArrays_p: glMultiDrawArrays_t,
  glMultiDrawElements_p: glMultiDrawElements_t,
  glMultiDrawElementsBaseVertex_p: glMultiDrawElementsBaseVertex_t,
  glPixelStoref_p: glPixelStoref_t,
  glPixelStorei_p: glPixelStorei_t,
  glPointParameterf_p: glPointParameterf_t,
  glPointParameterfv_p: glPointParameterfv_t,
  glPointParameteri_p: glPointParameteri_t,
  glPointParameteriv_p: glPointParameteriv_t,
  glPointSize_p: glPointSize_t,
  glPolygonMode_p: glPolygonMode_t,
  glPolygonOffset_p: glPolygonOffset_t,
  glPrimitiveRestartIndex_p: glPrimitiveRestartIndex_t,
  glProvokingVertex_p: glProvokingVertex_t,
  glQueryCounter_p: glQueryCounter_t,
  glReadBuffer_p: glReadBuffer_t,
  glReadPixels_p: glReadPixels_t,
  glRenderbufferStorage_p: glRenderbufferStorage_t,
  glRenderbufferStorageMultisample_p: glRenderbufferStorageMultisample_t,
  glSampleCoverage_p: glSampleCoverage_t,
  glSampleMaski_p: glSampleMaski_t,
  glSamplerParameterIiv_p: glSamplerParameterIiv_t,
  glSamplerParameterIuiv_p: glSamplerParameterIuiv_t,
  glSamplerParameterf_p: glSamplerParameterf_t,
  glSamplerParameterfv_p: glSamplerParameterfv_t,
  glSamplerParameteri_p: glSamplerParameteri_t,
  glSamplerParameteriv_p: glSamplerParameteriv_t,
  glScissor_p: glScissor_t,
  glShaderSource_p: glShaderSource_t,
  glStencilFunc_p: glStencilFunc_t,
  glStencilFuncSeparate_p: glStencilFuncSeparate_t,
  glStencilMask_p: glStencilMask_t,
  glStencilMaskSeparate_p: glStencilMaskSeparate_t,
  glStencilOp_p: glStencilOp_t,
  glStencilOpSeparate_p: glStencilOpSeparate_t,
  glTexBuffer_p: glTexBuffer_t,
  glTexImage1D_p: glTexImage1D_t,
  glTexImage2D_p: glTexImage2D_t,
  glTexImage2DMultisample_p: glTexImage2DMultisample_t,
  glTexImage3D_p: glTexImage3D_t,
  glTexImage3DMultisample_p: glTexImage3DMultisample_t,
  glTexParameterIiv_p: glTexParameterIiv_t,
  glTexParameterIuiv_p: glTexParameterIuiv_t,
  glTexParameterf_p: glTexParameterf_t,
  glTexParameterfv_p: glTexParameterfv_t,
  glTexParameteri_p: glTexParameteri_t,
  glTexParameteriv_p: glTexParameteriv_t,
  glTexSubImage1D_p: glTexSubImage1D_t,
  glTexSubImage2D_p: glTexSubImage2D_t,
  glTexSubImage3D_p: glTexSubImage3D_t,
  glTransformFeedbackVaryings_p: glTransformFeedbackVaryings_t,
  glUniform1f_p: glUniform1f_t,
  glUniform1fv_p: glUniform1fv_t,
  glUniform1i_p: glUniform1i_t,
  glUniform1iv_p: glUniform1iv_t,
  glUniform1ui_p: glUniform1ui_t,
  glUniform1uiv_p: glUniform1uiv_t,
  glUniform2f_p: glUniform2f_t,
  glUniform2fv_p: glUniform2fv_t,
  glUniform2i_p: glUniform2i_t,
  glUniform2iv_p: glUniform2iv_t,
  glUniform2ui_p: glUniform2ui_t,
  glUniform2uiv_p: glUniform2uiv_t,
  glUniform3f_p: glUniform3f_t,
  glUniform3fv_p: glUniform3fv_t,
  glUniform3i_p: glUniform3i_t,
  glUniform3iv_p: glUniform3iv_t,
  glUniform3ui_p: glUniform3ui_t,
  glUniform3uiv_p: glUniform3uiv_t,
  glUniform4f_p: glUniform4f_t,
  glUniform4fv_p: glUniform4fv_t,
  glUniform4i_p: glUniform4i_t,
  glUniform4iv_p: glUniform4iv_t,
  glUniform4ui_p: glUniform4ui_t,
  glUniform4uiv_p: glUniform4uiv_t,
  glUniformBlockBinding_p: glUniformBlockBinding_t,
  glUniformMatrix2fv_p: glUniformMatrix2fv_t,
  glUniformMatrix2x3fv_p: glUniformMatrix2x3fv_t,
  glUniformMatrix2x4fv_p: glUniformMatrix2x4fv_t,
  glUniformMatrix3fv_p: glUniformMatrix3fv_t,
  glUniformMatrix3x2fv_p: glUniformMatrix3x2fv_t,
  glUniformMatrix3x4fv_p: glUniformMatrix3x4fv_t,
  glUniformMatrix4fv_p: glUniformMatrix4fv_t,
  glUniformMatrix4x2fv_p: glUniformMatrix4x2fv_t,
  glUniformMatrix4x3fv_p: glUniformMatrix4x3fv_t,
  glUnmapBuffer_p: glUnmapBuffer_t,
  glUseProgram_p: glUseProgram_t,
  glValidateProgram_p: glValidateProgram_t,
  glVertexAttrib1d_p: glVertexAttrib1d_t,
  glVertexAttrib1dv_p: glVertexAttrib1dv_t,
  glVertexAttrib1f_p: glVertexAttrib1f_t,
  glVertexAttrib1fv_p: glVertexAttrib1fv_t,
  glVertexAttrib1s_p: glVertexAttrib1s_t,
  glVertexAttrib1sv_p: glVertexAttrib1sv_t,
  glVertexAttrib2d_p: glVertexAttrib2d_t,
  glVertexAttrib2dv_p: glVertexAttrib2dv_t,
  glVertexAttrib2f_p: glVertexAttrib2f_t,
  glVertexAttrib2fv_p: glVertexAttrib2fv_t,
  glVertexAttrib2s_p: glVertexAttrib2s_t,
  glVertexAttrib2sv_p: glVertexAttrib2sv_t,
  glVertexAttrib3d_p: glVertexAttrib3d_t,
  glVertexAttrib3dv_p: glVertexAttrib3dv_t,
  glVertexAttrib3f_p: glVertexAttrib3f_t,
  glVertexAttrib3fv_p: glVertexAttrib3fv_t,
  glVertexAttrib3s_p: glVertexAttrib3s_t,
  glVertexAttrib3sv_p: glVertexAttrib3sv_t,
  glVertexAttrib4Nbv_p: glVertexAttrib4Nbv_t,
  glVertexAttrib4Niv_p: glVertexAttrib4Niv_t,
  glVertexAttrib4Nsv_p: glVertexAttrib4Nsv_t,
  glVertexAttrib4Nub_p: glVertexAttrib4Nub_t,
  glVertexAttrib4Nubv_p: glVertexAttrib4Nubv_t,
  glVertexAttrib4Nuiv_p: glVertexAttrib4Nuiv_t,
  glVertexAttrib4Nusv_p: glVertexAttrib4Nusv_t,
  glVertexAttrib4bv_p: glVertexAttrib4bv_t,
  glVertexAttrib4d_p: glVertexAttrib4d_t,
  glVertexAttrib4dv_p: glVertexAttrib4dv_t,
  glVertexAttrib4f_p: glVertexAttrib4f_t,
  glVertexAttrib4fv_p: glVertexAttrib4fv_t,
  glVertexAttrib4iv_p: glVertexAttrib4iv_t,
  glVertexAttrib4s_p: glVertexAttrib4s_t,
  glVertexAttrib4sv_p: glVertexAttrib4sv_t,
  glVertexAttrib4ubv_p: glVertexAttrib4ubv_t,
  glVertexAttrib4uiv_p: glVertexAttrib4uiv_t,
  glVertexAttrib4usv_p: glVertexAttrib4usv_t,
  glVertexAttribDivisor_p: glVertexAttribDivisor_t,
  glVertexAttribI1i_p: glVertexAttribI1i_t,
  glVertexAttribI1iv_p: glVertexAttribI1iv_t,
  glVertexAttribI1ui_p: glVertexAttribI1ui_t,
  glVertexAttribI1uiv_p: glVertexAttribI1uiv_t,
  glVertexAttribI2i_p: glVertexAttribI2i_t,
  glVertexAttribI2iv_p: glVertexAttribI2iv_t,
  glVertexAttribI2ui_p: glVertexAttribI2ui_t,
  glVertexAttribI2uiv_p: glVertexAttribI2uiv_t,
  glVertexAttribI3i_p: glVertexAttribI3i_t,
  glVertexAttribI3iv_p: glVertexAttribI3iv_t,
  glVertexAttribI3ui_p: glVertexAttribI3ui_t,
  glVertexAttribI3uiv_p: glVertexAttribI3uiv_t,
  glVertexAttribI4bv_p: glVertexAttribI4bv_t,
  glVertexAttribI4i_p: glVertexAttribI4i_t,
  glVertexAttribI4iv_p: glVertexAttribI4iv_t,
  glVertexAttribI4sv_p: glVertexAttribI4sv_t,
  glVertexAttribI4ubv_p: glVertexAttribI4ubv_t,
  glVertexAttribI4ui_p: glVertexAttribI4ui_t,
  glVertexAttribI4uiv_p: glVertexAttribI4uiv_t,
  glVertexAttribI4usv_p: glVertexAttribI4usv_t,
  glVertexAttribIPointer_p: glVertexAttribIPointer_t,
  glVertexAttribP1ui_p: glVertexAttribP1ui_t,
  glVertexAttribP1uiv_p: glVertexAttribP1uiv_t,
  glVertexAttribP2ui_p: glVertexAttribP2ui_t,
  glVertexAttribP2uiv_p: glVertexAttribP2uiv_t,
  glVertexAttribP3ui_p: glVertexAttribP3ui_t,
  glVertexAttribP3uiv_p: glVertexAttribP3uiv_t,
  glVertexAttribP4ui_p: glVertexAttribP4ui_t,
  glVertexAttribP4uiv_p: glVertexAttribP4uiv_t,
  glVertexAttribPointer_p: glVertexAttribPointer_t,
  glViewport_p: glViewport_t,
  glWaitSync_p: glWaitSync_t,
  glDebugMessageCallback_p: Option<glDebugMessageCallback_t>,
  glDebugMessageControl_p: Option<glDebugMessageControl_t>,
  glDebugMessageInsert_p: Option<glDebugMessageInsert_t>,
  glGetDebugMessageLog_p: Option<glGetDebugMessageLog_t>,
  glGetObjectLabel_p: Option<glGetObjectLabel_t>,
  glGetObjectPtrLabel_p: Option<glGetObjectPtrLabel_t>,
  glGetPointerv_p: Option<glGetPointerv_t>,
  glObjectLabel_p: Option<glObjectLabel_t>,
  glObjectPtrLabel_p: Option<glObjectPtrLabel_t>,
  glPopDebugGroup_p: Option<glPopDebugGroup_t>,
  glPushDebugGroup_p: Option<glPushDebugGroup_t>,
}

impl GlFns {
  fn ptr_filter(p: *const c_void) -> Option<core::ptr::NonNull<c_void>> {
    match p as usize {
      // Note(Lokathor): wgl is known to sometimes give phony non-null pointer values.
      0 | 1 | 2 | 3 | usize::MAX => None,
      _ => unsafe { core::mem::transmute(p) },
    }
  }
  #[cold]
  #[inline(never)]
  #[cfg_attr(feature = "track_caller", track_caller)]
  fn not_loaded(name: &str) -> ! {
    panic!("Function Not Loaded: {}", name);
  }

  /// Loads all GL functions from the loader given.
  ///
  /// ## Failure
  /// This fails if any of the OpenGL 3.3 Core functions don't load. The error
  /// value will be the name of the first function that doesn't load.
  ///
  /// ## Safety
  /// * The "Get Proc Address" function you provide will always be given a
  ///   pointer to the start of a null-terminated string containing the name of
  ///   a GL function to load.
  /// * The "Get Proc Address" function given must always return accurate
  ///   function pointer values, or null on failure.
  /// * Some "Get Proc Address" implementations will return context-specific
  ///   function pointers. Others will return general function pointers even if
  ///   the current context doesn't support the command. It's **up to you** to
  ///   only load and use this struct with an appropriate GL context as the
  ///   current context.
  pub unsafe fn load_from(f: &dyn Fn(*const u8) -> *const c_void) -> Result<Self, &'static str> {
    use core::mem::transmute;
    type nn_cv = core::ptr::NonNull<c_void>;
    // non-nullable loads
    let glActiveTexture_p = transmute::<nn_cv, glActiveTexture_t>(Self::ptr_filter(f(b"glActiveTexture\0".as_ptr())).ok_or("glActiveTexture")?);
    let glAttachShader_p = transmute::<nn_cv, glAttachShader_t>(Self::ptr_filter(f(b"glAttachShader\0".as_ptr())).ok_or("glAttachShader")?);
    let glBeginConditionalRender_p = transmute::<nn_cv, glBeginConditionalRender_t>(Self::ptr_filter(f(b"glBeginConditionalRender\0".as_ptr())).ok_or("glBeginConditionalRender")?);
    let glBeginQuery_p = transmute::<nn_cv, glBeginQuery_t>(Self::ptr_filter(f(b"glBeginQuery\0".as_ptr())).ok_or("glBeginQuery")?);
    let glBeginTransformFeedback_p = transmute::<nn_cv, glBeginTransformFeedback_t>(Self::ptr_filter(f(b"glBeginTransformFeedback\0".as_ptr())).ok_or("glBeginTransformFeedback")?);
    let glBindAttribLocation_p = transmute::<nn_cv, glBindAttribLocation_t>(Self::ptr_filter(f(b"glBindAttribLocation\0".as_ptr())).ok_or("glBindAttribLocation")?);
    let glBindBuffer_p = transmute::<nn_cv, glBindBuffer_t>(Self::ptr_filter(f(b"glBindBuffer\0".as_ptr())).ok_or("glBindBuffer")?);
    let glBindBufferBase_p = transmute::<nn_cv, glBindBufferBase_t>(Self::ptr_filter(f(b"glBindBufferBase\0".as_ptr())).ok_or("glBindBufferBase")?);
    let glBindBufferRange_p = transmute::<nn_cv, glBindBufferRange_t>(Self::ptr_filter(f(b"glBindBufferRange\0".as_ptr())).ok_or("glBindBufferRange")?);
    let glBindFragDataLocation_p = transmute::<nn_cv, glBindFragDataLocation_t>(Self::ptr_filter(f(b"glBindFragDataLocation\0".as_ptr())).ok_or("glBindFragDataLocation")?);
    let glBindFragDataLocationIndexed_p = transmute::<nn_cv, glBindFragDataLocationIndexed_t>(Self::ptr_filter(f(b"glBindFragDataLocationIndexed\0".as_ptr())).ok_or("glBindFragDataLocationIndexed")?);
    let glBindFramebuffer_p = transmute::<nn_cv, glBindFramebuffer_t>(Self::ptr_filter(f(b"glBindFramebuffer\0".as_ptr())).ok_or("glBindFramebuffer")?);
    let glBindRenderbuffer_p = transmute::<nn_cv, glBindRenderbuffer_t>(Self::ptr_filter(f(b"glBindRenderbuffer\0".as_ptr())).ok_or("glBindRenderbuffer")?);
    let glBindSampler_p = transmute::<nn_cv, glBindSampler_t>(Self::ptr_filter(f(b"glBindSampler\0".as_ptr())).ok_or("glBindSampler")?);
    let glBindTexture_p = transmute::<nn_cv, glBindTexture_t>(Self::ptr_filter(f(b"glBindTexture\0".as_ptr())).ok_or("glBindTexture")?);
    let glBindVertexArray_p = transmute::<nn_cv, glBindVertexArray_t>(Self::ptr_filter(f(b"glBindVertexArray\0".as_ptr())).ok_or("glBindVertexArray")?);
    let glBlendColor_p = transmute::<nn_cv, glBlendColor_t>(Self::ptr_filter(f(b"glBlendColor\0".as_ptr())).ok_or("glBlendColor")?);
    let glBlendEquation_p = transmute::<nn_cv, glBlendEquation_t>(Self::ptr_filter(f(b"glBlendEquation\0".as_ptr())).ok_or("glBlendEquation")?);
    let glBlendEquationSeparate_p = transmute::<nn_cv, glBlendEquationSeparate_t>(Self::ptr_filter(f(b"glBlendEquationSeparate\0".as_ptr())).ok_or("glBlendEquationSeparate")?);
    let glBlendFunc_p = transmute::<nn_cv, glBlendFunc_t>(Self::ptr_filter(f(b"glBlendFunc\0".as_ptr())).ok_or("glBlendFunc")?);
    let glBlendFuncSeparate_p = transmute::<nn_cv, glBlendFuncSeparate_t>(Self::ptr_filter(f(b"glBlendFuncSeparate\0".as_ptr())).ok_or("glBlendFuncSeparate")?);
    let glBlitFramebuffer_p = transmute::<nn_cv, glBlitFramebuffer_t>(Self::ptr_filter(f(b"glBlitFramebuffer\0".as_ptr())).ok_or("glBlitFramebuffer")?);
    let glBufferData_p = transmute::<nn_cv, glBufferData_t>(Self::ptr_filter(f(b"glBufferData\0".as_ptr())).ok_or("glBufferData")?);
    let glBufferSubData_p = transmute::<nn_cv, glBufferSubData_t>(Self::ptr_filter(f(b"glBufferSubData\0".as_ptr())).ok_or("glBufferSubData")?);
    let glCheckFramebufferStatus_p = transmute::<nn_cv, glCheckFramebufferStatus_t>(Self::ptr_filter(f(b"glCheckFramebufferStatus\0".as_ptr())).ok_or("glCheckFramebufferStatus")?);
    let glClampColor_p = transmute::<nn_cv, glClampColor_t>(Self::ptr_filter(f(b"glClampColor\0".as_ptr())).ok_or("glClampColor")?);
    let glClear_p = transmute::<nn_cv, glClear_t>(Self::ptr_filter(f(b"glClear\0".as_ptr())).ok_or("glClear")?);
    let glClearBufferfi_p = transmute::<nn_cv, glClearBufferfi_t>(Self::ptr_filter(f(b"glClearBufferfi\0".as_ptr())).ok_or("glClearBufferfi")?);
    let glClearBufferfv_p = transmute::<nn_cv, glClearBufferfv_t>(Self::ptr_filter(f(b"glClearBufferfv\0".as_ptr())).ok_or("glClearBufferfv")?);
    let glClearBufferiv_p = transmute::<nn_cv, glClearBufferiv_t>(Self::ptr_filter(f(b"glClearBufferiv\0".as_ptr())).ok_or("glClearBufferiv")?);
    let glClearBufferuiv_p = transmute::<nn_cv, glClearBufferuiv_t>(Self::ptr_filter(f(b"glClearBufferuiv\0".as_ptr())).ok_or("glClearBufferuiv")?);
    let glClearColor_p = transmute::<nn_cv, glClearColor_t>(Self::ptr_filter(f(b"glClearColor\0".as_ptr())).ok_or("glClearColor")?);
    let glClearDepth_p = transmute::<nn_cv, glClearDepth_t>(Self::ptr_filter(f(b"glClearDepth\0".as_ptr())).ok_or("glClearDepth")?);
    let glClearStencil_p = transmute::<nn_cv, glClearStencil_t>(Self::ptr_filter(f(b"glClearStencil\0".as_ptr())).ok_or("glClearStencil")?);
    let glClientWaitSync_p = transmute::<nn_cv, glClientWaitSync_t>(Self::ptr_filter(f(b"glClientWaitSync\0".as_ptr())).ok_or("glClientWaitSync")?);
    let glColorMask_p = transmute::<nn_cv, glColorMask_t>(Self::ptr_filter(f(b"glColorMask\0".as_ptr())).ok_or("glColorMask")?);
    let glColorMaski_p = transmute::<nn_cv, glColorMaski_t>(Self::ptr_filter(f(b"glColorMaski\0".as_ptr())).ok_or("glColorMaski")?);
    let glCompileShader_p = transmute::<nn_cv, glCompileShader_t>(Self::ptr_filter(f(b"glCompileShader\0".as_ptr())).ok_or("glCompileShader")?);
    let glCompressedTexImage1D_p = transmute::<nn_cv, glCompressedTexImage1D_t>(Self::ptr_filter(f(b"glCompressedTexImage1D\0".as_ptr())).ok_or("glCompressedTexImage1D")?);
    let glCompressedTexImage2D_p = transmute::<nn_cv, glCompressedTexImage2D_t>(Self::ptr_filter(f(b"glCompressedTexImage2D\0".as_ptr())).ok_or("glCompressedTexImage2D")?);
    let glCompressedTexImage3D_p = transmute::<nn_cv, glCompressedTexImage3D_t>(Self::ptr_filter(f(b"glCompressedTexImage3D\0".as_ptr())).ok_or("glCompressedTexImage3D")?);
    let glCompressedTexSubImage1D_p = transmute::<nn_cv, glCompressedTexSubImage1D_t>(Self::ptr_filter(f(b"glCompressedTexSubImage1D\0".as_ptr())).ok_or("glCompressedTexSubImage1D")?);
    let glCompressedTexSubImage2D_p = transmute::<nn_cv, glCompressedTexSubImage2D_t>(Self::ptr_filter(f(b"glCompressedTexSubImage2D\0".as_ptr())).ok_or("glCompressedTexSubImage2D")?);
    let glCompressedTexSubImage3D_p = transmute::<nn_cv, glCompressedTexSubImage3D_t>(Self::ptr_filter(f(b"glCompressedTexSubImage3D\0".as_ptr())).ok_or("glCompressedTexSubImage3D")?);
    let glCopyBufferSubData_p = transmute::<nn_cv, glCopyBufferSubData_t>(Self::ptr_filter(f(b"glCopyBufferSubData\0".as_ptr())).ok_or("glCopyBufferSubData")?);
    let glCopyTexImage1D_p = transmute::<nn_cv, glCopyTexImage1D_t>(Self::ptr_filter(f(b"glCopyTexImage1D\0".as_ptr())).ok_or("glCopyTexImage1D")?);
    let glCopyTexImage2D_p = transmute::<nn_cv, glCopyTexImage2D_t>(Self::ptr_filter(f(b"glCopyTexImage2D\0".as_ptr())).ok_or("glCopyTexImage2D")?);
    let glCopyTexSubImage1D_p = transmute::<nn_cv, glCopyTexSubImage1D_t>(Self::ptr_filter(f(b"glCopyTexSubImage1D\0".as_ptr())).ok_or("glCopyTexSubImage1D")?);
    let glCopyTexSubImage2D_p = transmute::<nn_cv, glCopyTexSubImage2D_t>(Self::ptr_filter(f(b"glCopyTexSubImage2D\0".as_ptr())).ok_or("glCopyTexSubImage2D")?);
    let glCopyTexSubImage3D_p = transmute::<nn_cv, glCopyTexSubImage3D_t>(Self::ptr_filter(f(b"glCopyTexSubImage3D\0".as_ptr())).ok_or("glCopyTexSubImage3D")?);
    let glCreateProgram_p = transmute::<nn_cv, glCreateProgram_t>(Self::ptr_filter(f(b"glCreateProgram\0".as_ptr())).ok_or("glCreateProgram")?);
    let glCreateShader_p = transmute::<nn_cv, glCreateShader_t>(Self::ptr_filter(f(b"glCreateShader\0".as_ptr())).ok_or("glCreateShader")?);
    let glCullFace_p = transmute::<nn_cv, glCullFace_t>(Self::ptr_filter(f(b"glCullFace\0".as_ptr())).ok_or("glCullFace")?);
    let glDeleteBuffers_p = transmute::<nn_cv, glDeleteBuffers_t>(Self::ptr_filter(f(b"glDeleteBuffers\0".as_ptr())).ok_or("glDeleteBuffers")?);
    let glDeleteFramebuffers_p = transmute::<nn_cv, glDeleteFramebuffers_t>(Self::ptr_filter(f(b"glDeleteFramebuffers\0".as_ptr())).ok_or("glDeleteFramebuffers")?);
    let glDeleteProgram_p = transmute::<nn_cv, glDeleteProgram_t>(Self::ptr_filter(f(b"glDeleteProgram\0".as_ptr())).ok_or("glDeleteProgram")?);
    let glDeleteQueries_p = transmute::<nn_cv, glDeleteQueries_t>(Self::ptr_filter(f(b"glDeleteQueries\0".as_ptr())).ok_or("glDeleteQueries")?);
    let glDeleteRenderbuffers_p = transmute::<nn_cv, glDeleteRenderbuffers_t>(Self::ptr_filter(f(b"glDeleteRenderbuffers\0".as_ptr())).ok_or("glDeleteRenderbuffers")?);
    let glDeleteSamplers_p = transmute::<nn_cv, glDeleteSamplers_t>(Self::ptr_filter(f(b"glDeleteSamplers\0".as_ptr())).ok_or("glDeleteSamplers")?);
    let glDeleteShader_p = transmute::<nn_cv, glDeleteShader_t>(Self::ptr_filter(f(b"glDeleteShader\0".as_ptr())).ok_or("glDeleteShader")?);
    let glDeleteSync_p = transmute::<nn_cv, glDeleteSync_t>(Self::ptr_filter(f(b"glDeleteSync\0".as_ptr())).ok_or("glDeleteSync")?);
    let glDeleteTextures_p = transmute::<nn_cv, glDeleteTextures_t>(Self::ptr_filter(f(b"glDeleteTextures\0".as_ptr())).ok_or("glDeleteTextures")?);
    let glDeleteVertexArrays_p = transmute::<nn_cv, glDeleteVertexArrays_t>(Self::ptr_filter(f(b"glDeleteVertexArrays\0".as_ptr())).ok_or("glDeleteVertexArrays")?);
    let glDepthFunc_p = transmute::<nn_cv, glDepthFunc_t>(Self::ptr_filter(f(b"glDepthFunc\0".as_ptr())).ok_or("glDepthFunc")?);
    let glDepthMask_p = transmute::<nn_cv, glDepthMask_t>(Self::ptr_filter(f(b"glDepthMask\0".as_ptr())).ok_or("glDepthMask")?);
    let glDepthRange_p = transmute::<nn_cv, glDepthRange_t>(Self::ptr_filter(f(b"glDepthRange\0".as_ptr())).ok_or("glDepthRange")?);
    let glDetachShader_p = transmute::<nn_cv, glDetachShader_t>(Self::ptr_filter(f(b"glDetachShader\0".as_ptr())).ok_or("glDetachShader")?);
    let glDisable_p = transmute::<nn_cv, glDisable_t>(Self::ptr_filter(f(b"glDisable\0".as_ptr())).ok_or("glDisable")?);
    let glDisableVertexAttribArray_p = transmute::<nn_cv, glDisableVertexAttribArray_t>(Self::ptr_filter(f(b"glDisableVertexAttribArray\0".as_ptr())).ok_or("glDisableVertexAttribArray")?);
    let glDisablei_p = transmute::<nn_cv, glDisablei_t>(Self::ptr_filter(f(b"glDisablei\0".as_ptr())).ok_or("glDisablei")?);
    let glDrawArrays_p = transmute::<nn_cv, glDrawArrays_t>(Self::ptr_filter(f(b"glDrawArrays\0".as_ptr())).ok_or("glDrawArrays")?);
    let glDrawArraysInstanced_p = transmute::<nn_cv, glDrawArraysInstanced_t>(Self::ptr_filter(f(b"glDrawArraysInstanced\0".as_ptr())).ok_or("glDrawArraysInstanced")?);
    let glDrawBuffer_p = transmute::<nn_cv, glDrawBuffer_t>(Self::ptr_filter(f(b"glDrawBuffer\0".as_ptr())).ok_or("glDrawBuffer")?);
    let glDrawBuffers_p = transmute::<nn_cv, glDrawBuffers_t>(Self::ptr_filter(f(b"glDrawBuffers\0".as_ptr())).ok_or("glDrawBuffers")?);
    let glDrawElements_p = transmute::<nn_cv, glDrawElements_t>(Self::ptr_filter(f(b"glDrawElements\0".as_ptr())).ok_or("glDrawElements")?);
    let glDrawElementsBaseVertex_p = transmute::<nn_cv, glDrawElementsBaseVertex_t>(Self::ptr_filter(f(b"glDrawElementsBaseVertex\0".as_ptr())).ok_or("glDrawElementsBaseVertex")?);
    let glDrawElementsInstanced_p = transmute::<nn_cv, glDrawElementsInstanced_t>(Self::ptr_filter(f(b"glDrawElementsInstanced\0".as_ptr())).ok_or("glDrawElementsInstanced")?);
    let glDrawElementsInstancedBaseVertex_p = transmute::<nn_cv, glDrawElementsInstancedBaseVertex_t>(Self::ptr_filter(f(b"glDrawElementsInstancedBaseVertex\0".as_ptr())).ok_or("glDrawElementsInstancedBaseVertex")?);
    let glDrawRangeElements_p = transmute::<nn_cv, glDrawRangeElements_t>(Self::ptr_filter(f(b"glDrawRangeElements\0".as_ptr())).ok_or("glDrawRangeElements")?);
    let glDrawRangeElementsBaseVertex_p = transmute::<nn_cv, glDrawRangeElementsBaseVertex_t>(Self::ptr_filter(f(b"glDrawRangeElementsBaseVertex\0".as_ptr())).ok_or("glDrawRangeElementsBaseVertex")?);
    let glEnable_p = transmute::<nn_cv, glEnable_t>(Self::ptr_filter(f(b"glEnable\0".as_ptr())).ok_or("glEnable")?);
    let glEnableVertexAttribArray_p = transmute::<nn_cv, glEnableVertexAttribArray_t>(Self::ptr_filter(f(b"glEnableVertexAttribArray\0".as_ptr())).ok_or("glEnableVertexAttribArray")?);
    let glEnablei_p = transmute::<nn_cv, glEnablei_t>(Self::ptr_filter(f(b"glEnablei\0".as_ptr())).ok_or("glEnablei")?);
    let glEndConditionalRender_p = transmute::<nn_cv, glEndConditionalRender_t>(Self::ptr_filter(f(b"glEndConditionalRender\0".as_ptr())).ok_or("glEndConditionalRender")?);
    let glEndQuery_p = transmute::<nn_cv, glEndQuery_t>(Self::ptr_filter(f(b"glEndQuery\0".as_ptr())).ok_or("glEndQuery")?);
    let glEndTransformFeedback_p = transmute::<nn_cv, glEndTransformFeedback_t>(Self::ptr_filter(f(b"glEndTransformFeedback\0".as_ptr())).ok_or("glEndTransformFeedback")?);
    let glFenceSync_p = transmute::<nn_cv, glFenceSync_t>(Self::ptr_filter(f(b"glFenceSync\0".as_ptr())).ok_or("glFenceSync")?);
    let glFinish_p = transmute::<nn_cv, glFinish_t>(Self::ptr_filter(f(b"glFinish\0".as_ptr())).ok_or("glFinish")?);
    let glFlush_p = transmute::<nn_cv, glFlush_t>(Self::ptr_filter(f(b"glFlush\0".as_ptr())).ok_or("glFlush")?);
    let glFlushMappedBufferRange_p = transmute::<nn_cv, glFlushMappedBufferRange_t>(Self::ptr_filter(f(b"glFlushMappedBufferRange\0".as_ptr())).ok_or("glFlushMappedBufferRange")?);
    let glFramebufferRenderbuffer_p = transmute::<nn_cv, glFramebufferRenderbuffer_t>(Self::ptr_filter(f(b"glFramebufferRenderbuffer\0".as_ptr())).ok_or("glFramebufferRenderbuffer")?);
    let glFramebufferTexture_p = transmute::<nn_cv, glFramebufferTexture_t>(Self::ptr_filter(f(b"glFramebufferTexture\0".as_ptr())).ok_or("glFramebufferTexture")?);
    let glFramebufferTexture1D_p = transmute::<nn_cv, glFramebufferTexture1D_t>(Self::ptr_filter(f(b"glFramebufferTexture1D\0".as_ptr())).ok_or("glFramebufferTexture1D")?);
    let glFramebufferTexture2D_p = transmute::<nn_cv, glFramebufferTexture2D_t>(Self::ptr_filter(f(b"glFramebufferTexture2D\0".as_ptr())).ok_or("glFramebufferTexture2D")?);
    let glFramebufferTexture3D_p = transmute::<nn_cv, glFramebufferTexture3D_t>(Self::ptr_filter(f(b"glFramebufferTexture3D\0".as_ptr())).ok_or("glFramebufferTexture3D")?);
    let glFramebufferTextureLayer_p = transmute::<nn_cv, glFramebufferTextureLayer_t>(Self::ptr_filter(f(b"glFramebufferTextureLayer\0".as_ptr())).ok_or("glFramebufferTextureLayer")?);
    let glFrontFace_p = transmute::<nn_cv, glFrontFace_t>(Self::ptr_filter(f(b"glFrontFace\0".as_ptr())).ok_or("glFrontFace")?);
    let glGenBuffers_p = transmute::<nn_cv, glGenBuffers_t>(Self::ptr_filter(f(b"glGenBuffers\0".as_ptr())).ok_or("glGenBuffers")?);
    let glGenFramebuffers_p = transmute::<nn_cv, glGenFramebuffers_t>(Self::ptr_filter(f(b"glGenFramebuffers\0".as_ptr())).ok_or("glGenFramebuffers")?);
    let glGenQueries_p = transmute::<nn_cv, glGenQueries_t>(Self::ptr_filter(f(b"glGenQueries\0".as_ptr())).ok_or("glGenQueries")?);
    let glGenRenderbuffers_p = transmute::<nn_cv, glGenRenderbuffers_t>(Self::ptr_filter(f(b"glGenRenderbuffers\0".as_ptr())).ok_or("glGenRenderbuffers")?);
    let glGenSamplers_p = transmute::<nn_cv, glGenSamplers_t>(Self::ptr_filter(f(b"glGenSamplers\0".as_ptr())).ok_or("glGenSamplers")?);
    let glGenTextures_p = transmute::<nn_cv, glGenTextures_t>(Self::ptr_filter(f(b"glGenTextures\0".as_ptr())).ok_or("glGenTextures")?);
    let glGenVertexArrays_p = transmute::<nn_cv, glGenVertexArrays_t>(Self::ptr_filter(f(b"glGenVertexArrays\0".as_ptr())).ok_or("glGenVertexArrays")?);
    let glGenerateMipmap_p = transmute::<nn_cv, glGenerateMipmap_t>(Self::ptr_filter(f(b"glGenerateMipmap\0".as_ptr())).ok_or("glGenerateMipmap")?);
    let glGetActiveAttrib_p = transmute::<nn_cv, glGetActiveAttrib_t>(Self::ptr_filter(f(b"glGetActiveAttrib\0".as_ptr())).ok_or("glGetActiveAttrib")?);
    let glGetActiveUniform_p = transmute::<nn_cv, glGetActiveUniform_t>(Self::ptr_filter(f(b"glGetActiveUniform\0".as_ptr())).ok_or("glGetActiveUniform")?);
    let glGetActiveUniformBlockName_p = transmute::<nn_cv, glGetActiveUniformBlockName_t>(Self::ptr_filter(f(b"glGetActiveUniformBlockName\0".as_ptr())).ok_or("glGetActiveUniformBlockName")?);
    let glGetActiveUniformBlockiv_p = transmute::<nn_cv, glGetActiveUniformBlockiv_t>(Self::ptr_filter(f(b"glGetActiveUniformBlockiv\0".as_ptr())).ok_or("glGetActiveUniformBlockiv")?);
    let glGetActiveUniformName_p = transmute::<nn_cv, glGetActiveUniformName_t>(Self::ptr_filter(f(b"glGetActiveUniformName\0".as_ptr())).ok_or("glGetActiveUniformName")?);
    let glGetActiveUniformsiv_p = transmute::<nn_cv, glGetActiveUniformsiv_t>(Self::ptr_filter(f(b"glGetActiveUniformsiv\0".as_ptr())).ok_or("glGetActiveUniformsiv")?);
    let glGetAttachedShaders_p = transmute::<nn_cv, glGetAttachedShaders_t>(Self::ptr_filter(f(b"glGetAttachedShaders\0".as_ptr())).ok_or("glGetAttachedShaders")?);
    let glGetAttribLocation_p = transmute::<nn_cv, glGetAttribLocation_t>(Self::ptr_filter(f(b"glGetAttribLocation\0".as_ptr())).ok_or("glGetAttribLocation")?);
    let glGetBooleani_v_p = transmute::<nn_cv, glGetBooleani_v_t>(Self::ptr_filter(f(b"glGetBooleani_v\0".as_ptr())).ok_or("glGetBooleani_v")?);
    let glGetBooleanv_p = transmute::<nn_cv, glGetBooleanv_t>(Self::ptr_filter(f(b"glGetBooleanv\0".as_ptr())).ok_or("glGetBooleanv")?);
    let glGetBufferParameteri64v_p = transmute::<nn_cv, glGetBufferParameteri64v_t>(Self::ptr_filter(f(b"glGetBufferParameteri64v\0".as_ptr())).ok_or("glGetBufferParameteri64v")?);
    let glGetBufferParameteriv_p = transmute::<nn_cv, glGetBufferParameteriv_t>(Self::ptr_filter(f(b"glGetBufferParameteriv\0".as_ptr())).ok_or("glGetBufferParameteriv")?);
    let glGetBufferPointerv_p = transmute::<nn_cv, glGetBufferPointerv_t>(Self::ptr_filter(f(b"glGetBufferPointerv\0".as_ptr())).ok_or("glGetBufferPointerv")?);
    let glGetBufferSubData_p = transmute::<nn_cv, glGetBufferSubData_t>(Self::ptr_filter(f(b"glGetBufferSubData\0".as_ptr())).ok_or("glGetBufferSubData")?);
    let glGetCompressedTexImage_p = transmute::<nn_cv, glGetCompressedTexImage_t>(Self::ptr_filter(f(b"glGetCompressedTexImage\0".as_ptr())).ok_or("glGetCompressedTexImage")?);
    let glGetDoublev_p = transmute::<nn_cv, glGetDoublev_t>(Self::ptr_filter(f(b"glGetDoublev\0".as_ptr())).ok_or("glGetDoublev")?);
    let glGetError_p = transmute::<nn_cv, glGetError_t>(Self::ptr_filter(f(b"glGetError\0".as_ptr())).ok_or("glGetError")?);
    let glGetFloatv_p = transmute::<nn_cv, glGetFloatv_t>(Self::ptr_filter(f(b"glGetFloatv\0".as_ptr())).ok_or("glGetFloatv")?);
    let glGetFragDataIndex_p = transmute::<nn_cv, glGetFragDataIndex_t>(Self::ptr_filter(f(b"glGetFragDataIndex\0".as_ptr())).ok_or("glGetFragDataIndex")?);
    let glGetFragDataLocation_p = transmute::<nn_cv, glGetFragDataLocation_t>(Self::ptr_filter(f(b"glGetFragDataLocation\0".as_ptr())).ok_or("glGetFragDataLocation")?);
    let glGetFramebufferAttachmentParameteriv_p = transmute::<nn_cv, glGetFramebufferAttachmentParameteriv_t>(Self::ptr_filter(f(b"glGetFramebufferAttachmentParameteriv\0".as_ptr())).ok_or("glGetFramebufferAttachmentParameteriv")?);
    let glGetInteger64i_v_p = transmute::<nn_cv, glGetInteger64i_v_t>(Self::ptr_filter(f(b"glGetInteger64i_v\0".as_ptr())).ok_or("glGetInteger64i_v")?);
    let glGetInteger64v_p = transmute::<nn_cv, glGetInteger64v_t>(Self::ptr_filter(f(b"glGetInteger64v\0".as_ptr())).ok_or("glGetInteger64v")?);
    let glGetIntegeri_v_p = transmute::<nn_cv, glGetIntegeri_v_t>(Self::ptr_filter(f(b"glGetIntegeri_v\0".as_ptr())).ok_or("glGetIntegeri_v")?);
    let glGetIntegerv_p = transmute::<nn_cv, glGetIntegerv_t>(Self::ptr_filter(f(b"glGetIntegerv\0".as_ptr())).ok_or("glGetIntegerv")?);
    let glGetMultisamplefv_p = transmute::<nn_cv, glGetMultisamplefv_t>(Self::ptr_filter(f(b"glGetMultisamplefv\0".as_ptr())).ok_or("glGetMultisamplefv")?);
    let glGetProgramInfoLog_p = transmute::<nn_cv, glGetProgramInfoLog_t>(Self::ptr_filter(f(b"glGetProgramInfoLog\0".as_ptr())).ok_or("glGetProgramInfoLog")?);
    let glGetProgramiv_p = transmute::<nn_cv, glGetProgramiv_t>(Self::ptr_filter(f(b"glGetProgramiv\0".as_ptr())).ok_or("glGetProgramiv")?);
    let glGetQueryObjecti64v_p = transmute::<nn_cv, glGetQueryObjecti64v_t>(Self::ptr_filter(f(b"glGetQueryObjecti64v\0".as_ptr())).ok_or("glGetQueryObjecti64v")?);
    let glGetQueryObjectiv_p = transmute::<nn_cv, glGetQueryObjectiv_t>(Self::ptr_filter(f(b"glGetQueryObjectiv\0".as_ptr())).ok_or("glGetQueryObjectiv")?);
    let glGetQueryObjectui64v_p = transmute::<nn_cv, glGetQueryObjectui64v_t>(Self::ptr_filter(f(b"glGetQueryObjectui64v\0".as_ptr())).ok_or("glGetQueryObjectui64v")?);
    let glGetQueryObjectuiv_p = transmute::<nn_cv, glGetQueryObjectuiv_t>(Self::ptr_filter(f(b"glGetQueryObjectuiv\0".as_ptr())).ok_or("glGetQueryObjectuiv")?);
    let glGetQueryiv_p = transmute::<nn_cv, glGetQueryiv_t>(Self::ptr_filter(f(b"glGetQueryiv\0".as_ptr())).ok_or("glGetQueryiv")?);
    let glGetRenderbufferParameteriv_p = transmute::<nn_cv, glGetRenderbufferParameteriv_t>(Self::ptr_filter(f(b"glGetRenderbufferParameteriv\0".as_ptr())).ok_or("glGetRenderbufferParameteriv")?);
    let glGetSamplerParameterIiv_p = transmute::<nn_cv, glGetSamplerParameterIiv_t>(Self::ptr_filter(f(b"glGetSamplerParameterIiv\0".as_ptr())).ok_or("glGetSamplerParameterIiv")?);
    let glGetSamplerParameterIuiv_p = transmute::<nn_cv, glGetSamplerParameterIuiv_t>(Self::ptr_filter(f(b"glGetSamplerParameterIuiv\0".as_ptr())).ok_or("glGetSamplerParameterIuiv")?);
    let glGetSamplerParameterfv_p = transmute::<nn_cv, glGetSamplerParameterfv_t>(Self::ptr_filter(f(b"glGetSamplerParameterfv\0".as_ptr())).ok_or("glGetSamplerParameterfv")?);
    let glGetSamplerParameteriv_p = transmute::<nn_cv, glGetSamplerParameteriv_t>(Self::ptr_filter(f(b"glGetSamplerParameteriv\0".as_ptr())).ok_or("glGetSamplerParameteriv")?);
    let glGetShaderInfoLog_p = transmute::<nn_cv, glGetShaderInfoLog_t>(Self::ptr_filter(f(b"glGetShaderInfoLog\0".as_ptr())).ok_or("glGetShaderInfoLog")?);
    let glGetShaderSource_p = transmute::<nn_cv, glGetShaderSource_t>(Self::ptr_filter(f(b"glGetShaderSource\0".as_ptr())).ok_or("glGetShaderSource")?);
    let glGetShaderiv_p = transmute::<nn_cv, glGetShaderiv_t>(Self::ptr_filter(f(b"glGetShaderiv\0".as_ptr())).ok_or("glGetShaderiv")?);
    let glGetString_p = transmute::<nn_cv, glGetString_t>(Self::ptr_filter(f(b"glGetString\0".as_ptr())).ok_or("glGetString")?);
    let glGetStringi_p = transmute::<nn_cv, glGetStringi_t>(Self::ptr_filter(f(b"glGetStringi\0".as_ptr())).ok_or("glGetStringi")?);
    let glGetSynciv_p = transmute::<nn_cv, glGetSynciv_t>(Self::ptr_filter(f(b"glGetSynciv\0".as_ptr())).ok_or("glGetSynciv")?);
    let glGetTexImage_p = transmute::<nn_cv, glGetTexImage_t>(Self::ptr_filter(f(b"glGetTexImage\0".as_ptr())).ok_or("glGetTexImage")?);
    let glGetTexLevelParameterfv_p = transmute::<nn_cv, glGetTexLevelParameterfv_t>(Self::ptr_filter(f(b"glGetTexLevelParameterfv\0".as_ptr())).ok_or("glGetTexLevelParameterfv")?);
    let glGetTexLevelParameteriv_p = transmute::<nn_cv, glGetTexLevelParameteriv_t>(Self::ptr_filter(f(b"glGetTexLevelParameteriv\0".as_ptr())).ok_or("glGetTexLevelParameteriv")?);
    let glGetTexParameterIiv_p = transmute::<nn_cv, glGetTexParameterIiv_t>(Self::ptr_filter(f(b"glGetTexParameterIiv\0".as_ptr())).ok_or("glGetTexParameterIiv")?);
    let glGetTexParameterIuiv_p = transmute::<nn_cv, glGetTexParameterIuiv_t>(Self::ptr_filter(f(b"glGetTexParameterIuiv\0".as_ptr())).ok_or("glGetTexParameterIuiv")?);
    let glGetTexParameterfv_p = transmute::<nn_cv, glGetTexParameterfv_t>(Self::ptr_filter(f(b"glGetTexParameterfv\0".as_ptr())).ok_or("glGetTexParameterfv")?);
    let glGetTexParameteriv_p = transmute::<nn_cv, glGetTexParameteriv_t>(Self::ptr_filter(f(b"glGetTexParameteriv\0".as_ptr())).ok_or("glGetTexParameteriv")?);
    let glGetTransformFeedbackVarying_p = transmute::<nn_cv, glGetTransformFeedbackVarying_t>(Self::ptr_filter(f(b"glGetTransformFeedbackVarying\0".as_ptr())).ok_or("glGetTransformFeedbackVarying")?);
    let glGetUniformBlockIndex_p = transmute::<nn_cv, glGetUniformBlockIndex_t>(Self::ptr_filter(f(b"glGetUniformBlockIndex\0".as_ptr())).ok_or("glGetUniformBlockIndex")?);
    let glGetUniformIndices_p = transmute::<nn_cv, glGetUniformIndices_t>(Self::ptr_filter(f(b"glGetUniformIndices\0".as_ptr())).ok_or("glGetUniformIndices")?);
    let glGetUniformLocation_p = transmute::<nn_cv, glGetUniformLocation_t>(Self::ptr_filter(f(b"glGetUniformLocation\0".as_ptr())).ok_or("glGetUniformLocation")?);
    let glGetUniformfv_p = transmute::<nn_cv, glGetUniformfv_t>(Self::ptr_filter(f(b"glGetUniformfv\0".as_ptr())).ok_or("glGetUniformfv")?);
    let glGetUniformiv_p = transmute::<nn_cv, glGetUniformiv_t>(Self::ptr_filter(f(b"glGetUniformiv\0".as_ptr())).ok_or("glGetUniformiv")?);
    let glGetUniformuiv_p = transmute::<nn_cv, glGetUniformuiv_t>(Self::ptr_filter(f(b"glGetUniformuiv\0".as_ptr())).ok_or("glGetUniformuiv")?);
    let glGetVertexAttribIiv_p = transmute::<nn_cv, glGetVertexAttribIiv_t>(Self::ptr_filter(f(b"glGetVertexAttribIiv\0".as_ptr())).ok_or("glGetVertexAttribIiv")?);
    let glGetVertexAttribIuiv_p = transmute::<nn_cv, glGetVertexAttribIuiv_t>(Self::ptr_filter(f(b"glGetVertexAttribIuiv\0".as_ptr())).ok_or("glGetVertexAttribIuiv")?);
    let glGetVertexAttribPointerv_p = transmute::<nn_cv, glGetVertexAttribPointerv_t>(Self::ptr_filter(f(b"glGetVertexAttribPointerv\0".as_ptr())).ok_or("glGetVertexAttribPointerv")?);
    let glGetVertexAttribdv_p = transmute::<nn_cv, glGetVertexAttribdv_t>(Self::ptr_filter(f(b"glGetVertexAttribdv\0".as_ptr())).ok_or("glGetVertexAttribdv")?);
    let glGetVertexAttribfv_p = transmute::<nn_cv, glGetVertexAttribfv_t>(Self::ptr_filter(f(b"glGetVertexAttribfv\0".as_ptr())).ok_or("glGetVertexAttribfv")?);
    let glGetVertexAttribiv_p = transmute::<nn_cv, glGetVertexAttribiv_t>(Self::ptr_filter(f(b"glGetVertexAttribiv\0".as_ptr())).ok_or("glGetVertexAttribiv")?);
    let glHint_p = transmute::<nn_cv, glHint_t>(Self::ptr_filter(f(b"glHint\0".as_ptr())).ok_or("glHint")?);
    let glIsBuffer_p = transmute::<nn_cv, glIsBuffer_t>(Self::ptr_filter(f(b"glIsBuffer\0".as_ptr())).ok_or("glIsBuffer")?);
    let glIsEnabled_p = transmute::<nn_cv, glIsEnabled_t>(Self::ptr_filter(f(b"glIsEnabled\0".as_ptr())).ok_or("glIsEnabled")?);
    let glIsEnabledi_p = transmute::<nn_cv, glIsEnabledi_t>(Self::ptr_filter(f(b"glIsEnabledi\0".as_ptr())).ok_or("glIsEnabledi")?);
    let glIsFramebuffer_p = transmute::<nn_cv, glIsFramebuffer_t>(Self::ptr_filter(f(b"glIsFramebuffer\0".as_ptr())).ok_or("glIsFramebuffer")?);
    let glIsProgram_p = transmute::<nn_cv, glIsProgram_t>(Self::ptr_filter(f(b"glIsProgram\0".as_ptr())).ok_or("glIsProgram")?);
    let glIsQuery_p = transmute::<nn_cv, glIsQuery_t>(Self::ptr_filter(f(b"glIsQuery\0".as_ptr())).ok_or("glIsQuery")?);
    let glIsRenderbuffer_p = transmute::<nn_cv, glIsRenderbuffer_t>(Self::ptr_filter(f(b"glIsRenderbuffer\0".as_ptr())).ok_or("glIsRenderbuffer")?);
    let glIsSampler_p = transmute::<nn_cv, glIsSampler_t>(Self::ptr_filter(f(b"glIsSampler\0".as_ptr())).ok_or("glIsSampler")?);
    let glIsShader_p = transmute::<nn_cv, glIsShader_t>(Self::ptr_filter(f(b"glIsShader\0".as_ptr())).ok_or("glIsShader")?);
    let glIsSync_p = transmute::<nn_cv, glIsSync_t>(Self::ptr_filter(f(b"glIsSync\0".as_ptr())).ok_or("glIsSync")?);
    let glIsTexture_p = transmute::<nn_cv, glIsTexture_t>(Self::ptr_filter(f(b"glIsTexture\0".as_ptr())).ok_or("glIsTexture")?);
    let glIsVertexArray_p = transmute::<nn_cv, glIsVertexArray_t>(Self::ptr_filter(f(b"glIsVertexArray\0".as_ptr())).ok_or("glIsVertexArray")?);
    let glLineWidth_p = transmute::<nn_cv, glLineWidth_t>(Self::ptr_filter(f(b"glLineWidth\0".as_ptr())).ok_or("glLineWidth")?);
    let glLinkProgram_p = transmute::<nn_cv, glLinkProgram_t>(Self::ptr_filter(f(b"glLinkProgram\0".as_ptr())).ok_or("glLinkProgram")?);
    let glLogicOp_p = transmute::<nn_cv, glLogicOp_t>(Self::ptr_filter(f(b"glLogicOp\0".as_ptr())).ok_or("glLogicOp")?);
    let glMapBuffer_p = transmute::<nn_cv, glMapBuffer_t>(Self::ptr_filter(f(b"glMapBuffer\0".as_ptr())).ok_or("glMapBuffer")?);
    let glMapBufferRange_p = transmute::<nn_cv, glMapBufferRange_t>(Self::ptr_filter(f(b"glMapBufferRange\0".as_ptr())).ok_or("glMapBufferRange")?);
    let glMultiDrawArrays_p = transmute::<nn_cv, glMultiDrawArrays_t>(Self::ptr_filter(f(b"glMultiDrawArrays\0".as_ptr())).ok_or("glMultiDrawArrays")?);
    let glMultiDrawElements_p = transmute::<nn_cv, glMultiDrawElements_t>(Self::ptr_filter(f(b"glMultiDrawElements\0".as_ptr())).ok_or("glMultiDrawElements")?);
    let glMultiDrawElementsBaseVertex_p = transmute::<nn_cv, glMultiDrawElementsBaseVertex_t>(Self::ptr_filter(f(b"glMultiDrawElementsBaseVertex\0".as_ptr())).ok_or("glMultiDrawElementsBaseVertex")?);
    let glPixelStoref_p = transmute::<nn_cv, glPixelStoref_t>(Self::ptr_filter(f(b"glPixelStoref\0".as_ptr())).ok_or("glPixelStoref")?);
    let glPixelStorei_p = transmute::<nn_cv, glPixelStorei_t>(Self::ptr_filter(f(b"glPixelStorei\0".as_ptr())).ok_or("glPixelStorei")?);
    let glPointParameterf_p = transmute::<nn_cv, glPointParameterf_t>(Self::ptr_filter(f(b"glPointParameterf\0".as_ptr())).ok_or("glPointParameterf")?);
    let glPointParameterfv_p = transmute::<nn_cv, glPointParameterfv_t>(Self::ptr_filter(f(b"glPointParameterfv\0".as_ptr())).ok_or("glPointParameterfv")?);
    let glPointParameteri_p = transmute::<nn_cv, glPointParameteri_t>(Self::ptr_filter(f(b"glPointParameteri\0".as_ptr())).ok_or("glPointParameteri")?);
    let glPointParameteriv_p = transmute::<nn_cv, glPointParameteriv_t>(Self::ptr_filter(f(b"glPointParameteriv\0".as_ptr())).ok_or("glPointParameteriv")?);
    let glPointSize_p = transmute::<nn_cv, glPointSize_t>(Self::ptr_filter(f(b"glPointSize\0".as_ptr())).ok_or("glPointSize")?);
    let glPolygonMode_p = transmute::<nn_cv, glPolygonMode_t>(Self::ptr_filter(f(b"glPolygonMode\0".as_ptr())).ok_or("glPolygonMode")?);
    let glPolygonOffset_p = transmute::<nn_cv, glPolygonOffset_t>(Self::ptr_filter(f(b"glPolygonOffset\0".as_ptr())).ok_or("glPolygonOffset")?);
    let glPrimitiveRestartIndex_p = transmute::<nn_cv, glPrimitiveRestartIndex_t>(Self::ptr_filter(f(b"glPrimitiveRestartIndex\0".as_ptr())).ok_or("glPrimitiveRestartIndex")?);
    let glProvokingVertex_p = transmute::<nn_cv, glProvokingVertex_t>(Self::ptr_filter(f(b"glProvokingVertex\0".as_ptr())).ok_or("glProvokingVertex")?);
    let glQueryCounter_p = transmute::<nn_cv, glQueryCounter_t>(Self::ptr_filter(f(b"glQueryCounter\0".as_ptr())).ok_or("glQueryCounter")?);
    let glReadBuffer_p = transmute::<nn_cv, glReadBuffer_t>(Self::ptr_filter(f(b"glReadBuffer\0".as_ptr())).ok_or("glReadBuffer")?);
    let glReadPixels_p = transmute::<nn_cv, glReadPixels_t>(Self::ptr_filter(f(b"glReadPixels\0".as_ptr())).ok_or("glReadPixels")?);
    let glRenderbufferStorage_p = transmute::<nn_cv, glRenderbufferStorage_t>(Self::ptr_filter(f(b"glRenderbufferStorage\0".as_ptr())).ok_or("glRenderbufferStorage")?);
    let glRenderbufferStorageMultisample_p = transmute::<nn_cv, glRenderbufferStorageMultisample_t>(Self::ptr_filter(f(b"glRenderbufferStorageMultisample\0".as_ptr())).ok_or("glRenderbufferStorageMultisample")?);
    let glSampleCoverage_p = transmute::<nn_cv, glSampleCoverage_t>(Self::ptr_filter(f(b"glSampleCoverage\0".as_ptr())).ok_or("glSampleCoverage")?);
    let glSampleMaski_p = transmute::<nn_cv, glSampleMaski_t>(Self::ptr_filter(f(b"glSampleMaski\0".as_ptr())).ok_or("glSampleMaski")?);
    let glSamplerParameterIiv_p = transmute::<nn_cv, glSamplerParameterIiv_t>(Self::ptr_filter(f(b"glSamplerParameterIiv\0".as_ptr())).ok_or("glSamplerParameterIiv")?);
    let glSamplerParameterIuiv_p = transmute::<nn_cv, glSamplerParameterIuiv_t>(Self::ptr_filter(f(b"glSamplerParameterIuiv\0".as_ptr())).ok_or("glSamplerParameterIuiv")?);
    let glSamplerParameterf_p = transmute::<nn_cv, glSamplerParameterf_t>(Self::ptr_filter(f(b"glSamplerParameterf\0".as_ptr())).ok_or("glSamplerParameterf")?);
    let glSamplerParameterfv_p = transmute::<nn_cv, glSamplerParameterfv_t>(Self::ptr_filter(f(b"glSamplerParameterfv\0".as_ptr())).ok_or("glSamplerParameterfv")?);
    let glSamplerParameteri_p = transmute::<nn_cv, glSamplerParameteri_t>(Self::ptr_filter(f(b"glSamplerParameteri\0".as_ptr())).ok_or("glSamplerParameteri")?);
    let glSamplerParameteriv_p = transmute::<nn_cv, glSamplerParameteriv_t>(Self::ptr_filter(f(b"glSamplerParameteriv\0".as_ptr())).ok_or("glSamplerParameteriv")?);
    let glScissor_p = transmute::<nn_cv, glScissor_t>(Self::ptr_filter(f(b"glScissor\0".as_ptr())).ok_or("glScissor")?);
    let glShaderSource_p = transmute::<nn_cv, glShaderSource_t>(Self::ptr_filter(f(b"glShaderSource\0".as_ptr())).ok_or("glShaderSource")?);
    let glStencilFunc_p = transmute::<nn_cv, glStencilFunc_t>(Self::ptr_filter(f(b"glStencilFunc\0".as_ptr())).ok_or("glStencilFunc")?);
    let glStencilFuncSeparate_p = transmute::<nn_cv, glStencilFuncSeparate_t>(Self::ptr_filter(f(b"glStencilFuncSeparate\0".as_ptr())).ok_or("glStencilFuncSeparate")?);
    let glStencilMask_p = transmute::<nn_cv, glStencilMask_t>(Self::ptr_filter(f(b"glStencilMask\0".as_ptr())).ok_or("glStencilMask")?);
    let glStencilMaskSeparate_p = transmute::<nn_cv, glStencilMaskSeparate_t>(Self::ptr_filter(f(b"glStencilMaskSeparate\0".as_ptr())).ok_or("glStencilMaskSeparate")?);
    let glStencilOp_p = transmute::<nn_cv, glStencilOp_t>(Self::ptr_filter(f(b"glStencilOp\0".as_ptr())).ok_or("glStencilOp")?);
    let glStencilOpSeparate_p = transmute::<nn_cv, glStencilOpSeparate_t>(Self::ptr_filter(f(b"glStencilOpSeparate\0".as_ptr())).ok_or("glStencilOpSeparate")?);
    let glTexBuffer_p = transmute::<nn_cv, glTexBuffer_t>(Self::ptr_filter(f(b"glTexBuffer\0".as_ptr())).ok_or("glTexBuffer")?);
    let glTexImage1D_p = transmute::<nn_cv, glTexImage1D_t>(Self::ptr_filter(f(b"glTexImage1D\0".as_ptr())).ok_or("glTexImage1D")?);
    let glTexImage2D_p = transmute::<nn_cv, glTexImage2D_t>(Self::ptr_filter(f(b"glTexImage2D\0".as_ptr())).ok_or("glTexImage2D")?);
    let glTexImage2DMultisample_p = transmute::<nn_cv, glTexImage2DMultisample_t>(Self::ptr_filter(f(b"glTexImage2DMultisample\0".as_ptr())).ok_or("glTexImage2DMultisample")?);
    let glTexImage3D_p = transmute::<nn_cv, glTexImage3D_t>(Self::ptr_filter(f(b"glTexImage3D\0".as_ptr())).ok_or("glTexImage3D")?);
    let glTexImage3DMultisample_p = transmute::<nn_cv, glTexImage3DMultisample_t>(Self::ptr_filter(f(b"glTexImage3DMultisample\0".as_ptr())).ok_or("glTexImage3DMultisample")?);
    let glTexParameterIiv_p = transmute::<nn_cv, glTexParameterIiv_t>(Self::ptr_filter(f(b"glTexParameterIiv\0".as_ptr())).ok_or("glTexParameterIiv")?);
    let glTexParameterIuiv_p = transmute::<nn_cv, glTexParameterIuiv_t>(Self::ptr_filter(f(b"glTexParameterIuiv\0".as_ptr())).ok_or("glTexParameterIuiv")?);
    let glTexParameterf_p = transmute::<nn_cv, glTexParameterf_t>(Self::ptr_filter(f(b"glTexParameterf\0".as_ptr())).ok_or("glTexParameterf")?);
    let glTexParameterfv_p = transmute::<nn_cv, glTexParameterfv_t>(Self::ptr_filter(f(b"glTexParameterfv\0".as_ptr())).ok_or("glTexParameterfv")?);
    let glTexParameteri_p = transmute::<nn_cv, glTexParameteri_t>(Self::ptr_filter(f(b"glTexParameteri\0".as_ptr())).ok_or("glTexParameteri")?);
    let glTexParameteriv_p = transmute::<nn_cv, glTexParameteriv_t>(Self::ptr_filter(f(b"glTexParameteriv\0".as_ptr())).ok_or("glTexParameteriv")?);
    let glTexSubImage1D_p = transmute::<nn_cv, glTexSubImage1D_t>(Self::ptr_filter(f(b"glTexSubImage1D\0".as_ptr())).ok_or("glTexSubImage1D")?);
    let glTexSubImage2D_p = transmute::<nn_cv, glTexSubImage2D_t>(Self::ptr_filter(f(b"glTexSubImage2D\0".as_ptr())).ok_or("glTexSubImage2D")?);
    let glTexSubImage3D_p = transmute::<nn_cv, glTexSubImage3D_t>(Self::ptr_filter(f(b"glTexSubImage3D\0".as_ptr())).ok_or("glTexSubImage3D")?);
    let glTransformFeedbackVaryings_p = transmute::<nn_cv, glTransformFeedbackVaryings_t>(Self::ptr_filter(f(b"glTransformFeedbackVaryings\0".as_ptr())).ok_or("glTransformFeedbackVaryings")?);
    let glUniform1f_p = transmute::<nn_cv, glUniform1f_t>(Self::ptr_filter(f(b"glUniform1f\0".as_ptr())).ok_or("glUniform1f")?);
    let glUniform1fv_p = transmute::<nn_cv, glUniform1fv_t>(Self::ptr_filter(f(b"glUniform1fv\0".as_ptr())).ok_or("glUniform1fv")?);
    let glUniform1i_p = transmute::<nn_cv, glUniform1i_t>(Self::ptr_filter(f(b"glUniform1i\0".as_ptr())).ok_or("glUniform1i")?);
    let glUniform1iv_p = transmute::<nn_cv, glUniform1iv_t>(Self::ptr_filter(f(b"glUniform1iv\0".as_ptr())).ok_or("glUniform1iv")?);
    let glUniform1ui_p = transmute::<nn_cv, glUniform1ui_t>(Self::ptr_filter(f(b"glUniform1ui\0".as_ptr())).ok_or("glUniform1ui")?);
    let glUniform1uiv_p = transmute::<nn_cv, glUniform1uiv_t>(Self::ptr_filter(f(b"glUniform1uiv\0".as_ptr())).ok_or("glUniform1uiv")?);
    let glUniform2f_p = transmute::<nn_cv, glUniform2f_t>(Self::ptr_filter(f(b"glUniform2f\0".as_ptr())).ok_or("glUniform2f")?);
    let glUniform2fv_p = transmute::<nn_cv, glUniform2fv_t>(Self::ptr_filter(f(b"glUniform2fv\0".as_ptr())).ok_or("glUniform2fv")?);
    let glUniform2i_p = transmute::<nn_cv, glUniform2i_t>(Self::ptr_filter(f(b"glUniform2i\0".as_ptr())).ok_or("glUniform2i")?);
    let glUniform2iv_p = transmute::<nn_cv, glUniform2iv_t>(Self::ptr_filter(f(b"glUniform2iv\0".as_ptr())).ok_or("glUniform2iv")?);
    let glUniform2ui_p = transmute::<nn_cv, glUniform2ui_t>(Self::ptr_filter(f(b"glUniform2ui\0".as_ptr())).ok_or("glUniform2ui")?);
    let glUniform2uiv_p = transmute::<nn_cv, glUniform2uiv_t>(Self::ptr_filter(f(b"glUniform2uiv\0".as_ptr())).ok_or("glUniform2uiv")?);
    let glUniform3f_p = transmute::<nn_cv, glUniform3f_t>(Self::ptr_filter(f(b"glUniform3f\0".as_ptr())).ok_or("glUniform3f")?);
    let glUniform3fv_p = transmute::<nn_cv, glUniform3fv_t>(Self::ptr_filter(f(b"glUniform3fv\0".as_ptr())).ok_or("glUniform3fv")?);
    let glUniform3i_p = transmute::<nn_cv, glUniform3i_t>(Self::ptr_filter(f(b"glUniform3i\0".as_ptr())).ok_or("glUniform3i")?);
    let glUniform3iv_p = transmute::<nn_cv, glUniform3iv_t>(Self::ptr_filter(f(b"glUniform3iv\0".as_ptr())).ok_or("glUniform3iv")?);
    let glUniform3ui_p = transmute::<nn_cv, glUniform3ui_t>(Self::ptr_filter(f(b"glUniform3ui\0".as_ptr())).ok_or("glUniform3ui")?);
    let glUniform3uiv_p = transmute::<nn_cv, glUniform3uiv_t>(Self::ptr_filter(f(b"glUniform3uiv\0".as_ptr())).ok_or("glUniform3uiv")?);
    let glUniform4f_p = transmute::<nn_cv, glUniform4f_t>(Self::ptr_filter(f(b"glUniform4f\0".as_ptr())).ok_or("glUniform4f")?);
    let glUniform4fv_p = transmute::<nn_cv, glUniform4fv_t>(Self::ptr_filter(f(b"glUniform4fv\0".as_ptr())).ok_or("glUniform4fv")?);
    let glUniform4i_p = transmute::<nn_cv, glUniform4i_t>(Self::ptr_filter(f(b"glUniform4i\0".as_ptr())).ok_or("glUniform4i")?);
    let glUniform4iv_p = transmute::<nn_cv, glUniform4iv_t>(Self::ptr_filter(f(b"glUniform4iv\0".as_ptr())).ok_or("glUniform4iv")?);
    let glUniform4ui_p = transmute::<nn_cv, glUniform4ui_t>(Self::ptr_filter(f(b"glUniform4ui\0".as_ptr())).ok_or("glUniform4ui")?);
    let glUniform4uiv_p = transmute::<nn_cv, glUniform4uiv_t>(Self::ptr_filter(f(b"glUniform4uiv\0".as_ptr())).ok_or("glUniform4uiv")?);
    let glUniformBlockBinding_p = transmute::<nn_cv, glUniformBlockBinding_t>(Self::ptr_filter(f(b"glUniformBlockBinding\0".as_ptr())).ok_or("glUniformBlockBinding")?);
    let glUniformMatrix2fv_p = transmute::<nn_cv, glUniformMatrix2fv_t>(Self::ptr_filter(f(b"glUniformMatrix2fv\0".as_ptr())).ok_or("glUniformMatrix2fv")?);
    let glUniformMatrix2x3fv_p = transmute::<nn_cv, glUniformMatrix2x3fv_t>(Self::ptr_filter(f(b"glUniformMatrix2x3fv\0".as_ptr())).ok_or("glUniformMatrix2x3fv")?);
    let glUniformMatrix2x4fv_p = transmute::<nn_cv, glUniformMatrix2x4fv_t>(Self::ptr_filter(f(b"glUniformMatrix2x4fv\0".as_ptr())).ok_or("glUniformMatrix2x4fv")?);
    let glUniformMatrix3fv_p = transmute::<nn_cv, glUniformMatrix3fv_t>(Self::ptr_filter(f(b"glUniformMatrix3fv\0".as_ptr())).ok_or("glUniformMatrix3fv")?);
    let glUniformMatrix3x2fv_p = transmute::<nn_cv, glUniformMatrix3x2fv_t>(Self::ptr_filter(f(b"glUniformMatrix3x2fv\0".as_ptr())).ok_or("glUniformMatrix3x2fv")?);
    let glUniformMatrix3x4fv_p = transmute::<nn_cv, glUniformMatrix3x4fv_t>(Self::ptr_filter(f(b"glUniformMatrix3x4fv\0".as_ptr())).ok_or("glUniformMatrix3x4fv")?);
    let glUniformMatrix4fv_p = transmute::<nn_cv, glUniformMatrix4fv_t>(Self::ptr_filter(f(b"glUniformMatrix4fv\0".as_ptr())).ok_or("glUniformMatrix4fv")?);
    let glUniformMatrix4x2fv_p = transmute::<nn_cv, glUniformMatrix4x2fv_t>(Self::ptr_filter(f(b"glUniformMatrix4x2fv\0".as_ptr())).ok_or("glUniformMatrix4x2fv")?);
    let glUniformMatrix4x3fv_p = transmute::<nn_cv, glUniformMatrix4x3fv_t>(Self::ptr_filter(f(b"glUniformMatrix4x3fv\0".as_ptr())).ok_or("glUniformMatrix4x3fv")?);
    let glUnmapBuffer_p = transmute::<nn_cv, glUnmapBuffer_t>(Self::ptr_filter(f(b"glUnmapBuffer\0".as_ptr())).ok_or("glUnmapBuffer")?);
    let glUseProgram_p = transmute::<nn_cv, glUseProgram_t>(Self::ptr_filter(f(b"glUseProgram\0".as_ptr())).ok_or("glUseProgram")?);
    let glValidateProgram_p = transmute::<nn_cv, glValidateProgram_t>(Self::ptr_filter(f(b"glValidateProgram\0".as_ptr())).ok_or("glValidateProgram")?);
    let glVertexAttrib1d_p = transmute::<nn_cv, glVertexAttrib1d_t>(Self::ptr_filter(f(b"glVertexAttrib1d\0".as_ptr())).ok_or("glVertexAttrib1d")?);
    let glVertexAttrib1dv_p = transmute::<nn_cv, glVertexAttrib1dv_t>(Self::ptr_filter(f(b"glVertexAttrib1dv\0".as_ptr())).ok_or("glVertexAttrib1dv")?);
    let glVertexAttrib1f_p = transmute::<nn_cv, glVertexAttrib1f_t>(Self::ptr_filter(f(b"glVertexAttrib1f\0".as_ptr())).ok_or("glVertexAttrib1f")?);
    let glVertexAttrib1fv_p = transmute::<nn_cv, glVertexAttrib1fv_t>(Self::ptr_filter(f(b"glVertexAttrib1fv\0".as_ptr())).ok_or("glVertexAttrib1fv")?);
    let glVertexAttrib1s_p = transmute::<nn_cv, glVertexAttrib1s_t>(Self::ptr_filter(f(b"glVertexAttrib1s\0".as_ptr())).ok_or("glVertexAttrib1s")?);
    let glVertexAttrib1sv_p = transmute::<nn_cv, glVertexAttrib1sv_t>(Self::ptr_filter(f(b"glVertexAttrib1sv\0".as_ptr())).ok_or("glVertexAttrib1sv")?);
    let glVertexAttrib2d_p = transmute::<nn_cv, glVertexAttrib2d_t>(Self::ptr_filter(f(b"glVertexAttrib2d\0".as_ptr())).ok_or("glVertexAttrib2d")?);
    let glVertexAttrib2dv_p = transmute::<nn_cv, glVertexAttrib2dv_t>(Self::ptr_filter(f(b"glVertexAttrib2dv\0".as_ptr())).ok_or("glVertexAttrib2dv")?);
    let glVertexAttrib2f_p = transmute::<nn_cv, glVertexAttrib2f_t>(Self::ptr_filter(f(b"glVertexAttrib2f\0".as_ptr())).ok_or("glVertexAttrib2f")?);
    let glVertexAttrib2fv_p = transmute::<nn_cv, glVertexAttrib2fv_t>(Self::ptr_filter(f(b"glVertexAttrib2fv\0".as_ptr())).ok_or("glVertexAttrib2fv")?);
    let glVertexAttrib2s_p = transmute::<nn_cv, glVertexAttrib2s_t>(Self::ptr_filter(f(b"glVertexAttrib2s\0".as_ptr())).ok_or("glVertexAttrib2s")?);
    let glVertexAttrib2sv_p = transmute::<nn_cv, glVertexAttrib2sv_t>(Self::ptr_filter(f(b"glVertexAttrib2sv\0".as_ptr())).ok_or("glVertexAttrib2sv")?);
    let glVertexAttrib3d_p = transmute::<nn_cv, glVertexAttrib3d_t>(Self::ptr_filter(f(b"glVertexAttrib3d\0".as_ptr())).ok_or("glVertexAttrib3d")?);
    let glVertexAttrib3dv_p = transmute::<nn_cv, glVertexAttrib3dv_t>(Self::ptr_filter(f(b"glVertexAttrib3dv\0".as_ptr())).ok_or("glVertexAttrib3dv")?);
    let glVertexAttrib3f_p = transmute::<nn_cv, glVertexAttrib3f_t>(Self::ptr_filter(f(b"glVertexAttrib3f\0".as_ptr())).ok_or("glVertexAttrib3f")?);
    let glVertexAttrib3fv_p = transmute::<nn_cv, glVertexAttrib3fv_t>(Self::ptr_filter(f(b"glVertexAttrib3fv\0".as_ptr())).ok_or("glVertexAttrib3fv")?);
    let glVertexAttrib3s_p = transmute::<nn_cv, glVertexAttrib3s_t>(Self::ptr_filter(f(b"glVertexAttrib3s\0".as_ptr())).ok_or("glVertexAttrib3s")?);
    let glVertexAttrib3sv_p = transmute::<nn_cv, glVertexAttrib3sv_t>(Self::ptr_filter(f(b"glVertexAttrib3sv\0".as_ptr())).ok_or("glVertexAttrib3sv")?);
    let glVertexAttrib4Nbv_p = transmute::<nn_cv, glVertexAttrib4Nbv_t>(Self::ptr_filter(f(b"glVertexAttrib4Nbv\0".as_ptr())).ok_or("glVertexAttrib4Nbv")?);
    let glVertexAttrib4Niv_p = transmute::<nn_cv, glVertexAttrib4Niv_t>(Self::ptr_filter(f(b"glVertexAttrib4Niv\0".as_ptr())).ok_or("glVertexAttrib4Niv")?);
    let glVertexAttrib4Nsv_p = transmute::<nn_cv, glVertexAttrib4Nsv_t>(Self::ptr_filter(f(b"glVertexAttrib4Nsv\0".as_ptr())).ok_or("glVertexAttrib4Nsv")?);
    let glVertexAttrib4Nub_p = transmute::<nn_cv, glVertexAttrib4Nub_t>(Self::ptr_filter(f(b"glVertexAttrib4Nub\0".as_ptr())).ok_or("glVertexAttrib4Nub")?);
    let glVertexAttrib4Nubv_p = transmute::<nn_cv, glVertexAttrib4Nubv_t>(Self::ptr_filter(f(b"glVertexAttrib4Nubv\0".as_ptr())).ok_or("glVertexAttrib4Nubv")?);
    let glVertexAttrib4Nuiv_p = transmute::<nn_cv, glVertexAttrib4Nuiv_t>(Self::ptr_filter(f(b"glVertexAttrib4Nuiv\0".as_ptr())).ok_or("glVertexAttrib4Nuiv")?);
    let glVertexAttrib4Nusv_p = transmute::<nn_cv, glVertexAttrib4Nusv_t>(Self::ptr_filter(f(b"glVertexAttrib4Nusv\0".as_ptr())).ok_or("glVertexAttrib4Nusv")?);
    let glVertexAttrib4bv_p = transmute::<nn_cv, glVertexAttrib4bv_t>(Self::ptr_filter(f(b"glVertexAttrib4bv\0".as_ptr())).ok_or("glVertexAttrib4bv")?);
    let glVertexAttrib4d_p = transmute::<nn_cv, glVertexAttrib4d_t>(Self::ptr_filter(f(b"glVertexAttrib4d\0".as_ptr())).ok_or("glVertexAttrib4d")?);
    let glVertexAttrib4dv_p = transmute::<nn_cv, glVertexAttrib4dv_t>(Self::ptr_filter(f(b"glVertexAttrib4dv\0".as_ptr())).ok_or("glVertexAttrib4dv")?);
    let glVertexAttrib4f_p = transmute::<nn_cv, glVertexAttrib4f_t>(Self::ptr_filter(f(b"glVertexAttrib4f\0".as_ptr())).ok_or("glVertexAttrib4f")?);
    let glVertexAttrib4fv_p = transmute::<nn_cv, glVertexAttrib4fv_t>(Self::ptr_filter(f(b"glVertexAttrib4fv\0".as_ptr())).ok_or("glVertexAttrib4fv")?);
    let glVertexAttrib4iv_p = transmute::<nn_cv, glVertexAttrib4iv_t>(Self::ptr_filter(f(b"glVertexAttrib4iv\0".as_ptr())).ok_or("glVertexAttrib4iv")?);
    let glVertexAttrib4s_p = transmute::<nn_cv, glVertexAttrib4s_t>(Self::ptr_filter(f(b"glVertexAttrib4s\0".as_ptr())).ok_or("glVertexAttrib4s")?);
    let glVertexAttrib4sv_p = transmute::<nn_cv, glVertexAttrib4sv_t>(Self::ptr_filter(f(b"glVertexAttrib4sv\0".as_ptr())).ok_or("glVertexAttrib4sv")?);
    let glVertexAttrib4ubv_p = transmute::<nn_cv, glVertexAttrib4ubv_t>(Self::ptr_filter(f(b"glVertexAttrib4ubv\0".as_ptr())).ok_or("glVertexAttrib4ubv")?);
    let glVertexAttrib4uiv_p = transmute::<nn_cv, glVertexAttrib4uiv_t>(Self::ptr_filter(f(b"glVertexAttrib4uiv\0".as_ptr())).ok_or("glVertexAttrib4uiv")?);
    let glVertexAttrib4usv_p = transmute::<nn_cv, glVertexAttrib4usv_t>(Self::ptr_filter(f(b"glVertexAttrib4usv\0".as_ptr())).ok_or("glVertexAttrib4usv")?);
    let glVertexAttribDivisor_p = transmute::<nn_cv, glVertexAttribDivisor_t>(Self::ptr_filter(f(b"glVertexAttribDivisor\0".as_ptr())).ok_or("glVertexAttribDivisor")?);
    let glVertexAttribI1i_p = transmute::<nn_cv, glVertexAttribI1i_t>(Self::ptr_filter(f(b"glVertexAttribI1i\0".as_ptr())).ok_or("glVertexAttribI1i")?);
    let glVertexAttribI1iv_p = transmute::<nn_cv, glVertexAttribI1iv_t>(Self::ptr_filter(f(b"glVertexAttribI1iv\0".as_ptr())).ok_or("glVertexAttribI1iv")?);
    let glVertexAttribI1ui_p = transmute::<nn_cv, glVertexAttribI1ui_t>(Self::ptr_filter(f(b"glVertexAttribI1ui\0".as_ptr())).ok_or("glVertexAttribI1ui")?);
    let glVertexAttribI1uiv_p = transmute::<nn_cv, glVertexAttribI1uiv_t>(Self::ptr_filter(f(b"glVertexAttribI1uiv\0".as_ptr())).ok_or("glVertexAttribI1uiv")?);
    let glVertexAttribI2i_p = transmute::<nn_cv, glVertexAttribI2i_t>(Self::ptr_filter(f(b"glVertexAttribI2i\0".as_ptr())).ok_or("glVertexAttribI2i")?);
    let glVertexAttribI2iv_p = transmute::<nn_cv, glVertexAttribI2iv_t>(Self::ptr_filter(f(b"glVertexAttribI2iv\0".as_ptr())).ok_or("glVertexAttribI2iv")?);
    let glVertexAttribI2ui_p = transmute::<nn_cv, glVertexAttribI2ui_t>(Self::ptr_filter(f(b"glVertexAttribI2ui\0".as_ptr())).ok_or("glVertexAttribI2ui")?);
    let glVertexAttribI2uiv_p = transmute::<nn_cv, glVertexAttribI2uiv_t>(Self::ptr_filter(f(b"glVertexAttribI2uiv\0".as_ptr())).ok_or("glVertexAttribI2uiv")?);
    let glVertexAttribI3i_p = transmute::<nn_cv, glVertexAttribI3i_t>(Self::ptr_filter(f(b"glVertexAttribI3i\0".as_ptr())).ok_or("glVertexAttribI3i")?);
    let glVertexAttribI3iv_p = transmute::<nn_cv, glVertexAttribI3iv_t>(Self::ptr_filter(f(b"glVertexAttribI3iv\0".as_ptr())).ok_or("glVertexAttribI3iv")?);
    let glVertexAttribI3ui_p = transmute::<nn_cv, glVertexAttribI3ui_t>(Self::ptr_filter(f(b"glVertexAttribI3ui\0".as_ptr())).ok_or("glVertexAttribI3ui")?);
    let glVertexAttribI3uiv_p = transmute::<nn_cv, glVertexAttribI3uiv_t>(Self::ptr_filter(f(b"glVertexAttribI3uiv\0".as_ptr())).ok_or("glVertexAttribI3uiv")?);
    let glVertexAttribI4bv_p = transmute::<nn_cv, glVertexAttribI4bv_t>(Self::ptr_filter(f(b"glVertexAttribI4bv\0".as_ptr())).ok_or("glVertexAttribI4bv")?);
    let glVertexAttribI4i_p = transmute::<nn_cv, glVertexAttribI4i_t>(Self::ptr_filter(f(b"glVertexAttribI4i\0".as_ptr())).ok_or("glVertexAttribI4i")?);
    let glVertexAttribI4iv_p = transmute::<nn_cv, glVertexAttribI4iv_t>(Self::ptr_filter(f(b"glVertexAttribI4iv\0".as_ptr())).ok_or("glVertexAttribI4iv")?);
    let glVertexAttribI4sv_p = transmute::<nn_cv, glVertexAttribI4sv_t>(Self::ptr_filter(f(b"glVertexAttribI4sv\0".as_ptr())).ok_or("glVertexAttribI4sv")?);
    let glVertexAttribI4ubv_p = transmute::<nn_cv, glVertexAttribI4ubv_t>(Self::ptr_filter(f(b"glVertexAttribI4ubv\0".as_ptr())).ok_or("glVertexAttribI4ubv")?);
    let glVertexAttribI4ui_p = transmute::<nn_cv, glVertexAttribI4ui_t>(Self::ptr_filter(f(b"glVertexAttribI4ui\0".as_ptr())).ok_or("glVertexAttribI4ui")?);
    let glVertexAttribI4uiv_p = transmute::<nn_cv, glVertexAttribI4uiv_t>(Self::ptr_filter(f(b"glVertexAttribI4uiv\0".as_ptr())).ok_or("glVertexAttribI4uiv")?);
    let glVertexAttribI4usv_p = transmute::<nn_cv, glVertexAttribI4usv_t>(Self::ptr_filter(f(b"glVertexAttribI4usv\0".as_ptr())).ok_or("glVertexAttribI4usv")?);
    let glVertexAttribIPointer_p = transmute::<nn_cv, glVertexAttribIPointer_t>(Self::ptr_filter(f(b"glVertexAttribIPointer\0".as_ptr())).ok_or("glVertexAttribIPointer")?);
    let glVertexAttribP1ui_p = transmute::<nn_cv, glVertexAttribP1ui_t>(Self::ptr_filter(f(b"glVertexAttribP1ui\0".as_ptr())).ok_or("glVertexAttribP1ui")?);
    let glVertexAttribP1uiv_p = transmute::<nn_cv, glVertexAttribP1uiv_t>(Self::ptr_filter(f(b"glVertexAttribP1uiv\0".as_ptr())).ok_or("glVertexAttribP1uiv")?);
    let glVertexAttribP2ui_p = transmute::<nn_cv, glVertexAttribP2ui_t>(Self::ptr_filter(f(b"glVertexAttribP2ui\0".as_ptr())).ok_or("glVertexAttribP2ui")?);
    let glVertexAttribP2uiv_p = transmute::<nn_cv, glVertexAttribP2uiv_t>(Self::ptr_filter(f(b"glVertexAttribP2uiv\0".as_ptr())).ok_or("glVertexAttribP2uiv")?);
    let glVertexAttribP3ui_p = transmute::<nn_cv, glVertexAttribP3ui_t>(Self::ptr_filter(f(b"glVertexAttribP3ui\0".as_ptr())).ok_or("glVertexAttribP3ui")?);
    let glVertexAttribP3uiv_p = transmute::<nn_cv, glVertexAttribP3uiv_t>(Self::ptr_filter(f(b"glVertexAttribP3uiv\0".as_ptr())).ok_or("glVertexAttribP3uiv")?);
    let glVertexAttribP4ui_p = transmute::<nn_cv, glVertexAttribP4ui_t>(Self::ptr_filter(f(b"glVertexAttribP4ui\0".as_ptr())).ok_or("glVertexAttribP4ui")?);
    let glVertexAttribP4uiv_p = transmute::<nn_cv, glVertexAttribP4uiv_t>(Self::ptr_filter(f(b"glVertexAttribP4uiv\0".as_ptr())).ok_or("glVertexAttribP4uiv")?);
    let glVertexAttribPointer_p = transmute::<nn_cv, glVertexAttribPointer_t>(Self::ptr_filter(f(b"glVertexAttribPointer\0".as_ptr())).ok_or("glVertexAttribPointer")?);
    let glViewport_p = transmute::<nn_cv, glViewport_t>(Self::ptr_filter(f(b"glViewport\0".as_ptr())).ok_or("glViewport")?);
    let glWaitSync_p = transmute::<nn_cv, glWaitSync_t>(Self::ptr_filter(f(b"glWaitSync\0".as_ptr())).ok_or("glWaitSync")?);
    // nullable loads
    let glDebugMessageCallback_p = transmute::<Option<nn_cv>, Option<glDebugMessageCallback_t>>(Self::ptr_filter(f(b"glDebugMessageCallback\0".as_ptr())));
    let glDebugMessageControl_p = transmute::<Option<nn_cv>, Option<glDebugMessageControl_t>>(Self::ptr_filter(f(b"glDebugMessageControl\0".as_ptr())));
    let glDebugMessageInsert_p = transmute::<Option<nn_cv>, Option<glDebugMessageInsert_t>>(Self::ptr_filter(f(b"glDebugMessageInsert\0".as_ptr())));
    let glGetDebugMessageLog_p = transmute::<Option<nn_cv>, Option<glGetDebugMessageLog_t>>(Self::ptr_filter(f(b"glGetDebugMessageLog\0".as_ptr())));
    let glGetObjectLabel_p = transmute::<Option<nn_cv>, Option<glGetObjectLabel_t>>(Self::ptr_filter(f(b"glGetObjectLabel\0".as_ptr())));
    let glGetObjectPtrLabel_p = transmute::<Option<nn_cv>, Option<glGetObjectPtrLabel_t>>(Self::ptr_filter(f(b"glGetObjectPtrLabel\0".as_ptr())));
    let glGetPointerv_p = transmute::<Option<nn_cv>, Option<glGetPointerv_t>>(Self::ptr_filter(f(b"glGetPointerv\0".as_ptr())));
    let glObjectLabel_p = transmute::<Option<nn_cv>, Option<glObjectLabel_t>>(Self::ptr_filter(f(b"glObjectLabel\0".as_ptr())));
    let glObjectPtrLabel_p = transmute::<Option<nn_cv>, Option<glObjectPtrLabel_t>>(Self::ptr_filter(f(b"glObjectPtrLabel\0".as_ptr())));
    let glPopDebugGroup_p = transmute::<Option<nn_cv>, Option<glPopDebugGroup_t>>(Self::ptr_filter(f(b"glPopDebugGroup\0".as_ptr())));
    let glPushDebugGroup_p = transmute::<Option<nn_cv>, Option<glPushDebugGroup_t>>(Self::ptr_filter(f(b"glPushDebugGroup\0".as_ptr())));
    // we're all good!
    Ok(Self {
      glActiveTexture_p,
      glAttachShader_p,
      glBeginConditionalRender_p,
      glBeginQuery_p,
      glBeginTransformFeedback_p,
      glBindAttribLocation_p,
      glBindBuffer_p,
      glBindBufferBase_p,
      glBindBufferRange_p,
      glBindFragDataLocation_p,
      glBindFragDataLocationIndexed_p,
      glBindFramebuffer_p,
      glBindRenderbuffer_p,
      glBindSampler_p,
      glBindTexture_p,
      glBindVertexArray_p,
      glBlendColor_p,
      glBlendEquation_p,
      glBlendEquationSeparate_p,
      glBlendFunc_p,
      glBlendFuncSeparate_p,
      glBlitFramebuffer_p,
      glBufferData_p,
      glBufferSubData_p,
      glCheckFramebufferStatus_p,
      glClampColor_p,
      glClear_p,
      glClearBufferfi_p,
      glClearBufferfv_p,
      glClearBufferiv_p,
      glClearBufferuiv_p,
      glClearColor_p,
      glClearDepth_p,
      glClearStencil_p,
      glClientWaitSync_p,
      glColorMask_p,
      glColorMaski_p,
      glCompileShader_p,
      glCompressedTexImage1D_p,
      glCompressedTexImage2D_p,
      glCompressedTexImage3D_p,
      glCompressedTexSubImage1D_p,
      glCompressedTexSubImage2D_p,
      glCompressedTexSubImage3D_p,
      glCopyBufferSubData_p,
      glCopyTexImage1D_p,
      glCopyTexImage2D_p,
      glCopyTexSubImage1D_p,
      glCopyTexSubImage2D_p,
      glCopyTexSubImage3D_p,
      glCreateProgram_p,
      glCreateShader_p,
      glCullFace_p,
      glDeleteBuffers_p,
      glDeleteFramebuffers_p,
      glDeleteProgram_p,
      glDeleteQueries_p,
      glDeleteRenderbuffers_p,
      glDeleteSamplers_p,
      glDeleteShader_p,
      glDeleteSync_p,
      glDeleteTextures_p,
      glDeleteVertexArrays_p,
      glDepthFunc_p,
      glDepthMask_p,
      glDepthRange_p,
      glDetachShader_p,
      glDisable_p,
      glDisableVertexAttribArray_p,
      glDisablei_p,
      glDrawArrays_p,
      glDrawArraysInstanced_p,
      glDrawBuffer_p,
      glDrawBuffers_p,
      glDrawElements_p,
      glDrawElementsBaseVertex_p,
      glDrawElementsInstanced_p,
      glDrawElementsInstancedBaseVertex_p,
      glDrawRangeElements_p,
      glDrawRangeElementsBaseVertex_p,
      glEnable_p,
      glEnableVertexAttribArray_p,
      glEnablei_p,
      glEndConditionalRender_p,
      glEndQuery_p,
      glEndTransformFeedback_p,
      glFenceSync_p,
      glFinish_p,
      glFlush_p,
      glFlushMappedBufferRange_p,
      glFramebufferRenderbuffer_p,
      glFramebufferTexture_p,
      glFramebufferTexture1D_p,
      glFramebufferTexture2D_p,
      glFramebufferTexture3D_p,
      glFramebufferTextureLayer_p,
      glFrontFace_p,
      glGenBuffers_p,
      glGenFramebuffers_p,
      glGenQueries_p,
      glGenRenderbuffers_p,
      glGenSamplers_p,
      glGenTextures_p,
      glGenVertexArrays_p,
      glGenerateMipmap_p,
      glGetActiveAttrib_p,
      glGetActiveUniform_p,
      glGetActiveUniformBlockName_p,
      glGetActiveUniformBlockiv_p,
      glGetActiveUniformName_p,
      glGetActiveUniformsiv_p,
      glGetAttachedShaders_p,
      glGetAttribLocation_p,
      glGetBooleani_v_p,
      glGetBooleanv_p,
      glGetBufferParameteri64v_p,
      glGetBufferParameteriv_p,
      glGetBufferPointerv_p,
      glGetBufferSubData_p,
      glGetCompressedTexImage_p,
      glGetDoublev_p,
      glGetError_p,
      glGetFloatv_p,
      glGetFragDataIndex_p,
      glGetFragDataLocation_p,
      glGetFramebufferAttachmentParameteriv_p,
      glGetInteger64i_v_p,
      glGetInteger64v_p,
      glGetIntegeri_v_p,
      glGetIntegerv_p,
      glGetMultisamplefv_p,
      glGetProgramInfoLog_p,
      glGetProgramiv_p,
      glGetQueryObjecti64v_p,
      glGetQueryObjectiv_p,
      glGetQueryObjectui64v_p,
      glGetQueryObjectuiv_p,
      glGetQueryiv_p,
      glGetRenderbufferParameteriv_p,
      glGetSamplerParameterIiv_p,
      glGetSamplerParameterIuiv_p,
      glGetSamplerParameterfv_p,
      glGetSamplerParameteriv_p,
      glGetShaderInfoLog_p,
      glGetShaderSource_p,
      glGetShaderiv_p,
      glGetString_p,
      glGetStringi_p,
      glGetSynciv_p,
      glGetTexImage_p,
      glGetTexLevelParameterfv_p,
      glGetTexLevelParameteriv_p,
      glGetTexParameterIiv_p,
      glGetTexParameterIuiv_p,
      glGetTexParameterfv_p,
      glGetTexParameteriv_p,
      glGetTransformFeedbackVarying_p,
      glGetUniformBlockIndex_p,
      glGetUniformIndices_p,
      glGetUniformLocation_p,
      glGetUniformfv_p,
      glGetUniformiv_p,
      glGetUniformuiv_p,
      glGetVertexAttribIiv_p,
      glGetVertexAttribIuiv_p,
      glGetVertexAttribPointerv_p,
      glGetVertexAttribdv_p,
      glGetVertexAttribfv_p,
      glGetVertexAttribiv_p,
      glHint_p,
      glIsBuffer_p,
      glIsEnabled_p,
      glIsEnabledi_p,
      glIsFramebuffer_p,
      glIsProgram_p,
      glIsQuery_p,
      glIsRenderbuffer_p,
      glIsSampler_p,
      glIsShader_p,
      glIsSync_p,
      glIsTexture_p,
      glIsVertexArray_p,
      glLineWidth_p,
      glLinkProgram_p,
      glLogicOp_p,
      glMapBuffer_p,
      glMapBufferRange_p,
      glMultiDrawArrays_p,
      glMultiDrawElements_p,
      glMultiDrawElementsBaseVertex_p,
      glPixelStoref_p,
      glPixelStorei_p,
      glPointParameterf_p,
      glPointParameterfv_p,
      glPointParameteri_p,
      glPointParameteriv_p,
      glPointSize_p,
      glPolygonMode_p,
      glPolygonOffset_p,
      glPrimitiveRestartIndex_p,
      glProvokingVertex_p,
      glQueryCounter_p,
      glReadBuffer_p,
      glReadPixels_p,
      glRenderbufferStorage_p,
      glRenderbufferStorageMultisample_p,
      glSampleCoverage_p,
      glSampleMaski_p,
      glSamplerParameterIiv_p,
      glSamplerParameterIuiv_p,
      glSamplerParameterf_p,
      glSamplerParameterfv_p,
      glSamplerParameteri_p,
      glSamplerParameteriv_p,
      glScissor_p,
      glShaderSource_p,
      glStencilFunc_p,
      glStencilFuncSeparate_p,
      glStencilMask_p,
      glStencilMaskSeparate_p,
      glStencilOp_p,
      glStencilOpSeparate_p,
      glTexBuffer_p,
      glTexImage1D_p,
      glTexImage2D_p,
      glTexImage2DMultisample_p,
      glTexImage3D_p,
      glTexImage3DMultisample_p,
      glTexParameterIiv_p,
      glTexParameterIuiv_p,
      glTexParameterf_p,
      glTexParameterfv_p,
      glTexParameteri_p,
      glTexParameteriv_p,
      glTexSubImage1D_p,
      glTexSubImage2D_p,
      glTexSubImage3D_p,
      glTransformFeedbackVaryings_p,
      glUniform1f_p,
      glUniform1fv_p,
      glUniform1i_p,
      glUniform1iv_p,
      glUniform1ui_p,
      glUniform1uiv_p,
      glUniform2f_p,
      glUniform2fv_p,
      glUniform2i_p,
      glUniform2iv_p,
      glUniform2ui_p,
      glUniform2uiv_p,
      glUniform3f_p,
      glUniform3fv_p,
      glUniform3i_p,
      glUniform3iv_p,
      glUniform3ui_p,
      glUniform3uiv_p,
      glUniform4f_p,
      glUniform4fv_p,
      glUniform4i_p,
      glUniform4iv_p,
      glUniform4ui_p,
      glUniform4uiv_p,
      glUniformBlockBinding_p,
      glUniformMatrix2fv_p,
      glUniformMatrix2x3fv_p,
      glUniformMatrix2x4fv_p,
      glUniformMatrix3fv_p,
      glUniformMatrix3x2fv_p,
      glUniformMatrix3x4fv_p,
      glUniformMatrix4fv_p,
      glUniformMatrix4x2fv_p,
      glUniformMatrix4x3fv_p,
      glUnmapBuffer_p,
      glUseProgram_p,
      glValidateProgram_p,
      glVertexAttrib1d_p,
      glVertexAttrib1dv_p,
      glVertexAttrib1f_p,
      glVertexAttrib1fv_p,
      glVertexAttrib1s_p,
      glVertexAttrib1sv_p,
      glVertexAttrib2d_p,
      glVertexAttrib2dv_p,
      glVertexAttrib2f_p,
      glVertexAttrib2fv_p,
      glVertexAttrib2s_p,
      glVertexAttrib2sv_p,
      glVertexAttrib3d_p,
      glVertexAttrib3dv_p,
      glVertexAttrib3f_p,
      glVertexAttrib3fv_p,
      glVertexAttrib3s_p,
      glVertexAttrib3sv_p,
      glVertexAttrib4Nbv_p,
      glVertexAttrib4Niv_p,
      glVertexAttrib4Nsv_p,
      glVertexAttrib4Nub_p,
      glVertexAttrib4Nubv_p,
      glVertexAttrib4Nuiv_p,
      glVertexAttrib4Nusv_p,
      glVertexAttrib4bv_p,
      glVertexAttrib4d_p,
      glVertexAttrib4dv_p,
      glVertexAttrib4f_p,
      glVertexAttrib4fv_p,
      glVertexAttrib4iv_p,
      glVertexAttrib4s_p,
      glVertexAttrib4sv_p,
      glVertexAttrib4ubv_p,
      glVertexAttrib4uiv_p,
      glVertexAttrib4usv_p,
      glVertexAttribDivisor_p,
      glVertexAttribI1i_p,
      glVertexAttribI1iv_p,
      glVertexAttribI1ui_p,
      glVertexAttribI1uiv_p,
      glVertexAttribI2i_p,
      glVertexAttribI2iv_p,
      glVertexAttribI2ui_p,
      glVertexAttribI2uiv_p,
      glVertexAttribI3i_p,
      glVertexAttribI3iv_p,
      glVertexAttribI3ui_p,
      glVertexAttribI3uiv_p,
      glVertexAttribI4bv_p,
      glVertexAttribI4i_p,
      glVertexAttribI4iv_p,
      glVertexAttribI4sv_p,
      glVertexAttribI4ubv_p,
      glVertexAttribI4ui_p,
      glVertexAttribI4uiv_p,
      glVertexAttribI4usv_p,
      glVertexAttribIPointer_p,
      glVertexAttribP1ui_p,
      glVertexAttribP1uiv_p,
      glVertexAttribP2ui_p,
      glVertexAttribP2uiv_p,
      glVertexAttribP3ui_p,
      glVertexAttribP3uiv_p,
      glVertexAttribP4ui_p,
      glVertexAttribP4uiv_p,
      glVertexAttribPointer_p,
      glViewport_p,
      glWaitSync_p,
      glDebugMessageCallback_p,
      glDebugMessageControl_p,
      glDebugMessageInsert_p,
      glGetDebugMessageLog_p,
      glGetObjectLabel_p,
      glGetObjectPtrLabel_p,
      glGetPointerv_p,
      glObjectLabel_p,
      glObjectPtrLabel_p,
      glPopDebugGroup_p,
      glPushDebugGroup_p,
    })
  }
}

impl GlFns {
  /// glActiveTexture
  /// * `texture` group: TextureUnit
  pub unsafe fn ActiveTexture(&self, texture: TextureUnit) {
    (self.glActiveTexture_p)(texture)
  }
  /// [glAttachShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachShader.xhtml)
  ///
  /// Attaches the given shader object to the given program object. You can
  /// attach more than one shader of the same type to a program.
  ///
  /// * `program` is the program you're attaching the shader object to.
  /// * `shader` is the shader you're attaching.
  pub fn AttachShader(&self, program: GLuint, shader: GLuint) {
    (self.glAttachShader_p)(program, shader)
  }
  /// glBeginConditionalRender
  /// * `mode` group: ConditionalRenderMode
  pub unsafe fn BeginConditionalRender(&self, id: GLuint, mode: ConditionalRenderMode) {
    (self.glBeginConditionalRender_p)(id, mode)
  }
  /// glBeginQuery
  /// * `target` group: QueryTarget
  /// * `id` class: query
  pub unsafe fn BeginQuery(&self, target: QueryTarget, id: GLuint) {
    (self.glBeginQuery_p)(target, id)
  }
  /// glBeginTransformFeedback
  /// * `primitiveMode` group: PrimitiveType
  pub unsafe fn BeginTransformFeedback(&self, primitiveMode: PrimitiveType) {
    (self.glBeginTransformFeedback_p)(primitiveMode)
  }
  /// glBindAttribLocation
  /// * `program` class: program
  pub unsafe fn BindAttribLocation(&self, program: GLuint, index: GLuint, name: *const GLchar) {
    (self.glBindAttribLocation_p)(program, index, name)
  }
  /// glBindBuffer
  /// * `target` group: BufferTargetARB
  /// * `buffer` class: buffer
  pub unsafe fn BindBuffer(&self, target: BufferTargetARB, buffer: GLuint) {
    (self.glBindBuffer_p)(target, buffer)
  }
  /// glBindBufferBase
  /// * `target` group: BufferTargetARB
  /// * `buffer` class: buffer
  pub unsafe fn BindBufferBase(&self, target: BufferTargetARB, index: GLuint, buffer: GLuint) {
    (self.glBindBufferBase_p)(target, index, buffer)
  }
  /// glBindBufferRange
  /// * `target` group: BufferTargetARB
  /// * `buffer` class: buffer
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  pub unsafe fn BindBufferRange(&self, target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
    (self.glBindBufferRange_p)(target, index, buffer, offset, size)
  }
  /// glBindFragDataLocation
  /// * `program` class: program
  /// * `name` len: COMPSIZE(name)
  pub unsafe fn BindFragDataLocation(&self, program: GLuint, color: GLuint, name: *const GLchar) {
    (self.glBindFragDataLocation_p)(program, color, name)
  }
  /// glBindFragDataLocationIndexed
  /// * `program` class: program
  pub unsafe fn BindFragDataLocationIndexed(&self, program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) {
    (self.glBindFragDataLocationIndexed_p)(program, colorNumber, index, name)
  }
  /// glBindFramebuffer
  /// * `target` group: FramebufferTarget
  /// * `framebuffer` class: framebuffer
  pub unsafe fn BindFramebuffer(&self, target: FramebufferTarget, framebuffer: GLuint) {
    (self.glBindFramebuffer_p)(target, framebuffer)
  }
  /// glBindRenderbuffer
  /// * `target` group: RenderbufferTarget
  /// * `renderbuffer` class: renderbuffer
  pub unsafe fn BindRenderbuffer(&self, target: RenderbufferTarget, renderbuffer: GLuint) {
    (self.glBindRenderbuffer_p)(target, renderbuffer)
  }
  /// glBindSampler
  /// * `sampler` class: sampler
  pub unsafe fn BindSampler(&self, unit: GLuint, sampler: GLuint) {
    (self.glBindSampler_p)(unit, sampler)
  }
  /// glBindTexture
  /// * `target` group: TextureTarget
  /// * `texture` group: Texture
  /// * `texture` class: texture
  pub unsafe fn BindTexture(&self, target: TextureTarget, texture: GLuint) {
    (self.glBindTexture_p)(target, texture)
  }
  /// [glBindVertexArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml)
  ///
  /// Binds a given vertex array object as the active vertex array object.
  /// Passing 0 will make it so that no vertex array object is bound.
  ///
  /// * `array` names the vertex array object to bind.
  pub fn BindVertexArray(&self, array: GLuint) {
    (self.glBindVertexArray_p)(array)
  }
  /// glBlendColor
  /// * `red` group: ColorF
  /// * `green` group: ColorF
  /// * `blue` group: ColorF
  /// * `alpha` group: ColorF
  pub unsafe fn BlendColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    (self.glBlendColor_p)(red, green, blue, alpha)
  }
  /// glBlendEquation
  /// * `mode` group: BlendEquationModeEXT
  pub unsafe fn BlendEquation(&self, mode: BlendEquationModeEXT) {
    (self.glBlendEquation_p)(mode)
  }
  /// glBlendEquationSeparate
  /// * `modeRGB` group: BlendEquationModeEXT
  /// * `modeAlpha` group: BlendEquationModeEXT
  pub unsafe fn BlendEquationSeparate(&self, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT) {
    (self.glBlendEquationSeparate_p)(modeRGB, modeAlpha)
  }
  /// glBlendFunc
  /// * `sfactor` group: BlendingFactor
  /// * `dfactor` group: BlendingFactor
  pub unsafe fn BlendFunc(&self, sfactor: BlendingFactor, dfactor: BlendingFactor) {
    (self.glBlendFunc_p)(sfactor, dfactor)
  }
  /// glBlendFuncSeparate
  /// * `sfactorRGB` group: BlendingFactor
  /// * `dfactorRGB` group: BlendingFactor
  /// * `sfactorAlpha` group: BlendingFactor
  /// * `dfactorAlpha` group: BlendingFactor
  pub unsafe fn BlendFuncSeparate(&self, sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor) {
    (self.glBlendFuncSeparate_p)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)
  }
  /// glBlitFramebuffer
  /// * `mask` group: ClearBufferMask
  /// * `filter` group: BlitFramebufferFilter
  pub unsafe fn BlitFramebuffer(&self, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter) {
    (self.glBlitFramebuffer_p)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
  }
  /// glBufferData
  /// * `target` group: BufferTargetARB
  /// * `size` group: BufferSize
  /// * `data` len: size
  /// * `usage` group: BufferUsageARB
  pub unsafe fn BufferData(&self, target: BufferTargetARB, size: GLsizeiptr, data: *const void, usage: BufferUsageARB) {
    (self.glBufferData_p)(target, size, data, usage)
  }
  /// glBufferSubData
  /// * `target` group: BufferTargetARB
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  /// * `data` len: size
  pub unsafe fn BufferSubData(&self, target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *const void) {
    (self.glBufferSubData_p)(target, offset, size, data)
  }
  /// glCheckFramebufferStatus
  /// * `target` group: FramebufferTarget
  pub unsafe fn CheckFramebufferStatus(&self, target: FramebufferTarget) -> FramebufferStatus {
    (self.glCheckFramebufferStatus_p)(target)
  }
  /// glClampColor
  /// * `target` group: ClampColorTargetARB
  /// * `clamp` group: ClampColorModeARB
  pub unsafe fn ClampColor(&self, target: ClampColorTargetARB, clamp: ClampColorModeARB) {
    (self.glClampColor_p)(target, clamp)
  }
  /// glClear
  /// * `mask` group: ClearBufferMask
  pub unsafe fn Clear(&self, mask: GLbitfield) {
    (self.glClear_p)(mask)
  }
  /// glClearBufferfi
  /// * `buffer` group: Buffer
  /// * `drawbuffer` group: DrawBufferName
  pub unsafe fn ClearBufferfi(&self, buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
    (self.glClearBufferfi_p)(buffer, drawbuffer, depth, stencil)
  }
  /// [glClearBufferfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBuffer.xhtml)
  ///
  /// Clears a specified buffer of the currently bound draw framebuffer object
  /// to a specified value.
  ///
  /// * If `buffer` is `GL_COLOR`, a particular draw buffer `GL_DRAW_BUFFERi` is
  ///   specified by passing `i` as `drawbuffer` (eg: to affect
  ///   `GL_DRAW_BUFFER0` you'd pass 0). In this case, `value` points to a
  ///   four-element vector specifying the R, G, B, and A color to clear that
  ///   draw buffer to. If the value of `GL_DRAW_BUFFERi` is `GL_NONE`, the
  ///   command has no effect. Otherwise, the value of `GL_DRAW_BUFFERi`
  ///   identifies one or more color buffers, each of which is cleared to the
  ///   same value. Clamping and type conversion for fixed-point color buffers
  ///   are performed in the same fashion as for `glClearColor`.
  /// * If `buffer` is `GL_DEPTH`, `drawbuffer` must be zero, and `value` points
  ///   to a single value to clear the depth buffer to. Clamping and type
  ///   conversion for fixed-point depth buffers are performed in the same
  ///   fashion as for `glClearDepth`.
  ///
  /// ## Errors
  /// * `GL_INVALID_ENUM` is generated if `buffer` is not `GL_COLOR` or
  ///   `GL_DEPTH`.
  pub unsafe fn ClearBufferfv(&self, buffer: Buffer, drawbuffer: GLint, value: *const GLfloat) {
    (self.glClearBufferfv_p)(buffer, drawbuffer, value)
  }
  /// glClearBufferiv
  /// * `buffer` group: Buffer
  /// * `drawbuffer` group: DrawBufferName
  /// * `value` len: COMPSIZE(buffer)
  pub unsafe fn ClearBufferiv(&self, buffer: Buffer, drawbuffer: GLint, value: *const GLint) {
    (self.glClearBufferiv_p)(buffer, drawbuffer, value)
  }
  /// glClearBufferuiv
  /// * `buffer` group: Buffer
  /// * `drawbuffer` group: DrawBufferName
  /// * `value` len: COMPSIZE(buffer)
  pub unsafe fn ClearBufferuiv(&self, buffer: Buffer, drawbuffer: GLint, value: *const GLuint) {
    (self.glClearBufferuiv_p)(buffer, drawbuffer, value)
  }
  /// glClearColor
  /// * `red` group: ColorF
  /// * `green` group: ColorF
  /// * `blue` group: ColorF
  /// * `alpha` group: ColorF
  pub unsafe fn ClearColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    (self.glClearColor_p)(red, green, blue, alpha)
  }
  /// glClearDepth
  pub unsafe fn ClearDepth(&self, depth: GLdouble) {
    (self.glClearDepth_p)(depth)
  }
  /// glClearStencil
  /// * `s` group: StencilValue
  pub unsafe fn ClearStencil(&self, s: GLint) {
    (self.glClearStencil_p)(s)
  }
  /// glClientWaitSync
  /// * `sync` group: sync
  /// * `sync` class: sync
  /// * `flags` group: SyncObjectMask
  pub unsafe fn ClientWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> SyncStatus {
    (self.glClientWaitSync_p)(sync, flags, timeout)
  }
  /// glColorMask
  /// * `red` group: Boolean
  /// * `green` group: Boolean
  /// * `blue` group: Boolean
  /// * `alpha` group: Boolean
  pub unsafe fn ColorMask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
    (self.glColorMask_p)(red, green, blue, alpha)
  }
  /// glColorMaski
  /// * `r` group: Boolean
  /// * `g` group: Boolean
  /// * `b` group: Boolean
  /// * `a` group: Boolean
  pub unsafe fn ColorMaski(&self, index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) {
    (self.glColorMaski_p)(index, r, g, b, a)
  }
  /// [glCompileShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileShader.xhtml)
  ///
  /// Compiles the source code assigned to the shader. The compilation status is
  /// stored as part of the shader object's state, check it with `glGetShader`
  /// and `glGetShaderInfoLog`.
  ///
  /// * `shader` names the shader to compile.
  pub fn CompileShader(&self, shader: GLuint) {
    (self.glCompileShader_p)(shader)
  }
  /// glCompressedTexImage1D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  pub unsafe fn CompressedTexImage1D(&self, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTexImage1D_p)(target, level, internalformat, width, border, imageSize, data)
  }
  /// glCompressedTexImage2D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  pub unsafe fn CompressedTexImage2D(&self, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTexImage2D_p)(target, level, internalformat, width, height, border, imageSize, data)
  }
  /// glCompressedTexImage3D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  pub unsafe fn CompressedTexImage3D(&self, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTexImage3D_p)(target, level, internalformat, width, height, depth, border, imageSize, data)
  }
  /// glCompressedTexSubImage1D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  pub unsafe fn CompressedTexSubImage1D(&self, target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTexSubImage1D_p)(target, level, xoffset, width, format, imageSize, data)
  }
  /// glCompressedTexSubImage2D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  pub unsafe fn CompressedTexSubImage2D(&self, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTexSubImage2D_p)(target, level, xoffset, yoffset, width, height, format, imageSize, data)
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
  pub unsafe fn CompressedTexSubImage3D(&self, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTexSubImage3D_p)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data)
  }
  /// glCopyBufferSubData
  /// * `readTarget` group: CopyBufferSubDataTarget
  /// * `writeTarget` group: CopyBufferSubDataTarget
  /// * `readOffset` group: BufferOffset
  /// * `writeOffset` group: BufferOffset
  /// * `size` group: BufferSize
  pub unsafe fn CopyBufferSubData(&self, readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
    (self.glCopyBufferSubData_p)(readTarget, writeTarget, readOffset, writeOffset, size)
  }
  /// glCopyTexImage1D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  /// * `border` group: CheckedInt32
  pub unsafe fn CopyTexImage1D(&self, target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint) {
    (self.glCopyTexImage1D_p)(target, level, internalformat, x, y, width, border)
  }
  /// glCopyTexImage2D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  /// * `border` group: CheckedInt32
  pub unsafe fn CopyTexImage2D(&self, target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) {
    (self.glCopyTexImage2D_p)(target, level, internalformat, x, y, width, height, border)
  }
  /// glCopyTexSubImage1D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  pub unsafe fn CopyTexSubImage1D(&self, target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
    (self.glCopyTexSubImage1D_p)(target, level, xoffset, x, y, width)
  }
  /// glCopyTexSubImage2D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  pub unsafe fn CopyTexSubImage2D(&self, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    (self.glCopyTexSubImage2D_p)(target, level, xoffset, yoffset, x, y, width, height)
  }
  /// glCopyTexSubImage3D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `zoffset` group: CheckedInt32
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  pub unsafe fn CopyTexSubImage3D(&self, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    (self.glCopyTexSubImage3D_p)(target, level, xoffset, yoffset, zoffset, x, y, width, height)
  }
  /// [glCreateProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgram.xhtml)
  ///
  /// Creates an empty program object, returning its name (a non-zero ID value).
  ///
  /// ## Failure
  /// * If this fails, 0 is returned.
  pub fn CreateProgram(&self) -> GLuint {
    (self.glCreateProgram_p)()
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
  pub fn CreateShader(&self, type_: ShaderType) -> GLuint {
    (self.glCreateShader_p)(type_)
  }
  /// glCullFace
  /// * `mode` group: CullFaceMode
  pub unsafe fn CullFace(&self, mode: CullFaceMode) {
    (self.glCullFace_p)(mode)
  }
  /// glDeleteBuffers
  /// * `buffers` class: buffer
  /// * `buffers` len: n
  pub unsafe fn DeleteBuffers(&self, n: GLsizei, buffers: *const GLuint) {
    (self.glDeleteBuffers_p)(n, buffers)
  }
  /// glDeleteFramebuffers
  /// * `framebuffers` class: framebuffer
  /// * `framebuffers` len: n
  pub unsafe fn DeleteFramebuffers(&self, n: GLsizei, framebuffers: *const GLuint) {
    (self.glDeleteFramebuffers_p)(n, framebuffers)
  }
  /// [glDeleteProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgram.xhtml)
  ///
  /// Marks a program object for deletion. If the shader program is not in use
  /// it will be immediately deleted, otherwise it will be deleted once it's
  /// no longer in use. When a program object is deleted any shaders attached
  /// to it are automatically unattached from it.
  ///
  /// * `program` names the program to mark for deletion.
  pub fn DeleteProgram(&self, program: GLuint) {
    (self.glDeleteProgram_p)(program)
  }
  /// glDeleteQueries
  /// * `ids` class: query
  /// * `ids` len: n
  pub unsafe fn DeleteQueries(&self, n: GLsizei, ids: *const GLuint) {
    (self.glDeleteQueries_p)(n, ids)
  }
  /// glDeleteRenderbuffers
  /// * `renderbuffers` class: renderbuffer
  /// * `renderbuffers` len: n
  pub unsafe fn DeleteRenderbuffers(&self, n: GLsizei, renderbuffers: *const GLuint) {
    (self.glDeleteRenderbuffers_p)(n, renderbuffers)
  }
  /// glDeleteSamplers
  /// * `samplers` class: sampler
  /// * `samplers` len: count
  pub unsafe fn DeleteSamplers(&self, count: GLsizei, samplers: *const GLuint) {
    (self.glDeleteSamplers_p)(count, samplers)
  }
  /// [glDeleteShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteShader.xhtml)
  ///
  /// Marks a shader to be deleted. If it's not attached to a program it will be
  /// deleted immediately, otherwise it won't be deleted until it's unattached.
  ///
  /// * `shader` names the shader to mark for deletion.
  pub fn DeleteShader(&self, shader: GLuint) {
    (self.glDeleteShader_p)(shader)
  }
  /// glDeleteSync
  /// * `sync` group: sync
  /// * `sync` class: sync
  pub unsafe fn DeleteSync(&self, sync: GLsync) {
    (self.glDeleteSync_p)(sync)
  }
  /// glDeleteTextures
  /// * `textures` group: Texture
  /// * `textures` class: texture
  /// * `textures` len: n
  pub unsafe fn DeleteTextures(&self, n: GLsizei, textures: *const GLuint) {
    (self.glDeleteTextures_p)(n, textures)
  }
  /// [glDeleteVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteVertexArrays.xhtml)
  ///
  /// Deletes a list of vertex array objects. If a vertex array object that is
  /// bound is deleted then the binding reverts to 0 and the default vertex
  /// array becomes current. Passing any vertex array object IDs not currently
  /// in use, or passing 0, is silently ignored.
  ///
  /// * `n` the size of the list
  /// * `arrays` the vertex array objects to delete.
  pub unsafe fn DeleteVertexArrays(&self, n: GLsizei, arrays: *const GLuint) {
    (self.glDeleteVertexArrays_p)(n, arrays)
  }
  /// glDepthFunc
  /// * `func` group: DepthFunction
  pub unsafe fn DepthFunc(&self, func: DepthFunction) {
    (self.glDepthFunc_p)(func)
  }
  /// glDepthMask
  /// * `flag` group: Boolean
  pub unsafe fn DepthMask(&self, flag: GLboolean) {
    (self.glDepthMask_p)(flag)
  }
  /// glDepthRange
  pub unsafe fn DepthRange(&self, n: GLdouble, f: GLdouble) {
    (self.glDepthRange_p)(n, f)
  }
  /// glDetachShader
  /// * `program` class: program
  /// * `shader` class: shader
  pub unsafe fn DetachShader(&self, program: GLuint, shader: GLuint) {
    (self.glDetachShader_p)(program, shader)
  }
  /// glDisable
  /// * `cap` group: EnableCap
  pub unsafe fn Disable(&self, cap: EnableCap) {
    (self.glDisable_p)(cap)
  }
  /// glDisableVertexAttribArray
  pub unsafe fn DisableVertexAttribArray(&self, index: GLuint) {
    (self.glDisableVertexAttribArray_p)(index)
  }
  /// glDisablei
  /// * `target` group: EnableCap
  pub unsafe fn Disablei(&self, target: EnableCap, index: GLuint) {
    (self.glDisablei_p)(target, index)
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
  pub unsafe fn DrawArrays(&self, mode: PrimitiveType, first: GLint, count: GLsizei) {
    (self.glDrawArrays_p)(mode, first, count)
  }
  /// glDrawArraysInstanced
  /// * `mode` group: PrimitiveType
  pub unsafe fn DrawArraysInstanced(&self, mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei) {
    (self.glDrawArraysInstanced_p)(mode, first, count, instancecount)
  }
  /// glDrawBuffer
  /// * `buf` group: DrawBufferMode
  pub unsafe fn DrawBuffer(&self, buf: DrawBufferMode) {
    (self.glDrawBuffer_p)(buf)
  }
  /// glDrawBuffers
  /// * `bufs` group: DrawBufferMode
  /// * `bufs` len: n
  pub unsafe fn DrawBuffers(&self, n: GLsizei, bufs: *const DrawBufferMode) {
    (self.glDrawBuffers_p)(n, bufs)
  }
  /// glDrawElements
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  pub unsafe fn DrawElements(&self, mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void) {
    (self.glDrawElements_p)(mode, count, type_, indices)
  }
  /// glDrawElementsBaseVertex
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  pub unsafe fn DrawElementsBaseVertex(&self, mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint) {
    (self.glDrawElementsBaseVertex_p)(mode, count, type_, indices, basevertex)
  }
  /// glDrawElementsInstanced
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  pub unsafe fn DrawElementsInstanced(&self, mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei) {
    (self.glDrawElementsInstanced_p)(mode, count, type_, indices, instancecount)
  }
  /// glDrawElementsInstancedBaseVertex
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  pub unsafe fn DrawElementsInstancedBaseVertex(&self, mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint) {
    (self.glDrawElementsInstancedBaseVertex_p)(mode, count, type_, indices, instancecount, basevertex)
  }
  /// glDrawRangeElements
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  pub unsafe fn DrawRangeElements(&self, mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void) {
    (self.glDrawRangeElements_p)(mode, start, end, count, type_, indices)
  }
  /// glDrawRangeElementsBaseVertex
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  pub unsafe fn DrawRangeElementsBaseVertex(&self, mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint) {
    (self.glDrawRangeElementsBaseVertex_p)(mode, start, end, count, type_, indices, basevertex)
  }
  /// glEnable
  /// * `cap` group: EnableCap
  pub unsafe fn Enable(&self, cap: EnableCap) {
    (self.glEnable_p)(cap)
  }
  /// glEnableVertexAttribArray
  pub unsafe fn EnableVertexAttribArray(&self, index: GLuint) {
    (self.glEnableVertexAttribArray_p)(index)
  }
  /// glEnablei
  /// * `target` group: EnableCap
  pub unsafe fn Enablei(&self, target: EnableCap, index: GLuint) {
    (self.glEnablei_p)(target, index)
  }
  /// glEndConditionalRender
  pub unsafe fn EndConditionalRender(&self) {
    (self.glEndConditionalRender_p)()
  }
  /// glEndQuery
  /// * `target` group: QueryTarget
  pub unsafe fn EndQuery(&self, target: QueryTarget) {
    (self.glEndQuery_p)(target)
  }
  /// glEndTransformFeedback
  pub unsafe fn EndTransformFeedback(&self) {
    (self.glEndTransformFeedback_p)()
  }
  /// glFenceSync
  /// * `condition` group: SyncCondition
  /// * `flags` group: SyncBehaviorFlags
  pub unsafe fn FenceSync(&self, condition: SyncCondition, flags: GLbitfield) -> GLsync {
    (self.glFenceSync_p)(condition, flags)
  }
  /// glFinish
  pub unsafe fn Finish(&self) {
    (self.glFinish_p)()
  }
  /// glFlush
  pub unsafe fn Flush(&self) {
    (self.glFlush_p)()
  }
  /// glFlushMappedBufferRange
  /// * `target` group: BufferTargetARB
  /// * `offset` group: BufferOffset
  /// * `length` group: BufferSize
  pub unsafe fn FlushMappedBufferRange(&self, target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr) {
    (self.glFlushMappedBufferRange_p)(target, offset, length)
  }
  /// glFramebufferRenderbuffer
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `renderbuffertarget` group: RenderbufferTarget
  /// * `renderbuffer` class: renderbuffer
  pub unsafe fn FramebufferRenderbuffer(&self, target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint) {
    (self.glFramebufferRenderbuffer_p)(target, attachment, renderbuffertarget, renderbuffer)
  }
  /// glFramebufferTexture
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `texture` class: texture
  pub unsafe fn FramebufferTexture(&self, target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint) {
    (self.glFramebufferTexture_p)(target, attachment, texture, level)
  }
  /// glFramebufferTexture1D
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `textarget` group: TextureTarget
  /// * `texture` class: texture
  pub unsafe fn FramebufferTexture1D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint) {
    (self.glFramebufferTexture1D_p)(target, attachment, textarget, texture, level)
  }
  /// glFramebufferTexture2D
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `textarget` group: TextureTarget
  /// * `texture` class: texture
  pub unsafe fn FramebufferTexture2D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint) {
    (self.glFramebufferTexture2D_p)(target, attachment, textarget, texture, level)
  }
  /// glFramebufferTexture3D
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `textarget` group: TextureTarget
  /// * `texture` class: texture
  pub unsafe fn FramebufferTexture3D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint) {
    (self.glFramebufferTexture3D_p)(target, attachment, textarget, texture, level, zoffset)
  }
  /// glFramebufferTextureLayer
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `texture` group: Texture
  /// * `texture` class: texture
  /// * `level` group: CheckedInt32
  /// * `layer` group: CheckedInt32
  pub unsafe fn FramebufferTextureLayer(&self, target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint) {
    (self.glFramebufferTextureLayer_p)(target, attachment, texture, level, layer)
  }
  /// glFrontFace
  /// * `mode` group: FrontFaceDirection
  pub unsafe fn FrontFace(&self, mode: FrontFaceDirection) {
    (self.glFrontFace_p)(mode)
  }
  /// glGenBuffers
  /// * `buffers` class: buffer
  /// * `buffers` len: n
  pub unsafe fn GenBuffers(&self, n: GLsizei, buffers: *mut GLuint) {
    (self.glGenBuffers_p)(n, buffers)
  }
  /// glGenFramebuffers
  /// * `framebuffers` class: framebuffer
  /// * `framebuffers` len: n
  pub unsafe fn GenFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) {
    (self.glGenFramebuffers_p)(n, framebuffers)
  }
  /// glGenQueries
  /// * `ids` class: query
  /// * `ids` len: n
  pub unsafe fn GenQueries(&self, n: GLsizei, ids: *mut GLuint) {
    (self.glGenQueries_p)(n, ids)
  }
  /// glGenRenderbuffers
  /// * `renderbuffers` class: renderbuffer
  /// * `renderbuffers` len: n
  pub unsafe fn GenRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) {
    (self.glGenRenderbuffers_p)(n, renderbuffers)
  }
  /// glGenSamplers
  /// * `samplers` class: sampler
  /// * `samplers` len: count
  pub unsafe fn GenSamplers(&self, count: GLsizei, samplers: *mut GLuint) {
    (self.glGenSamplers_p)(count, samplers)
  }
  /// glGenTextures
  /// * `textures` group: Texture
  /// * `textures` class: texture
  /// * `textures` len: n
  pub unsafe fn GenTextures(&self, n: GLsizei, textures: *mut GLuint) {
    (self.glGenTextures_p)(n, textures)
  }
  /// glGenVertexArrays
  /// * `arrays` class: vertex array
  /// * `arrays` len: n
  pub unsafe fn GenVertexArrays(&self, n: GLsizei, arrays: *mut GLuint) {
    (self.glGenVertexArrays_p)(n, arrays)
  }
  /// glGenerateMipmap
  /// * `target` group: TextureTarget
  pub unsafe fn GenerateMipmap(&self, target: TextureTarget) {
    (self.glGenerateMipmap_p)(target)
  }
  /// glGetActiveAttrib
  /// * `program` class: program
  /// * `type` group: AttributeType
  /// * `name` len: bufSize
  pub unsafe fn GetActiveAttrib(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut AttributeType, name: *mut GLchar) {
    (self.glGetActiveAttrib_p)(program, index, bufSize, length, size, type_, name)
  }
  /// glGetActiveUniform
  /// * `program` class: program
  /// * `type` group: UniformType
  /// * `name` len: bufSize
  pub unsafe fn GetActiveUniform(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut UniformType, name: *mut GLchar) {
    (self.glGetActiveUniform_p)(program, index, bufSize, length, size, type_, name)
  }
  /// glGetActiveUniformBlockName
  /// * `program` class: program
  /// * `uniformBlockName` len: bufSize
  pub unsafe fn GetActiveUniformBlockName(&self, program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) {
    (self.glGetActiveUniformBlockName_p)(program, uniformBlockIndex, bufSize, length, uniformBlockName)
  }
  /// glGetActiveUniformBlockiv
  /// * `program` class: program
  /// * `pname` group: UniformBlockPName
  /// * `params` len: COMPSIZE(program,uniformBlockIndex,pname)
  pub unsafe fn GetActiveUniformBlockiv(&self, program: GLuint, uniformBlockIndex: GLuint, pname: UniformBlockPName, params: *mut GLint) {
    (self.glGetActiveUniformBlockiv_p)(program, uniformBlockIndex, pname, params)
  }
  /// glGetActiveUniformName
  /// * `program` class: program
  /// * `uniformName` len: bufSize
  pub unsafe fn GetActiveUniformName(&self, program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) {
    (self.glGetActiveUniformName_p)(program, uniformIndex, bufSize, length, uniformName)
  }
  /// glGetActiveUniformsiv
  /// * `program` class: program
  /// * `uniformIndices` len: uniformCount
  /// * `pname` group: UniformPName
  /// * `params` len: COMPSIZE(uniformCount,pname)
  pub unsafe fn GetActiveUniformsiv(&self, program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: UniformPName, params: *mut GLint) {
    (self.glGetActiveUniformsiv_p)(program, uniformCount, uniformIndices, pname, params)
  }
  /// glGetAttachedShaders
  /// * `program` class: program
  /// * `shaders` class: shader
  /// * `shaders` len: maxCount
  pub unsafe fn GetAttachedShaders(&self, program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) {
    (self.glGetAttachedShaders_p)(program, maxCount, count, shaders)
  }
  /// glGetAttribLocation
  /// * `program` class: program
  pub unsafe fn GetAttribLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
    (self.glGetAttribLocation_p)(program, name)
  }
  /// glGetBooleani_v
  /// * `target` group: BufferTargetARB
  /// * `data` group: Boolean
  /// * `data` len: COMPSIZE(target)
  pub unsafe fn GetBooleani_v(&self, target: BufferTargetARB, index: GLuint, data: *mut GLboolean) {
    (self.glGetBooleani_v_p)(target, index, data)
  }
  /// glGetBooleanv
  /// * `pname` group: GetPName
  /// * `data` group: Boolean
  /// * `data` len: COMPSIZE(pname)
  pub unsafe fn GetBooleanv(&self, pname: GetPName, data: *mut GLboolean) {
    (self.glGetBooleanv_p)(pname, data)
  }
  /// glGetBufferParameteri64v
  /// * `target` group: BufferTargetARB
  /// * `pname` group: BufferPNameARB
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetBufferParameteri64v(&self, target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint64) {
    (self.glGetBufferParameteri64v_p)(target, pname, params)
  }
  /// glGetBufferParameteriv
  /// * `target` group: BufferTargetARB
  /// * `pname` group: BufferPNameARB
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetBufferParameteriv(&self, target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint) {
    (self.glGetBufferParameteriv_p)(target, pname, params)
  }
  /// glGetBufferPointerv
  /// * `target` group: BufferTargetARB
  /// * `pname` group: BufferPointerNameARB
  pub unsafe fn GetBufferPointerv(&self, target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void) {
    (self.glGetBufferPointerv_p)(target, pname, params)
  }
  /// glGetBufferSubData
  /// * `target` group: BufferTargetARB
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  /// * `data` len: size
  pub unsafe fn GetBufferSubData(&self, target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *mut void) {
    (self.glGetBufferSubData_p)(target, offset, size, data)
  }
  /// glGetCompressedTexImage
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `img` group: CompressedTextureARB
  /// * `img` len: COMPSIZE(target,level)
  pub unsafe fn GetCompressedTexImage(&self, target: TextureTarget, level: GLint, img: *mut void) {
    (self.glGetCompressedTexImage_p)(target, level, img)
  }
  /// glGetDoublev
  /// * `pname` group: GetPName
  /// * `data` len: COMPSIZE(pname)
  pub unsafe fn GetDoublev(&self, pname: GetPName, data: *mut GLdouble) {
    (self.glGetDoublev_p)(pname, data)
  }
  /// glGetError
  pub unsafe fn GetError(&self) -> ErrorCode {
    (self.glGetError_p)()
  }
  /// glGetFloatv
  /// * `pname` group: GetPName
  /// * `data` len: COMPSIZE(pname)
  pub unsafe fn GetFloatv(&self, pname: GetPName, data: *mut GLfloat) {
    (self.glGetFloatv_p)(pname, data)
  }
  /// glGetFragDataIndex
  /// * `program` class: program
  pub unsafe fn GetFragDataIndex(&self, program: GLuint, name: *const GLchar) -> GLint {
    (self.glGetFragDataIndex_p)(program, name)
  }
  /// glGetFragDataLocation
  /// * `program` class: program
  /// * `name` len: COMPSIZE(name)
  pub unsafe fn GetFragDataLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
    (self.glGetFragDataLocation_p)(program, name)
  }
  /// glGetFramebufferAttachmentParameteriv
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `pname` group: FramebufferAttachmentParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetFramebufferAttachmentParameteriv(&self, target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint) {
    (self.glGetFramebufferAttachmentParameteriv_p)(target, attachment, pname, params)
  }
  /// glGetInteger64i_v
  /// * `target` group: GetPName
  /// * `data` len: COMPSIZE(target)
  pub unsafe fn GetInteger64i_v(&self, target: GetPName, index: GLuint, data: *mut GLint64) {
    (self.glGetInteger64i_v_p)(target, index, data)
  }
  /// glGetInteger64v
  /// * `pname` group: GetPName
  /// * `data` len: COMPSIZE(pname)
  pub unsafe fn GetInteger64v(&self, pname: GetPName, data: *mut GLint64) {
    (self.glGetInteger64v_p)(pname, data)
  }
  /// glGetIntegeri_v
  /// * `target` group: GetPName
  /// * `data` len: COMPSIZE(target)
  pub unsafe fn GetIntegeri_v(&self, target: GetPName, index: GLuint, data: *mut GLint) {
    (self.glGetIntegeri_v_p)(target, index, data)
  }
  /// glGetIntegerv
  /// * `pname` group: GetPName
  /// * `data` len: COMPSIZE(pname)
  pub unsafe fn GetIntegerv(&self, pname: GetPName, data: *mut GLint) {
    (self.glGetIntegerv_p)(pname, data)
  }
  /// glGetMultisamplefv
  /// * `pname` group: GetMultisamplePNameNV
  /// * `val` len: COMPSIZE(pname)
  pub unsafe fn GetMultisamplefv(&self, pname: GetMultisamplePNameNV, index: GLuint, val: *mut GLfloat) {
    (self.glGetMultisamplefv_p)(pname, index, val)
  }
  /// glGetProgramInfoLog
  /// * `program` class: program
  /// * `infoLog` len: bufSize
  pub unsafe fn GetProgramInfoLog(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
    (self.glGetProgramInfoLog_p)(program, bufSize, length, infoLog)
  }
  /// glGetProgramiv
  /// * `program` class: program
  /// * `pname` group: ProgramPropertyARB
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetProgramiv(&self, program: GLuint, pname: ProgramPropertyARB, params: *mut GLint) {
    (self.glGetProgramiv_p)(program, pname, params)
  }
  /// glGetQueryObjecti64v
  /// * `id` class: query
  /// * `pname` group: QueryObjectParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetQueryObjecti64v(&self, id: GLuint, pname: QueryObjectParameterName, params: *mut GLint64) {
    (self.glGetQueryObjecti64v_p)(id, pname, params)
  }
  /// glGetQueryObjectiv
  /// * `id` class: query
  /// * `pname` group: QueryObjectParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetQueryObjectiv(&self, id: GLuint, pname: QueryObjectParameterName, params: *mut GLint) {
    (self.glGetQueryObjectiv_p)(id, pname, params)
  }
  /// glGetQueryObjectui64v
  /// * `id` class: query
  /// * `pname` group: QueryObjectParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetQueryObjectui64v(&self, id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint64) {
    (self.glGetQueryObjectui64v_p)(id, pname, params)
  }
  /// glGetQueryObjectuiv
  /// * `id` class: query
  /// * `pname` group: QueryObjectParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetQueryObjectuiv(&self, id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint) {
    (self.glGetQueryObjectuiv_p)(id, pname, params)
  }
  /// glGetQueryiv
  /// * `target` group: QueryTarget
  /// * `pname` group: QueryParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetQueryiv(&self, target: QueryTarget, pname: QueryParameterName, params: *mut GLint) {
    (self.glGetQueryiv_p)(target, pname, params)
  }
  /// glGetRenderbufferParameteriv
  /// * `target` group: RenderbufferTarget
  /// * `pname` group: RenderbufferParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetRenderbufferParameteriv(&self, target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint) {
    (self.glGetRenderbufferParameteriv_p)(target, pname, params)
  }
  /// glGetSamplerParameterIiv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetSamplerParameterIiv(&self, sampler: GLuint, pname: SamplerParameterI, params: *mut GLint) {
    (self.glGetSamplerParameterIiv_p)(sampler, pname, params)
  }
  /// glGetSamplerParameterIuiv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetSamplerParameterIuiv(&self, sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint) {
    (self.glGetSamplerParameterIuiv_p)(sampler, pname, params)
  }
  /// glGetSamplerParameterfv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterF
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetSamplerParameterfv(&self, sampler: GLuint, pname: SamplerParameterF, params: *mut GLfloat) {
    (self.glGetSamplerParameterfv_p)(sampler, pname, params)
  }
  /// glGetSamplerParameteriv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetSamplerParameteriv(&self, sampler: GLuint, pname: SamplerParameterI, params: *mut GLint) {
    (self.glGetSamplerParameteriv_p)(sampler, pname, params)
  }
  /// glGetShaderInfoLog
  /// * `shader` class: shader
  /// * `infoLog` len: bufSize
  pub unsafe fn GetShaderInfoLog(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
    (self.glGetShaderInfoLog_p)(shader, bufSize, length, infoLog)
  }
  /// glGetShaderSource
  /// * `shader` class: shader
  /// * `source` len: bufSize
  pub unsafe fn GetShaderSource(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) {
    (self.glGetShaderSource_p)(shader, bufSize, length, source)
  }
  /// glGetShaderiv
  /// * `shader` class: shader
  /// * `pname` group: ShaderParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetShaderiv(&self, shader: GLuint, pname: ShaderParameterName, params: *mut GLint) {
    (self.glGetShaderiv_p)(shader, pname, params)
  }
  /// glGetString
  /// * `name` group: StringName
  pub unsafe fn GetString(&self, name: StringName) -> GLubyte {
    (self.glGetString_p)(name)
  }
  /// glGetStringi
  /// * `name` group: StringName
  pub unsafe fn GetStringi(&self, name: StringName, index: GLuint) -> GLubyte {
    (self.glGetStringi_p)(name, index)
  }
  /// glGetSynciv
  /// * `sync` group: sync
  /// * `sync` class: sync
  /// * `pname` group: SyncParameterName
  /// * `values` len: count
  pub unsafe fn GetSynciv(&self, sync: GLsync, pname: SyncParameterName, count: GLsizei, length: *mut GLsizei, values: *mut GLint) {
    (self.glGetSynciv_p)(sync, pname, count, length, values)
  }
  /// glGetTexImage
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(target,level,format,type)
  pub unsafe fn GetTexImage(&self, target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void) {
    (self.glGetTexImage_p)(target, level, format, type_, pixels)
  }
  /// glGetTexLevelParameterfv
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetTexLevelParameterfv(&self, target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat) {
    (self.glGetTexLevelParameterfv_p)(target, level, pname, params)
  }
  /// glGetTexLevelParameteriv
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetTexLevelParameteriv(&self, target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint) {
    (self.glGetTexLevelParameteriv_p)(target, level, pname, params)
  }
  /// glGetTexParameterIiv
  /// * `target` group: TextureTarget
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetTexParameterIiv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut GLint) {
    (self.glGetTexParameterIiv_p)(target, pname, params)
  }
  /// glGetTexParameterIuiv
  /// * `target` group: TextureTarget
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetTexParameterIuiv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint) {
    (self.glGetTexParameterIuiv_p)(target, pname, params)
  }
  /// glGetTexParameterfv
  /// * `target` group: TextureTarget
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetTexParameterfv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat) {
    (self.glGetTexParameterfv_p)(target, pname, params)
  }
  /// glGetTexParameteriv
  /// * `target` group: TextureTarget
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetTexParameteriv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut GLint) {
    (self.glGetTexParameteriv_p)(target, pname, params)
  }
  /// glGetTransformFeedbackVarying
  /// * `program` class: program
  /// * `type` group: AttributeType
  /// * `name` len: bufSize
  pub unsafe fn GetTransformFeedbackVarying(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut AttributeType, name: *mut GLchar) {
    (self.glGetTransformFeedbackVarying_p)(program, index, bufSize, length, size, type_, name)
  }
  /// glGetUniformBlockIndex
  /// * `program` class: program
  /// * `uniformBlockName` len: COMPSIZE()
  pub unsafe fn GetUniformBlockIndex(&self, program: GLuint, uniformBlockName: *const GLchar) -> GLuint {
    (self.glGetUniformBlockIndex_p)(program, uniformBlockName)
  }
  /// glGetUniformIndices
  /// * `program` class: program
  /// * `uniformNames` len: COMPSIZE(uniformCount)
  /// * `uniformIndices` len: COMPSIZE(uniformCount)
  pub unsafe fn GetUniformIndices(&self, program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) {
    (self.glGetUniformIndices_p)(program, uniformCount, uniformNames, uniformIndices)
  }
  /// glGetUniformLocation
  /// * `program` class: program
  pub unsafe fn GetUniformLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
    (self.glGetUniformLocation_p)(program, name)
  }
  /// glGetUniformfv
  /// * `program` class: program
  /// * `params` len: COMPSIZE(program,location)
  pub unsafe fn GetUniformfv(&self, program: GLuint, location: GLint, params: *mut GLfloat) {
    (self.glGetUniformfv_p)(program, location, params)
  }
  /// glGetUniformiv
  /// * `program` class: program
  /// * `params` len: COMPSIZE(program,location)
  pub unsafe fn GetUniformiv(&self, program: GLuint, location: GLint, params: *mut GLint) {
    (self.glGetUniformiv_p)(program, location, params)
  }
  /// glGetUniformuiv
  /// * `program` class: program
  /// * `params` len: COMPSIZE(program,location)
  pub unsafe fn GetUniformuiv(&self, program: GLuint, location: GLint, params: *mut GLuint) {
    (self.glGetUniformuiv_p)(program, location, params)
  }
  /// glGetVertexAttribIiv
  /// * `pname` group: VertexAttribEnum
  pub unsafe fn GetVertexAttribIiv(&self, index: GLuint, pname: VertexAttribEnum, params: *mut GLint) {
    (self.glGetVertexAttribIiv_p)(index, pname, params)
  }
  /// glGetVertexAttribIuiv
  /// * `pname` group: VertexAttribEnum
  pub unsafe fn GetVertexAttribIuiv(&self, index: GLuint, pname: VertexAttribEnum, params: *mut GLuint) {
    (self.glGetVertexAttribIuiv_p)(index, pname, params)
  }
  /// glGetVertexAttribPointerv
  /// * `pname` group: VertexAttribPointerPropertyARB
  pub unsafe fn GetVertexAttribPointerv(&self, index: GLuint, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut void) {
    (self.glGetVertexAttribPointerv_p)(index, pname, pointer)
  }
  /// glGetVertexAttribdv
  /// * `pname` group: VertexAttribPropertyARB
  pub unsafe fn GetVertexAttribdv(&self, index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLdouble; 4]) {
    (self.glGetVertexAttribdv_p)(index, pname, params)
  }
  /// glGetVertexAttribfv
  /// * `pname` group: VertexAttribPropertyARB
  pub unsafe fn GetVertexAttribfv(&self, index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLfloat; 4]) {
    (self.glGetVertexAttribfv_p)(index, pname, params)
  }
  /// glGetVertexAttribiv
  /// * `pname` group: VertexAttribPropertyARB
  pub unsafe fn GetVertexAttribiv(&self, index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLint; 4]) {
    (self.glGetVertexAttribiv_p)(index, pname, params)
  }
  /// glHint
  /// * `target` group: HintTarget
  /// * `mode` group: HintMode
  pub unsafe fn Hint(&self, target: HintTarget, mode: HintMode) {
    (self.glHint_p)(target, mode)
  }
  /// glIsBuffer
  /// * `buffer` class: buffer
  pub unsafe fn IsBuffer(&self, buffer: GLuint) -> GLboolean {
    (self.glIsBuffer_p)(buffer)
  }
  /// glIsEnabled
  /// * `cap` group: EnableCap
  pub unsafe fn IsEnabled(&self, cap: EnableCap) -> GLboolean {
    (self.glIsEnabled_p)(cap)
  }
  /// glIsEnabledi
  /// * `target` group: EnableCap
  pub unsafe fn IsEnabledi(&self, target: EnableCap, index: GLuint) -> GLboolean {
    (self.glIsEnabledi_p)(target, index)
  }
  /// glIsFramebuffer
  /// * `framebuffer` class: framebuffer
  pub unsafe fn IsFramebuffer(&self, framebuffer: GLuint) -> GLboolean {
    (self.glIsFramebuffer_p)(framebuffer)
  }
  /// glIsProgram
  /// * `program` class: program
  pub unsafe fn IsProgram(&self, program: GLuint) -> GLboolean {
    (self.glIsProgram_p)(program)
  }
  /// glIsQuery
  /// * `id` class: query
  pub unsafe fn IsQuery(&self, id: GLuint) -> GLboolean {
    (self.glIsQuery_p)(id)
  }
  /// glIsRenderbuffer
  /// * `renderbuffer` class: renderbuffer
  pub unsafe fn IsRenderbuffer(&self, renderbuffer: GLuint) -> GLboolean {
    (self.glIsRenderbuffer_p)(renderbuffer)
  }
  /// glIsSampler
  /// * `sampler` class: sampler
  pub unsafe fn IsSampler(&self, sampler: GLuint) -> GLboolean {
    (self.glIsSampler_p)(sampler)
  }
  /// glIsShader
  /// * `shader` class: shader
  pub unsafe fn IsShader(&self, shader: GLuint) -> GLboolean {
    (self.glIsShader_p)(shader)
  }
  /// glIsSync
  /// * `sync` group: sync
  /// * `sync` class: sync
  pub unsafe fn IsSync(&self, sync: GLsync) -> GLboolean {
    (self.glIsSync_p)(sync)
  }
  /// glIsTexture
  /// * `texture` group: Texture
  /// * `texture` class: texture
  pub unsafe fn IsTexture(&self, texture: GLuint) -> GLboolean {
    (self.glIsTexture_p)(texture)
  }
  /// glIsVertexArray
  /// * `array` class: vertex array
  pub unsafe fn IsVertexArray(&self, array: GLuint) -> GLboolean {
    (self.glIsVertexArray_p)(array)
  }
  /// glLineWidth
  /// * `width` group: CheckedFloat32
  pub unsafe fn LineWidth(&self, width: GLfloat) {
    (self.glLineWidth_p)(width)
  }
  /// [glLinkProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLinkProgram.xhtml)
  ///
  /// Performs linking on a program object. The link status of the program will
  /// be stored in its object state, you can check it with `glGetProgram`
  /// and/or `glGetProgramInfoLog`.
  ///
  /// * `program` the name of the program to link
  pub fn LinkProgram(&self, program: GLuint) {
    (self.glLinkProgram_p)(program)
  }
  /// glLogicOp
  /// * `opcode` group: LogicOp
  pub unsafe fn LogicOp(&self, opcode: LogicOp) {
    (self.glLogicOp_p)(opcode)
  }
  /// glMapBuffer
  /// * `target` group: BufferTargetARB
  /// * `access` group: BufferAccessARB
  pub unsafe fn MapBuffer(&self, target: BufferTargetARB, access: BufferAccessARB) -> *mut void {
    (self.glMapBuffer_p)(target, access)
  }
  /// glMapBufferRange
  /// * `target` group: BufferTargetARB
  /// * `offset` group: BufferOffset
  /// * `length` group: BufferSize
  /// * `access` group: MapBufferAccessMask
  pub unsafe fn MapBufferRange(&self, target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void {
    (self.glMapBufferRange_p)(target, offset, length, access)
  }
  /// glMultiDrawArrays
  /// * `mode` group: PrimitiveType
  /// * `first` len: COMPSIZE(drawcount)
  /// * `count` len: COMPSIZE(drawcount)
  pub unsafe fn MultiDrawArrays(&self, mode: PrimitiveType, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) {
    (self.glMultiDrawArrays_p)(mode, first, count, drawcount)
  }
  /// glMultiDrawElements
  /// * `mode` group: PrimitiveType
  /// * `count` len: COMPSIZE(drawcount)
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(drawcount)
  pub unsafe fn MultiDrawElements(&self, mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei) {
    (self.glMultiDrawElements_p)(mode, count, type_, indices, drawcount)
  }
  /// glMultiDrawElementsBaseVertex
  /// * `mode` group: PrimitiveType
  /// * `count` len: COMPSIZE(drawcount)
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(drawcount)
  /// * `basevertex` len: COMPSIZE(drawcount)
  pub unsafe fn MultiDrawElementsBaseVertex(&self, mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint) {
    (self.glMultiDrawElementsBaseVertex_p)(mode, count, type_, indices, drawcount, basevertex)
  }
  /// glPixelStoref
  /// * `pname` group: PixelStoreParameter
  /// * `param` group: CheckedFloat32
  pub unsafe fn PixelStoref(&self, pname: PixelStoreParameter, param: GLfloat) {
    (self.glPixelStoref_p)(pname, param)
  }
  /// glPixelStorei
  /// * `pname` group: PixelStoreParameter
  /// * `param` group: CheckedInt32
  pub unsafe fn PixelStorei(&self, pname: PixelStoreParameter, param: GLint) {
    (self.glPixelStorei_p)(pname, param)
  }
  /// glPointParameterf
  /// * `pname` group: PointParameterNameARB
  /// * `param` group: CheckedFloat32
  pub unsafe fn PointParameterf(&self, pname: PointParameterNameARB, param: GLfloat) {
    (self.glPointParameterf_p)(pname, param)
  }
  /// glPointParameterfv
  /// * `pname` group: PointParameterNameARB
  /// * `params` group: CheckedFloat32
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn PointParameterfv(&self, pname: PointParameterNameARB, params: *const GLfloat) {
    (self.glPointParameterfv_p)(pname, params)
  }
  /// glPointParameteri
  /// * `pname` group: PointParameterNameARB
  pub unsafe fn PointParameteri(&self, pname: PointParameterNameARB, param: GLint) {
    (self.glPointParameteri_p)(pname, param)
  }
  /// glPointParameteriv
  /// * `pname` group: PointParameterNameARB
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn PointParameteriv(&self, pname: PointParameterNameARB, params: *const GLint) {
    (self.glPointParameteriv_p)(pname, params)
  }
  /// [glPointSize](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointSize.xhtml)
  ///
  /// Sets the diameter of rasterized points if `GL_PROGRAM_POINT_SIZE` is
  /// *disabled*. (Otherwise, this setting is ignored and you must modify
  /// `gl_PointSize` from within a shader to change point size.)
  ///
  /// The default point size is 1.0, and it cannot be set to less than 0.0.
  pub fn PointSize(&self, size: GLfloat) {
    (self.glPointSize_p)(size)
  }
  /// glPolygonMode
  /// * `face` group: MaterialFace
  /// * `mode` group: PolygonMode
  pub unsafe fn PolygonMode(&self, face: MaterialFace, mode: PolygonMode) {
    (self.glPolygonMode_p)(face, mode)
  }
  /// glPolygonOffset
  pub unsafe fn PolygonOffset(&self, factor: GLfloat, units: GLfloat) {
    (self.glPolygonOffset_p)(factor, units)
  }
  /// glPrimitiveRestartIndex
  pub unsafe fn PrimitiveRestartIndex(&self, index: GLuint) {
    (self.glPrimitiveRestartIndex_p)(index)
  }
  /// glProvokingVertex
  /// * `mode` group: VertexProvokingMode
  pub unsafe fn ProvokingVertex(&self, mode: VertexProvokingMode) {
    (self.glProvokingVertex_p)(mode)
  }
  /// glQueryCounter
  /// * `id` class: query
  /// * `target` group: QueryCounterTarget
  pub unsafe fn QueryCounter(&self, id: GLuint, target: QueryCounterTarget) {
    (self.glQueryCounter_p)(id, target)
  }
  /// glReadBuffer
  /// * `src` group: ReadBufferMode
  pub unsafe fn ReadBuffer(&self, src: ReadBufferMode) {
    (self.glReadBuffer_p)(src)
  }
  /// glReadPixels
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height)
  pub unsafe fn ReadPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *mut void) {
    (self.glReadPixels_p)(x, y, width, height, format, type_, pixels)
  }
  /// glRenderbufferStorage
  /// * `target` group: RenderbufferTarget
  /// * `internalformat` group: InternalFormat
  pub unsafe fn RenderbufferStorage(&self, target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
    (self.glRenderbufferStorage_p)(target, internalformat, width, height)
  }
  /// glRenderbufferStorageMultisample
  /// * `target` group: RenderbufferTarget
  /// * `internalformat` group: InternalFormat
  pub unsafe fn RenderbufferStorageMultisample(&self, target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
    (self.glRenderbufferStorageMultisample_p)(target, samples, internalformat, width, height)
  }
  /// glSampleCoverage
  /// * `invert` group: Boolean
  pub unsafe fn SampleCoverage(&self, value: GLfloat, invert: GLboolean) {
    (self.glSampleCoverage_p)(value, invert)
  }
  /// glSampleMaski
  pub unsafe fn SampleMaski(&self, maskNumber: GLuint, mask: GLbitfield) {
    (self.glSampleMaski_p)(maskNumber, mask)
  }
  /// glSamplerParameterIiv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `param` len: COMPSIZE(pname)
  pub unsafe fn SamplerParameterIiv(&self, sampler: GLuint, pname: SamplerParameterI, param: *const GLint) {
    (self.glSamplerParameterIiv_p)(sampler, pname, param)
  }
  /// glSamplerParameterIuiv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `param` len: COMPSIZE(pname)
  pub unsafe fn SamplerParameterIuiv(&self, sampler: GLuint, pname: SamplerParameterI, param: *const GLuint) {
    (self.glSamplerParameterIuiv_p)(sampler, pname, param)
  }
  /// glSamplerParameterf
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterF
  pub unsafe fn SamplerParameterf(&self, sampler: GLuint, pname: SamplerParameterF, param: GLfloat) {
    (self.glSamplerParameterf_p)(sampler, pname, param)
  }
  /// glSamplerParameterfv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterF
  /// * `param` len: COMPSIZE(pname)
  pub unsafe fn SamplerParameterfv(&self, sampler: GLuint, pname: SamplerParameterF, param: *const GLfloat) {
    (self.glSamplerParameterfv_p)(sampler, pname, param)
  }
  /// glSamplerParameteri
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  pub unsafe fn SamplerParameteri(&self, sampler: GLuint, pname: SamplerParameterI, param: GLint) {
    (self.glSamplerParameteri_p)(sampler, pname, param)
  }
  /// glSamplerParameteriv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `param` len: COMPSIZE(pname)
  pub unsafe fn SamplerParameteriv(&self, sampler: GLuint, pname: SamplerParameterI, param: *const GLint) {
    (self.glSamplerParameteriv_p)(sampler, pname, param)
  }
  /// glScissor
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  pub unsafe fn Scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    (self.glScissor_p)(x, y, width, height)
  }
  /// [glShaderSource](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSource.xhtml)
  ///
  /// Sets the source string of the named shader. This replaces any previously
  /// set source. OpenGL copies the data into its own memory, so you can free
  /// your instance of the source string after the call (if necessary).
  ///
  /// * `shader` the shader ID to attach the source to.
  /// * `count` the length of the `string` and `length` arrays.
  /// * `string` an array of pointers to the start of shader source code
  ///   strings.
  /// * `length` if non-null, this is an array of lengths for each pointer in
  ///   `string` (or negative if that entry is null-termiated). if `length` is
  ///   null then *all* strings in `string` must each be null-termianted.
  pub unsafe fn ShaderSource(&self, shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
    (self.glShaderSource_p)(shader, count, string, length)
  }
  /// glStencilFunc
  /// * `func` group: StencilFunction
  /// * `ref` group: StencilValue
  /// * `mask` group: MaskedStencilValue
  pub unsafe fn StencilFunc(&self, func: StencilFunction, ref_: GLint, mask: GLuint) {
    (self.glStencilFunc_p)(func, ref_, mask)
  }
  /// glStencilFuncSeparate
  /// * `face` group: StencilFaceDirection
  /// * `func` group: StencilFunction
  /// * `ref` group: StencilValue
  /// * `mask` group: MaskedStencilValue
  pub unsafe fn StencilFuncSeparate(&self, face: StencilFaceDirection, func: StencilFunction, ref_: GLint, mask: GLuint) {
    (self.glStencilFuncSeparate_p)(face, func, ref_, mask)
  }
  /// glStencilMask
  /// * `mask` group: MaskedStencilValue
  pub unsafe fn StencilMask(&self, mask: GLuint) {
    (self.glStencilMask_p)(mask)
  }
  /// glStencilMaskSeparate
  /// * `face` group: StencilFaceDirection
  /// * `mask` group: MaskedStencilValue
  pub unsafe fn StencilMaskSeparate(&self, face: StencilFaceDirection, mask: GLuint) {
    (self.glStencilMaskSeparate_p)(face, mask)
  }
  /// glStencilOp
  /// * `fail` group: StencilOp
  /// * `zfail` group: StencilOp
  /// * `zpass` group: StencilOp
  pub unsafe fn StencilOp(&self, fail: StencilOp, zfail: StencilOp, zpass: StencilOp) {
    (self.glStencilOp_p)(fail, zfail, zpass)
  }
  /// glStencilOpSeparate
  /// * `face` group: StencilFaceDirection
  /// * `sfail` group: StencilOp
  /// * `dpfail` group: StencilOp
  /// * `dppass` group: StencilOp
  pub unsafe fn StencilOpSeparate(&self, face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp) {
    (self.glStencilOpSeparate_p)(face, sfail, dpfail, dppass)
  }
  /// glTexBuffer
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `buffer` class: buffer
  pub unsafe fn TexBuffer(&self, target: TextureTarget, internalformat: InternalFormat, buffer: GLuint) {
    (self.glTexBuffer_p)(target, internalformat, buffer)
  }
  /// glTexImage1D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width)
  pub unsafe fn TexImage1D(&self, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTexImage1D_p)(target, level, internalformat, width, border, format, type_, pixels)
  }
  /// glTexImage2D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height)
  pub unsafe fn TexImage2D(&self, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTexImage2D_p)(target, level, internalformat, width, height, border, format, type_, pixels)
  }
  /// glTexImage2DMultisample
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  pub unsafe fn TexImage2DMultisample(&self, target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
    (self.glTexImage2DMultisample_p)(target, samples, internalformat, width, height, fixedsamplelocations)
  }
  /// glTexImage3D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height,depth)
  pub unsafe fn TexImage3D(&self, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTexImage3D_p)(target, level, internalformat, width, height, depth, border, format, type_, pixels)
  }
  /// glTexImage3DMultisample
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  pub unsafe fn TexImage3DMultisample(&self, target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
    (self.glTexImage3DMultisample_p)(target, samples, internalformat, width, height, depth, fixedsamplelocations)
  }
  /// glTexParameterIiv
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn TexParameterIiv(&self, target: TextureTarget, pname: TextureParameterName, params: *const GLint) {
    (self.glTexParameterIiv_p)(target, pname, params)
  }
  /// glTexParameterIuiv
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn TexParameterIuiv(&self, target: TextureTarget, pname: TextureParameterName, params: *const GLuint) {
    (self.glTexParameterIuiv_p)(target, pname, params)
  }
  /// glTexParameterf
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `param` group: CheckedFloat32
  pub unsafe fn TexParameterf(&self, target: TextureTarget, pname: TextureParameterName, param: GLfloat) {
    (self.glTexParameterf_p)(target, pname, param)
  }
  /// glTexParameterfv
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `params` group: CheckedFloat32
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn TexParameterfv(&self, target: TextureTarget, pname: TextureParameterName, params: *const GLfloat) {
    (self.glTexParameterfv_p)(target, pname, params)
  }
  /// glTexParameteri
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `param` group: CheckedInt32
  pub unsafe fn TexParameteri(&self, target: TextureTarget, pname: TextureParameterName, param: GLint) {
    (self.glTexParameteri_p)(target, pname, param)
  }
  /// glTexParameteriv
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `params` group: CheckedInt32
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn TexParameteriv(&self, target: TextureTarget, pname: TextureParameterName, params: *const GLint) {
    (self.glTexParameteriv_p)(target, pname, params)
  }
  /// glTexSubImage1D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width)
  pub unsafe fn TexSubImage1D(&self, target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTexSubImage1D_p)(target, level, xoffset, width, format, type_, pixels)
  }
  /// glTexSubImage2D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height)
  pub unsafe fn TexSubImage2D(&self, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTexSubImage2D_p)(target, level, xoffset, yoffset, width, height, format, type_, pixels)
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
  pub unsafe fn TexSubImage3D(&self, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTexSubImage3D_p)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels)
  }
  /// glTransformFeedbackVaryings
  /// * `program` class: program
  /// * `varyings` len: count
  /// * `bufferMode` group: TransformFeedbackBufferMode
  pub unsafe fn TransformFeedbackVaryings(&self, program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: TransformFeedbackBufferMode) {
    (self.glTransformFeedbackVaryings_p)(program, count, varyings, bufferMode)
  }
  /// glUniform1f
  pub unsafe fn Uniform1f(&self, location: GLint, v0: GLfloat) {
    (self.glUniform1f_p)(location, v0)
  }
  /// glUniform1fv
  /// * `value` len: count*1
  pub unsafe fn Uniform1fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
    (self.glUniform1fv_p)(location, count, value)
  }
  /// glUniform1i
  pub unsafe fn Uniform1i(&self, location: GLint, v0: GLint) {
    (self.glUniform1i_p)(location, v0)
  }
  /// glUniform1iv
  /// * `value` len: count*1
  pub unsafe fn Uniform1iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
    (self.glUniform1iv_p)(location, count, value)
  }
  /// glUniform1ui
  pub unsafe fn Uniform1ui(&self, location: GLint, v0: GLuint) {
    (self.glUniform1ui_p)(location, v0)
  }
  /// glUniform1uiv
  /// * `value` len: count*1
  pub unsafe fn Uniform1uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
    (self.glUniform1uiv_p)(location, count, value)
  }
  /// glUniform2f
  pub unsafe fn Uniform2f(&self, location: GLint, v0: GLfloat, v1: GLfloat) {
    (self.glUniform2f_p)(location, v0, v1)
  }
  /// glUniform2fv
  /// * `value` len: count*2
  pub unsafe fn Uniform2fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
    (self.glUniform2fv_p)(location, count, value)
  }
  /// glUniform2i
  pub unsafe fn Uniform2i(&self, location: GLint, v0: GLint, v1: GLint) {
    (self.glUniform2i_p)(location, v0, v1)
  }
  /// glUniform2iv
  /// * `value` len: count*2
  pub unsafe fn Uniform2iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
    (self.glUniform2iv_p)(location, count, value)
  }
  /// glUniform2ui
  pub unsafe fn Uniform2ui(&self, location: GLint, v0: GLuint, v1: GLuint) {
    (self.glUniform2ui_p)(location, v0, v1)
  }
  /// glUniform2uiv
  /// * `value` len: count*2
  pub unsafe fn Uniform2uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
    (self.glUniform2uiv_p)(location, count, value)
  }
  /// glUniform3f
  pub unsafe fn Uniform3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
    (self.glUniform3f_p)(location, v0, v1, v2)
  }
  /// glUniform3fv
  /// * `value` len: count*3
  pub unsafe fn Uniform3fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
    (self.glUniform3fv_p)(location, count, value)
  }
  /// glUniform3i
  pub unsafe fn Uniform3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    (self.glUniform3i_p)(location, v0, v1, v2)
  }
  /// glUniform3iv
  /// * `value` len: count*3
  pub unsafe fn Uniform3iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
    (self.glUniform3iv_p)(location, count, value)
  }
  /// glUniform3ui
  pub unsafe fn Uniform3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
    (self.glUniform3ui_p)(location, v0, v1, v2)
  }
  /// glUniform3uiv
  /// * `value` len: count*3
  pub unsafe fn Uniform3uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
    (self.glUniform3uiv_p)(location, count, value)
  }
  /// glUniform4f
  pub unsafe fn Uniform4f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
    (self.glUniform4f_p)(location, v0, v1, v2, v3)
  }
  /// glUniform4fv
  /// * `value` len: count*4
  pub unsafe fn Uniform4fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
    (self.glUniform4fv_p)(location, count, value)
  }
  /// glUniform4i
  pub unsafe fn Uniform4i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
    (self.glUniform4i_p)(location, v0, v1, v2, v3)
  }
  /// glUniform4iv
  /// * `value` len: count*4
  pub unsafe fn Uniform4iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
    (self.glUniform4iv_p)(location, count, value)
  }
  /// glUniform4ui
  pub unsafe fn Uniform4ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
    (self.glUniform4ui_p)(location, v0, v1, v2, v3)
  }
  /// glUniform4uiv
  /// * `value` len: count*4
  pub unsafe fn Uniform4uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
    (self.glUniform4uiv_p)(location, count, value)
  }
  /// glUniformBlockBinding
  /// * `program` class: program
  pub unsafe fn UniformBlockBinding(&self, program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) {
    (self.glUniformBlockBinding_p)(program, uniformBlockIndex, uniformBlockBinding)
  }
  /// glUniformMatrix2fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*4
  pub unsafe fn UniformMatrix2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix2fv_p)(location, count, transpose, value)
  }
  /// glUniformMatrix2x3fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  pub unsafe fn UniformMatrix2x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix2x3fv_p)(location, count, transpose, value)
  }
  /// glUniformMatrix2x4fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  pub unsafe fn UniformMatrix2x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix2x4fv_p)(location, count, transpose, value)
  }
  /// glUniformMatrix3fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*9
  pub unsafe fn UniformMatrix3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix3fv_p)(location, count, transpose, value)
  }
  /// glUniformMatrix3x2fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  pub unsafe fn UniformMatrix3x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix3x2fv_p)(location, count, transpose, value)
  }
  /// glUniformMatrix3x4fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  pub unsafe fn UniformMatrix3x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix3x4fv_p)(location, count, transpose, value)
  }
  /// glUniformMatrix4fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*16
  pub unsafe fn UniformMatrix4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix4fv_p)(location, count, transpose, value)
  }
  /// glUniformMatrix4x2fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  pub unsafe fn UniformMatrix4x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix4x2fv_p)(location, count, transpose, value)
  }
  /// glUniformMatrix4x3fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  pub unsafe fn UniformMatrix4x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix4x3fv_p)(location, count, transpose, value)
  }
  /// glUnmapBuffer
  /// * `target` group: BufferTargetARB
  pub unsafe fn UnmapBuffer(&self, target: BufferTargetARB) -> GLboolean {
    (self.glUnmapBuffer_p)(target)
  }
  /// [glUseProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgram.xhtml)
  ///
  /// Sets a given shader program for use during rendering.
  ///
  /// Setting 0 as the program object makes the output of all rendering actions
  /// undefined, but this is not an error.
  ///
  /// * `program` names the program to set for use.
  pub fn UseProgram(&self, program: GLuint) {
    (self.glUseProgram_p)(program)
  }
  /// glValidateProgram
  /// * `program` class: program
  pub unsafe fn ValidateProgram(&self, program: GLuint) {
    (self.glValidateProgram_p)(program)
  }
  /// glVertexAttrib1d
  pub unsafe fn VertexAttrib1d(&self, index: GLuint, x: GLdouble) {
    (self.glVertexAttrib1d_p)(index, x)
  }
  /// glVertexAttrib1dv
  pub unsafe fn VertexAttrib1dv(&self, index: GLuint, v: *const GLdouble) {
    (self.glVertexAttrib1dv_p)(index, v)
  }
  /// glVertexAttrib1f
  pub unsafe fn VertexAttrib1f(&self, index: GLuint, x: GLfloat) {
    (self.glVertexAttrib1f_p)(index, x)
  }
  /// glVertexAttrib1fv
  pub unsafe fn VertexAttrib1fv(&self, index: GLuint, v: *const GLfloat) {
    (self.glVertexAttrib1fv_p)(index, v)
  }
  /// glVertexAttrib1s
  pub unsafe fn VertexAttrib1s(&self, index: GLuint, x: GLshort) {
    (self.glVertexAttrib1s_p)(index, x)
  }
  /// glVertexAttrib1sv
  pub unsafe fn VertexAttrib1sv(&self, index: GLuint, v: *const GLshort) {
    (self.glVertexAttrib1sv_p)(index, v)
  }
  /// glVertexAttrib2d
  pub unsafe fn VertexAttrib2d(&self, index: GLuint, x: GLdouble, y: GLdouble) {
    (self.glVertexAttrib2d_p)(index, x, y)
  }
  /// glVertexAttrib2dv
  pub unsafe fn VertexAttrib2dv(&self, index: GLuint, v: *const [GLdouble; 2]) {
    (self.glVertexAttrib2dv_p)(index, v)
  }
  /// glVertexAttrib2f
  pub unsafe fn VertexAttrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
    (self.glVertexAttrib2f_p)(index, x, y)
  }
  /// glVertexAttrib2fv
  pub unsafe fn VertexAttrib2fv(&self, index: GLuint, v: *const [GLfloat; 2]) {
    (self.glVertexAttrib2fv_p)(index, v)
  }
  /// glVertexAttrib2s
  pub unsafe fn VertexAttrib2s(&self, index: GLuint, x: GLshort, y: GLshort) {
    (self.glVertexAttrib2s_p)(index, x, y)
  }
  /// glVertexAttrib2sv
  pub unsafe fn VertexAttrib2sv(&self, index: GLuint, v: *const [GLshort; 2]) {
    (self.glVertexAttrib2sv_p)(index, v)
  }
  /// glVertexAttrib3d
  pub unsafe fn VertexAttrib3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
    (self.glVertexAttrib3d_p)(index, x, y, z)
  }
  /// glVertexAttrib3dv
  pub unsafe fn VertexAttrib3dv(&self, index: GLuint, v: *const [GLdouble; 3]) {
    (self.glVertexAttrib3dv_p)(index, v)
  }
  /// glVertexAttrib3f
  pub unsafe fn VertexAttrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
    (self.glVertexAttrib3f_p)(index, x, y, z)
  }
  /// glVertexAttrib3fv
  pub unsafe fn VertexAttrib3fv(&self, index: GLuint, v: *const [GLfloat; 3]) {
    (self.glVertexAttrib3fv_p)(index, v)
  }
  /// glVertexAttrib3s
  pub unsafe fn VertexAttrib3s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
    (self.glVertexAttrib3s_p)(index, x, y, z)
  }
  /// glVertexAttrib3sv
  pub unsafe fn VertexAttrib3sv(&self, index: GLuint, v: *const [GLshort; 3]) {
    (self.glVertexAttrib3sv_p)(index, v)
  }
  /// glVertexAttrib4Nbv
  pub unsafe fn VertexAttrib4Nbv(&self, index: GLuint, v: *const [GLbyte; 4]) {
    (self.glVertexAttrib4Nbv_p)(index, v)
  }
  /// glVertexAttrib4Niv
  pub unsafe fn VertexAttrib4Niv(&self, index: GLuint, v: *const [GLint; 4]) {
    (self.glVertexAttrib4Niv_p)(index, v)
  }
  /// glVertexAttrib4Nsv
  pub unsafe fn VertexAttrib4Nsv(&self, index: GLuint, v: *const [GLshort; 4]) {
    (self.glVertexAttrib4Nsv_p)(index, v)
  }
  /// glVertexAttrib4Nub
  pub unsafe fn VertexAttrib4Nub(&self, index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) {
    (self.glVertexAttrib4Nub_p)(index, x, y, z, w)
  }
  /// glVertexAttrib4Nubv
  pub unsafe fn VertexAttrib4Nubv(&self, index: GLuint, v: *const [GLubyte; 4]) {
    (self.glVertexAttrib4Nubv_p)(index, v)
  }
  /// glVertexAttrib4Nuiv
  pub unsafe fn VertexAttrib4Nuiv(&self, index: GLuint, v: *const [GLuint; 4]) {
    (self.glVertexAttrib4Nuiv_p)(index, v)
  }
  /// glVertexAttrib4Nusv
  pub unsafe fn VertexAttrib4Nusv(&self, index: GLuint, v: *const [GLushort; 4]) {
    (self.glVertexAttrib4Nusv_p)(index, v)
  }
  /// glVertexAttrib4bv
  pub unsafe fn VertexAttrib4bv(&self, index: GLuint, v: *const [GLbyte; 4]) {
    (self.glVertexAttrib4bv_p)(index, v)
  }
  /// glVertexAttrib4d
  pub unsafe fn VertexAttrib4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
    (self.glVertexAttrib4d_p)(index, x, y, z, w)
  }
  /// glVertexAttrib4dv
  pub unsafe fn VertexAttrib4dv(&self, index: GLuint, v: *const [GLdouble; 4]) {
    (self.glVertexAttrib4dv_p)(index, v)
  }
  /// glVertexAttrib4f
  pub unsafe fn VertexAttrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    (self.glVertexAttrib4f_p)(index, x, y, z, w)
  }
  /// glVertexAttrib4fv
  pub unsafe fn VertexAttrib4fv(&self, index: GLuint, v: *const [GLfloat; 4]) {
    (self.glVertexAttrib4fv_p)(index, v)
  }
  /// glVertexAttrib4iv
  pub unsafe fn VertexAttrib4iv(&self, index: GLuint, v: *const [GLint; 4]) {
    (self.glVertexAttrib4iv_p)(index, v)
  }
  /// glVertexAttrib4s
  pub unsafe fn VertexAttrib4s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) {
    (self.glVertexAttrib4s_p)(index, x, y, z, w)
  }
  /// glVertexAttrib4sv
  pub unsafe fn VertexAttrib4sv(&self, index: GLuint, v: *const [GLshort; 4]) {
    (self.glVertexAttrib4sv_p)(index, v)
  }
  /// glVertexAttrib4ubv
  pub unsafe fn VertexAttrib4ubv(&self, index: GLuint, v: *const [GLubyte; 4]) {
    (self.glVertexAttrib4ubv_p)(index, v)
  }
  /// glVertexAttrib4uiv
  pub unsafe fn VertexAttrib4uiv(&self, index: GLuint, v: *const [GLuint; 4]) {
    (self.glVertexAttrib4uiv_p)(index, v)
  }
  /// glVertexAttrib4usv
  pub unsafe fn VertexAttrib4usv(&self, index: GLuint, v: *const [GLushort; 4]) {
    (self.glVertexAttrib4usv_p)(index, v)
  }
  /// glVertexAttribDivisor
  pub unsafe fn VertexAttribDivisor(&self, index: GLuint, divisor: GLuint) {
    (self.glVertexAttribDivisor_p)(index, divisor)
  }
  /// glVertexAttribI1i
  pub unsafe fn VertexAttribI1i(&self, index: GLuint, x: GLint) {
    (self.glVertexAttribI1i_p)(index, x)
  }
  /// glVertexAttribI1iv
  pub unsafe fn VertexAttribI1iv(&self, index: GLuint, v: *const GLint) {
    (self.glVertexAttribI1iv_p)(index, v)
  }
  /// glVertexAttribI1ui
  pub unsafe fn VertexAttribI1ui(&self, index: GLuint, x: GLuint) {
    (self.glVertexAttribI1ui_p)(index, x)
  }
  /// glVertexAttribI1uiv
  pub unsafe fn VertexAttribI1uiv(&self, index: GLuint, v: *const GLuint) {
    (self.glVertexAttribI1uiv_p)(index, v)
  }
  /// glVertexAttribI2i
  pub unsafe fn VertexAttribI2i(&self, index: GLuint, x: GLint, y: GLint) {
    (self.glVertexAttribI2i_p)(index, x, y)
  }
  /// glVertexAttribI2iv
  pub unsafe fn VertexAttribI2iv(&self, index: GLuint, v: *const [GLint; 2]) {
    (self.glVertexAttribI2iv_p)(index, v)
  }
  /// glVertexAttribI2ui
  pub unsafe fn VertexAttribI2ui(&self, index: GLuint, x: GLuint, y: GLuint) {
    (self.glVertexAttribI2ui_p)(index, x, y)
  }
  /// glVertexAttribI2uiv
  pub unsafe fn VertexAttribI2uiv(&self, index: GLuint, v: *const [GLuint; 2]) {
    (self.glVertexAttribI2uiv_p)(index, v)
  }
  /// glVertexAttribI3i
  pub unsafe fn VertexAttribI3i(&self, index: GLuint, x: GLint, y: GLint, z: GLint) {
    (self.glVertexAttribI3i_p)(index, x, y, z)
  }
  /// glVertexAttribI3iv
  pub unsafe fn VertexAttribI3iv(&self, index: GLuint, v: *const [GLint; 3]) {
    (self.glVertexAttribI3iv_p)(index, v)
  }
  /// glVertexAttribI3ui
  pub unsafe fn VertexAttribI3ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
    (self.glVertexAttribI3ui_p)(index, x, y, z)
  }
  /// glVertexAttribI3uiv
  pub unsafe fn VertexAttribI3uiv(&self, index: GLuint, v: *const [GLuint; 3]) {
    (self.glVertexAttribI3uiv_p)(index, v)
  }
  /// glVertexAttribI4bv
  pub unsafe fn VertexAttribI4bv(&self, index: GLuint, v: *const [GLbyte; 4]) {
    (self.glVertexAttribI4bv_p)(index, v)
  }
  /// glVertexAttribI4i
  pub unsafe fn VertexAttribI4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
    (self.glVertexAttribI4i_p)(index, x, y, z, w)
  }
  /// glVertexAttribI4iv
  pub unsafe fn VertexAttribI4iv(&self, index: GLuint, v: *const [GLint; 4]) {
    (self.glVertexAttribI4iv_p)(index, v)
  }
  /// glVertexAttribI4sv
  pub unsafe fn VertexAttribI4sv(&self, index: GLuint, v: *const [GLshort; 4]) {
    (self.glVertexAttribI4sv_p)(index, v)
  }
  /// glVertexAttribI4ubv
  pub unsafe fn VertexAttribI4ubv(&self, index: GLuint, v: *const [GLubyte; 4]) {
    (self.glVertexAttribI4ubv_p)(index, v)
  }
  /// glVertexAttribI4ui
  pub unsafe fn VertexAttribI4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
    (self.glVertexAttribI4ui_p)(index, x, y, z, w)
  }
  /// glVertexAttribI4uiv
  pub unsafe fn VertexAttribI4uiv(&self, index: GLuint, v: *const [GLuint; 4]) {
    (self.glVertexAttribI4uiv_p)(index, v)
  }
  /// glVertexAttribI4usv
  pub unsafe fn VertexAttribI4usv(&self, index: GLuint, v: *const [GLushort; 4]) {
    (self.glVertexAttribI4usv_p)(index, v)
  }
  /// glVertexAttribIPointer
  /// * `type` group: VertexAttribIType
  /// * `pointer` len: COMPSIZE(size,type,stride)
  pub unsafe fn VertexAttribIPointer(&self, index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei, pointer: *const void) {
    (self.glVertexAttribIPointer_p)(index, size, type_, stride, pointer)
  }
  /// glVertexAttribP1ui
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP1ui(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
    (self.glVertexAttribP1ui_p)(index, type_, normalized, value)
  }
  /// glVertexAttribP1uiv
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP1uiv(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
    (self.glVertexAttribP1uiv_p)(index, type_, normalized, value)
  }
  /// glVertexAttribP2ui
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP2ui(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
    (self.glVertexAttribP2ui_p)(index, type_, normalized, value)
  }
  /// glVertexAttribP2uiv
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP2uiv(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
    (self.glVertexAttribP2uiv_p)(index, type_, normalized, value)
  }
  /// glVertexAttribP3ui
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP3ui(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
    (self.glVertexAttribP3ui_p)(index, type_, normalized, value)
  }
  /// glVertexAttribP3uiv
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP3uiv(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
    (self.glVertexAttribP3uiv_p)(index, type_, normalized, value)
  }
  /// glVertexAttribP4ui
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP4ui(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
    (self.glVertexAttribP4ui_p)(index, type_, normalized, value)
  }
  /// glVertexAttribP4uiv
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP4uiv(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
    (self.glVertexAttribP4uiv_p)(index, type_, normalized, value)
  }
  /// glVertexAttribPointer
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  /// * `pointer` len: COMPSIZE(size,type,stride)
  pub unsafe fn VertexAttribPointer(&self, index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, pointer: *const void) {
    (self.glVertexAttribPointer_p)(index, size, type_, normalized, stride, pointer)
  }
  /// glViewport
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  pub unsafe fn Viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    (self.glViewport_p)(x, y, width, height)
  }
  /// glWaitSync
  /// * `sync` group: sync
  /// * `sync` class: sync
  /// * `flags` group: SyncBehaviorFlags
  pub unsafe fn WaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
    (self.glWaitSync_p)(sync, flags, timeout)
  }
  /// glDebugMessageCallback
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn DebugMessageCallback(&self, callback: GLDEBUGPROC, userParam: *const void) {
    match self.glDebugMessageCallback_p {
      Some(f) => f(callback, userParam),
      None => Self::not_loaded("glDebugMessageCallback"),
    }
  }
  #[doc(hidden)]
  pub fn DebugMessageCallback_is_loaded(&self) -> bool {
    self.glDebugMessageCallback_p.is_some()
  }
  /// glDebugMessageControl
  /// * `source` group: DebugSource
  /// * `type` group: DebugType
  /// * `severity` group: DebugSeverity
  /// * `ids` len: count
  /// * `enabled` group: Boolean
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn DebugMessageControl(&self, source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
    match self.glDebugMessageControl_p {
      Some(f) => f(source, type_, severity, count, ids, enabled),
      None => Self::not_loaded("glDebugMessageControl"),
    }
  }
  #[doc(hidden)]
  pub fn DebugMessageControl_is_loaded(&self) -> bool {
    self.glDebugMessageControl_p.is_some()
  }
  /// glDebugMessageInsert
  /// * `source` group: DebugSource
  /// * `type` group: DebugType
  /// * `severity` group: DebugSeverity
  /// * `buf` len: COMPSIZE(buf,length)
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn DebugMessageInsert(&self, source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar) {
    match self.glDebugMessageInsert_p {
      Some(f) => f(source, type_, id, severity, length, buf),
      None => Self::not_loaded("glDebugMessageInsert"),
    }
  }
  #[doc(hidden)]
  pub fn DebugMessageInsert_is_loaded(&self) -> bool {
    self.glDebugMessageInsert_p.is_some()
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
  pub unsafe fn GetDebugMessageLog(&self, count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
    match self.glGetDebugMessageLog_p {
      Some(f) => f(count, bufSize, sources, types, ids, severities, lengths, messageLog),
      None => Self::not_loaded("glGetDebugMessageLog"),
    }
  }
  #[doc(hidden)]
  pub fn GetDebugMessageLog_is_loaded(&self) -> bool {
    self.glGetDebugMessageLog_p.is_some()
  }
  /// glGetObjectLabel
  /// * `identifier` group: ObjectIdentifier
  /// * `label` len: bufSize
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn GetObjectLabel(&self, identifier: ObjectIdentifier, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
    match self.glGetObjectLabel_p {
      Some(f) => f(identifier, name, bufSize, length, label),
      None => Self::not_loaded("glGetObjectLabel"),
    }
  }
  #[doc(hidden)]
  pub fn GetObjectLabel_is_loaded(&self) -> bool {
    self.glGetObjectLabel_p.is_some()
  }
  /// glGetObjectPtrLabel
  /// * `label` len: bufSize
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn GetObjectPtrLabel(&self, ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
    match self.glGetObjectPtrLabel_p {
      Some(f) => f(ptr, bufSize, length, label),
      None => Self::not_loaded("glGetObjectPtrLabel"),
    }
  }
  #[doc(hidden)]
  pub fn GetObjectPtrLabel_is_loaded(&self) -> bool {
    self.glGetObjectPtrLabel_p.is_some()
  }
  /// glGetPointerv
  /// * `pname` group: GetPointervPName
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn GetPointerv(&self, pname: GetPointervPName, params: *mut *mut void) {
    match self.glGetPointerv_p {
      Some(f) => f(pname, params),
      None => Self::not_loaded("glGetPointerv"),
    }
  }
  #[doc(hidden)]
  pub fn GetPointerv_is_loaded(&self) -> bool {
    self.glGetPointerv_p.is_some()
  }
  /// glObjectLabel
  /// * `identifier` group: ObjectIdentifier
  /// * `label` len: COMPSIZE(label,length)
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn ObjectLabel(&self, identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar) {
    match self.glObjectLabel_p {
      Some(f) => f(identifier, name, length, label),
      None => Self::not_loaded("glObjectLabel"),
    }
  }
  #[doc(hidden)]
  pub fn ObjectLabel_is_loaded(&self) -> bool {
    self.glObjectLabel_p.is_some()
  }
  /// glObjectPtrLabel
  /// * `label` len: COMPSIZE(label,length)
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn ObjectPtrLabel(&self, ptr: *const void, length: GLsizei, label: *const GLchar) {
    match self.glObjectPtrLabel_p {
      Some(f) => f(ptr, length, label),
      None => Self::not_loaded("glObjectPtrLabel"),
    }
  }
  #[doc(hidden)]
  pub fn ObjectPtrLabel_is_loaded(&self) -> bool {
    self.glObjectPtrLabel_p.is_some()
  }
  /// glPopDebugGroup
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn PopDebugGroup(&self) {
    match self.glPopDebugGroup_p {
      Some(f) => f(),
      None => Self::not_loaded("glPopDebugGroup"),
    }
  }
  #[doc(hidden)]
  pub fn PopDebugGroup_is_loaded(&self) -> bool {
    self.glPopDebugGroup_p.is_some()
  }
  /// glPushDebugGroup
  /// * `source` group: DebugSource
  /// * `message` len: COMPSIZE(message,length)
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn PushDebugGroup(&self, source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar) {
    match self.glPushDebugGroup_p {
      Some(f) => f(source, id, length, message),
      None => Self::not_loaded("glPushDebugGroup"),
    }
  }
  #[doc(hidden)]
  pub fn PushDebugGroup_is_loaded(&self) -> bool {
    self.glPushDebugGroup_p.is_some()
  }
}
