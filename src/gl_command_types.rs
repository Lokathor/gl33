use super::*;

pub(crate) type glActiveTexture_t = unsafe extern "system" fn(texture: TextureUnit);

pub(crate) type glAttachShader_t = extern "system" fn(program: GLuint, shader: GLuint);

pub(crate) type glBeginConditionalRender_t = unsafe extern "system" fn(id: GLuint, mode: ConditionalRenderMode);

pub(crate) type glBeginQuery_t = unsafe extern "system" fn(target: QueryTarget, id: GLuint);

pub(crate) type glBeginTransformFeedback_t = unsafe extern "system" fn(primitiveMode: PrimitiveType);

pub(crate) type glBindAttribLocation_t = unsafe extern "system" fn(program: GLuint, index: GLuint, name: *const GLchar);

pub(crate) type glBindBuffer_t = unsafe extern "system" fn(target: BufferTargetARB, buffer: GLuint);

pub(crate) type glBindBufferBase_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint);

pub(crate) type glBindBufferRange_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub(crate) type glBindFragDataLocation_t = unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar);

pub(crate) type glBindFragDataLocationIndexed_t = unsafe extern "system" fn(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);

pub(crate) type glBindFramebuffer_t = unsafe extern "system" fn(target: FramebufferTarget, framebuffer: GLuint);

pub(crate) type glBindRenderbuffer_t = unsafe extern "system" fn(target: RenderbufferTarget, renderbuffer: GLuint);

pub(crate) type glBindSampler_t = unsafe extern "system" fn(unit: GLuint, sampler: GLuint);

pub(crate) type glBindTexture_t = unsafe extern "system" fn(target: TextureTarget, texture: GLuint);

pub(crate) type glBindVertexArray_t = extern "system" fn(array: GLuint);

pub(crate) type glBlendColor_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

pub(crate) type glBlendEquation_t = unsafe extern "system" fn(mode: BlendEquationModeEXT);

pub(crate) type glBlendEquationSeparate_t = unsafe extern "system" fn(modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

pub(crate) type glBlendFunc_t = unsafe extern "system" fn(sfactor: BlendingFactor, dfactor: BlendingFactor);

pub(crate) type glBlendFuncSeparate_t = unsafe extern "system" fn(sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor);

pub(crate) type glBlitFramebuffer_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

pub(crate) type glBufferData_t = unsafe extern "system" fn(target: BufferTargetARB, size: GLsizeiptr, data: *const void, usage: BufferUsageARB);

pub(crate) type glBufferSubData_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *const void);

pub(crate) type glCheckFramebufferStatus_t = unsafe extern "system" fn(target: FramebufferTarget) -> FramebufferStatus;

pub(crate) type glClampColor_t = unsafe extern "system" fn(target: ClampColorTargetARB, clamp: ClampColorModeARB);

pub(crate) type glClear_t = unsafe extern "system" fn(mask: GLbitfield);

pub(crate) type glClearBufferfi_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint);

pub(crate) type glClearBufferfv_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, value: *const GLfloat);

pub(crate) type glClearBufferiv_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, value: *const GLint);

pub(crate) type glClearBufferuiv_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, value: *const GLuint);

pub(crate) type glClearColor_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

pub(crate) type glClearDepth_t = unsafe extern "system" fn(depth: GLdouble);

pub(crate) type glClearStencil_t = unsafe extern "system" fn(s: GLint);

pub(crate) type glClientWaitSync_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> SyncStatus;

pub(crate) type glColorMask_t = unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);

pub(crate) type glColorMaski_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

pub(crate) type glCompileShader_t = extern "system" fn(shader: GLuint);

pub(crate) type glCompressedTexImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTexImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTexImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTexSubImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTexSubImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTexSubImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

pub(crate) type glCopyBufferSubData_t = unsafe extern "system" fn(readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

pub(crate) type glCopyTexImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint);

pub(crate) type glCopyTexImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

pub(crate) type glCopyTexSubImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

