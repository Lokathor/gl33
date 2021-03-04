use super::*;

/// AttributeType
/// * [`GL_BOOL`]
/// * [`GL_BOOL_VEC2`]
/// * [`GL_BOOL_VEC3`]
/// * [`GL_BOOL_VEC4`]
/// * [`GL_DOUBLE`]
/// * [`GL_FLOAT`]
/// * [`GL_FLOAT_MAT2`]
/// * [`GL_FLOAT_MAT2x3`]
/// * [`GL_FLOAT_MAT2x4`]
/// * [`GL_FLOAT_MAT3`]
/// * [`GL_FLOAT_MAT3x2`]
/// * [`GL_FLOAT_MAT3x4`]
/// * [`GL_FLOAT_MAT4`]
/// * [`GL_FLOAT_MAT4x2`]
/// * [`GL_FLOAT_MAT4x3`]
/// * [`GL_FLOAT_VEC2`]
/// * [`GL_FLOAT_VEC3`]
/// * [`GL_FLOAT_VEC4`]
/// * [`GL_INT`]
/// * [`GL_INT_SAMPLER_1D`]
/// * [`GL_INT_SAMPLER_1D_ARRAY`]
/// * [`GL_INT_SAMPLER_2D`]
/// * [`GL_INT_SAMPLER_2D_ARRAY`]
/// * [`GL_INT_SAMPLER_2D_MULTISAMPLE`]
/// * [`GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY`]
/// * [`GL_INT_SAMPLER_2D_RECT`]
/// * [`GL_INT_SAMPLER_3D`]
/// * [`GL_INT_SAMPLER_BUFFER`]
/// * [`GL_INT_SAMPLER_CUBE`]
/// * [`GL_INT_VEC2`]
/// * [`GL_INT_VEC3`]
/// * [`GL_INT_VEC4`]
/// * [`GL_SAMPLER_1D`]
/// * [`GL_SAMPLER_1D_ARRAY_SHADOW`]
/// * [`GL_SAMPLER_1D_SHADOW`]
/// * [`GL_SAMPLER_2D`]
/// * [`GL_SAMPLER_2D_ARRAY_SHADOW`]
/// * [`GL_SAMPLER_2D_MULTISAMPLE`]
/// * [`GL_SAMPLER_2D_MULTISAMPLE_ARRAY`]
/// * [`GL_SAMPLER_2D_RECT`]
/// * [`GL_SAMPLER_2D_RECT_SHADOW`]
/// * [`GL_SAMPLER_2D_SHADOW`]
/// * [`GL_SAMPLER_3D`]
/// * [`GL_SAMPLER_BUFFER`]
/// * [`GL_SAMPLER_CUBE`]
/// * [`GL_SAMPLER_CUBE_SHADOW`]
/// * [`GL_UNSIGNED_INT`]
/// * [`GL_UNSIGNED_INT_SAMPLER_1D`]
/// * [`GL_UNSIGNED_INT_SAMPLER_1D_ARRAY`]
/// * [`GL_UNSIGNED_INT_SAMPLER_2D`]
/// * [`GL_UNSIGNED_INT_SAMPLER_2D_ARRAY`]
/// * [`GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE`]
/// * [`GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY`]
/// * [`GL_UNSIGNED_INT_SAMPLER_2D_RECT`]
/// * [`GL_UNSIGNED_INT_SAMPLER_3D`]
/// * [`GL_UNSIGNED_INT_SAMPLER_BUFFER`]
/// * [`GL_UNSIGNED_INT_SAMPLER_CUBE`]
/// * [`GL_UNSIGNED_INT_VEC2`]
/// * [`GL_UNSIGNED_INT_VEC3`]
/// * [`GL_UNSIGNED_INT_VEC4`]
pub type AttributeType = GLenum;

/// BlendEquationModeEXT
/// * [`GL_FUNC_ADD`]
/// * [`GL_FUNC_REVERSE_SUBTRACT`]
/// * [`GL_FUNC_SUBTRACT`]
/// * [`GL_MAX`]
/// * [`GL_MIN`]
pub type BlendEquationModeEXT = GLenum;

/// BlendingFactor
/// * [`GL_CONSTANT_ALPHA`]
/// * [`GL_CONSTANT_COLOR`]
/// * [`GL_DST_ALPHA`]
/// * [`GL_DST_COLOR`]
/// * [`GL_ONE`]
/// * [`GL_ONE_MINUS_CONSTANT_ALPHA`]
/// * [`GL_ONE_MINUS_CONSTANT_COLOR`]
/// * [`GL_ONE_MINUS_DST_ALPHA`]
/// * [`GL_ONE_MINUS_DST_COLOR`]
/// * [`GL_ONE_MINUS_SRC1_ALPHA`]
/// * [`GL_ONE_MINUS_SRC1_COLOR`]
/// * [`GL_ONE_MINUS_SRC_ALPHA`]
/// * [`GL_ONE_MINUS_SRC_COLOR`]
/// * [`GL_SRC1_ALPHA`]
/// * [`GL_SRC1_COLOR`]
/// * [`GL_SRC_ALPHA`]
/// * [`GL_SRC_ALPHA_SATURATE`]
/// * [`GL_SRC_COLOR`]
/// * [`GL_ZERO`]
pub type BlendingFactor = GLenum;

/// BlitFramebufferFilter
/// * [`GL_LINEAR`]
/// * [`GL_NEAREST`]
pub type BlitFramebufferFilter = GLenum;

/// Boolean
/// * [`GL_FALSE`]
/// * [`GL_TRUE`]
pub type Boolean = GLenum;

/// Buffer
/// * [`GL_COLOR`]
/// * [`GL_DEPTH`]
/// * [`GL_STENCIL`]
pub type Buffer = GLenum;

/// BufferAccessARB
/// * [`GL_READ_ONLY`]
/// * [`GL_READ_WRITE`]
/// * [`GL_WRITE_ONLY`]
pub type BufferAccessARB = GLenum;

/// BufferPNameARB
/// * [`GL_BUFFER_ACCESS`]
/// * [`GL_BUFFER_ACCESS_FLAGS`]
/// * [`GL_BUFFER_MAPPED`]
/// * [`GL_BUFFER_MAP_LENGTH`]
/// * [`GL_BUFFER_MAP_OFFSET`]
/// * [`GL_BUFFER_SIZE`]
/// * [`GL_BUFFER_USAGE`]
pub type BufferPNameARB = GLenum;

/// BufferPointerNameARB
/// * [`GL_BUFFER_MAP_POINTER`]
pub type BufferPointerNameARB = GLenum;

/// BufferTargetARB
/// * [`GL_ARRAY_BUFFER`]
/// * [`GL_COPY_READ_BUFFER`]
/// * [`GL_COPY_WRITE_BUFFER`]
/// * [`GL_ELEMENT_ARRAY_BUFFER`]
/// * [`GL_PIXEL_PACK_BUFFER`]
/// * [`GL_PIXEL_UNPACK_BUFFER`]
/// * [`GL_TEXTURE_BUFFER`]
/// * [`GL_TRANSFORM_FEEDBACK_BUFFER`]
/// * [`GL_UNIFORM_BUFFER`]
pub type BufferTargetARB = GLenum;

/// BufferUsageARB
/// * [`GL_DYNAMIC_COPY`]
/// * [`GL_DYNAMIC_DRAW`]
/// * [`GL_DYNAMIC_READ`]
/// * [`GL_STATIC_COPY`]
/// * [`GL_STATIC_DRAW`]
/// * [`GL_STATIC_READ`]
/// * [`GL_STREAM_COPY`]
/// * [`GL_STREAM_DRAW`]
/// * [`GL_STREAM_READ`]
pub type BufferUsageARB = GLenum;

/// ClampColorModeARB
/// * [`GL_FALSE`]
/// * [`GL_FIXED_ONLY`]
/// * [`GL_TRUE`]
pub type ClampColorModeARB = GLenum;

