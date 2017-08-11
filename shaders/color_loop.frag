#version 140

uniform float time;
uniform vec2 resolution;
out vec4 color;

void main() {
    vec2 r = resolution,
        o = gl_FragCoord.xy - r / 2.;
    o = vec2(length(o) / r.y - .3, atan(o.y, o.x));
    vec4 s = .15 * sin(1.5 * sin(time) * vec4(0, 1, 2, 3) + o.y + sin(o.y)),
        e = s.wxyz,
        f = max(o.x - s, e - o.x);
    color = dot(clamp(f * r.y, 0., 1.), 32. * (s - e)) * (s - .1) + f;
}