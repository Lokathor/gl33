use super::*;

pub const GL_ACTIVE_ATTRIBUTES: GLenum = GLenum(0x8B89);

pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = GLenum(0x8B8A);

pub const GL_ACTIVE_TEXTURE: GLenum = GLenum(0x84E0);

pub const GL_ACTIVE_UNIFORMS: GLenum = GLenum(0x8B86);

pub const GL_ACTIVE_UNIFORM_BLOCKS: GLenum = GLenum(0x8A36);

pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = GLenum(0x8A35);

pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: GLenum = GLenum(0x8B87);

pub const GL_ALIASED_LINE_WIDTH_RANGE: GLenum = GLenum(0x846E);

pub const GL_ALPHA: GLenum = GLenum(0x1906);

pub const GL_ALREADY_SIGNALED: GLenum = GLenum(0x911A);

pub const GL_ALWAYS: GLenum = GLenum(0x0207);

pub const GL_AND: GLenum = GLenum(0x1501);

pub const GL_AND_INVERTED: GLenum = GLenum(0x1504);

pub const GL_AND_REVERSE: GLenum = GLenum(0x1502);

pub const GL_ANY_SAMPLES_PASSED: GLenum = GLenum(0x8C2F);

pub const GL_ARRAY_BUFFER: GLenum = GLenum(0x8892);

pub const GL_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x8894);

pub const GL_ATTACHED_SHADERS: GLenum = GLenum(0x8B85);

pub const GL_BACK: GLenum = GLenum(0x0405);

pub const GL_BACK_LEFT: GLenum = GLenum(0x0402);

pub const GL_BACK_RIGHT: GLenum = GLenum(0x0403);

pub const GL_BGR: GLenum = GLenum(0x80E0);

pub const GL_BGRA: GLenum = GLenum(0x80E1);

pub const GL_BGRA_INTEGER: GLenum = GLenum(0x8D9B);

pub const GL_BGR_INTEGER: GLenum = GLenum(0x8D9A);

pub const GL_BLEND: GLenum = GLenum(0x0BE2);

pub const GL_BLEND_COLOR: GLenum = GLenum(0x8005);

pub const GL_BLEND_DST: GLenum = GLenum(0x0BE0);

pub const GL_BLEND_DST_ALPHA: GLenum = GLenum(0x80CA);

pub const GL_BLEND_DST_RGB: GLenum = GLenum(0x80C8);

pub const GL_BLEND_EQUATION: GLenum = GLenum(0x8009);

pub const GL_BLEND_EQUATION_ALPHA: GLenum = GLenum(0x883D);

pub const GL_BLEND_EQUATION_RGB: GLenum = GLenum(0x8009);

pub const GL_BLEND_SRC: GLenum = GLenum(0x0BE1);

pub const GL_BLEND_SRC_ALPHA: GLenum = GLenum(0x80CB);

pub const GL_BLEND_SRC_RGB: GLenum = GLenum(0x80C9);

pub const GL_BLUE: GLenum = GLenum(0x1905);

pub const GL_BLUE_INTEGER: GLenum = GLenum(0x8D96);

pub const GL_BOOL: GLenum = GLenum(0x8B56);

pub const GL_BOOL_VEC2: GLenum = GLenum(0x8B57);

pub const GL_BOOL_VEC3: GLenum = GLenum(0x8B58);

pub const GL_BOOL_VEC4: GLenum = GLenum(0x8B59);

pub const GL_BUFFER: GLenum = GLenum(0x82E0);

pub const GL_BUFFER_ACCESS: GLenum = GLenum(0x88BB);

pub const GL_BUFFER_ACCESS_FLAGS: GLenum = GLenum(0x911F);

pub const GL_BUFFER_MAPPED: GLenum = GLenum(0x88BC);

pub const GL_BUFFER_MAP_LENGTH: GLenum = GLenum(0x9120);

pub const GL_BUFFER_MAP_OFFSET: GLenum = GLenum(0x9121);

pub const GL_BUFFER_MAP_POINTER: GLenum = GLenum(0x88BD);

pub const GL_BUFFER_SIZE: GLenum = GLenum(0x8764);

pub const GL_BUFFER_USAGE: GLenum = GLenum(0x8765);

pub const GL_BYTE: GLenum = GLenum(0x1400);

pub const GL_CCW: GLenum = GLenum(0x0901);

pub const GL_CLAMP_READ_COLOR: GLenum = GLenum(0x891C);

pub const GL_CLAMP_TO_BORDER: GLenum = GLenum(0x812D);

pub const GL_CLAMP_TO_EDGE: GLenum = GLenum(0x812F);

pub const GL_CLEAR: GLenum = GLenum(0x1500);

pub const GL_CLIPPING_INPUT_PRIMITIVES_ARB: GLenum = GLenum(0x82F6);

pub const GL_CLIPPING_OUTPUT_PRIMITIVES_ARB: GLenum = GLenum(0x82F7);

pub const GL_CLIP_DISTANCE0: GLenum = GLenum(0x3000);

pub const GL_CLIP_DISTANCE1: GLenum = GLenum(0x3001);

pub const GL_CLIP_DISTANCE2: GLenum = GLenum(0x3002);

pub const GL_CLIP_DISTANCE3: GLenum = GLenum(0x3003);

pub const GL_CLIP_DISTANCE4: GLenum = GLenum(0x3004);

pub const GL_CLIP_DISTANCE5: GLenum = GLenum(0x3005);

pub const GL_CLIP_DISTANCE6: GLenum = GLenum(0x3006);

pub const GL_CLIP_DISTANCE7: GLenum = GLenum(0x3007);

pub const GL_COLOR: GLenum = GLenum(0x1800);

pub const GL_COLOR_ATTACHMENT0: GLenum = GLenum(0x8CE0);

pub const GL_COLOR_ATTACHMENT1: GLenum = GLenum(0x8CE1);

pub const GL_COLOR_ATTACHMENT10: GLenum = GLenum(0x8CEA);

pub const GL_COLOR_ATTACHMENT11: GLenum = GLenum(0x8CEB);

pub const GL_COLOR_ATTACHMENT12: GLenum = GLenum(0x8CEC);

pub const GL_COLOR_ATTACHMENT13: GLenum = GLenum(0x8CED);

pub const GL_COLOR_ATTACHMENT14: GLenum = GLenum(0x8CEE);

pub const GL_COLOR_ATTACHMENT15: GLenum = GLenum(0x8CEF);

pub const GL_COLOR_ATTACHMENT16: GLenum = GLenum(0x8CF0);

pub const GL_COLOR_ATTACHMENT17: GLenum = GLenum(0x8CF1);

pub const GL_COLOR_ATTACHMENT18: GLenum = GLenum(0x8CF2);

pub const GL_COLOR_ATTACHMENT19: GLenum = GLenum(0x8CF3);

pub const GL_COLOR_ATTACHMENT2: GLenum = GLenum(0x8CE2);

pub const GL_COLOR_ATTACHMENT20: GLenum = GLenum(0x8CF4);

pub const GL_COLOR_ATTACHMENT21: GLenum = GLenum(0x8CF5);

pub const GL_COLOR_ATTACHMENT22: GLenum = GLenum(0x8CF6);

pub const GL_COLOR_ATTACHMENT23: GLenum = GLenum(0x8CF7);

pub const GL_COLOR_ATTACHMENT24: GLenum = GLenum(0x8CF8);

pub const GL_COLOR_ATTACHMENT25: GLenum = GLenum(0x8CF9);

pub const GL_COLOR_ATTACHMENT26: GLenum = GLenum(0x8CFA);

pub const GL_COLOR_ATTACHMENT27: GLenum = GLenum(0x8CFB);

pub const GL_COLOR_ATTACHMENT28: GLenum = GLenum(0x8CFC);

pub const GL_COLOR_ATTACHMENT29: GLenum = GLenum(0x8CFD);

pub const GL_COLOR_ATTACHMENT3: GLenum = GLenum(0x8CE3);

pub const GL_COLOR_ATTACHMENT30: GLenum = GLenum(0x8CFE);

pub const GL_COLOR_ATTACHMENT31: GLenum = GLenum(0x8CFF);

pub const GL_COLOR_ATTACHMENT4: GLenum = GLenum(0x8CE4);

pub const GL_COLOR_ATTACHMENT5: GLenum = GLenum(0x8CE5);

pub const GL_COLOR_ATTACHMENT6: GLenum = GLenum(0x8CE6);

pub const GL_COLOR_ATTACHMENT7: GLenum = GLenum(0x8CE7);

pub const GL_COLOR_ATTACHMENT8: GLenum = GLenum(0x8CE8);

pub const GL_COLOR_ATTACHMENT9: GLenum = GLenum(0x8CE9);

pub const GL_COLOR_BUFFER_BIT: GLbitfield = GLbitfield(0x00004000);

pub const GL_COLOR_CLEAR_VALUE: GLenum = GLenum(0x0C22);