/// ClampColorTargetARB
/// * [`GL_CLAMP_READ_COLOR`]
pub type ClampColorTargetARB = GLenum;

/// ConditionalRenderMode
/// * [`GL_QUERY_BY_REGION_NO_WAIT`]
/// * [`GL_QUERY_BY_REGION_WAIT`]
/// * [`GL_QUERY_NO_WAIT`]
/// * [`GL_QUERY_WAIT`]
pub type ConditionalRenderMode = GLenum;

/// CopyBufferSubDataTarget
/// * [`GL_ARRAY_BUFFER`]
/// * [`GL_COPY_READ_BUFFER`]
/// * [`GL_COPY_WRITE_BUFFER`]
/// * [`GL_ELEMENT_ARRAY_BUFFER`]
/// * [`GL_PIXEL_PACK_BUFFER`]
/// * [`GL_PIXEL_UNPACK_BUFFER`]
/// * [`GL_TEXTURE_BUFFER`]
/// * [`GL_TRANSFORM_FEEDBACK_BUFFER`]
/// * [`GL_UNIFORM_BUFFER`]
pub type CopyBufferSubDataTarget = GLenum;

/// CullFaceMode
/// * [`GL_BACK`]
/// * [`GL_FRONT`]
/// * [`GL_FRONT_AND_BACK`]
pub type CullFaceMode = GLenum;

/// DebugSeverity
/// * [`GL_DEBUG_SEVERITY_HIGH`]
/// * [`GL_DEBUG_SEVERITY_LOW`]
/// * [`GL_DEBUG_SEVERITY_MEDIUM`]
/// * [`GL_DEBUG_SEVERITY_NOTIFICATION`]
/// * [`GL_DONT_CARE`]
pub type DebugSeverity = GLenum;

/// DebugSource
/// * [`GL_DEBUG_SOURCE_API`]
/// * [`GL_DEBUG_SOURCE_APPLICATION`]
/// * [`GL_DEBUG_SOURCE_OTHER`]
/// * [`GL_DEBUG_SOURCE_SHADER_COMPILER`]
/// * [`GL_DEBUG_SOURCE_THIRD_PARTY`]
/// * [`GL_DEBUG_SOURCE_WINDOW_SYSTEM`]
/// * [`GL_DONT_CARE`]
pub type DebugSource = GLenum;

/// DebugType
/// * [`GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR`]
/// * [`GL_DEBUG_TYPE_ERROR`]
/// * [`GL_DEBUG_TYPE_MARKER`]
/// * [`GL_DEBUG_TYPE_OTHER`]
/// * [`GL_DEBUG_TYPE_PERFORMANCE`]
/// * [`GL_DEBUG_TYPE_POP_GROUP`]
/// * [`GL_DEBUG_TYPE_PORTABILITY`]
/// * [`GL_DEBUG_TYPE_PUSH_GROUP`]
/// * [`GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR`]
/// * [`GL_DONT_CARE`]
pub type DebugType = GLenum;

/// DepthFunction
/// * [`GL_ALWAYS`]
/// * [`GL_EQUAL`]
/// * [`GL_GEQUAL`]
/// * [`GL_GREATER`]
/// * [`GL_LEQUAL`]
/// * [`GL_LESS`]
/// * [`GL_NEVER`]
/// * [`GL_NOTEQUAL`]
pub type DepthFunction = GLenum;

/// DrawBufferMode
/// * [`GL_BACK`]
/// * [`GL_BACK_LEFT`]
/// * [`GL_BACK_RIGHT`]
/// * [`GL_COLOR_ATTACHMENT0`]
/// * [`GL_COLOR_ATTACHMENT1`]
/// * [`GL_COLOR_ATTACHMENT10`]
/// * [`GL_COLOR_ATTACHMENT11`]
/// * [`GL_COLOR_ATTACHMENT12`]
/// * [`GL_COLOR_ATTACHMENT13`]
/// * [`GL_COLOR_ATTACHMENT14`]
/// * [`GL_COLOR_ATTACHMENT15`]
/// * [`GL_COLOR_ATTACHMENT16`]
/// * [`GL_COLOR_ATTACHMENT17`]
/// * [`GL_COLOR_ATTACHMENT18`]
/// * [`GL_COLOR_ATTACHMENT19`]
/// * [`GL_COLOR_ATTACHMENT2`]
/// * [`GL_COLOR_ATTACHMENT20`]
/// * [`GL_COLOR_ATTACHMENT21`]
/// * [`GL_COLOR_ATTACHMENT22`]
/// * [`GL_COLOR_ATTACHMENT23`]
/// * [`GL_COLOR_ATTACHMENT24`]
/// * [`GL_COLOR_ATTACHMENT25`]
/// * [`GL_COLOR_ATTACHMENT26`]
/// * [`GL_COLOR_ATTACHMENT27`]
/// * [`GL_COLOR_ATTACHMENT28`]
/// * [`GL_COLOR_ATTACHMENT29`]
/// * [`GL_COLOR_ATTACHMENT3`]
/// * [`GL_COLOR_ATTACHMENT30`]
/// * [`GL_COLOR_ATTACHMENT31`]
/// * [`GL_COLOR_ATTACHMENT4`]
/// * [`GL_COLOR_ATTACHMENT5`]
/// * [`GL_COLOR_ATTACHMENT6`]
/// * [`GL_COLOR_ATTACHMENT7`]
/// * [`GL_COLOR_ATTACHMENT8`]
/// * [`GL_COLOR_ATTACHMENT9`]
/// * [`GL_FRONT`]
/// * [`GL_FRONT_AND_BACK`]
/// * [`GL_FRONT_LEFT`]
/// * [`GL_FRONT_RIGHT`]
/// * [`GL_LEFT`]
/// * [`GL_NONE`]
/// * [`GL_RIGHT`]
pub type DrawBufferMode = GLenum;

/// DrawElementsType
/// * [`GL_UNSIGNED_BYTE`]
/// * [`GL_UNSIGNED_INT`]
/// * [`GL_UNSIGNED_SHORT`]
pub type DrawElementsType = GLenum;

/// EnableCap
/// * [`GL_BLEND`]
/// * [`GL_CLIP_DISTANCE0`]
/// * [`GL_CLIP_DISTANCE1`]
/// * [`GL_CLIP_DISTANCE2`]
/// * [`GL_CLIP_DISTANCE3`]
/// * [`GL_CLIP_DISTANCE4`]
/// * [`GL_CLIP_DISTANCE5`]
/// * [`GL_CLIP_DISTANCE6`]
/// * [`GL_CLIP_DISTANCE7`]
/// * [`GL_COLOR_LOGIC_OP`]
/// * [`GL_CULL_FACE`]
/// * [`GL_DEBUG_OUTPUT`]
/// * [`GL_DEBUG_OUTPUT_SYNCHRONOUS`]
/// * [`GL_DEPTH_CLAMP`]
/// * [`GL_DEPTH_TEST`]
/// * [`GL_DITHER`]
/// * [`GL_FRAMEBUFFER_SRGB`]
/// * [`GL_LINE_SMOOTH`]
/// * [`GL_MULTISAMPLE`]
/// * [`GL_POLYGON_OFFSET_FILL`]
/// * [`GL_POLYGON_OFFSET_LINE`]
/// * [`GL_POLYGON_OFFSET_POINT`]
/// * [`GL_POLYGON_SMOOTH`]
/// * [`GL_PRIMITIVE_RESTART`]
/// * [`GL_PROGRAM_POINT_SIZE`]
/// * [`GL_RASTERIZER_DISCARD`]
/// * [`GL_SAMPLE_ALPHA_TO_COVERAGE`]
/// * [`GL_SAMPLE_ALPHA_TO_ONE`]
/// * [`GL_SAMPLE_COVERAGE`]
/// * [`GL_SAMPLE_MASK`]
/// * [`GL_SCISSOR_TEST`]
/// * [`GL_STENCIL_TEST`]
/// * [`GL_TEXTURE_1D`]
/// * [`GL_TEXTURE_2D`]
/// * [`GL_TEXTURE_CUBE_MAP_SEAMLESS`]
/// * [`GL_VERTEX_ARRAY`]
pub type EnableCap = GLenum;

