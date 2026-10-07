//! One fixed-size scene buffer and one five-tap phosphor pass. No frame history,
//! multipass blur, dynamic target allocation, or displacement of game geometry.
use arkonk::game::{HEIGHT, WIDTH};
use macroquad::prelude::*;

pub const CURVATURE: f32 = 0.018;

pub struct Crt {
    camera: Camera2D,
    target: RenderTarget,
    material: Material,
    plain: Material,
    metal: bool,
}

impl Crt {
    pub fn new() -> Self {
        let target = render_target(WIDTH as u32, HEIGHT as u32);
        target.texture.set_filter(FilterMode::Nearest);
        let camera = Camera2D {
            render_target: Some(target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH, HEIGHT))
        };
        let metal =
            unsafe { get_internal_gl().quad_context.info().backend == miniquad::Backend::Metal };
        let material = load_material(
            if metal {
                ShaderSource::Msl { program: METAL_CRT }
            } else {
                ShaderSource::Glsl {
                    vertex: VERTEX,
                    fragment: FRAGMENT,
                }
            },
            MaterialParams {
                uniforms: vec![UniformDesc::new("CrtParams", UniformType::Float4)],
                ..Default::default()
            },
        )
        .expect("CRT shader compilation failed");
        let plain = load_material(
            if metal {
                ShaderSource::Msl {
                    program: METAL_PLAIN,
                }
            } else {
                ShaderSource::Glsl {
                    vertex: VERTEX,
                    fragment: PLAIN,
                }
            },
            Default::default(),
        )
        .expect("Plain presentation shader compilation failed");
        Self {
            camera,
            target,
            material,
            plain,
            metal,
        }
    }

    pub fn capture(&self, path: &str) {
        // Miniquad's Metal backend has no texture readback. Native Metal
        // captures use macOS window capture in the external test harness.
        if !self.metal {
            get_screen_data().export_png(path);
        }
    }

    pub fn begin(&self) {
        set_camera(&self.camera);
    }

    pub fn present(&self, x: f32, y: f32, scale: f32, enabled: bool) {
        set_default_camera();
        clear_background(Color::new(0.006, 0.007, 0.013, 1.0));
        if enabled {
            self.material.set_uniform(
                "CrtParams",
                vec4(get_time() as f32, HEIGHT * scale, 0.0, 0.0),
            );
            gl_use_material(&self.material);
        } else {
            // The scene is opaque. Keep the no-effects copy on an explicit
            // opaque pipeline too, instead of the alpha-blended shape pipeline.
            gl_use_material(&self.plain);
        }
        draw_texture_ex(
            &self.target.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(WIDTH * scale, HEIGHT * scale)),
                // The display camera renders the scene upside-down into GL textures.
                flip_y: !self.metal,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}

const VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying highp vec2 uv;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
"#;

const FRAGMENT: &str = r#"#version 100
precision highp float;
varying highp vec2 uv;
uniform sampler2D Texture;
uniform vec4 CrtParams;

vec3 hot(vec3 c) { return max(c - vec3(0.55), vec3(0.0)); }
void main() {
    vec2 p = uv * 2.0 - 1.0;
    vec2 q = p * (1.0 + 0.018 * p.yx * p.yx);
    vec2 at = q * 0.5 + 0.5;
    if (at.x < 0.0 || at.x > 1.0 || at.y < 0.0 || at.y > 1.0) {
        gl_FragColor = vec4(0.006, 0.007, 0.013, 1.0);
        return;
    }
    vec2 pixel = vec2(1.0 / 960.0, 1.0 / 900.0);
    vec3 c = texture2D(Texture, at).rgb;
    vec3 l = texture2D(Texture, at - vec2(pixel.x * 1.5, 0.0)).rgb;
    vec3 r = texture2D(Texture, at + vec2(pixel.x * 1.5, 0.0)).rgb;
    vec3 u = texture2D(Texture, at - vec2(0.0, pixel.y * 1.5)).rgb;
    vec3 d = texture2D(Texture, at + vec2(0.0, pixel.y * 1.5)).rgb;
    // Reuse the bloom taps for faint color separation near the glass edges.
    float fringe = dot(p, p) * 0.035;
    c.r = mix(c.r, r.r, fringe);
    c.b = mix(c.b, l.b, fringe);
    c += (hot(l) + hot(r) + hot(u) + hot(d)) * 0.17;
    float strength = smoothstep(360.0, 800.0, CrtParams.y);
    float scan = 0.94 + 0.06 * cos(at.y * 900.0 * 2.0943951);
    vec3 mask = 0.975 + 0.025 * cos(at.x * 960.0 * 2.0943951 + vec3(0.0, 2.0943951, 4.1887902));
    c *= mix(vec3(1.0), scan * mask, strength);
    c *= 1.0 - 0.14 * dot(p, p) - 0.10 * p.x * p.x * p.y * p.y;
    float grain = fract(52.9829189 * fract(dot(floor(at / pixel), vec2(0.06711056, 0.00583715))));
    c += (grain - 0.5) * 0.004;
    gl_FragColor = vec4(c, 1.0);
}
"#;