pub const GL_COLOR_LOGIC_OP: GLenum = GLenum(0x0BF2);

pub const GL_COLOR_WRITEMASK: GLenum = GLenum(0x0C23);

pub const GL_COMPARE_REF_TO_TEXTURE: GLenum = GLenum(0x884E);

pub const GL_COMPILE_STATUS: GLenum = GLenum(0x8B81);

pub const GL_COMPRESSED_RED: GLenum = GLenum(0x8225);

pub const GL_COMPRESSED_RED_RGTC1: GLenum = GLenum(0x8DBB);

pub const GL_COMPRESSED_RG: GLenum = GLenum(0x8226);

pub const GL_COMPRESSED_RGB: GLenum = GLenum(0x84ED);

pub const GL_COMPRESSED_RGBA: GLenum = GLenum(0x84EE);

pub const GL_COMPRESSED_RG_RGTC2: GLenum = GLenum(0x8DBD);

pub const GL_COMPRESSED_SIGNED_RED_RGTC1: GLenum = GLenum(0x8DBC);

pub const GL_COMPRESSED_SIGNED_RG_RGTC2: GLenum = GLenum(0x8DBE);

pub const GL_COMPRESSED_SRGB: GLenum = GLenum(0x8C48);

pub const GL_COMPRESSED_SRGB_ALPHA: GLenum = GLenum(0x8C49);

pub const GL_COMPRESSED_TEXTURE_FORMATS: GLenum = GLenum(0x86A3);

pub const GL_COMPUTE_SHADER_INVOCATIONS_ARB: GLenum = GLenum(0x82F5);

pub const GL_CONDITION_SATISFIED: GLenum = GLenum(0x911C);

pub const GL_CONSTANT_ALPHA: GLenum = GLenum(0x8003);

pub const GL_CONSTANT_COLOR: GLenum = GLenum(0x8001);

pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: GLbitfield = GLbitfield(0x00000002);

pub const GL_CONTEXT_CORE_PROFILE_BIT: GLbitfield = GLbitfield(0x00000001);

pub const GL_CONTEXT_FLAGS: GLenum = GLenum(0x821E);

pub const GL_CONTEXT_FLAG_DEBUG_BIT: GLbitfield = GLbitfield(0x00000002);

pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLbitfield = GLbitfield(0x00000001);

pub const GL_CONTEXT_PROFILE_MASK: GLenum = GLenum(0x9126);

pub const GL_COPY: GLenum = GLenum(0x1503);

pub const GL_COPY_INVERTED: GLenum = GLenum(0x150C);

pub const GL_COPY_READ_BUFFER: GLenum = GLenum(0x8F36);

pub const GL_COPY_WRITE_BUFFER: GLenum = GLenum(0x8F37);

pub const GL_CULL_FACE: GLenum = GLenum(0x0B44);

pub const GL_CULL_FACE_MODE: GLenum = GLenum(0x0B45);

pub const GL_CURRENT_PROGRAM: GLenum = GLenum(0x8B8D);

pub const GL_CURRENT_QUERY: GLenum = GLenum(0x8865);

pub const GL_CURRENT_VERTEX_ATTRIB: GLenum = GLenum(0x8626);

pub const GL_CW: GLenum = GLenum(0x0900);

pub const GL_DEBUG_CALLBACK_FUNCTION: GLenum = GLenum(0x8244);

pub const GL_DEBUG_CALLBACK_USER_PARAM: GLenum = GLenum(0x8245);

pub const GL_DEBUG_GROUP_STACK_DEPTH: GLenum = GLenum(0x826D);

pub const GL_DEBUG_LOGGED_MESSAGES: GLenum = GLenum(0x9145);

pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = GLenum(0x8243);

pub const GL_DEBUG_OUTPUT: GLenum = GLenum(0x92E0);

pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: GLenum = GLenum(0x8242);

pub const GL_DEBUG_SEVERITY_HIGH: GLenum = GLenum(0x9146);

pub const GL_DEBUG_SEVERITY_LOW: GLenum = GLenum(0x9148);

pub const GL_DEBUG_SEVERITY_MEDIUM: GLenum = GLenum(0x9147);

pub const GL_DEBUG_SEVERITY_NOTIFICATION: GLenum = GLenum(0x826B);

pub const GL_DEBUG_SOURCE_API: GLenum = GLenum(0x8246);

pub const GL_DEBUG_SOURCE_APPLICATION: GLenum = GLenum(0x824A);

pub const GL_DEBUG_SOURCE_OTHER: GLenum = GLenum(0x824B);

pub const GL_DEBUG_SOURCE_SHADER_COMPILER: GLenum = GLenum(0x8248);

pub const GL_DEBUG_SOURCE_THIRD_PARTY: GLenum = GLenum(0x8249);

pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = GLenum(0x8247);

pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = GLenum(0x824D);

pub const GL_DEBUG_TYPE_ERROR: GLenum = GLenum(0x824C);

pub const GL_DEBUG_TYPE_MARKER: GLenum = GLenum(0x8268);

pub const GL_DEBUG_TYPE_OTHER: GLenum = GLenum(0x8251);

pub const GL_DEBUG_TYPE_PERFORMANCE: GLenum = GLenum(0x8250);

pub const GL_DEBUG_TYPE_POP_GROUP: GLenum = GLenum(0x826A);

pub const GL_DEBUG_TYPE_PORTABILITY: GLenum = GLenum(0x824F);

pub const GL_DEBUG_TYPE_PUSH_GROUP: GLenum = GLenum(0x8269);

pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = GLenum(0x824E);

pub const GL_DECR: GLenum = GLenum(0x1E03);

pub const GL_DECR_WRAP: GLenum = GLenum(0x8508);

pub const GL_DELETE_STATUS: GLenum = GLenum(0x8B80);

pub const GL_DEPTH: GLenum = GLenum(0x1801);

pub const GL_DEPTH24_STENCIL8: GLenum = GLenum(0x88F0);

pub const GL_DEPTH32F_STENCIL8: GLenum = GLenum(0x8CAD);

pub const GL_DEPTH_ATTACHMENT: GLenum = GLenum(0x8D00);

pub const GL_DEPTH_BUFFER_BIT: GLbitfield = GLbitfield(0x00000100);

pub const GL_DEPTH_CLAMP: GLenum = GLenum(0x864F);

pub const GL_DEPTH_CLEAR_VALUE: GLenum = GLenum(0x0B73);

pub const GL_DEPTH_COMPONENT: GLenum = GLenum(0x1902);

pub const GL_DEPTH_COMPONENT16: GLenum = GLenum(0x81A5);

pub const GL_DEPTH_COMPONENT24: GLenum = GLenum(0x81A6);

pub const GL_DEPTH_COMPONENT32: GLenum = GLenum(0x81A7);

pub const GL_DEPTH_COMPONENT32F: GLenum = GLenum(0x8CAC);

pub const GL_DEPTH_FUNC: GLenum = GLenum(0x0B74);

pub const GL_DEPTH_RANGE: GLenum = GLenum(0x0B70);

pub const GL_DEPTH_STENCIL: GLenum = GLenum(0x84F9);

pub const GL_DEPTH_STENCIL_ATTACHMENT: GLenum = GLenum(0x821A);

pub const GL_DEPTH_TEST: GLenum = GLenum(0x0B71);

pub const GL_DEPTH_WRITEMASK: GLenum = GLenum(0x0B72);

pub const GL_DITHER: GLenum = GLenum(0x0BD0);

pub const GL_DONT_CARE: GLenum = GLenum(0x1100);

pub const GL_DOUBLE: GLenum = GLenum(0x140A);

pub const GL_DOUBLEBUFFER: GLenum = GLenum(0x0C32);

pub const GL_DRAW_BUFFER: GLenum = GLenum(0x0C01);

pub const GL_DRAW_BUFFER0: GLenum = GLenum(0x8825);

pub const GL_DRAW_BUFFER1: GLenum = GLenum(0x8826);

pub const GL_DRAW_BUFFER10: GLenum = GLenum(0x882F);

pub const GL_DRAW_BUFFER11: GLenum = GLenum(0x8830);

pub const GL_DRAW_BUFFER12: GLenum = GLenum(0x8831);

pub const GL_DRAW_BUFFER13: GLenum = GLenum(0x8832);

pub const GL_DRAW_BUFFER14: GLenum = GLenum(0x8833);

pub const GL_DRAW_BUFFER15: GLenum = GLenum(0x8834);

pub const GL_DRAW_BUFFER2: GLenum = GLenum(0x8827);

pub const GL_DRAW_BUFFER3: GLenum = GLenum(0x8828);

pub const GL_DRAW_BUFFER4: GLenum = GLenum(0x8829);

pub const GL_DRAW_BUFFER5: GLenum = GLenum(0x882A);

pub const GL_DRAW_BUFFER6: GLenum = GLenum(0x882B);

pub const GL_DRAW_BUFFER7: GLenum = GLenum(0x882C);

