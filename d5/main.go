
package main

import (
    "bufio"
    "fmt"
    "os"
    "strconv"
)

var inFile string = "d5.input"

func main() {
	
	file, err := os.Open(inFile)
	if err != nil {
		fmt.Fprintln(os.Stderr, "reading [", inFile, "]: ", err)
		os.Exit(1)
	}
	defer file.Close()
	
	inputInts := []int{}
	
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {

		if err := scanner.Err(); err != nil {
			fmt.Fprintln(os.Stderr, "reading [", inFile, "]: ", err)
		}
		
		toInt, err := strconv.Atoi(scanner.Text())
		if err != nil {
			fmt.Fprintln(os.Stderr, "failed to convert [", scanner.Text() , "] to int: ", err)
			os.Exit(1)
		}
		
		inputInts = append(inputInts, toInt)
		
	}
	
	fmt.Println(inputInts, "\n")
	copyInputInts := make([]int, len(inputInts))
	copy(copyInputInts, inputInts)
	
	// PART 1
	
	iterations := 0
	
	for i := 0; i >=0 && i < len(inputInts); {
		
		jump := inputInts[i]
		inputInts[i]++
		i += jump
		
		iterations++
		
	}
	
	fmt.Println("P1 iterations: ", iterations)
	
	// PART 2
	
	inputInts = copyInputInts
	iterations = 0
	
	for i := 0; i >=0 && i < len(inputInts); {
		
		jump := inputInts[i]
		if jump > 2 {
			inputInts[i]--
		} else {
			inputInts[i]++
		}
		
		i += jump
		
		iterations++
		
	}
	
	fmt.Println("P2 iterations: ", iterations)
	
}