/// ErrorCode
/// * [`GL_INVALID_ENUM`]
/// * [`GL_INVALID_FRAMEBUFFER_OPERATION`]
/// * [`GL_INVALID_OPERATION`]
/// * [`GL_INVALID_VALUE`]
/// * [`GL_NO_ERROR`]
/// * [`GL_OUT_OF_MEMORY`]
/// * [`GL_STACK_OVERFLOW`]
/// * [`GL_STACK_UNDERFLOW`]
pub type ErrorCode = GLenum;

/// FramebufferAttachment
/// * [`GL_COLOR_ATTACHMENT0`]
/// * [`GL_COLOR_ATTACHMENT1`]
/// * [`GL_COLOR_ATTACHMENT10`]
/// * [`GL_COLOR_ATTACHMENT11`]
/// * [`GL_COLOR_ATTACHMENT12`]
/// * [`GL_COLOR_ATTACHMENT13`]
/// * [`GL_COLOR_ATTACHMENT14`]
/// * [`GL_COLOR_ATTACHMENT15`]
/// * [`GL_COLOR_ATTACHMENT16`]
/// * [`GL_COLOR_ATTACHMENT17`]
/// * [`GL_COLOR_ATTACHMENT18`]
/// * [`GL_COLOR_ATTACHMENT19`]
/// * [`GL_COLOR_ATTACHMENT2`]
/// * [`GL_COLOR_ATTACHMENT20`]
/// * [`GL_COLOR_ATTACHMENT21`]
/// * [`GL_COLOR_ATTACHMENT22`]
/// * [`GL_COLOR_ATTACHMENT23`]
/// * [`GL_COLOR_ATTACHMENT24`]
/// * [`GL_COLOR_ATTACHMENT25`]
/// * [`GL_COLOR_ATTACHMENT26`]
/// * [`GL_COLOR_ATTACHMENT27`]
/// * [`GL_COLOR_ATTACHMENT28`]
/// * [`GL_COLOR_ATTACHMENT29`]
/// * [`GL_COLOR_ATTACHMENT3`]
/// * [`GL_COLOR_ATTACHMENT30`]
/// * [`GL_COLOR_ATTACHMENT31`]
/// * [`GL_COLOR_ATTACHMENT4`]
/// * [`GL_COLOR_ATTACHMENT5`]
/// * [`GL_COLOR_ATTACHMENT6`]
/// * [`GL_COLOR_ATTACHMENT7`]
/// * [`GL_COLOR_ATTACHMENT8`]
/// * [`GL_COLOR_ATTACHMENT9`]
/// * [`GL_DEPTH_ATTACHMENT`]
/// * [`GL_STENCIL_ATTACHMENT`]
pub type FramebufferAttachment = GLenum;

/// FramebufferAttachmentParameterName
/// * [`GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_LAYERED`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER`]
/// * [`GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL`]
pub type FramebufferAttachmentParameterName = GLenum;

/// FramebufferStatus
/// * [`GL_FRAMEBUFFER_COMPLETE`]
/// * [`GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT`]
/// * [`GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER`]
/// * [`GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS`]
/// * [`GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT`]
/// * [`GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE`]
/// * [`GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER`]
/// * [`GL_FRAMEBUFFER_UNDEFINED`]
/// * [`GL_FRAMEBUFFER_UNSUPPORTED`]
pub type FramebufferStatus = GLenum;

/// FramebufferTarget
/// * [`GL_DRAW_FRAMEBUFFER`]
/// * [`GL_FRAMEBUFFER`]
/// * [`GL_READ_FRAMEBUFFER`]
pub type FramebufferTarget = GLenum;

/// FrontFaceDirection
/// * [`GL_CCW`]
/// * [`GL_CW`]
pub type FrontFaceDirection = GLenum;

/// GetMultisamplePNameNV
/// * [`GL_SAMPLE_POSITION`]
pub type GetMultisamplePNameNV = GLenum;