pub const GL_DRAW_BUFFER8: GLenum = GLenum(0x882D);

pub const GL_DRAW_BUFFER9: GLenum = GLenum(0x882E);

pub const GL_DRAW_FRAMEBUFFER: GLenum = GLenum(0x8CA9);

pub const GL_DRAW_FRAMEBUFFER_BINDING: GLenum = GLenum(0x8CA6);

pub const GL_DST_ALPHA: GLenum = GLenum(0x0304);

pub const GL_DST_COLOR: GLenum = GLenum(0x0306);

pub const GL_DYNAMIC_COPY: GLenum = GLenum(0x88EA);

pub const GL_DYNAMIC_DRAW: GLenum = GLenum(0x88E8);

pub const GL_DYNAMIC_READ: GLenum = GLenum(0x88E9);

pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = GLenum(0x8893);

pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x8895);

pub const GL_EQUAL: GLenum = GLenum(0x0202);

pub const GL_EQUIV: GLenum = GLenum(0x1509);

pub const GL_EXTENSIONS: GLenum = GLenum(0x1F03);

pub const GL_FALSE: GLenum = GLenum(0);

pub const GL_FASTEST: GLenum = GLenum(0x1101);

pub const GL_FILL: GLenum = GLenum(0x1B02);

pub const GL_FIRST_VERTEX_CONVENTION: GLenum = GLenum(0x8E4D);

pub const GL_FIXED_ONLY: GLenum = GLenum(0x891D);

pub const GL_FLOAT: GLenum = GLenum(0x1406);

pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = GLenum(0x8DAD);

pub const GL_FLOAT_MAT2: GLenum = GLenum(0x8B5A);

pub const GL_FLOAT_MAT2x3: GLenum = GLenum(0x8B65);

pub const GL_FLOAT_MAT2x4: GLenum = GLenum(0x8B66);

pub const GL_FLOAT_MAT3: GLenum = GLenum(0x8B5B);

pub const GL_FLOAT_MAT3x2: GLenum = GLenum(0x8B67);

pub const GL_FLOAT_MAT3x4: GLenum = GLenum(0x8B68);

pub const GL_FLOAT_MAT4: GLenum = GLenum(0x8B5C);

pub const GL_FLOAT_MAT4x2: GLenum = GLenum(0x8B69);

pub const GL_FLOAT_MAT4x3: GLenum = GLenum(0x8B6A);

pub const GL_FLOAT_VEC2: GLenum = GLenum(0x8B50);

pub const GL_FLOAT_VEC3: GLenum = GLenum(0x8B51);

pub const GL_FLOAT_VEC4: GLenum = GLenum(0x8B52);

pub const GL_FRAGMENT_SHADER: GLenum = GLenum(0x8B30);

pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = GLenum(0x8B8B);

pub const GL_FRAGMENT_SHADER_INVOCATIONS_ARB: GLenum = GLenum(0x82F4);

pub const GL_FRAMEBUFFER: GLenum = GLenum(0x8D40);

pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = GLenum(0x8215);

pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = GLenum(0x8214);

pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = GLenum(0x8210);

pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = GLenum(0x8211);

pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = GLenum(0x8216);

pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = GLenum(0x8213);

pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = GLenum(0x8DA7);

pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = GLenum(0x8CD1);

pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = GLenum(0x8CD0);

pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = GLenum(0x8212);

pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = GLenum(0x8217);

pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = GLenum(0x8CD3);

pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = GLenum(0x8CD4);

pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = GLenum(0x8CD2);

pub const GL_FRAMEBUFFER_BINDING: GLenum = GLenum(0x8CA6);

pub const GL_FRAMEBUFFER_COMPLETE: GLenum = GLenum(0x8CD5);

pub const GL_FRAMEBUFFER_DEFAULT: GLenum = GLenum(0x8218);

pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = GLenum(0x8CD6);

pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = GLenum(0x8CDB);

pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = GLenum(0x8DA8);

pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = GLenum(0x8CD7);

pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = GLenum(0x8D56);

pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = GLenum(0x8CDC);

pub const GL_FRAMEBUFFER_SRGB: GLenum = GLenum(0x8DB9);

pub const GL_FRAMEBUFFER_UNDEFINED: GLenum = GLenum(0x8219);

pub const GL_FRAMEBUFFER_UNSUPPORTED: GLenum = GLenum(0x8CDD);

pub const GL_FRONT: GLenum = GLenum(0x0404);

pub const GL_FRONT_AND_BACK: GLenum = GLenum(0x0408);

pub const GL_FRONT_FACE: GLenum = GLenum(0x0B46);

pub const GL_FRONT_LEFT: GLenum = GLenum(0x0400);

pub const GL_FRONT_RIGHT: GLenum = GLenum(0x0401);

pub const GL_FUNC_ADD: GLenum = GLenum(0x8006);

pub const GL_FUNC_REVERSE_SUBTRACT: GLenum = GLenum(0x800B);

pub const GL_FUNC_SUBTRACT: GLenum = GLenum(0x800A);

pub const GL_GEOMETRY_INPUT_TYPE: GLenum = GLenum(0x8917);

pub const GL_GEOMETRY_OUTPUT_TYPE: GLenum = GLenum(0x8918);

pub const GL_GEOMETRY_SHADER: GLenum = GLenum(0x8DD9);

pub const GL_GEOMETRY_SHADER_INVOCATIONS: GLenum = GLenum(0x887F);

pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED_ARB: GLenum = GLenum(0x82F3);

pub const GL_GEOMETRY_VERTICES_OUT: GLenum = GLenum(0x8916);

pub const GL_GEQUAL: GLenum = GLenum(0x0206);

pub const GL_GREATER: GLenum = GLenum(0x0204);

pub const GL_GREEN: GLenum = GLenum(0x1904);

pub const GL_GREEN_INTEGER: GLenum = GLenum(0x8D95);

pub const GL_HALF_FLOAT: GLenum = GLenum(0x140B);

pub const GL_INCR: GLenum = GLenum(0x1E02);

pub const GL_INCR_WRAP: GLenum = GLenum(0x8507);

pub const GL_INFO_LOG_LENGTH: GLenum = GLenum(0x8B84);

pub const GL_INT: GLenum = GLenum(0x1404);

pub const GL_INTERLEAVED_ATTRIBS: GLenum = GLenum(0x8C8C);

pub const GL_INT_2_10_10_10_REV: GLenum = GLenum(0x8D9F);

pub const GL_INT_SAMPLER_1D: GLenum = GLenum(0x8DC9);

pub const GL_INT_SAMPLER_1D_ARRAY: GLenum = GLenum(0x8DCE);

pub const GL_INT_SAMPLER_2D: GLenum = GLenum(0x8DCA);

pub const GL_INT_SAMPLER_2D_ARRAY: GLenum = GLenum(0x8DCF);

pub const GL_INT_SAMPLER_2D_MULTISAMPLE: GLenum = GLenum(0x9109);

pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x910C);

pub const GL_INT_SAMPLER_2D_RECT: GLenum = GLenum(0x8DCD);

pub const GL_INT_SAMPLER_3D: GLenum = GLenum(0x8DCB);

pub const GL_INT_SAMPLER_BUFFER: GLenum = GLenum(0x8DD0);

pub const GL_INT_SAMPLER_CUBE: GLenum = GLenum(0x8DCC);

pub const GL_INT_VEC2: GLenum = GLenum(0x8B53);

pub const GL_INT_VEC3: GLenum = GLenum(0x8B54);

pub const GL_INT_VEC4: GLenum = GLenum(0x8B55);

pub const GL_INVALID_ENUM: GLenum = GLenum(0x0500);

pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = GLenum(0x0506);

pub const GL_INVALID_INDEX: c_uint = 0xFFFFFFFF;

pub const GL_INVALID_OPERATION: GLenum = GLenum(0x0502);

pub const GL_INVALID_VALUE: GLenum = GLenum(0x0501);

pub const GL_INVERT: GLenum = GLenum(0x150A);

pub const GL_KEEP: GLenum = GLenum(0x1E00);

pub const GL_LAST_VERTEX_CONVENTION: GLenum = GLenum(0x8E4E);

pub const GL_LEFT: GLenum = GLenum(0x0406);

pub const GL_LEQUAL: GLenum = GLenum(0x0203);

pub const GL_LESS: GLenum = GLenum(0x0201);

pub const GL_LINE: GLenum = GLenum(0x1B01);

pub const GL_LINEAR: GLenum = GLenum(0x2601);

pub const GL_LINEAR_MIPMAP_LINEAR: GLenum = GLenum(0x2703);

pub const GL_LINEAR_MIPMAP_NEAREST: GLenum = GLenum(0x2701);

pub const GL_LINES: GLenum = GLenum(0x0001);

pub const GL_LINES_ADJACENCY: GLenum = GLenum(0x000A);