pub(crate) type glCopyTexSubImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glCopyTexSubImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glCreateProgram_t = extern "system" fn() -> GLuint;

pub(crate) type glCreateShader_t = extern "system" fn(type_: ShaderType) -> GLuint;

pub(crate) type glCullFace_t = unsafe extern "system" fn(mode: CullFaceMode);

pub(crate) type glDebugMessageCallback_t = unsafe extern "system" fn(callback: GLDEBUGPROC, userParam: *const void);

pub(crate) type glDebugMessageControl_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

pub(crate) type glDebugMessageInsert_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar);

pub(crate) type glDeleteBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);

pub(crate) type glDeleteFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);

pub(crate) type glDeleteProgram_t = extern "system" fn(program: GLuint);

pub(crate) type glDeleteQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

pub(crate) type glDeleteRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);

pub(crate) type glDeleteSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *const GLuint);

pub(crate) type glDeleteShader_t = extern "system" fn(shader: GLuint);

pub(crate) type glDeleteSync_t = unsafe extern "system" fn(sync: GLsync);

pub(crate) type glDeleteTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);

pub(crate) type glDeleteVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);

pub(crate) type glDepthFunc_t = unsafe extern "system" fn(func: DepthFunction);

pub(crate) type glDepthMask_t = unsafe extern "system" fn(flag: GLboolean);

pub(crate) type glDepthRange_t = unsafe extern "system" fn(n: GLdouble, f: GLdouble);

pub(crate) type glDetachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);

pub(crate) type glDisable_t = unsafe extern "system" fn(cap: EnableCap);

pub(crate) type glDisableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);

pub(crate) type glDisablei_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

pub(crate) type glDrawArrays_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei);

pub(crate) type glDrawArraysInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei);

pub(crate) type glDrawBuffer_t = unsafe extern "system" fn(buf: DrawBufferMode);

pub(crate) type glDrawBuffers_t = unsafe extern "system" fn(n: GLsizei, bufs: *const DrawBufferMode);

pub(crate) type glDrawElements_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void);

pub(crate) type glDrawElementsBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

pub(crate) type glDrawElementsInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei);

pub(crate) type glDrawElementsInstancedBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint);

pub(crate) type glDrawRangeElements_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void);

pub(crate) type glDrawRangeElementsBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

pub(crate) type glEnable_t = unsafe extern "system" fn(cap: EnableCap);

pub(crate) type glEnableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);

pub(crate) type glEnablei_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

pub(crate) type glEndConditionalRender_t = unsafe extern "system" fn();

pub(crate) type glEndQuery_t = unsafe extern "system" fn(target: QueryTarget);

pub(crate) type glEndTransformFeedback_t = unsafe extern "system" fn();

pub(crate) type glFenceSync_t = unsafe extern "system" fn(condition: SyncCondition, flags: GLbitfield) -> GLsync;

pub(crate) type glFinish_t = unsafe extern "system" fn();

pub(crate) type glFlush_t = unsafe extern "system" fn();

pub(crate) type glFlushMappedBufferRange_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr);

pub(crate) type glFramebufferRenderbuffer_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

pub(crate) type glFramebufferTexture_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

pub(crate) type glFramebufferTexture1D_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

pub(crate) type glFramebufferTexture2D_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

pub(crate) type glFramebufferTexture3D_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint);

pub(crate) type glFramebufferTextureLayer_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

pub(crate) type glFrontFace_t = unsafe extern "system" fn(mode: FrontFaceDirection);

pub(crate) type glGenBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);

pub(crate) type glGenFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

pub(crate) type glGenQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

pub(crate) type glGenRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

pub(crate) type glGenSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *mut GLuint);

pub(crate) type glGenTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);

pub(crate) type glGenVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

pub(crate) type glGenerateMipmap_t = unsafe extern "system" fn(target: TextureTarget);

pub(crate) type glGetActiveAttrib_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut AttributeType, name: *mut GLchar);

