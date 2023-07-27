struct V {
    @builtin(instance_index) instance_index: u32,
}

@vertex
fn vertex(v: V) -> @location(0) u32 {
    return v.instance_index;
}

@fragment
fn fragment(@location(0) index: u32) -> @location(0) vec4<f32> {
    return vec4<f32>(index);
}