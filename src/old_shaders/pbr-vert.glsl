#version 300 es

// Originally taken from https://github.com/KhronosGroup/glTF-WebGL-PBR
// Commit a94655275e5e4e8ae580b1d95ce678b74ab87426



// layout (location = 0) in vec4 a_Position;

// layout (location = 1) in vec4 a_Normal;

// layout (location = 2) in vec4 a_Tangent;



uniform mat4 u_MVPMatrix;
// uniform mat4 u_ModelMatrix;

// out vec3 v_Position;
// out vec2 v_UV;
// out vec4 v_Color;

// // #ifdef HAS_NORMALS
// // #ifdef HAS_TANGENTS
// out mat3 v_TBN;
// // #else
// out vec3 v_Normal;
// // #endif
// // #endif


void main()
{

  gl_Position = vec4(1.0, 1.0, 1.0, 1.0);

}