pub(crate) type glGetActiveUniform_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut UniformType, name: *mut GLchar);

pub(crate) type glGetActiveUniformBlockName_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar);

pub(crate) type glGetActiveUniformBlockiv_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, pname: UniformBlockPName, params: *mut GLint);

pub(crate) type glGetActiveUniformName_t = unsafe extern "system" fn(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar);

pub(crate) type glGetActiveUniformsiv_t = unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: UniformPName, params: *mut GLint);

pub(crate) type glGetAttachedShaders_t = unsafe extern "system" fn(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);

pub(crate) type glGetAttribLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub(crate) type glGetBooleani_v_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, data: *mut GLboolean);

pub(crate) type glGetBooleanv_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLboolean);

pub(crate) type glGetBufferParameteri64v_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint64);

pub(crate) type glGetBufferParameteriv_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint);

pub(crate) type glGetBufferPointerv_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void);

pub(crate) type glGetBufferSubData_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *mut void);

pub(crate) type glGetCompressedTexImage_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, img: *mut void);

pub(crate) type glGetDebugMessageLog_t = unsafe extern "system" fn(count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint;

pub(crate) type glGetDoublev_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLdouble);

pub(crate) type glGetError_t = unsafe extern "system" fn() -> ErrorCode;

pub(crate) type glGetFloatv_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLfloat);

pub(crate) type glGetFragDataIndex_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub(crate) type glGetFragDataLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub(crate) type glGetFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

pub(crate) type glGetInteger64i_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLint64);

pub(crate) type glGetInteger64v_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLint64);

pub(crate) type glGetIntegeri_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLint);

pub(crate) type glGetIntegerv_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLint);

pub(crate) type glGetMultisamplefv_t = unsafe extern "system" fn(pname: GetMultisamplePNameNV, index: GLuint, val: *mut GLfloat);

pub(crate) type glGetObjectLabel_t = unsafe extern "system" fn(identifier: ObjectIdentifier, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

pub(crate) type glGetObjectPtrLabel_t = unsafe extern "system" fn(ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

pub(crate) type glGetPointerv_t = unsafe extern "system" fn(pname: GetPointervPName, params: *mut *mut void);

pub(crate) type glGetProgramInfoLog_t = unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

pub(crate) type glGetProgramiv_t = unsafe extern "system" fn(program: GLuint, pname: ProgramPropertyARB, params: *mut GLint);

pub(crate) type glGetQueryObjecti64v_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint64);

pub(crate) type glGetQueryObjectiv_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint);

pub(crate) type glGetQueryObjectui64v_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint64);

pub(crate) type glGetQueryObjectuiv_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint);

pub(crate) type glGetQueryiv_t = unsafe extern "system" fn(target: QueryTarget, pname: QueryParameterName, params: *mut GLint);

pub(crate) type glGetRenderbufferParameteriv_t = unsafe extern "system" fn(target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint);

pub(crate) type glGetSamplerParameterIiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);

pub(crate) type glGetSamplerParameterIuiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint);

pub(crate) type glGetSamplerParameterfv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterF, params: *mut GLfloat);

pub(crate) type glGetSamplerParameteriv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);

pub(crate) type glGetShaderInfoLog_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

pub(crate) type glGetShaderSource_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);

pub(crate) type glGetShaderiv_t = unsafe extern "system" fn(shader: GLuint, pname: ShaderParameterName, params: *mut GLint);

pub(crate) type glGetString_t = unsafe extern "system" fn(name: StringName) -> GLubyte;

pub(crate) type glGetStringi_t = unsafe extern "system" fn(name: StringName, index: GLuint) -> GLubyte;

pub(crate) type glGetSynciv_t = unsafe extern "system" fn(sync: GLsync, pname: SyncParameterName, count: GLsizei, length: *mut GLsizei, values: *mut GLint);

pub(crate) type glGetTexImage_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void);

pub(crate) type glGetTexLevelParameterfv_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);

pub(crate) type glGetTexLevelParameteriv_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint);