pub const GL_LINE_LOOP: GLenum = GLenum(0x0002);

pub const GL_LINE_SMOOTH: GLenum = GLenum(0x0B20);

pub const GL_LINE_SMOOTH_HINT: GLenum = GLenum(0x0C52);

pub const GL_LINE_STRIP: GLenum = GLenum(0x0003);

pub const GL_LINE_STRIP_ADJACENCY: GLenum = GLenum(0x000B);

pub const GL_LINE_WIDTH: GLenum = GLenum(0x0B21);

pub const GL_LINE_WIDTH_GRANULARITY: GLenum = GLenum(0x0B23);

pub const GL_LINE_WIDTH_RANGE: GLenum = GLenum(0x0B22);

pub const GL_LINK_STATUS: GLenum = GLenum(0x8B82);

pub const GL_LOGIC_OP_MODE: GLenum = GLenum(0x0BF0);

pub const GL_LOWER_LEFT: GLenum = GLenum(0x8CA1);

pub const GL_MAJOR_VERSION: GLenum = GLenum(0x821B);

pub const GL_MAP_FLUSH_EXPLICIT_BIT: GLbitfield = GLbitfield(0x0010);

pub const GL_MAP_INVALIDATE_BUFFER_BIT: GLbitfield = GLbitfield(0x0008);

pub const GL_MAP_INVALIDATE_RANGE_BIT: GLbitfield = GLbitfield(0x0004);

pub const GL_MAP_READ_BIT: GLbitfield = GLbitfield(0x0001);

pub const GL_MAP_UNSYNCHRONIZED_BIT: GLbitfield = GLbitfield(0x0020);

pub const GL_MAP_WRITE_BIT: GLbitfield = GLbitfield(0x0002);

pub const GL_MAX: GLenum = GLenum(0x8008);

pub const GL_MAX_3D_TEXTURE_SIZE: GLenum = GLenum(0x8073);

pub const GL_MAX_ARRAY_TEXTURE_LAYERS: GLenum = GLenum(0x88FF);

pub const GL_MAX_CLIP_DISTANCES: GLenum = GLenum(0x0D32);

pub const GL_MAX_COLOR_ATTACHMENTS: GLenum = GLenum(0x8CDF);

pub const GL_MAX_COLOR_TEXTURE_SAMPLES: GLenum = GLenum(0x910E);

pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = GLenum(0x8A33);

pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = GLenum(0x8A32);

pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8B4D);

pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: GLenum = GLenum(0x8A2E);

pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = GLenum(0x8A31);

pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = GLenum(0x851C);

pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = GLenum(0x826C);

pub const GL_MAX_DEBUG_LOGGED_MESSAGES: GLenum = GLenum(0x9144);

pub const GL_MAX_DEBUG_MESSAGE_LENGTH: GLenum = GLenum(0x9143);

pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: GLenum = GLenum(0x910F);

pub const GL_MAX_DRAW_BUFFERS: GLenum = GLenum(0x8824);

pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = GLenum(0x88FC);

pub const GL_MAX_ELEMENTS_INDICES: GLenum = GLenum(0x80E9);

pub const GL_MAX_ELEMENTS_VERTICES: GLenum = GLenum(0x80E8);

pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = GLenum(0x9125);

pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = GLenum(0x8A2D);

pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = GLenum(0x8B49);

pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = GLenum(0x9123);

pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = GLenum(0x9124);

pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = GLenum(0x8DE0);

pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8C29);

pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = GLenum(0x8DE1);

pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = GLenum(0x8A2C);

pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = GLenum(0x8DDF);

pub const GL_MAX_INTEGER_SAMPLES: GLenum = GLenum(0x9110);

pub const GL_MAX_LABEL_LENGTH: GLenum = GLenum(0x82E8);

pub const GL_MAX_PROGRAM_TEXEL_OFFSET: GLenum = GLenum(0x8905);

pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: GLenum = GLenum(0x84F8);

pub const GL_MAX_RENDERBUFFER_SIZE: GLenum = GLenum(0x84E8);

pub const GL_MAX_SAMPLES: GLenum = GLenum(0x8D57);

pub const GL_MAX_SAMPLE_MASK_WORDS: GLenum = GLenum(0x8E59);

pub const GL_MAX_SERVER_WAIT_TIMEOUT: GLenum = GLenum(0x9111);

pub const GL_MAX_TEXTURE_BUFFER_SIZE: GLenum = GLenum(0x8C2B);

pub const GL_MAX_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8872);

pub const GL_MAX_TEXTURE_LOD_BIAS: GLenum = GLenum(0x84FD);

pub const GL_MAX_TEXTURE_MAX_ANISOTROPY: GLenum = GLenum(0x84FF);

pub const GL_MAX_TEXTURE_SIZE: GLenum = GLenum(0x0D33);

pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = GLenum(0x8C8A);

pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = GLenum(0x8C8B);

pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = GLenum(0x8C80);

pub const GL_MAX_UNIFORM_BLOCK_SIZE: GLenum = GLenum(0x8A30);

pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: GLenum = GLenum(0x8A2F);

pub const GL_MAX_VARYING_COMPONENTS: GLenum = GLenum(0x8B4B);

pub const GL_MAX_VARYING_FLOATS: GLenum = GLenum(0x8B4B);

pub const GL_MAX_VERTEX_ATTRIBS: GLenum = GLenum(0x8869);

pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = GLenum(0x9122);

pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8B4C);

pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: GLenum = GLenum(0x8A2B);

pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = GLenum(0x8B4A);

pub const GL_MAX_VIEWPORT_DIMS: GLenum = GLenum(0x0D3A);

pub const GL_MIN: GLenum = GLenum(0x8007);

pub const GL_MINOR_VERSION: GLenum = GLenum(0x821C);

pub const GL_MIN_PROGRAM_TEXEL_OFFSET: GLenum = GLenum(0x8904);

pub const GL_MIRRORED_REPEAT: GLenum = GLenum(0x8370);

pub const GL_MULTISAMPLE: GLenum = GLenum(0x809D);

pub const GL_NAND: GLenum = GLenum(0x150E);

pub const GL_NEAREST: GLenum = GLenum(0x2600);

pub const GL_NEAREST_MIPMAP_LINEAR: GLenum = GLenum(0x2702);

pub const GL_NEAREST_MIPMAP_NEAREST: GLenum = GLenum(0x2700);

pub const GL_NEVER: GLenum = GLenum(0x0200);

pub const GL_NICEST: GLenum = GLenum(0x1102);

pub const GL_NONE: GLenum = GLenum(0);

pub const GL_NOOP: GLenum = GLenum(0x1505);

pub const GL_NOR: GLenum = GLenum(0x1508);

pub const GL_NOTEQUAL: GLenum = GLenum(0x0205);

pub const GL_NO_ERROR: GLenum = GLenum(0);

pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = GLenum(0x86A2);

pub const GL_NUM_EXTENSIONS: GLenum = GLenum(0x821D);

pub const GL_OBJECT_TYPE: GLenum = GLenum(0x9112);

pub const GL_ONE: GLenum = GLenum(1);

pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLenum = GLenum(0x8004);

pub const GL_ONE_MINUS_CONSTANT_COLOR: GLenum = GLenum(0x8002);

pub const GL_ONE_MINUS_DST_ALPHA: GLenum = GLenum(0x0305);

pub const GL_ONE_MINUS_DST_COLOR: GLenum = GLenum(0x0307);

pub const GL_ONE_MINUS_SRC1_ALPHA: GLenum = GLenum(0x88FB);

pub const GL_ONE_MINUS_SRC1_COLOR: GLenum = GLenum(0x88FA);

pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = GLenum(0x0303);

pub const GL_ONE_MINUS_SRC_COLOR: GLenum = GLenum(0x0301);

pub const GL_OR: GLenum = GLenum(0x1507);

pub const GL_OR_INVERTED: GLenum = GLenum(0x150D);

pub const GL_OR_REVERSE: GLenum = GLenum(0x150B);

pub const GL_OUT_OF_MEMORY: GLenum = GLenum(0x0505);

pub const GL_PACK_ALIGNMENT: GLenum = GLenum(0x0D05);

pub const GL_PACK_IMAGE_HEIGHT: GLenum = GLenum(0x806C);

pub const GL_PACK_LSB_FIRST: GLenum = GLenum(0x0D01);

pub const GL_PACK_ROW_LENGTH: GLenum = GLenum(0x0D02);

pub const GL_PACK_SKIP_IMAGES: GLenum = GLenum(0x806B);

pub const GL_PACK_SKIP_PIXELS: GLenum = GLenum(0x0D04);

pub const GL_PACK_SKIP_ROWS: GLenum = GLenum(0x0D03);

pub const GL_PACK_SWAP_BYTES: GLenum = GLenum(0x0D00);

pub const GL_PIXEL_PACK_BUFFER: GLenum = GLenum(0x88EB);

pub const GL_PIXEL_PACK_BUFFER_BINDING: GLenum = GLenum(0x88ED);

