
package main

import(
	"fmt"
)

func init_input() [][]uint {

	input := [][]uint{}

	row00 := []uint{ 493, 458, 321, 120, 49, 432, 433, 92, 54, 452, 41, 461, 388, 409, 263, 58                  }
	row01 := []uint{ 961, 98, 518, 188, 958, 114, 1044, 881, 948, 590, 972, 398, 115, 116, 451, 492             }
	row02 := []uint{ 76, 783, 709, 489, 617, 72, 824, 452, 748, 737, 691, 90, 94, 77, 84, 756                   }
	row03 := []uint{ 204, 217, 90, 335, 220, 127, 302, 205, 242, 202, 259, 110, 118, 111, 200, 112              }
	row04 := []uint{ 249, 679, 4015, 106, 3358, 1642, 228, 4559, 307, 193, 4407, 3984, 3546, 2635, 3858, 924    }
	row05 := []uint{ 1151, 1060, 2002, 168, 3635, 3515, 3158, 141, 4009, 3725, 996, 142, 3672, 153, 134, 1438   }
	row06 := []uint{ 95, 600, 1171, 1896, 174, 1852, 1616, 928, 79, 1308, 2016, 88, 80, 1559, 1183, 107         }
	row07 := []uint{ 187, 567, 432, 553, 69, 38, 131, 166, 93, 132, 498, 153, 441, 451, 172, 575                }
	row08 := []uint{ 216, 599, 480, 208, 224, 240, 349, 593, 516, 450, 385, 188, 482, 461, 635, 220             }
	row09 := []uint{ 788, 1263, 1119, 1391, 1464, 179, 1200, 621, 1304, 55, 700, 1275, 226, 57, 43, 51          }
	row10 := []uint{ 1571, 58, 1331, 1253, 60, 1496, 1261, 1298, 1500, 1303, 201, 73, 1023, 582, 69, 339        }
	row11 := []uint{ 80, 438, 467, 512, 381, 74, 259, 73, 88, 448, 386, 509, 346, 61, 447, 435                  }
	row12 := []uint{ 215, 679, 117, 645, 137, 426, 195, 619, 268, 223, 792, 200, 720, 260, 303, 603             }
	row13 := []uint{ 631, 481, 185, 135, 665, 641, 492, 408, 164, 132, 478, 188, 444, 378, 633, 516             }
	row14 := []uint{ 1165, 1119, 194, 280, 223, 1181, 267, 898, 1108, 124, 618, 1135, 817, 997, 129, 227        }
	row15 := []uint{ 404, 1757, 358, 2293, 2626, 87, 613, 95, 1658, 147, 75, 930, 2394, 2349, 86, 385           }

	input = append(input, row00)
	input = append(input, row01)
	input = append(input, row02)
	input = append(input, row03)
	input = append(input, row04)
	input = append(input, row05)
	input = append(input, row06)
	input = append(input, row07)
	input = append(input, row08)
	input = append(input, row09)
	input = append(input, row10)
	input = append(input, row11)
	input = append(input, row12)
	input = append(input, row13)
	input = append(input, row14)
	input = append(input, row15)
	
	return input
	
}

func init_test() [][]uint {
	
	test := [][]uint{}
	
	row0 := []uint{ 5, 1, 9, 5}
	row1 := []uint{ 7, 5, 3,  }
	row2 := []uint{ 2, 4, 6, 8}

	test = append(test, row0)
	test = append(test, row1)
	test = append(test, row2)

	return test
}

func init_test2() [][]uint {
	
	test := [][]uint{}
	
	row0 := []uint{5, 9, 2, 8}
	row1 := []uint{9, 4, 7, 3}
	row2 := []uint{3, 8, 6, 5}

	test = append(test, row0)
	test = append(test, row1)
	test = append(test, row2)

	return test
}

func main(){
	
	data := [][]uint{}
	
	data = init_input()
	
	{	// PART 1
		
		var checksum uint = 0
		
		// for rows
		for i := 0; i < len(data); i++ {
			
			min := data[i][0]
			max := data[i][0]
			
			// for values in row
			for j := 1; j < len(data[i]); j++ {
				
				if data[i][j] < min { min = data[i][j] }
				if data[i][j] > max { max = data[i][j] }
				
			}
			
			checksum = checksum + (max - min)
			
		}
		
		fmt.Printf("p1 checksum: %d\n", checksum)
	
	}
	
	{	// PART 2
		
		var checksum uint = 0
		
		// for rows
		for i := 0; i < len(data); i++ {
			
			// for values in row
			for j := 0; j < len(data[i]); j++ {
				
				// check each pair
				for k := 0; k < len(data[i]); k++ {
					
					if k == j { continue } // don't compare to self
					
					if data[i][j] % data[i][k] == 0 {
						checksum += data[i][j] / data[i][k]
					}
					
				}
				
			}
			
		}
		
		fmt.Printf("p2 checksum: %d\n", checksum)
		
	}
	
}