pub(crate) type glGetTexParameterIiv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

pub(crate) type glGetTexParameterIuiv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);

pub(crate) type glGetTexParameterfv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat);

pub(crate) type glGetTexParameteriv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

pub(crate) type glGetTransformFeedbackVarying_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut AttributeType, name: *mut GLchar);

pub(crate) type glGetUniformBlockIndex_t = unsafe extern "system" fn(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;

pub(crate) type glGetUniformIndices_t = unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint);

pub(crate) type glGetUniformLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub(crate) type glGetUniformfv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLfloat);

pub(crate) type glGetUniformiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint);

pub(crate) type glGetUniformuiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint);

pub(crate) type glGetVertexAttribIiv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLint);

pub(crate) type glGetVertexAttribIuiv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint);

pub(crate) type glGetVertexAttribPointerv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut void);

pub(crate) type glGetVertexAttribdv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLdouble; 4]);

pub(crate) type glGetVertexAttribfv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLfloat; 4]);

pub(crate) type glGetVertexAttribiv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLint; 4]);

pub(crate) type glHint_t = unsafe extern "system" fn(target: HintTarget, mode: HintMode);

pub(crate) type glIsBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

pub(crate) type glIsEnabled_t = unsafe extern "system" fn(cap: EnableCap) -> GLboolean;

pub(crate) type glIsEnabledi_t = unsafe extern "system" fn(target: EnableCap, index: GLuint) -> GLboolean;

pub(crate) type glIsFramebuffer_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;

pub(crate) type glIsProgram_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;

pub(crate) type glIsQuery_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

pub(crate) type glIsRenderbuffer_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;

pub(crate) type glIsSampler_t = unsafe extern "system" fn(sampler: GLuint) -> GLboolean;

pub(crate) type glIsShader_t = unsafe extern "system" fn(shader: GLuint) -> GLboolean;

pub(crate) type glIsSync_t = unsafe extern "system" fn(sync: GLsync) -> GLboolean;

pub(crate) type glIsTexture_t = unsafe extern "system" fn(texture: GLuint) -> GLboolean;

pub(crate) type glIsVertexArray_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;

pub(crate) type glLineWidth_t = unsafe extern "system" fn(width: GLfloat);

pub(crate) type glLinkProgram_t = extern "system" fn(program: GLuint);

pub(crate) type glLogicOp_t = unsafe extern "system" fn(opcode: LogicOp);

pub(crate) type glMapBuffer_t = unsafe extern "system" fn(target: BufferTargetARB, access: BufferAccessARB) -> *mut void;

pub(crate) type glMapBufferRange_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

pub(crate) type glMultiDrawArrays_t = unsafe extern "system" fn(mode: PrimitiveType, first: *const GLint, count: *const GLsizei, drawcount: GLsizei);

pub(crate) type glMultiDrawElements_t = unsafe extern "system" fn(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei);

pub(crate) type glMultiDrawElementsBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint);

pub(crate) type glObjectLabel_t = unsafe extern "system" fn(identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar);

pub(crate) type glObjectPtrLabel_t = unsafe extern "system" fn(ptr: *const void, length: GLsizei, label: *const GLchar);

pub(crate) type glPixelStoref_t = unsafe extern "system" fn(pname: PixelStoreParameter, param: GLfloat);

pub(crate) type glPixelStorei_t = unsafe extern "system" fn(pname: PixelStoreParameter, param: GLint);

pub(crate) type glPointParameterf_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfloat);

pub(crate) type glPointParameterfv_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfloat);

pub(crate) type glPointParameteri_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLint);

pub(crate) type glPointParameteriv_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLint);

pub(crate) type glPointSize_t = extern "system" fn(size: GLfloat);

pub(crate) type glPolygonMode_t = unsafe extern "system" fn(face: MaterialFace, mode: PolygonMode);

pub(crate) type glPolygonOffset_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat);

pub(crate) type glPopDebugGroup_t = unsafe extern "system" fn();