pub const GL_PIXEL_UNPACK_BUFFER: GLenum = GLenum(0x88EC);

pub const GL_PIXEL_UNPACK_BUFFER_BINDING: GLenum = GLenum(0x88EF);

pub const GL_POINT: GLenum = GLenum(0x1B00);

pub const GL_POINTS: GLenum = GLenum(0x0000);

pub const GL_POINT_FADE_THRESHOLD_SIZE: GLenum = GLenum(0x8128);

pub const GL_POINT_SIZE: GLenum = GLenum(0x0B11);

pub const GL_POINT_SIZE_GRANULARITY: GLenum = GLenum(0x0B13);

pub const GL_POINT_SIZE_RANGE: GLenum = GLenum(0x0B12);

pub const GL_POINT_SPRITE_COORD_ORIGIN: GLenum = GLenum(0x8CA0);

pub const GL_POLYGON_MODE: GLenum = GLenum(0x0B40);

pub const GL_POLYGON_OFFSET_FACTOR: GLenum = GLenum(0x8038);

pub const GL_POLYGON_OFFSET_FILL: GLenum = GLenum(0x8037);

pub const GL_POLYGON_OFFSET_LINE: GLenum = GLenum(0x2A02);

pub const GL_POLYGON_OFFSET_POINT: GLenum = GLenum(0x2A01);

pub const GL_POLYGON_OFFSET_UNITS: GLenum = GLenum(0x2A00);

pub const GL_POLYGON_SMOOTH: GLenum = GLenum(0x0B41);

pub const GL_POLYGON_SMOOTH_HINT: GLenum = GLenum(0x0C53);

pub const GL_PRIMITIVES_GENERATED: GLenum = GLenum(0x8C87);

pub const GL_PRIMITIVES_SUBMITTED_ARB: GLenum = GLenum(0x82EF);

pub const GL_PRIMITIVE_RESTART: GLenum = GLenum(0x8F9D);

pub const GL_PRIMITIVE_RESTART_INDEX: GLenum = GLenum(0x8F9E);

pub const GL_PROGRAM: GLenum = GLenum(0x82E2);

pub const GL_PROGRAM_PIPELINE: GLenum = GLenum(0x82E4);

pub const GL_PROGRAM_POINT_SIZE: GLenum = GLenum(0x8642);

pub const GL_PROVOKING_VERTEX: GLenum = GLenum(0x8E4F);

pub const GL_PROXY_TEXTURE_1D: GLenum = GLenum(0x8063);

pub const GL_PROXY_TEXTURE_1D_ARRAY: GLenum = GLenum(0x8C19);

pub const GL_PROXY_TEXTURE_2D: GLenum = GLenum(0x8064);

pub const GL_PROXY_TEXTURE_2D_ARRAY: GLenum = GLenum(0x8C1B);

pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = GLenum(0x9101);

pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x9103);

pub const GL_PROXY_TEXTURE_3D: GLenum = GLenum(0x8070);

pub const GL_PROXY_TEXTURE_CUBE_MAP: GLenum = GLenum(0x851B);

pub const GL_PROXY_TEXTURE_RECTANGLE: GLenum = GLenum(0x84F7);

pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = GLenum(0x8E4C);

pub const GL_QUERY: GLenum = GLenum(0x82E3);

pub const GL_QUERY_BY_REGION_NO_WAIT: GLenum = GLenum(0x8E16);

pub const GL_QUERY_BY_REGION_WAIT: GLenum = GLenum(0x8E15);

pub const GL_QUERY_COUNTER_BITS: GLenum = GLenum(0x8864);

pub const GL_QUERY_NO_WAIT: GLenum = GLenum(0x8E14);

pub const GL_QUERY_RESULT: GLenum = GLenum(0x8866);

pub const GL_QUERY_RESULT_AVAILABLE: GLenum = GLenum(0x8867);

pub const GL_QUERY_WAIT: GLenum = GLenum(0x8E13);

pub const GL_R11F_G11F_B10F: GLenum = GLenum(0x8C3A);

pub const GL_R16: GLenum = GLenum(0x822A);

pub const GL_R16F: GLenum = GLenum(0x822D);

pub const GL_R16I: GLenum = GLenum(0x8233);

pub const GL_R16UI: GLenum = GLenum(0x8234);

pub const GL_R16_SNORM: GLenum = GLenum(0x8F98);

pub const GL_R32F: GLenum = GLenum(0x822E);

pub const GL_R32I: GLenum = GLenum(0x8235);

pub const GL_R32UI: GLenum = GLenum(0x8236);

pub const GL_R3_G3_B2: GLenum = GLenum(0x2A10);

pub const GL_R8: GLenum = GLenum(0x8229);

pub const GL_R8I: GLenum = GLenum(0x8231);

pub const GL_R8UI: GLenum = GLenum(0x8232);

pub const GL_R8_SNORM: GLenum = GLenum(0x8F94);

pub const GL_RASTERIZER_DISCARD: GLenum = GLenum(0x8C89);

pub const GL_READ_BUFFER: GLenum = GLenum(0x0C02);

pub const GL_READ_FRAMEBUFFER: GLenum = GLenum(0x8CA8);

pub const GL_READ_FRAMEBUFFER_BINDING: GLenum = GLenum(0x8CAA);

pub const GL_READ_ONLY: GLenum = GLenum(0x88B8);

pub const GL_READ_WRITE: GLenum = GLenum(0x88BA);

pub const GL_RED: GLenum = GLenum(0x1903);

pub const GL_RED_INTEGER: GLenum = GLenum(0x8D94);

pub const GL_RENDERBUFFER: GLenum = GLenum(0x8D41);

pub const GL_RENDERBUFFER_ALPHA_SIZE: GLenum = GLenum(0x8D53);

pub const GL_RENDERBUFFER_BINDING: GLenum = GLenum(0x8CA7);

pub const GL_RENDERBUFFER_BLUE_SIZE: GLenum = GLenum(0x8D52);

pub const GL_RENDERBUFFER_DEPTH_SIZE: GLenum = GLenum(0x8D54);

pub const GL_RENDERBUFFER_GREEN_SIZE: GLenum = GLenum(0x8D51);

pub const GL_RENDERBUFFER_HEIGHT: GLenum = GLenum(0x8D43);

pub const GL_RENDERBUFFER_INTERNAL_FORMAT: GLenum = GLenum(0x8D44);

pub const GL_RENDERBUFFER_RED_SIZE: GLenum = GLenum(0x8D50);

pub const GL_RENDERBUFFER_SAMPLES: GLenum = GLenum(0x8CAB);

pub const GL_RENDERBUFFER_STENCIL_SIZE: GLenum = GLenum(0x8D55);

pub const GL_RENDERBUFFER_WIDTH: GLenum = GLenum(0x8D42);

pub const GL_RENDERER: GLenum = GLenum(0x1F01);

pub const GL_REPEAT: GLenum = GLenum(0x2901);

pub const GL_REPLACE: GLenum = GLenum(0x1E01);

pub const GL_RG: GLenum = GLenum(0x8227);

pub const GL_RG16: GLenum = GLenum(0x822C);

pub const GL_RG16F: GLenum = GLenum(0x822F);

pub const GL_RG16I: GLenum = GLenum(0x8239);

pub const GL_RG16UI: GLenum = GLenum(0x823A);

pub const GL_RG16_SNORM: GLenum = GLenum(0x8F99);

pub const GL_RG32F: GLenum = GLenum(0x8230);

pub const GL_RG32I: GLenum = GLenum(0x823B);

pub const GL_RG32UI: GLenum = GLenum(0x823C);

pub const GL_RG8: GLenum = GLenum(0x822B);

pub const GL_RG8I: GLenum = GLenum(0x8237);

pub const GL_RG8UI: GLenum = GLenum(0x8238);

pub const GL_RG8_SNORM: GLenum = GLenum(0x8F95);

pub const GL_RGB: GLenum = GLenum(0x1907);

pub const GL_RGB10: GLenum = GLenum(0x8052);

pub const GL_RGB10_A2: GLenum = GLenum(0x8059);

pub const GL_RGB10_A2UI: GLenum = GLenum(0x906F);

pub const GL_RGB12: GLenum = GLenum(0x8053);

pub const GL_RGB16: GLenum = GLenum(0x8054);

pub const GL_RGB16F: GLenum = GLenum(0x881B);

pub const GL_RGB16I: GLenum = GLenum(0x8D89);

pub const GL_RGB16UI: GLenum = GLenum(0x8D77);

pub const GL_RGB16_SNORM: GLenum = GLenum(0x8F9A);

pub const GL_RGB32F: GLenum = GLenum(0x8815);

pub const GL_RGB32I: GLenum = GLenum(0x8D83);

pub const GL_RGB32UI: GLenum = GLenum(0x8D71);

pub const GL_RGB4: GLenum = GLenum(0x804F);