/// GetPName
/// * [`GL_ACTIVE_TEXTURE`]
/// * [`GL_ALIASED_LINE_WIDTH_RANGE`]
/// * [`GL_ARRAY_BUFFER_BINDING`]
/// * [`GL_BLEND`]
/// * [`GL_BLEND_COLOR`]
/// * [`GL_BLEND_DST`]
/// * [`GL_BLEND_DST_ALPHA`]
/// * [`GL_BLEND_DST_RGB`]
/// * [`GL_BLEND_EQUATION_ALPHA`]
/// * [`GL_BLEND_EQUATION_RGB`]
/// * [`GL_BLEND_SRC`]
/// * [`GL_BLEND_SRC_ALPHA`]
/// * [`GL_BLEND_SRC_RGB`]
/// * [`GL_COLOR_CLEAR_VALUE`]
/// * [`GL_COLOR_LOGIC_OP`]
/// * [`GL_COLOR_WRITEMASK`]
/// * [`GL_COMPRESSED_TEXTURE_FORMATS`]
/// * [`GL_CONTEXT_FLAGS`]
/// * [`GL_CONTEXT_PROFILE_MASK`]
/// * [`GL_CULL_FACE`]
/// * [`GL_CULL_FACE_MODE`]
/// * [`GL_CURRENT_PROGRAM`]
/// * [`GL_DEBUG_GROUP_STACK_DEPTH`]
/// * [`GL_DEPTH_CLEAR_VALUE`]
/// * [`GL_DEPTH_FUNC`]
/// * [`GL_DEPTH_RANGE`]
/// * [`GL_DEPTH_TEST`]
/// * [`GL_DEPTH_WRITEMASK`]
/// * [`GL_DITHER`]
/// * [`GL_DOUBLEBUFFER`]
/// * [`GL_DRAW_BUFFER`]
/// * [`GL_DRAW_FRAMEBUFFER_BINDING`]
/// * [`GL_ELEMENT_ARRAY_BUFFER_BINDING`]
/// * [`GL_FRAGMENT_SHADER_DERIVATIVE_HINT`]
/// * [`GL_FRONT_FACE`]
/// * [`GL_LINE_SMOOTH`]
/// * [`GL_LINE_SMOOTH_HINT`]
/// * [`GL_LINE_WIDTH`]
/// * [`GL_LINE_WIDTH_GRANULARITY`]
/// * [`GL_LINE_WIDTH_RANGE`]
/// * [`GL_LOGIC_OP_MODE`]
/// * [`GL_MAJOR_VERSION`]
/// * [`GL_MAX_3D_TEXTURE_SIZE`]
/// * [`GL_MAX_ARRAY_TEXTURE_LAYERS`]
/// * [`GL_MAX_CLIP_DISTANCES`]
/// * [`GL_MAX_COLOR_TEXTURE_SAMPLES`]
/// * [`GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS`]
/// * [`GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS`]
/// * [`GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS`]
/// * [`GL_MAX_COMBINED_UNIFORM_BLOCKS`]
/// * [`GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS`]
/// * [`GL_MAX_CUBE_MAP_TEXTURE_SIZE`]
/// * [`GL_MAX_DEBUG_GROUP_STACK_DEPTH`]
/// * [`GL_MAX_DEPTH_TEXTURE_SAMPLES`]
/// * [`GL_MAX_DRAW_BUFFERS`]
/// * [`GL_MAX_DUAL_SOURCE_DRAW_BUFFERS`]
/// * [`GL_MAX_ELEMENTS_INDICES`]
/// * [`GL_MAX_ELEMENTS_VERTICES`]
/// * [`GL_MAX_FRAGMENT_INPUT_COMPONENTS`]
/// * [`GL_MAX_FRAGMENT_UNIFORM_BLOCKS`]
/// * [`GL_MAX_FRAGMENT_UNIFORM_COMPONENTS`]
/// * [`GL_MAX_GEOMETRY_INPUT_COMPONENTS`]
/// * [`GL_MAX_GEOMETRY_OUTPUT_COMPONENTS`]
/// * [`GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS`]
/// * [`GL_MAX_GEOMETRY_UNIFORM_BLOCKS`]
/// * [`GL_MAX_GEOMETRY_UNIFORM_COMPONENTS`]
/// * [`GL_MAX_INTEGER_SAMPLES`]
/// * [`GL_MAX_LABEL_LENGTH`]
/// * [`GL_MAX_PROGRAM_TEXEL_OFFSET`]
/// * [`GL_MAX_RECTANGLE_TEXTURE_SIZE`]
/// * [`GL_MAX_RENDERBUFFER_SIZE`]
/// * [`GL_MAX_SAMPLE_MASK_WORDS`]
/// * [`GL_MAX_SERVER_WAIT_TIMEOUT`]
/// * [`GL_MAX_TEXTURE_BUFFER_SIZE`]
/// * [`GL_MAX_TEXTURE_IMAGE_UNITS`]
/// * [`GL_MAX_TEXTURE_LOD_BIAS`]
/// * [`GL_MAX_TEXTURE_SIZE`]
/// * [`GL_MAX_UNIFORM_BLOCK_SIZE`]
/// * [`GL_MAX_UNIFORM_BUFFER_BINDINGS`]
/// * [`GL_MAX_VARYING_COMPONENTS`]
/// * [`GL_MAX_VARYING_FLOATS`]
/// * [`GL_MAX_VERTEX_ATTRIBS`]
/// * [`GL_MAX_VERTEX_OUTPUT_COMPONENTS`]
/// * [`GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS`]
/// * [`GL_MAX_VERTEX_UNIFORM_BLOCKS`]
/// * [`GL_MAX_VERTEX_UNIFORM_COMPONENTS`]
/// * [`GL_MAX_VIEWPORT_DIMS`]
/// * [`GL_MINOR_VERSION`]
/// * [`GL_MIN_PROGRAM_TEXEL_OFFSET`]
/// * [`GL_NUM_COMPRESSED_TEXTURE_FORMATS`]
/// * [`GL_NUM_EXTENSIONS`]
/// * [`GL_PACK_ALIGNMENT`]
/// * [`GL_PACK_IMAGE_HEIGHT`]
/// * [`GL_PACK_LSB_FIRST`]
/// * [`GL_PACK_ROW_LENGTH`]
/// * [`GL_PACK_SKIP_IMAGES`]
/// * [`GL_PACK_SKIP_PIXELS`]
/// * [`GL_PACK_SKIP_ROWS`]
/// * [`GL_PACK_SWAP_BYTES`]
/// * [`GL_PIXEL_PACK_BUFFER_BINDING`]
/// * [`GL_PIXEL_UNPACK_BUFFER_BINDING`]
/// * [`GL_POINT_FADE_THRESHOLD_SIZE`]
/// * [`GL_POINT_SIZE`]
/// * [`GL_POINT_SIZE_GRANULARITY`]
/// * [`GL_POINT_SIZE_RANGE`]
/// * [`GL_POLYGON_MODE`]
/// * [`GL_POLYGON_OFFSET_FACTOR`]
/// * [`GL_POLYGON_OFFSET_FILL`]
/// * [`GL_POLYGON_OFFSET_LINE`]
/// * [`GL_POLYGON_OFFSET_POINT`]
/// * [`GL_POLYGON_OFFSET_UNITS`]
/// * [`GL_POLYGON_SMOOTH`]
/// * [`GL_POLYGON_SMOOTH_HINT`]
/// * [`GL_PRIMITIVE_RESTART_INDEX`]
/// * [`GL_PROGRAM_POINT_SIZE`]
/// * [`GL_PROVOKING_VERTEX`]
/// * [`GL_READ_BUFFER`]
/// * [`GL_READ_FRAMEBUFFER_BINDING`]
/// * [`GL_RENDERBUFFER_BINDING`]
/// * [`GL_SAMPLER_BINDING`]
/// * [`GL_SAMPLES`]
/// * [`GL_SAMPLE_BUFFERS`]
/// * [`GL_SAMPLE_COVERAGE_INVERT`]
/// * [`GL_SAMPLE_COVERAGE_VALUE`]
/// * [`GL_SCISSOR_BOX`]
/// * [`GL_SCISSOR_TEST`]
/// * [`GL_SMOOTH_LINE_WIDTH_GRANULARITY`]
/// * [`GL_SMOOTH_LINE_WIDTH_RANGE`]
/// * [`GL_SMOOTH_POINT_SIZE_GRANULARITY`]
/// * [`GL_SMOOTH_POINT_SIZE_RANGE`]
/// * [`GL_STENCIL_BACK_FAIL`]
/// * [`GL_STENCIL_BACK_FUNC`]
/// * [`GL_STENCIL_BACK_PASS_DEPTH_FAIL`]
/// * [`GL_STENCIL_BACK_PASS_DEPTH_PASS`]
/// * [`GL_STENCIL_BACK_REF`]
/// * [`GL_STENCIL_BACK_VALUE_MASK`]
/// * [`GL_STENCIL_BACK_WRITEMASK`]
/// * [`GL_STENCIL_CLEAR_VALUE`]
/// * [`GL_STENCIL_FAIL`]
/// * [`GL_STENCIL_FUNC`]
/// * [`GL_STENCIL_PASS_DEPTH_FAIL`]
/// * [`GL_STENCIL_PASS_DEPTH_PASS`]
/// * [`GL_STENCIL_REF`]
/// * [`GL_STENCIL_TEST`]
/// * [`GL_STENCIL_VALUE_MASK`]
/// * [`GL_STENCIL_WRITEMASK`]
/// * [`GL_STEREO`]
/// * [`GL_SUBPIXEL_BITS`]
/// * [`GL_TEXTURE_1D`]
/// * [`GL_TEXTURE_2D`]
/// * [`GL_TEXTURE_BINDING_1D`]
/// * [`GL_TEXTURE_BINDING_1D_ARRAY`]
/// * [`GL_TEXTURE_BINDING_2D`]
/// * [`GL_TEXTURE_BINDING_2D_ARRAY`]
/// * [`GL_TEXTURE_BINDING_2D_MULTISAMPLE`]
/// * [`GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY`]
/// * [`GL_TEXTURE_BINDING_3D`]
/// * [`GL_TEXTURE_BINDING_BUFFER`]
/// * [`GL_TEXTURE_BINDING_CUBE_MAP`]
/// * [`GL_TEXTURE_BINDING_RECTANGLE`]
/// * [`GL_TEXTURE_COMPRESSION_HINT`]
/// * [`GL_TIMESTAMP`]
/// * [`GL_TRANSFORM_FEEDBACK_BUFFER_BINDING`]
/// * [`GL_TRANSFORM_FEEDBACK_BUFFER_SIZE`]
/// * [`GL_TRANSFORM_FEEDBACK_BUFFER_START`]
/// * [`GL_UNIFORM_BUFFER_BINDING`]
/// * [`GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT`]
/// * [`GL_UNIFORM_BUFFER_SIZE`]
/// * [`GL_UNIFORM_BUFFER_START`]
/// * [`GL_UNPACK_ALIGNMENT`]
/// * [`GL_UNPACK_IMAGE_HEIGHT`]
/// * [`GL_UNPACK_LSB_FIRST`]
/// * [`GL_UNPACK_ROW_LENGTH`]
/// * [`GL_UNPACK_SKIP_IMAGES`]
/// * [`GL_UNPACK_SKIP_PIXELS`]
/// * [`GL_UNPACK_SKIP_ROWS`]
/// * [`GL_UNPACK_SWAP_BYTES`]
/// * [`GL_VERTEX_ARRAY`]
/// * [`GL_VERTEX_ARRAY_BINDING`]
/// * [`GL_VIEWPORT`]
pub type GetPName = GLenum;