pub(crate) type glPrimitiveRestartIndex_t = unsafe extern "system" fn(index: GLuint);

pub(crate) type glProvokingVertex_t = unsafe extern "system" fn(mode: VertexProvokingMode);

pub(crate) type glPushDebugGroup_t = unsafe extern "system" fn(source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar);

pub(crate) type glQueryCounter_t = unsafe extern "system" fn(id: GLuint, target: QueryCounterTarget);

pub(crate) type glReadBuffer_t = unsafe extern "system" fn(src: ReadBufferMode);

pub(crate) type glReadPixels_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *mut void);

pub(crate) type glRenderbufferStorage_t = unsafe extern "system" fn(target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

pub(crate) type glRenderbufferStorageMultisample_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

pub(crate) type glSampleCoverage_t = unsafe extern "system" fn(value: GLfloat, invert: GLboolean);

pub(crate) type glSampleMaski_t = unsafe extern "system" fn(maskNumber: GLuint, mask: GLbitfield);

pub(crate) type glSamplerParameterIiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);

pub(crate) type glSamplerParameterIuiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLuint);

pub(crate) type glSamplerParameterf_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterF, param: GLfloat);

pub(crate) type glSamplerParameterfv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterF, param: *const GLfloat);

pub(crate) type glSamplerParameteri_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: GLint);

pub(crate) type glSamplerParameteriv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);

pub(crate) type glScissor_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glShaderSource_t = unsafe extern "system" fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);

pub(crate) type glStencilFunc_t = unsafe extern "system" fn(func: StencilFunction, ref_: GLint, mask: GLuint);

pub(crate) type glStencilFuncSeparate_t = unsafe extern "system" fn(face: StencilFaceDirection, func: StencilFunction, ref_: GLint, mask: GLuint);

pub(crate) type glStencilMask_t = unsafe extern "system" fn(mask: GLuint);

pub(crate) type glStencilMaskSeparate_t = unsafe extern "system" fn(face: StencilFaceDirection, mask: GLuint);

pub(crate) type glStencilOp_t = unsafe extern "system" fn(fail: StencilOp, zfail: StencilOp, zpass: StencilOp);

pub(crate) type glStencilOpSeparate_t = unsafe extern "system" fn(face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp);

pub(crate) type glTexBuffer_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);

pub(crate) type glTexImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTexImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTexImage2DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

pub(crate) type glTexImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTexImage3DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

pub(crate) type glTexParameterIiv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLint);

pub(crate) type glTexParameterIuiv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLuint);

pub(crate) type glTexParameterf_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, param: GLfloat);

pub(crate) type glTexParameterfv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLfloat);

pub(crate) type glTexParameteri_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, param: GLint);

pub(crate) type glTexParameteriv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLint);

pub(crate) type glTexSubImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTexSubImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTexSubImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTransformFeedbackVaryings_t = unsafe extern "system" fn(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: TransformFeedbackBufferMode);

pub(crate) type glUniform1f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat);

pub(crate) type glUniform1fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub(crate) type glUniform1i_t = unsafe extern "system" fn(location: GLint, v0: GLint);

pub(crate) type glUniform1iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub(crate) type glUniform1ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint);

pub(crate) type glUniform1uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub(crate) type glUniform2f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat);

pub(crate) type glUniform2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub(crate) type glUniform2i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint);

pub(crate) type glUniform2iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub(crate) type glUniform2ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint);

pub(crate) type glUniform2uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub(crate) type glUniform3f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

pub(crate) type glUniform3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub(crate) type glUniform3i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);

pub(crate) type glUniform3iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub(crate) type glUniform3ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

pub(crate) type glUniform3uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub(crate) type glUniform4f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

pub(crate) type glUniform4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub(crate) type glUniform4i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

pub(crate) type glUniform4iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub(crate) type glUniform4ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

pub(crate) type glUniform4uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub(crate) type glUniformBlockBinding_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint);