pub const GL_RGB5: GLenum = GLenum(0x8050);

pub const GL_RGB5_A1: GLenum = GLenum(0x8057);

pub const GL_RGB8: GLenum = GLenum(0x8051);

pub const GL_RGB8I: GLenum = GLenum(0x8D8F);

pub const GL_RGB8UI: GLenum = GLenum(0x8D7D);

pub const GL_RGB8_SNORM: GLenum = GLenum(0x8F96);

pub const GL_RGB9_E5: GLenum = GLenum(0x8C3D);

pub const GL_RGBA: GLenum = GLenum(0x1908);

pub const GL_RGBA12: GLenum = GLenum(0x805A);

pub const GL_RGBA16: GLenum = GLenum(0x805B);

pub const GL_RGBA16F: GLenum = GLenum(0x881A);

pub const GL_RGBA16I: GLenum = GLenum(0x8D88);

pub const GL_RGBA16UI: GLenum = GLenum(0x8D76);

pub const GL_RGBA16_SNORM: GLenum = GLenum(0x8F9B);

pub const GL_RGBA2: GLenum = GLenum(0x8055);

pub const GL_RGBA32F: GLenum = GLenum(0x8814);

pub const GL_RGBA32I: GLenum = GLenum(0x8D82);

pub const GL_RGBA32UI: GLenum = GLenum(0x8D70);

pub const GL_RGBA4: GLenum = GLenum(0x8056);

pub const GL_RGBA8: GLenum = GLenum(0x8058);

pub const GL_RGBA8I: GLenum = GLenum(0x8D8E);

pub const GL_RGBA8UI: GLenum = GLenum(0x8D7C);

pub const GL_RGBA8_SNORM: GLenum = GLenum(0x8F97);

pub const GL_RGBA_INTEGER: GLenum = GLenum(0x8D99);

pub const GL_RGB_INTEGER: GLenum = GLenum(0x8D98);

pub const GL_RG_INTEGER: GLenum = GLenum(0x8228);

pub const GL_RIGHT: GLenum = GLenum(0x0407);

pub const GL_SAMPLER: GLenum = GLenum(0x82E6);

pub const GL_SAMPLER_1D: GLenum = GLenum(0x8B5D);

pub const GL_SAMPLER_1D_ARRAY: GLenum = GLenum(0x8DC0);

pub const GL_SAMPLER_1D_ARRAY_SHADOW: GLenum = GLenum(0x8DC3);

pub const GL_SAMPLER_1D_SHADOW: GLenum = GLenum(0x8B61);

pub const GL_SAMPLER_2D: GLenum = GLenum(0x8B5E);

pub const GL_SAMPLER_2D_ARRAY: GLenum = GLenum(0x8DC1);

pub const GL_SAMPLER_2D_ARRAY_SHADOW: GLenum = GLenum(0x8DC4);

pub const GL_SAMPLER_2D_MULTISAMPLE: GLenum = GLenum(0x9108);

pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x910B);

pub const GL_SAMPLER_2D_RECT: GLenum = GLenum(0x8B63);

pub const GL_SAMPLER_2D_RECT_SHADOW: GLenum = GLenum(0x8B64);

pub const GL_SAMPLER_2D_SHADOW: GLenum = GLenum(0x8B62);

pub const GL_SAMPLER_3D: GLenum = GLenum(0x8B5F);

pub const GL_SAMPLER_BINDING: GLenum = GLenum(0x8919);

pub const GL_SAMPLER_BUFFER: GLenum = GLenum(0x8DC2);

pub const GL_SAMPLER_CUBE: GLenum = GLenum(0x8B60);

pub const GL_SAMPLER_CUBE_SHADOW: GLenum = GLenum(0x8DC5);

pub const GL_SAMPLES: GLenum = GLenum(0x80A9);

pub const GL_SAMPLES_PASSED: GLenum = GLenum(0x8914);

pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = GLenum(0x809E);

pub const GL_SAMPLE_ALPHA_TO_ONE: GLenum = GLenum(0x809F);

pub const GL_SAMPLE_BUFFERS: GLenum = GLenum(0x80A8);

pub const GL_SAMPLE_COVERAGE: GLenum = GLenum(0x80A0);

pub const GL_SAMPLE_COVERAGE_INVERT: GLenum = GLenum(0x80AB);

pub const GL_SAMPLE_COVERAGE_VALUE: GLenum = GLenum(0x80AA);

pub const GL_SAMPLE_MASK: GLenum = GLenum(0x8E51);

pub const GL_SAMPLE_MASK_VALUE: GLenum = GLenum(0x8E52);

pub const GL_SAMPLE_POSITION: GLenum = GLenum(0x8E50);

pub const GL_SCISSOR_BOX: GLenum = GLenum(0x0C10);

pub const GL_SCISSOR_TEST: GLenum = GLenum(0x0C11);

pub const GL_SEPARATE_ATTRIBS: GLenum = GLenum(0x8C8D);

pub const GL_SET: GLenum = GLenum(0x150F);

pub const GL_SHADER: GLenum = GLenum(0x82E1);

pub const GL_SHADER_SOURCE_LENGTH: GLenum = GLenum(0x8B88);

pub const GL_SHADER_TYPE: GLenum = GLenum(0x8B4F);

pub const GL_SHADING_LANGUAGE_VERSION: GLenum = GLenum(0x8B8C);

pub const GL_SHORT: GLenum = GLenum(0x1402);

pub const GL_SIGNALED: GLenum = GLenum(0x9119);

pub const GL_SIGNED_NORMALIZED: GLenum = GLenum(0x8F9C);

pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = GLenum(0x0B23);

pub const GL_SMOOTH_LINE_WIDTH_RANGE: GLenum = GLenum(0x0B22);

pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: GLenum = GLenum(0x0B13);

pub const GL_SMOOTH_POINT_SIZE_RANGE: GLenum = GLenum(0x0B12);

pub const GL_SRC1_ALPHA: GLenum = GLenum(0x8589);

pub const GL_SRC1_COLOR: GLenum = GLenum(0x88F9);

pub const GL_SRC_ALPHA: GLenum = GLenum(0x0302);

pub const GL_SRC_ALPHA_SATURATE: GLenum = GLenum(0x0308);

pub const GL_SRC_COLOR: GLenum = GLenum(0x0300);

pub const GL_SRGB: GLenum = GLenum(0x8C40);

pub const GL_SRGB8: GLenum = GLenum(0x8C41);

pub const GL_SRGB8_ALPHA8: GLenum = GLenum(0x8C43);

pub const GL_SRGB_ALPHA: GLenum = GLenum(0x8C42);

pub const GL_STACK_OVERFLOW: GLenum = GLenum(0x0503);

pub const GL_STACK_UNDERFLOW: GLenum = GLenum(0x0504);

pub const GL_STATIC_COPY: GLenum = GLenum(0x88E6);

pub const GL_STATIC_DRAW: GLenum = GLenum(0x88E4);

pub const GL_STATIC_READ: GLenum = GLenum(0x88E5);

pub const GL_STENCIL: GLenum = GLenum(0x1802);

pub const GL_STENCIL_ATTACHMENT: GLenum = GLenum(0x8D20);

pub const GL_STENCIL_BACK_FAIL: GLenum = GLenum(0x8801);

pub const GL_STENCIL_BACK_FUNC: GLenum = GLenum(0x8800);

pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = GLenum(0x8802);

pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GLenum = GLenum(0x8803);

pub const GL_STENCIL_BACK_REF: GLenum = GLenum(0x8CA3);

pub const GL_STENCIL_BACK_VALUE_MASK: GLenum = GLenum(0x8CA4);

pub const GL_STENCIL_BACK_WRITEMASK: GLenum = GLenum(0x8CA5);

pub const GL_STENCIL_BUFFER_BIT: GLbitfield = GLbitfield(0x00000400);

pub const GL_STENCIL_CLEAR_VALUE: GLenum = GLenum(0x0B91);

pub const GL_STENCIL_FAIL: GLenum = GLenum(0x0B94);

pub const GL_STENCIL_FUNC: GLenum = GLenum(0x0B92);

pub const GL_STENCIL_INDEX: GLenum = GLenum(0x1901);

pub const GL_STENCIL_INDEX1: GLenum = GLenum(0x8D46);

pub const GL_STENCIL_INDEX16: GLenum = GLenum(0x8D49);

pub const GL_STENCIL_INDEX4: GLenum = GLenum(0x8D47);

pub const GL_STENCIL_INDEX8: GLenum = GLenum(0x8D48);

pub const GL_STENCIL_PASS_DEPTH_FAIL: GLenum = GLenum(0x0B95);

pub const GL_STENCIL_PASS_DEPTH_PASS: GLenum = GLenum(0x0B96);

pub const GL_STENCIL_REF: GLenum = GLenum(0x0B97);

pub const GL_STENCIL_TEST: GLenum = GLenum(0x0B90);

