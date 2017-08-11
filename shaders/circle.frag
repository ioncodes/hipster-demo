#version 140

uniform float size;
uniform vec2 resolution;

float sinm(float t, float vmin, float vmax)
{
	return (vmax-vmin)*0.5*sin(t)+vmin+(vmax-vmin)*0.5;
}

void main( void ) {

	float t = 0.0;
	float s = size;
	float thickness = 0.0021;

	vec2 p = gl_FragCoord.xy / resolution.xx;
	vec2 c = vec2(0.5,0.5*resolution.y/resolution.x);

	float d = 0.0;
	float alpha = 0.15;
	for(int i=0 ; i<7 ; i++)
	{
		float d1 = length(p-c);
		float d2 = abs(d1-s);
		d += thickness/d2;
		c += alpha*sign(p-c);
		s *= sinm(t,0.45,0.45);
		thickness *= 0.775;
		alpha *= 0.405;
	}

	gl_FragColor = vec4(d*vec3(0.25,0.35,0.45),1.0);
}