pub(crate) type glUniformMatrix2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix2x3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix2x4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix3x2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix3x4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix4x2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix4x3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUnmapBuffer_t = unsafe extern "system" fn(target: BufferTargetARB) -> GLboolean;

pub(crate) type glUseProgram_t = extern "system" fn(program: GLuint);

pub(crate) type glValidateProgram_t = unsafe extern "system" fn(program: GLuint);

pub(crate) type glVertexAttrib1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

pub(crate) type glVertexAttrib1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

pub(crate) type glVertexAttrib1f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);

pub(crate) type glVertexAttrib1fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);

pub(crate) type glVertexAttrib1s_t = unsafe extern "system" fn(index: GLuint, x: GLshort);

pub(crate) type glVertexAttrib1sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);

pub(crate) type glVertexAttrib2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

pub(crate) type glVertexAttrib2dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

pub(crate) type glVertexAttrib2f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);

pub(crate) type glVertexAttrib2fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 2]);

pub(crate) type glVertexAttrib2s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);

pub(crate) type glVertexAttrib2sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 2]);

pub(crate) type glVertexAttrib3d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

pub(crate) type glVertexAttrib3dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

pub(crate) type glVertexAttrib3f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

pub(crate) type glVertexAttrib3fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 3]);

pub(crate) type glVertexAttrib3s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);

pub(crate) type glVertexAttrib3sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 3]);

pub(crate) type glVertexAttrib4Nbv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

pub(crate) type glVertexAttrib4Niv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub(crate) type glVertexAttrib4Nsv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub(crate) type glVertexAttrib4Nub_t = unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);

pub(crate) type glVertexAttrib4Nubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub(crate) type glVertexAttrib4Nuiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

pub(crate) type glVertexAttrib4Nusv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

pub(crate) type glVertexAttrib4bv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

pub(crate) type glVertexAttrib4d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub(crate) type glVertexAttrib4dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

pub(crate) type glVertexAttrib4f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub(crate) type glVertexAttrib4fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

pub(crate) type glVertexAttrib4iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub(crate) type glVertexAttrib4s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

pub(crate) type glVertexAttrib4sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub(crate) type glVertexAttrib4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub(crate) type glVertexAttrib4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

pub(crate) type glVertexAttrib4usv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

pub(crate) type glVertexAttribDivisor_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

pub(crate) type glVertexAttribI1i_t = unsafe extern "system" fn(index: GLuint, x: GLint);

pub(crate) type glVertexAttribI1iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);

pub(crate) type glVertexAttribI1ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint);

pub(crate) type glVertexAttribI1uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);

pub(crate) type glVertexAttribI2i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint);

pub(crate) type glVertexAttribI2iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 2]);

pub(crate) type glVertexAttribI2ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint);

pub(crate) type glVertexAttribI2uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 2]);

pub(crate) type glVertexAttribI3i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint);

pub(crate) type glVertexAttribI3iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 3]);

pub(crate) type glVertexAttribI3ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);

pub(crate) type glVertexAttribI3uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 3]);

pub(crate) type glVertexAttribI4bv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

pub(crate) type glVertexAttribI4i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

pub(crate) type glVertexAttribI4iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub(crate) type glVertexAttribI4sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub(crate) type glVertexAttribI4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub(crate) type glVertexAttribI4ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

pub(crate) type glVertexAttribI4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

pub(crate) type glVertexAttribI4usv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

pub(crate) type glVertexAttribIPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei, pointer: *const void);

pub(crate) type glVertexAttribP1ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

pub(crate) type glVertexAttribP1uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

pub(crate) type glVertexAttribP2ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

pub(crate) type glVertexAttribP2uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

pub(crate) type glVertexAttribP3ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

pub(crate) type glVertexAttribP3uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

pub(crate) type glVertexAttribP4ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

pub(crate) type glVertexAttribP4uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

pub(crate) type glVertexAttribPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, pointer: *const void);

pub(crate) type glViewport_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glWaitSync_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);