pub const GL_STENCIL_VALUE_MASK: GLenum = GLenum(0x0B93);

pub const GL_STENCIL_WRITEMASK: GLenum = GLenum(0x0B98);

pub const GL_STEREO: GLenum = GLenum(0x0C33);

pub const GL_STREAM_COPY: GLenum = GLenum(0x88E2);

pub const GL_STREAM_DRAW: GLenum = GLenum(0x88E0);

pub const GL_STREAM_READ: GLenum = GLenum(0x88E1);

pub const GL_SUBPIXEL_BITS: GLenum = GLenum(0x0D50);

pub const GL_SYNC_CONDITION: GLenum = GLenum(0x9113);

pub const GL_SYNC_FENCE: GLenum = GLenum(0x9116);

pub const GL_SYNC_FLAGS: GLenum = GLenum(0x9115);

pub const GL_SYNC_FLUSH_COMMANDS_BIT: GLbitfield = GLbitfield(0x00000001);

pub const GL_SYNC_GPU_COMMANDS_COMPLETE: GLenum = GLenum(0x9117);

pub const GL_SYNC_STATUS: GLenum = GLenum(0x9114);

pub const GL_TESS_CONTROL_SHADER_PATCHES_ARB: GLenum = GLenum(0x82F1);

pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS_ARB: GLenum = GLenum(0x82F2);

pub const GL_TEXTURE: GLenum = GLenum(0x1702);

pub const GL_TEXTURE0: GLenum = GLenum(0x84C0);

pub const GL_TEXTURE1: GLenum = GLenum(0x84C1);

pub const GL_TEXTURE10: GLenum = GLenum(0x84CA);

pub const GL_TEXTURE11: GLenum = GLenum(0x84CB);

pub const GL_TEXTURE12: GLenum = GLenum(0x84CC);

pub const GL_TEXTURE13: GLenum = GLenum(0x84CD);

pub const GL_TEXTURE14: GLenum = GLenum(0x84CE);

pub const GL_TEXTURE15: GLenum = GLenum(0x84CF);

pub const GL_TEXTURE16: GLenum = GLenum(0x84D0);

pub const GL_TEXTURE17: GLenum = GLenum(0x84D1);

pub const GL_TEXTURE18: GLenum = GLenum(0x84D2);

pub const GL_TEXTURE19: GLenum = GLenum(0x84D3);

pub const GL_TEXTURE2: GLenum = GLenum(0x84C2);

pub const GL_TEXTURE20: GLenum = GLenum(0x84D4);

pub const GL_TEXTURE21: GLenum = GLenum(0x84D5);

pub const GL_TEXTURE22: GLenum = GLenum(0x84D6);

pub const GL_TEXTURE23: GLenum = GLenum(0x84D7);

pub const GL_TEXTURE24: GLenum = GLenum(0x84D8);

pub const GL_TEXTURE25: GLenum = GLenum(0x84D9);

pub const GL_TEXTURE26: GLenum = GLenum(0x84DA);

pub const GL_TEXTURE27: GLenum = GLenum(0x84DB);

pub const GL_TEXTURE28: GLenum = GLenum(0x84DC);

pub const GL_TEXTURE29: GLenum = GLenum(0x84DD);

pub const GL_TEXTURE3: GLenum = GLenum(0x84C3);

pub const GL_TEXTURE30: GLenum = GLenum(0x84DE);

pub const GL_TEXTURE31: GLenum = GLenum(0x84DF);

pub const GL_TEXTURE4: GLenum = GLenum(0x84C4);

pub const GL_TEXTURE5: GLenum = GLenum(0x84C5);

pub const GL_TEXTURE6: GLenum = GLenum(0x84C6);

pub const GL_TEXTURE7: GLenum = GLenum(0x84C7);

pub const GL_TEXTURE8: GLenum = GLenum(0x84C8);

pub const GL_TEXTURE9: GLenum = GLenum(0x84C9);

pub const GL_TEXTURE_1D: GLenum = GLenum(0x0DE0);

pub const GL_TEXTURE_1D_ARRAY: GLenum = GLenum(0x8C18);

pub const GL_TEXTURE_2D: GLenum = GLenum(0x0DE1);

pub const GL_TEXTURE_2D_ARRAY: GLenum = GLenum(0x8C1A);

pub const GL_TEXTURE_2D_MULTISAMPLE: GLenum = GLenum(0x9100);

pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x9102);

pub const GL_TEXTURE_3D: GLenum = GLenum(0x806F);

pub const GL_TEXTURE_ALPHA_SIZE: GLenum = GLenum(0x805F);

pub const GL_TEXTURE_ALPHA_TYPE: GLenum = GLenum(0x8C13);

pub const GL_TEXTURE_BASE_LEVEL: GLenum = GLenum(0x813C);

pub const GL_TEXTURE_BINDING_1D: GLenum = GLenum(0x8068);

pub const GL_TEXTURE_BINDING_1D_ARRAY: GLenum = GLenum(0x8C1C);

pub const GL_TEXTURE_BINDING_2D: GLenum = GLenum(0x8069);

pub const GL_TEXTURE_BINDING_2D_ARRAY: GLenum = GLenum(0x8C1D);

pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = GLenum(0x9104);

pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x9105);

pub const GL_TEXTURE_BINDING_3D: GLenum = GLenum(0x806A);

pub const GL_TEXTURE_BINDING_BUFFER: GLenum = GLenum(0x8C2C);

pub const GL_TEXTURE_BINDING_CUBE_MAP: GLenum = GLenum(0x8514);

pub const GL_TEXTURE_BINDING_RECTANGLE: GLenum = GLenum(0x84F6);

pub const GL_TEXTURE_BLUE_SIZE: GLenum = GLenum(0x805E);

pub const GL_TEXTURE_BLUE_TYPE: GLenum = GLenum(0x8C12);

pub const GL_TEXTURE_BORDER_COLOR: GLenum = GLenum(0x1004);

pub const GL_TEXTURE_BUFFER: GLenum = GLenum(0x8C2A);

pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = GLenum(0x8C2D);

pub const GL_TEXTURE_COMPARE_FUNC: GLenum = GLenum(0x884D);

pub const GL_TEXTURE_COMPARE_MODE: GLenum = GLenum(0x884C);

pub const GL_TEXTURE_COMPRESSED: GLenum = GLenum(0x86A1);

pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = GLenum(0x86A0);

pub const GL_TEXTURE_COMPRESSION_HINT: GLenum = GLenum(0x84EF);

pub const GL_TEXTURE_CUBE_MAP: GLenum = GLenum(0x8513);

pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = GLenum(0x8516);

pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = GLenum(0x8518);

pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = GLenum(0x851A);

pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = GLenum(0x8515);

pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = GLenum(0x8517);

pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = GLenum(0x8519);

pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: GLenum = GLenum(0x884F);

pub const GL_TEXTURE_DEPTH: GLenum = GLenum(0x8071);

pub const GL_TEXTURE_DEPTH_SIZE: GLenum = GLenum(0x884A);

pub const GL_TEXTURE_DEPTH_TYPE: GLenum = GLenum(0x8C16);

pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = GLenum(0x9107);

pub const GL_TEXTURE_GREEN_SIZE: GLenum = GLenum(0x805D);

pub const GL_TEXTURE_GREEN_TYPE: GLenum = GLenum(0x8C11);

pub const GL_TEXTURE_HEIGHT: GLenum = GLenum(0x1001);

pub const GL_TEXTURE_INTERNAL_FORMAT: GLenum = GLenum(0x1003);

pub const GL_TEXTURE_LOD_BIAS: GLenum = GLenum(0x8501);

pub const GL_TEXTURE_MAG_FILTER: GLenum = GLenum(0x2800);

pub const GL_TEXTURE_MAX_ANISOTROPY: GLenum = GLenum(0x84FE);

pub const GL_TEXTURE_MAX_LEVEL: GLenum = GLenum(0x813D);

pub const GL_TEXTURE_MAX_LOD: GLenum = GLenum(0x813B);

pub const GL_TEXTURE_MIN_FILTER: GLenum = GLenum(0x2801);

pub const GL_TEXTURE_MIN_LOD: GLenum = GLenum(0x813A);

pub const GL_TEXTURE_RECTANGLE: GLenum = GLenum(0x84F5);

pub const GL_TEXTURE_RED_SIZE: GLenum = GLenum(0x805C);

pub const GL_TEXTURE_RED_TYPE: GLenum = GLenum(0x8C10);

pub const GL_TEXTURE_SAMPLES: GLenum = GLenum(0x9106);

pub const GL_TEXTURE_SHARED_SIZE: GLenum = GLenum(0x8C3F);

pub const GL_TEXTURE_STENCIL_SIZE: GLenum = GLenum(0x88F1);

pub const GL_TEXTURE_SWIZZLE_A: GLenum = GLenum(0x8E45);

pub const GL_TEXTURE_SWIZZLE_B: GLenum = GLenum(0x8E44);