const PLAIN: &str = r#"#version 100
precision highp float;
varying highp vec2 uv;
uniform sampler2D Texture;
void main() { gl_FragColor = vec4(texture2D(Texture, uv).rgb, 1.0); }
"#;

// Uniform order follows Macroquad's actual buffer layout (Projection, Model,
// _Time, then user uniforms). Both paths use the same five-tap phosphor pass.
const METAL_CRT: &str = r#"
#include <metal_stdlib>
using namespace metal;
struct Uniforms { float4x4 Projection; float4x4 Model; float4 Time; float4 CrtParams; };
struct Vertex { float3 position [[attribute(0)]]; float2 texcoord [[attribute(1)]]; };
struct Raster { float4 position [[position]]; float2 uv; };
vertex Raster vertexShader(Vertex v [[stage_in]], constant Uniforms& u [[buffer(0)]]) {
    Raster out; out.position = u.Projection * u.Model * float4(v.position, 1.0); out.uv = v.texcoord; return out;
}
float3 hot(float3 c) { return max(c - float3(0.55), float3(0.0)); }
fragment float4 fragmentShader(Raster in [[stage_in]], texture2d<float> tex [[texture(0)]], sampler s [[sampler(0)]], constant Uniforms& u [[buffer(0)]]) {
    float2 p = in.uv * 2.0 - 1.0;
    float2 q = p * (1.0 + 0.018 * p.yx * p.yx);
    float2 at = q * 0.5 + 0.5;
    if (at.x < 0.0 || at.x > 1.0 || at.y < 0.0 || at.y > 1.0) return float4(0.006, 0.007, 0.013, 1.0);
    float2 pixel = float2(1.0 / 960.0, 1.0 / 900.0);
    float3 c = tex.sample(s, at).rgb;
    float3 l = tex.sample(s, at - float2(pixel.x * 1.5, 0.0)).rgb;
    float3 r = tex.sample(s, at + float2(pixel.x * 1.5, 0.0)).rgb;
    float3 t = tex.sample(s, at - float2(0.0, pixel.y * 1.5)).rgb;
    float3 d = tex.sample(s, at + float2(0.0, pixel.y * 1.5)).rgb;
    float fringe = dot(p, p) * 0.035;
    c.r = mix(c.r, r.r, fringe); c.b = mix(c.b, l.b, fringe);
    c += (hot(l) + hot(r) + hot(t) + hot(d)) * 0.17;
    float strength = smoothstep(360.0, 800.0, u.CrtParams.y);
    float scan = 0.94 + 0.06 * cos(at.y * 900.0 * 2.0943951);
    float3 mask = 0.975 + 0.025 * cos(at.x * 960.0 * 2.0943951 + float3(0.0, 2.0943951, 4.1887902));
    c *= mix(float3(1.0), scan * mask, strength);
    c *= 1.0 - 0.14 * dot(p, p) - 0.10 * p.x * p.x * p.y * p.y;
    float grain = fract(52.9829189 * fract(dot(floor(at / pixel), float2(0.06711056, 0.00583715))));
    return float4(c + (grain - 0.5) * 0.004, 1.0);
}
"#;
const METAL_PLAIN: &str = r#"
#include <metal_stdlib>
using namespace metal;
struct Uniforms { float4x4 Projection; float4x4 Model; };
struct Vertex { float3 position [[attribute(0)]]; float2 texcoord [[attribute(1)]]; };
struct Raster { float4 position [[position]]; float2 uv; };
vertex Raster vertexShader(Vertex v [[stage_in]], constant Uniforms& u [[buffer(0)]]) {
    Raster out; out.position = u.Projection * u.Model * float4(v.position, 1.0); out.uv = v.texcoord; return out;
}
fragment float4 fragmentShader(Raster in [[stage_in]], texture2d<float> tex [[texture(0)]], sampler s [[sampler(0)]]) { return float4(tex.sample(s, in.uv).rgb, 1.0); }
"#;
