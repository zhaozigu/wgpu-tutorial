struct Uniforms {
    model_mat : mat4x4<f32>,
    view_project_mat : mat4x4<f32>,
    normal_mat : mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct Input {
    @location(0) pos: vec4<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) uv: vec2<f32>,
}

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_position: vec4<f32>,
    @location(1) v_normal: vec4<f32>,
    @location(2) v_uv: vec2<f32>,
};

@vertex
fn vs_main(in: Input) -> Output {
    var output: Output;
    let m_position: vec4<f32> = uniforms.model_mat * in.pos;
    output.v_position = m_position;
    output.v_normal = uniforms.normal_mat * in.normal;
    output.v_uv = in.uv;
    output.position = uniforms.view_project_mat * m_position;
    return output;
}

struct FragUniforms {
    light_position: vec4<f32>,
    eye_position: vec4<f32>,
};
@group(0) @binding(1) var<uniform> frag_uniforms: FragUniforms;

struct LightUniforms {
    specular_color: vec4<f32>,
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
    is_two_side: i32,
};
@group(0) @binding(2) var<uniform> light_uniforms: LightUniforms;
@group(1) @binding(0) var texture_data: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    let texture_color:vec4<f32> = textureSample(texture_data, texture_sampler, in.v_uv);

    let N: vec3<f32> = normalize(in.v_normal.xyz);
    let L: vec3<f32> = normalize(frag_uniforms.light_position.xyz - in.v_position.xyz);
    let V: vec3<f32> = normalize(frag_uniforms.eye_position.xyz - in.v_position.xyz);
    let H: vec3<f32> = normalize(L + V);

    // front side
    var diffuse: f32 = light_uniforms.diffuse_intensity * max(dot(N, L), 0.0);
    var specular: f32 = light_uniforms.specular_intensity * pow(max(dot(N, H), 0.0), light_uniforms.specular_shininess);

    // back side
    if (light_uniforms.is_two_side == 1) {
        diffuse = diffuse + light_uniforms.diffuse_intensity * max(dot(-N, L), 0.0);
        specular = specular + light_uniforms.specular_intensity * pow(max(dot(-N, H), 0.0), light_uniforms.specular_shininess);
    }

    let ambient: f32 = light_uniforms.ambient_intensity;
    let final_color: vec3<f32> = texture_color.rgb * (ambient + diffuse) + light_uniforms.specular_color.xyz * specular;
    return vec4(final_color, 1.0);
}