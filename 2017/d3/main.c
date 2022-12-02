
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
	int x;
	int y;
} pos;

typedef enum {
	E,
	N,
	W,
	S
} bearing;

int main(){
	
	{ // PART 1
	
		pos p = { .x = 0, .y = 0 };
		unsigned int cur = 1;
		unsigned int find = 325489;
		bearing dir = E;
		
		unsigned int stride = 1;
		unsigned int in_stride = 0;
		
		for(int i = 1; i < find; i++){
			
			switch ( dir ) {
				case N:
					p.y++;
					break;
				case S:
					p.y--;
					break;
				case E:
					p.x++;
					break;
				case W:
					p.x--;
					break;
				default:
					printf("No direction!");
					return 1;
			}
			
			cur++;
			in_stride++;
			
			if( in_stride == stride ){ // time to turn left
				
				if ( dir == N || dir == S ){
					stride++;
				}
				
				dir++; // E -> N -> W -> S
				if(dir > S){ dir = E; }
				
				in_stride = 0;
			}
			
		}
		
		// abs of pos
		if ( p.x < 0 ) { p.x *= -1; }
		if ( p.y < 0 ) { p.y *= -1; }
		
		printf("P1: steps to %u: %u\n\n\n", find, (p.x + p.y) );
	
	} // END Part 1
	
	return 0;
}