/// GetPointervPName
/// * [`GL_DEBUG_CALLBACK_FUNCTION`]
/// * [`GL_DEBUG_CALLBACK_USER_PARAM`]
pub type GetPointervPName = GLenum;

/// GetTextureParameter
/// * [`GL_TEXTURE_ALPHA_SIZE`]
/// * [`GL_TEXTURE_BLUE_SIZE`]
/// * [`GL_TEXTURE_BORDER_COLOR`]
/// * [`GL_TEXTURE_GREEN_SIZE`]
/// * [`GL_TEXTURE_HEIGHT`]
/// * [`GL_TEXTURE_INTERNAL_FORMAT`]
/// * [`GL_TEXTURE_MAG_FILTER`]
/// * [`GL_TEXTURE_MIN_FILTER`]
/// * [`GL_TEXTURE_RED_SIZE`]
/// * [`GL_TEXTURE_WIDTH`]
/// * [`GL_TEXTURE_WRAP_S`]
/// * [`GL_TEXTURE_WRAP_T`]
pub type GetTextureParameter = GLenum;

/// HintMode
/// * [`GL_DONT_CARE`]
/// * [`GL_FASTEST`]
/// * [`GL_NICEST`]
pub type HintMode = GLenum;

/// HintTarget
/// * [`GL_FRAGMENT_SHADER_DERIVATIVE_HINT`]
/// * [`GL_LINE_SMOOTH_HINT`]
/// * [`GL_POLYGON_SMOOTH_HINT`]
/// * [`GL_TEXTURE_COMPRESSION_HINT`]
pub type HintTarget = GLenum;

/// InternalFormat
/// * [`GL_COMPRESSED_RED`]
/// * [`GL_COMPRESSED_RED_RGTC1`]
/// * [`GL_COMPRESSED_RG`]
/// * [`GL_COMPRESSED_RGB`]
/// * [`GL_COMPRESSED_RGBA`]
/// * [`GL_COMPRESSED_RG_RGTC2`]
/// * [`GL_COMPRESSED_SIGNED_RED_RGTC1`]
/// * [`GL_COMPRESSED_SIGNED_RG_RGTC2`]
/// * [`GL_COMPRESSED_SRGB`]
/// * [`GL_COMPRESSED_SRGB_ALPHA`]
/// * [`GL_DEPTH24_STENCIL8`]
/// * [`GL_DEPTH32F_STENCIL8`]
/// * [`GL_DEPTH_COMPONENT`]
/// * [`GL_DEPTH_COMPONENT16`]
/// * [`GL_DEPTH_COMPONENT32F`]
/// * [`GL_DEPTH_STENCIL`]
/// * [`GL_R11F_G11F_B10F`]
/// * [`GL_R16`]
/// * [`GL_R16F`]
/// * [`GL_R16I`]
/// * [`GL_R16UI`]
/// * [`GL_R16_SNORM`]
/// * [`GL_R32F`]
/// * [`GL_R32I`]
/// * [`GL_R32UI`]
/// * [`GL_R3_G3_B2`]
/// * [`GL_R8`]
/// * [`GL_R8I`]
/// * [`GL_R8UI`]
/// * [`GL_R8_SNORM`]
/// * [`GL_RED`]
/// * [`GL_RG`]
/// * [`GL_RG16`]
/// * [`GL_RG16F`]
/// * [`GL_RG16I`]
/// * [`GL_RG16UI`]
/// * [`GL_RG16_SNORM`]
/// * [`GL_RG32F`]
/// * [`GL_RG32I`]
/// * [`GL_RG32UI`]
/// * [`GL_RG8`]
/// * [`GL_RG8I`]
/// * [`GL_RG8UI`]
/// * [`GL_RG8_SNORM`]
/// * [`GL_RGB`]
/// * [`GL_RGB10`]
/// * [`GL_RGB10_A2`]
/// * [`GL_RGB10_A2UI`]
/// * [`GL_RGB12`]
/// * [`GL_RGB16`]
/// * [`GL_RGB16F`]
/// * [`GL_RGB16I`]
/// * [`GL_RGB16UI`]
/// * [`GL_RGB16_SNORM`]
/// * [`GL_RGB32F`]
/// * [`GL_RGB32I`]
/// * [`GL_RGB32UI`]
/// * [`GL_RGB4`]
/// * [`GL_RGB5`]
/// * [`GL_RGB5_A1`]
/// * [`GL_RGB8`]
/// * [`GL_RGB8I`]
/// * [`GL_RGB8UI`]
/// * [`GL_RGB8_SNORM`]
/// * [`GL_RGB9_E5`]
/// * [`GL_RGBA`]
/// * [`GL_RGBA12`]
/// * [`GL_RGBA16`]
/// * [`GL_RGBA16F`]
/// * [`GL_RGBA16I`]
/// * [`GL_RGBA16UI`]
/// * [`GL_RGBA32F`]
/// * [`GL_RGBA32I`]
/// * [`GL_RGBA32UI`]
/// * [`GL_RGBA4`]
/// * [`GL_RGBA8`]
/// * [`GL_RGBA8I`]
/// * [`GL_RGBA8UI`]
/// * [`GL_RGBA8_SNORM`]
/// * [`GL_SRGB`]
/// * [`GL_SRGB8`]
/// * [`GL_SRGB8_ALPHA8`]
/// * [`GL_SRGB_ALPHA`]
/// * [`GL_STENCIL_INDEX`]
/// * [`GL_STENCIL_INDEX1`]
/// * [`GL_STENCIL_INDEX16`]
/// * [`GL_STENCIL_INDEX4`]
/// * [`GL_STENCIL_INDEX8`]
pub type InternalFormat = GLenum;

/// LogicOp
/// * [`GL_AND`]
/// * [`GL_AND_INVERTED`]
/// * [`GL_AND_REVERSE`]
/// * [`GL_CLEAR`]
/// * [`GL_COPY`]
/// * [`GL_COPY_INVERTED`]
/// * [`GL_EQUIV`]
/// * [`GL_INVERT`]
/// * [`GL_NAND`]
/// * [`GL_NOOP`]
/// * [`GL_NOR`]
/// * [`GL_OR`]
/// * [`GL_OR_INVERTED`]
/// * [`GL_OR_REVERSE`]
/// * [`GL_SET`]
/// * [`GL_XOR`]
pub type LogicOp = GLenum;

/// MaterialFace
/// * [`GL_BACK`]
/// * [`GL_FRONT`]
/// * [`GL_FRONT_AND_BACK`]
pub type MaterialFace = GLenum;

/// ObjectIdentifier
/// * [`GL_BUFFER`]
/// * [`GL_FRAMEBUFFER`]
/// * [`GL_PROGRAM`]
/// * [`GL_PROGRAM_PIPELINE`]
/// * [`GL_QUERY`]
/// * [`GL_RENDERBUFFER`]
/// * [`GL_SAMPLER`]
/// * [`GL_SHADER`]
/// * [`GL_TEXTURE`]
/// * [`GL_VERTEX_ARRAY`]
pub type ObjectIdentifier = GLenum;

