
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
	
	fmt.Println(inputInts)
	
	iterations := 0
	
	for i := 0; i >=0 && i < len(inputInts); {
		
		jump := inputInts[i]
		inputInts[i]++
		i += jump
		
		iterations++
		
		//~ fmt.Println(inputInts)
		//~ fmt.Println("  i: ", i)
		
	}
	
	fmt.Println("iterations: ", iterations)
	
}
