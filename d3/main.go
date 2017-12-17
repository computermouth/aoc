
package main

import (
	"fmt"
)

type Pos struct {
	x int
	y int
}

// because go has no enums -- directions, in order of usage
const (
	E = iota
	N = iota
	W = iota
	S = iota
)

func main(){
	
	p := Pos{ x: 0, y: 0 } // start at center
	
	cur := 1 // "index"
	find := 325489 // target
	dir := E // Always head east first
	
	stride := 1 // how long to continue in direction
	in_stride := 0 // progress toward direction
	
	fmt.Println("Finding ", find)
	
	for i := 1; i < find; i++ {
		
		// increment position
		switch dir {
			case N:
				p.y++
			case S:
				p.y--
			case E:
				p.x++
			case W:
				p.x--
			default:
				fmt.Println("No direction!")
				return
		}
		
		cur++ // increase index
		in_stride++ // increase progress in direction
		if in_stride == stride { // time to turn left
			
			if dir == N || dir == S {
				stride++
			}
			
			dir++ // E -> N -> W -> S
			if dir > 3 { dir = 0 } // because go has no enums
			
			in_stride = 0
		}
		
		
		fmt.Println("at: ", p)
		fmt.Println("value: ", cur)
		fmt.Println("heading", dir)
		
	}
	
	// abs of pos
	if p.x < 0 { p.x *= -1 }
	if p.y < 0 { p.y *= -1 }
	
	fmt.Printf("steps to %d: %d\n", find, p.x + p.y )
	
}
