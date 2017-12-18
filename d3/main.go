
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
	
	{ // PART 1
	
		p := Pos{ x: 0, y: 0 } // start at center
		
		cur := 1 // "index"
		find := 325489 // target
		dir := E // Always head east first
		
		stride := 1 // how long to continue in direction
		in_stride := 0 // progress toward direction
		
		fmt.Println("P1: Finding ", find)
		
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
			
		}
		
		// abs of pos
		if p.x < 0 { p.x *= -1 }
		if p.y < 0 { p.y *= -1 }
		
		fmt.Printf("P1: steps to %d: %d\n\n\n", find, p.x + p.y )
	
	} // END PART 1
	
	{ // PART 2
	
		p := Pos{ x: 0, y: 0 } // start at center
		
		find := 325489 // target
		dir := E // Always head east first
		
		stride := 1 // how long to continue in direction
		in_stride := 0 // progress toward direction
		
		m := make(map[Pos]int) // map for growable/queryable 2d values
		m[Pos{x: 0, y: 0}] = 1
		
		fmt.Println("P2: Finding larger than", find)
		
		for i := 1; m[p] <= find; i++ {
			
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
			
			// add all adjacent numbers
			m[p] =
				m[Pos{x: p.x - 1, y: p.y - 1}] + // top left
				m[Pos{x: p.x    , y: p.y - 1}] + // top
				m[Pos{x: p.x + 1, y: p.y - 1}] + // top right
				m[Pos{x: p.x - 1, y: p.y    }] + // left
				m[Pos{x: p.x + 1, y: p.y    }] + // right
				m[Pos{x: p.x - 1, y: p.y + 1}] + // bot left
				m[Pos{x: p.x    , y: p.y + 1}] + // bot
				m[Pos{x: p.x + 1, y: p.y + 1}]   // bot right
			
			in_stride++ // increase progress in direction
			if in_stride == stride { // time to turn left
				
				if dir == N || dir == S {
					stride++
				}
				
				dir++ // E -> N -> W -> S
				if dir > 3 { dir = 0 } // because go has no enums
				
				in_stride = 0
			}
			
		}
		
		fmt.Println("P2: Found ", m[p])
	}
}