/// PixelFormat
/// * [`GL_ALPHA`]
/// * [`GL_BGR`]
/// * [`GL_BGRA`]
/// * [`GL_BGRA_INTEGER`]
/// * [`GL_BGR_INTEGER`]
/// * [`GL_BLUE`]
/// * [`GL_BLUE_INTEGER`]
/// * [`GL_DEPTH_COMPONENT`]
/// * [`GL_DEPTH_STENCIL`]
/// * [`GL_GREEN`]
/// * [`GL_GREEN_INTEGER`]
/// * [`GL_RED`]
/// * [`GL_RED_INTEGER`]
/// * [`GL_RG`]
/// * [`GL_RGB`]
/// * [`GL_RGBA`]
/// * [`GL_RGBA_INTEGER`]
/// * [`GL_RGB_INTEGER`]
/// * [`GL_RG_INTEGER`]
/// * [`GL_STENCIL_INDEX`]
/// * [`GL_UNSIGNED_INT`]
/// * [`GL_UNSIGNED_SHORT`]
pub type PixelFormat = GLenum;

/// PixelStoreParameter
/// * [`GL_PACK_ALIGNMENT`]
/// * [`GL_PACK_IMAGE_HEIGHT`]
/// * [`GL_PACK_LSB_FIRST`]
/// * [`GL_PACK_ROW_LENGTH`]
/// * [`GL_PACK_SKIP_IMAGES`]
/// * [`GL_PACK_SKIP_PIXELS`]
/// * [`GL_PACK_SKIP_ROWS`]
/// * [`GL_PACK_SWAP_BYTES`]
/// * [`GL_UNPACK_ALIGNMENT`]
/// * [`GL_UNPACK_IMAGE_HEIGHT`]
/// * [`GL_UNPACK_LSB_FIRST`]
/// * [`GL_UNPACK_ROW_LENGTH`]
/// * [`GL_UNPACK_SKIP_IMAGES`]
/// * [`GL_UNPACK_SKIP_PIXELS`]
/// * [`GL_UNPACK_SKIP_ROWS`]
/// * [`GL_UNPACK_SWAP_BYTES`]
pub type PixelStoreParameter = GLenum;

/// PixelType
/// * [`GL_BYTE`]
/// * [`GL_FLOAT`]
/// * [`GL_INT`]
/// * [`GL_SHORT`]
/// * [`GL_UNSIGNED_BYTE`]
/// * [`GL_UNSIGNED_BYTE_3_3_2`]
/// * [`GL_UNSIGNED_INT`]
/// * [`GL_UNSIGNED_INT_10_10_10_2`]
/// * [`GL_UNSIGNED_INT_8_8_8_8`]
/// * [`GL_UNSIGNED_SHORT`]
/// * [`GL_UNSIGNED_SHORT_4_4_4_4`]
/// * [`GL_UNSIGNED_SHORT_5_5_5_1`]
pub type PixelType = GLenum;

/// PointParameterNameARB
/// * [`GL_POINT_FADE_THRESHOLD_SIZE`]
pub type PointParameterNameARB = GLenum;

/// PolygonMode
/// * [`GL_FILL`]
/// * [`GL_LINE`]
/// * [`GL_POINT`]
pub type PolygonMode = GLenum;

/// PrimitiveType
/// * [`GL_LINES`]
/// * [`GL_LINES_ADJACENCY`]
/// * [`GL_LINE_LOOP`]
/// * [`GL_LINE_STRIP`]
/// * [`GL_LINE_STRIP_ADJACENCY`]
/// * [`GL_POINTS`]
/// * [`GL_TRIANGLES`]
/// * [`GL_TRIANGLES_ADJACENCY`]
/// * [`GL_TRIANGLE_FAN`]
/// * [`GL_TRIANGLE_STRIP`]
/// * [`GL_TRIANGLE_STRIP_ADJACENCY`]
pub type PrimitiveType = GLenum;

/// ProgramPropertyARB
/// * [`GL_ACTIVE_ATTRIBUTES`]
/// * [`GL_ACTIVE_ATTRIBUTE_MAX_LENGTH`]
/// * [`GL_ACTIVE_UNIFORMS`]
/// * [`GL_ACTIVE_UNIFORM_BLOCKS`]
/// * [`GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH`]
/// * [`GL_ACTIVE_UNIFORM_MAX_LENGTH`]
/// * [`GL_ATTACHED_SHADERS`]
/// * [`GL_DELETE_STATUS`]
/// * [`GL_GEOMETRY_INPUT_TYPE`]
/// * [`GL_GEOMETRY_OUTPUT_TYPE`]
/// * [`GL_GEOMETRY_VERTICES_OUT`]
/// * [`GL_INFO_LOG_LENGTH`]
/// * [`GL_LINK_STATUS`]
/// * [`GL_TRANSFORM_FEEDBACK_BUFFER_MODE`]
/// * [`GL_TRANSFORM_FEEDBACK_VARYINGS`]
/// * [`GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH`]
/// * [`GL_VALIDATE_STATUS`]
pub type ProgramPropertyARB = GLenum;

/// QueryCounterTarget
/// * [`GL_TIMESTAMP`]
pub type QueryCounterTarget = GLenum;

/// QueryObjectParameterName
/// * [`GL_QUERY_RESULT`]
/// * [`GL_QUERY_RESULT_AVAILABLE`]
pub type QueryObjectParameterName = GLenum;

/// QueryParameterName
/// * [`GL_CURRENT_QUERY`]
/// * [`GL_QUERY_COUNTER_BITS`]
pub type QueryParameterName = GLenum;

/// QueryTarget
/// * [`GL_ANY_SAMPLES_PASSED`]
/// * [`GL_PRIMITIVES_GENERATED`]
/// * [`GL_SAMPLES_PASSED`]
/// * [`GL_TIME_ELAPSED`]
/// * [`GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN`]
pub type QueryTarget = GLenum;

/// ReadBufferMode
/// * [`GL_BACK`]
/// * [`GL_BACK_LEFT`]
/// * [`GL_BACK_RIGHT`]
/// * [`GL_COLOR_ATTACHMENT0`]
/// * [`GL_COLOR_ATTACHMENT1`]
/// * [`GL_COLOR_ATTACHMENT10`]
/// * [`GL_COLOR_ATTACHMENT11`]
/// * [`GL_COLOR_ATTACHMENT12`]
/// * [`GL_COLOR_ATTACHMENT13`]
/// * [`GL_COLOR_ATTACHMENT14`]
/// * [`GL_COLOR_ATTACHMENT15`]
/// * [`GL_COLOR_ATTACHMENT2`]
/// * [`GL_COLOR_ATTACHMENT3`]
/// * [`GL_COLOR_ATTACHMENT4`]
/// * [`GL_COLOR_ATTACHMENT5`]
/// * [`GL_COLOR_ATTACHMENT6`]
/// * [`GL_COLOR_ATTACHMENT7`]
/// * [`GL_COLOR_ATTACHMENT8`]
/// * [`GL_COLOR_ATTACHMENT9`]
/// * [`GL_FRONT`]
/// * [`GL_FRONT_LEFT`]
/// * [`GL_FRONT_RIGHT`]
/// * [`GL_LEFT`]
/// * [`GL_NONE`]
/// * [`GL_RIGHT`]
pub type ReadBufferMode = GLenum;

/// RenderbufferParameterName
/// * [`GL_RENDERBUFFER_ALPHA_SIZE`]
/// * [`GL_RENDERBUFFER_BLUE_SIZE`]
/// * [`GL_RENDERBUFFER_DEPTH_SIZE`]
/// * [`GL_RENDERBUFFER_GREEN_SIZE`]
/// * [`GL_RENDERBUFFER_HEIGHT`]
/// * [`GL_RENDERBUFFER_INTERNAL_FORMAT`]
/// * [`GL_RENDERBUFFER_RED_SIZE`]
/// * [`GL_RENDERBUFFER_SAMPLES`]
/// * [`GL_RENDERBUFFER_STENCIL_SIZE`]
/// * [`GL_RENDERBUFFER_WIDTH`]
pub type RenderbufferParameterName = GLenum;

/// RenderbufferTarget
/// * [`GL_RENDERBUFFER`]
pub type RenderbufferTarget = GLenum;