pub const GL_TEXTURE_SWIZZLE_G: GLenum = GLenum(0x8E43);

pub const GL_TEXTURE_SWIZZLE_R: GLenum = GLenum(0x8E42);

pub const GL_TEXTURE_SWIZZLE_RGBA: GLenum = GLenum(0x8E46);

pub const GL_TEXTURE_WIDTH: GLenum = GLenum(0x1000);

pub const GL_TEXTURE_WRAP_R: GLenum = GLenum(0x8072);

pub const GL_TEXTURE_WRAP_S: GLenum = GLenum(0x2802);

pub const GL_TEXTURE_WRAP_T: GLenum = GLenum(0x2803);

pub const GL_TIMEOUT_EXPIRED: GLenum = GLenum(0x911B);

pub const GL_TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;

pub const GL_TIMESTAMP: GLenum = GLenum(0x8E28);

pub const GL_TIME_ELAPSED: GLenum = GLenum(0x88BF);

pub const GL_TRANSFORM_FEEDBACK_BUFFER: GLenum = GLenum(0x8C8E);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = GLenum(0x8C8F);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = GLenum(0x8C7F);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = GLenum(0x8C85);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: GLenum = GLenum(0x8C84);

pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = GLenum(0x8C88);

pub const GL_TRANSFORM_FEEDBACK_VARYINGS: GLenum = GLenum(0x8C83);

pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = GLenum(0x8C76);

pub const GL_TRIANGLES: GLenum = GLenum(0x0004);

pub const GL_TRIANGLES_ADJACENCY: GLenum = GLenum(0x000C);

pub const GL_TRIANGLE_FAN: GLenum = GLenum(0x0006);

pub const GL_TRIANGLE_STRIP: GLenum = GLenum(0x0005);

pub const GL_TRIANGLE_STRIP_ADJACENCY: GLenum = GLenum(0x000D);

pub const GL_TRUE: GLenum = GLenum(1);

pub const GL_UNIFORM_ARRAY_STRIDE: GLenum = GLenum(0x8A3C);

pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = GLenum(0x8A42);

pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = GLenum(0x8A43);

pub const GL_UNIFORM_BLOCK_BINDING: GLenum = GLenum(0x8A3F);

pub const GL_UNIFORM_BLOCK_DATA_SIZE: GLenum = GLenum(0x8A40);

pub const GL_UNIFORM_BLOCK_INDEX: GLenum = GLenum(0x8A3A);

pub const GL_UNIFORM_BLOCK_NAME_LENGTH: GLenum = GLenum(0x8A41);

pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = GLenum(0x8A46);

pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = GLenum(0x8A45);

pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = GLenum(0x8A44);

pub const GL_UNIFORM_BUFFER: GLenum = GLenum(0x8A11);

pub const GL_UNIFORM_BUFFER_BINDING: GLenum = GLenum(0x8A28);

pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = GLenum(0x8A34);

pub const GL_UNIFORM_BUFFER_SIZE: GLenum = GLenum(0x8A2A);

pub const GL_UNIFORM_BUFFER_START: GLenum = GLenum(0x8A29);

pub const GL_UNIFORM_IS_ROW_MAJOR: GLenum = GLenum(0x8A3E);

pub const GL_UNIFORM_MATRIX_STRIDE: GLenum = GLenum(0x8A3D);

pub const GL_UNIFORM_NAME_LENGTH: GLenum = GLenum(0x8A39);

pub const GL_UNIFORM_OFFSET: GLenum = GLenum(0x8A3B);

pub const GL_UNIFORM_SIZE: GLenum = GLenum(0x8A38);

pub const GL_UNIFORM_TYPE: GLenum = GLenum(0x8A37);

pub const GL_UNPACK_ALIGNMENT: GLenum = GLenum(0x0CF5);

pub const GL_UNPACK_IMAGE_HEIGHT: GLenum = GLenum(0x806E);

pub const GL_UNPACK_LSB_FIRST: GLenum = GLenum(0x0CF1);

pub const GL_UNPACK_ROW_LENGTH: GLenum = GLenum(0x0CF2);

pub const GL_UNPACK_SKIP_IMAGES: GLenum = GLenum(0x806D);

pub const GL_UNPACK_SKIP_PIXELS: GLenum = GLenum(0x0CF4);

pub const GL_UNPACK_SKIP_ROWS: GLenum = GLenum(0x0CF3);

pub const GL_UNPACK_SWAP_BYTES: GLenum = GLenum(0x0CF0);

pub const GL_UNSIGNALED: GLenum = GLenum(0x9118);

pub const GL_UNSIGNED_BYTE: GLenum = GLenum(0x1401);

pub const GL_UNSIGNED_BYTE_2_3_3_REV: GLenum = GLenum(0x8362);

pub const GL_UNSIGNED_BYTE_3_3_2: GLenum = GLenum(0x8032);

pub const GL_UNSIGNED_INT: GLenum = GLenum(0x1405);

pub const GL_UNSIGNED_INT_10F_11F_11F_REV: GLenum = GLenum(0x8C3B);

pub const GL_UNSIGNED_INT_10_10_10_2: GLenum = GLenum(0x8036);

pub const GL_UNSIGNED_INT_24_8: GLenum = GLenum(0x84FA);

pub const GL_UNSIGNED_INT_2_10_10_10_REV: GLenum = GLenum(0x8368);

pub const GL_UNSIGNED_INT_5_9_9_9_REV: GLenum = GLenum(0x8C3E);

pub const GL_UNSIGNED_INT_8_8_8_8: GLenum = GLenum(0x8035);

pub const GL_UNSIGNED_INT_8_8_8_8_REV: GLenum = GLenum(0x8367);

pub const GL_UNSIGNED_INT_SAMPLER_1D: GLenum = GLenum(0x8DD1);

pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = GLenum(0x8DD6);

pub const GL_UNSIGNED_INT_SAMPLER_2D: GLenum = GLenum(0x8DD2);

pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = GLenum(0x8DD7);

pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = GLenum(0x910A);

pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x910D);

pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = GLenum(0x8DD5);

pub const GL_UNSIGNED_INT_SAMPLER_3D: GLenum = GLenum(0x8DD3);

pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: GLenum = GLenum(0x8DD8);

pub const GL_UNSIGNED_INT_SAMPLER_CUBE: GLenum = GLenum(0x8DD4);

pub const GL_UNSIGNED_INT_VEC2: GLenum = GLenum(0x8DC6);

pub const GL_UNSIGNED_INT_VEC3: GLenum = GLenum(0x8DC7);

pub const GL_UNSIGNED_INT_VEC4: GLenum = GLenum(0x8DC8);

pub const GL_UNSIGNED_NORMALIZED: GLenum = GLenum(0x8C17);

pub const GL_UNSIGNED_SHORT: GLenum = GLenum(0x1403);

pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: GLenum = GLenum(0x8366);

pub const GL_UNSIGNED_SHORT_4_4_4_4: GLenum = GLenum(0x8033);

pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: GLenum = GLenum(0x8365);

pub const GL_UNSIGNED_SHORT_5_5_5_1: GLenum = GLenum(0x8034);

pub const GL_UNSIGNED_SHORT_5_6_5: GLenum = GLenum(0x8363);

pub const GL_UNSIGNED_SHORT_5_6_5_REV: GLenum = GLenum(0x8364);

pub const GL_UPPER_LEFT: GLenum = GLenum(0x8CA2);

pub const GL_VALIDATE_STATUS: GLenum = GLenum(0x8B83);

pub const GL_VENDOR: GLenum = GLenum(0x1F00);

pub const GL_VERSION: GLenum = GLenum(0x1F02);

pub const GL_VERTEX_ARRAY: GLenum = GLenum(0x8074);

pub const GL_VERTEX_ARRAY_BINDING: GLenum = GLenum(0x85B5);

pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x889F);

pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = GLenum(0x88FE);

pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = GLenum(0x8622);

pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = GLenum(0x88FD);

pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = GLenum(0x886A);

pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: GLenum = GLenum(0x8645);

pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: GLenum = GLenum(0x8623);

pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = GLenum(0x8624);

pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: GLenum = GLenum(0x8625);

pub const GL_VERTEX_PROGRAM_POINT_SIZE: GLenum = GLenum(0x8642);

pub const GL_VERTEX_SHADER: GLenum = GLenum(0x8B31);

pub const GL_VERTEX_SHADER_INVOCATIONS_ARB: GLenum = GLenum(0x82F0);

pub const GL_VERTICES_SUBMITTED_ARB: GLenum = GLenum(0x82EE);

pub const GL_VIEWPORT: GLenum = GLenum(0x0BA2);

pub const GL_WAIT_FAILED: GLenum = GLenum(0x911D);

pub const GL_WRITE_ONLY: GLenum = GLenum(0x88B9);

pub const GL_XOR: GLenum = GLenum(0x1506);

pub const GL_ZERO: GLenum = GLenum(0);
