uniform float time;
uniform vec2 resolution;

void main()
{
    vec4 p = vec4((gl_FragCoord.xy * 2.-resolution.xy)/resolution.y,0,1), r = p-p, q = r, c;
    
    float k = 0.;
	
    // many params here
    //k = .258;
    k = .06127*sin(time)+0.27522123;
    //k = .282;
    //k = .3;
	
    q.w += time * 0.0693 + 1.;
	
    // i is the color of pixel while hit 0. => 1.
	for (float i=1.; i>0.; i-=.01) 
	{
        float d=0.,s=1.;

        for (int j = 0; j <3; j++)
		{
			r = abs( mod(q * s + 1.,2.) - 1. );
            d = max(d, (k - length( sqrt(r * .6) ) * .3) / s );
			s *= 3.;
		}
		
        q += p * d;
        
        gl_FragColor = vec4(i);
			
        if(d < 1e-5) break;
    }
}