/// SamplerParameterF
/// * [`GL_TEXTURE_BORDER_COLOR`]
/// * [`GL_TEXTURE_LOD_BIAS`]
/// * [`GL_TEXTURE_MAX_ANISOTROPY`]
/// * [`GL_TEXTURE_MAX_LOD`]
/// * [`GL_TEXTURE_MIN_LOD`]
pub type SamplerParameterF = GLenum;

/// SamplerParameterI
/// * [`GL_TEXTURE_COMPARE_FUNC`]
/// * [`GL_TEXTURE_COMPARE_MODE`]
/// * [`GL_TEXTURE_MAG_FILTER`]
/// * [`GL_TEXTURE_MIN_FILTER`]
/// * [`GL_TEXTURE_WRAP_R`]
/// * [`GL_TEXTURE_WRAP_S`]
/// * [`GL_TEXTURE_WRAP_T`]
pub type SamplerParameterI = GLenum;

/// ShaderParameterName
/// * [`GL_COMPILE_STATUS`]
/// * [`GL_DELETE_STATUS`]
/// * [`GL_INFO_LOG_LENGTH`]
/// * [`GL_SHADER_SOURCE_LENGTH`]
/// * [`GL_SHADER_TYPE`]
pub type ShaderParameterName = GLenum;

/// ShaderType
/// * [`GL_FRAGMENT_SHADER`]
/// * [`GL_GEOMETRY_SHADER`]
/// * [`GL_VERTEX_SHADER`]
pub type ShaderType = GLenum;

/// StencilFaceDirection
/// * [`GL_BACK`]
/// * [`GL_FRONT`]
/// * [`GL_FRONT_AND_BACK`]
pub type StencilFaceDirection = GLenum;

/// StencilFunction
/// * [`GL_ALWAYS`]
/// * [`GL_EQUAL`]
/// * [`GL_GEQUAL`]
/// * [`GL_GREATER`]
/// * [`GL_LEQUAL`]
/// * [`GL_LESS`]
/// * [`GL_NEVER`]
/// * [`GL_NOTEQUAL`]
pub type StencilFunction = GLenum;

/// StencilOp
/// * [`GL_DECR`]
/// * [`GL_DECR_WRAP`]
/// * [`GL_INCR`]
/// * [`GL_INCR_WRAP`]
/// * [`GL_INVERT`]
/// * [`GL_KEEP`]
/// * [`GL_REPLACE`]
/// * [`GL_ZERO`]
pub type StencilOp = GLenum;

/// StringName
/// * [`GL_EXTENSIONS`]
/// * [`GL_RENDERER`]
/// * [`GL_SHADING_LANGUAGE_VERSION`]
/// * [`GL_VENDOR`]
/// * [`GL_VERSION`]
pub type StringName = GLenum;

/// SyncCondition
/// * [`GL_SYNC_GPU_COMMANDS_COMPLETE`]
pub type SyncCondition = GLenum;

/// SyncParameterName
/// * [`GL_OBJECT_TYPE`]
/// * [`GL_SYNC_CONDITION`]
/// * [`GL_SYNC_FLAGS`]
/// * [`GL_SYNC_STATUS`]
pub type SyncParameterName = GLenum;

/// SyncStatus
/// * [`GL_ALREADY_SIGNALED`]
/// * [`GL_CONDITION_SATISFIED`]
/// * [`GL_TIMEOUT_EXPIRED`]
/// * [`GL_WAIT_FAILED`]
pub type SyncStatus = GLenum;

/// TextureParameterName
/// * [`GL_TEXTURE_ALPHA_SIZE`]
/// * [`GL_TEXTURE_BASE_LEVEL`]
/// * [`GL_TEXTURE_BLUE_SIZE`]
/// * [`GL_TEXTURE_BORDER_COLOR`]
/// * [`GL_TEXTURE_COMPARE_FUNC`]
/// * [`GL_TEXTURE_COMPARE_MODE`]
/// * [`GL_TEXTURE_GREEN_SIZE`]
/// * [`GL_TEXTURE_HEIGHT`]
/// * [`GL_TEXTURE_INTERNAL_FORMAT`]
/// * [`GL_TEXTURE_LOD_BIAS`]
/// * [`GL_TEXTURE_MAG_FILTER`]
/// * [`GL_TEXTURE_MAX_LEVEL`]
/// * [`GL_TEXTURE_MAX_LOD`]
/// * [`GL_TEXTURE_MIN_FILTER`]
/// * [`GL_TEXTURE_MIN_LOD`]
/// * [`GL_TEXTURE_RED_SIZE`]
/// * [`GL_TEXTURE_SWIZZLE_A`]
/// * [`GL_TEXTURE_SWIZZLE_B`]
/// * [`GL_TEXTURE_SWIZZLE_G`]
/// * [`GL_TEXTURE_SWIZZLE_R`]
/// * [`GL_TEXTURE_SWIZZLE_RGBA`]
/// * [`GL_TEXTURE_WIDTH`]
/// * [`GL_TEXTURE_WRAP_R`]
/// * [`GL_TEXTURE_WRAP_S`]
/// * [`GL_TEXTURE_WRAP_T`]
pub type TextureParameterName = GLenum;

/// TextureTarget
/// * [`GL_PROXY_TEXTURE_1D`]
/// * [`GL_PROXY_TEXTURE_1D_ARRAY`]
/// * [`GL_PROXY_TEXTURE_2D`]
/// * [`GL_PROXY_TEXTURE_2D_ARRAY`]
/// * [`GL_PROXY_TEXTURE_2D_MULTISAMPLE`]
/// * [`GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY`]
/// * [`GL_PROXY_TEXTURE_3D`]
/// * [`GL_PROXY_TEXTURE_CUBE_MAP`]
/// * [`GL_PROXY_TEXTURE_RECTANGLE`]
/// * [`GL_TEXTURE_1D`]
/// * [`GL_TEXTURE_1D_ARRAY`]
/// * [`GL_TEXTURE_2D`]
/// * [`GL_TEXTURE_2D_ARRAY`]
/// * [`GL_TEXTURE_2D_MULTISAMPLE`]
/// * [`GL_TEXTURE_2D_MULTISAMPLE_ARRAY`]
/// * [`GL_TEXTURE_3D`]
/// * [`GL_TEXTURE_BUFFER`]
/// * [`GL_TEXTURE_CUBE_MAP`]
/// * [`GL_TEXTURE_CUBE_MAP_NEGATIVE_X`]
/// * [`GL_TEXTURE_CUBE_MAP_NEGATIVE_Y`]
/// * [`GL_TEXTURE_CUBE_MAP_NEGATIVE_Z`]
/// * [`GL_TEXTURE_CUBE_MAP_POSITIVE_X`]
/// * [`GL_TEXTURE_CUBE_MAP_POSITIVE_Y`]
/// * [`GL_TEXTURE_CUBE_MAP_POSITIVE_Z`]
/// * [`GL_TEXTURE_RECTANGLE`]
pub type TextureTarget = GLenum;

/// TextureUnit
/// * [`GL_TEXTURE0`]
/// * [`GL_TEXTURE1`]
/// * [`GL_TEXTURE10`]
/// * [`GL_TEXTURE11`]
/// * [`GL_TEXTURE12`]
/// * [`GL_TEXTURE13`]
/// * [`GL_TEXTURE14`]
/// * [`GL_TEXTURE15`]
/// * [`GL_TEXTURE16`]
/// * [`GL_TEXTURE17`]
/// * [`GL_TEXTURE18`]
/// * [`GL_TEXTURE19`]
/// * [`GL_TEXTURE2`]
/// * [`GL_TEXTURE20`]
/// * [`GL_TEXTURE21`]
/// * [`GL_TEXTURE22`]
/// * [`GL_TEXTURE23`]
/// * [`GL_TEXTURE24`]
/// * [`GL_TEXTURE25`]
/// * [`GL_TEXTURE26`]
/// * [`GL_TEXTURE27`]
/// * [`GL_TEXTURE28`]
/// * [`GL_TEXTURE29`]
/// * [`GL_TEXTURE3`]
/// * [`GL_TEXTURE30`]
/// * [`GL_TEXTURE31`]
/// * [`GL_TEXTURE4`]
/// * [`GL_TEXTURE5`]
/// * [`GL_TEXTURE6`]
/// * [`GL_TEXTURE7`]
/// * [`GL_TEXTURE8`]
/// * [`GL_TEXTURE9`]
pub type TextureUnit = GLenum;

