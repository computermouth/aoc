
package main

import (
	"fmt"
)

func recurse(in uint) uint {
	var empty [200]uint
	
	_ = empty
	
	in--
	
	if (in > 0) {
		in = recurse(in)
	}
	
	return in
}

func main(){
	
	fmt.Println("4: ", recurse(4))
	
	fmt.Println("4 * 1000 * 1000", recurse(4 * 1000 * 1000))
	
}