/// TransformFeedbackBufferMode
/// * [`GL_INTERLEAVED_ATTRIBS`]
/// * [`GL_SEPARATE_ATTRIBS`]
pub type TransformFeedbackBufferMode = GLenum;

/// UniformBlockPName
/// * [`GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS`]
/// * [`GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES`]
/// * [`GL_UNIFORM_BLOCK_BINDING`]
/// * [`GL_UNIFORM_BLOCK_DATA_SIZE`]
/// * [`GL_UNIFORM_BLOCK_NAME_LENGTH`]
/// * [`GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER`]
/// * [`GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER`]
/// * [`GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER`]
pub type UniformBlockPName = GLenum;

/// UniformPName
/// * [`GL_UNIFORM_ARRAY_STRIDE`]
/// * [`GL_UNIFORM_BLOCK_INDEX`]
/// * [`GL_UNIFORM_IS_ROW_MAJOR`]
/// * [`GL_UNIFORM_MATRIX_STRIDE`]
/// * [`GL_UNIFORM_NAME_LENGTH`]
/// * [`GL_UNIFORM_OFFSET`]
/// * [`GL_UNIFORM_SIZE`]
/// * [`GL_UNIFORM_TYPE`]
pub type UniformPName = GLenum;

/// UniformType
/// * [`GL_BOOL`]
/// * [`GL_BOOL_VEC2`]
/// * [`GL_BOOL_VEC3`]
/// * [`GL_BOOL_VEC4`]
/// * [`GL_DOUBLE`]
/// * [`GL_FLOAT`]
/// * [`GL_FLOAT_MAT2`]
/// * [`GL_FLOAT_MAT2x3`]
/// * [`GL_FLOAT_MAT2x4`]
/// * [`GL_FLOAT_MAT3`]
/// * [`GL_FLOAT_MAT3x2`]
/// * [`GL_FLOAT_MAT3x4`]
/// * [`GL_FLOAT_MAT4`]
/// * [`GL_FLOAT_MAT4x2`]
/// * [`GL_FLOAT_MAT4x3`]
/// * [`GL_FLOAT_VEC2`]
/// * [`GL_FLOAT_VEC3`]
/// * [`GL_FLOAT_VEC4`]
/// * [`GL_INT`]
/// * [`GL_INT_SAMPLER_1D`]
/// * [`GL_INT_SAMPLER_1D_ARRAY`]
/// * [`GL_INT_SAMPLER_2D`]
/// * [`GL_INT_SAMPLER_2D_ARRAY`]
/// * [`GL_INT_SAMPLER_2D_MULTISAMPLE`]
/// * [`GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY`]
/// * [`GL_INT_SAMPLER_2D_RECT`]
/// * [`GL_INT_SAMPLER_3D`]
/// * [`GL_INT_SAMPLER_BUFFER`]
/// * [`GL_INT_SAMPLER_CUBE`]
/// * [`GL_INT_VEC2`]
/// * [`GL_INT_VEC3`]
/// * [`GL_INT_VEC4`]
/// * [`GL_SAMPLER_1D`]
/// * [`GL_SAMPLER_1D_ARRAY`]
/// * [`GL_SAMPLER_1D_ARRAY_SHADOW`]
/// * [`GL_SAMPLER_1D_SHADOW`]
/// * [`GL_SAMPLER_2D`]
/// * [`GL_SAMPLER_2D_ARRAY`]
/// * [`GL_SAMPLER_2D_ARRAY_SHADOW`]
/// * [`GL_SAMPLER_2D_MULTISAMPLE`]
/// * [`GL_SAMPLER_2D_MULTISAMPLE_ARRAY`]
/// * [`GL_SAMPLER_2D_RECT`]
/// * [`GL_SAMPLER_2D_RECT_SHADOW`]
/// * [`GL_SAMPLER_2D_SHADOW`]
/// * [`GL_SAMPLER_3D`]
/// * [`GL_SAMPLER_BUFFER`]
/// * [`GL_SAMPLER_CUBE`]
/// * [`GL_SAMPLER_CUBE_SHADOW`]
/// * [`GL_UNSIGNED_INT`]
/// * [`GL_UNSIGNED_INT_SAMPLER_1D`]
/// * [`GL_UNSIGNED_INT_SAMPLER_1D_ARRAY`]
/// * [`GL_UNSIGNED_INT_SAMPLER_2D`]
/// * [`GL_UNSIGNED_INT_SAMPLER_2D_ARRAY`]
/// * [`GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE`]
/// * [`GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY`]
/// * [`GL_UNSIGNED_INT_SAMPLER_2D_RECT`]
/// * [`GL_UNSIGNED_INT_SAMPLER_3D`]
/// * [`GL_UNSIGNED_INT_SAMPLER_BUFFER`]
/// * [`GL_UNSIGNED_INT_SAMPLER_CUBE`]
/// * [`GL_UNSIGNED_INT_VEC2`]
/// * [`GL_UNSIGNED_INT_VEC3`]
/// * [`GL_UNSIGNED_INT_VEC4`]
pub type UniformType = GLenum;

/// VertexAttribEnum
/// * [`GL_CURRENT_VERTEX_ATTRIB`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_DIVISOR`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_ENABLED`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_INTEGER`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_NORMALIZED`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_SIZE`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_STRIDE`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_TYPE`]
pub type VertexAttribEnum = GLenum;

/// VertexAttribIType
/// * [`GL_BYTE`]
/// * [`GL_INT`]
/// * [`GL_SHORT`]
/// * [`GL_UNSIGNED_BYTE`]
/// * [`GL_UNSIGNED_INT`]
/// * [`GL_UNSIGNED_SHORT`]
pub type VertexAttribIType = GLenum;

/// VertexAttribPointerPropertyARB
/// * [`GL_VERTEX_ATTRIB_ARRAY_POINTER`]
pub type VertexAttribPointerPropertyARB = GLenum;

/// VertexAttribPointerType
/// * [`GL_BYTE`]
/// * [`GL_DOUBLE`]
/// * [`GL_FLOAT`]
/// * [`GL_HALF_FLOAT`]
/// * [`GL_INT`]
/// * [`GL_INT_2_10_10_10_REV`]
/// * [`GL_SHORT`]
/// * [`GL_UNSIGNED_BYTE`]
/// * [`GL_UNSIGNED_INT`]
/// * [`GL_UNSIGNED_INT_10F_11F_11F_REV`]
/// * [`GL_UNSIGNED_INT_2_10_10_10_REV`]
/// * [`GL_UNSIGNED_SHORT`]
pub type VertexAttribPointerType = GLenum;

/// VertexAttribPropertyARB
/// * [`GL_CURRENT_VERTEX_ATTRIB`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_DIVISOR`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_ENABLED`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_INTEGER`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_NORMALIZED`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_SIZE`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_STRIDE`]
/// * [`GL_VERTEX_ATTRIB_ARRAY_TYPE`]
pub type VertexAttribPropertyARB = GLenum;

/// VertexProvokingMode
/// * [`GL_FIRST_VERTEX_CONVENTION`]
/// * [`GL_LAST_VERTEX_CONVENTION`]
pub type VertexProvokingMode = GLenum;
