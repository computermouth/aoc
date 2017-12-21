
package main

import (
    "fmt"
    "os"
    "bufio"
    "strings"
    "strconv"
)

var inFile string = "d6.input"

func maxIndex ( arr []int ) int {
	m := 0
	for i := 1; i < len(arr); i++ {
		if arr[i] > arr[m] {
			m = i
		}
	}
	fmt.Println("max: ", m)
	return m
}

func main() {
	
	file, err := os.Open(inFile)
	if err != nil {
		fmt.Fprintln(os.Stderr, "reading [", inFile, "]: ", err)
		os.Exit(1)
	}
	defer file.Close()
	
	inputStrings := []string{}
	
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {

		if err := scanner.Err(); err != nil {
			fmt.Fprintln(os.Stderr, "reading [", inFile, "]: ", err)
		}
		
		onTabs := strings.Split(scanner.Text(), "\t")
		
		for i := 0; i < len(onTabs); i++ {
			inputStrings = append(inputStrings, onTabs[i])
		}
		
	}
	
	inputInts := []int{}
	
	for i := 0; i < len(inputStrings); i++ {
		
		toInt, err := strconv.Atoi(inputStrings[i])
		if err != nil {
			fmt.Fprintln(os.Stderr, "failed to convert [", scanner.Text() , "] to int: ", err)
			os.Exit(1)
		}
		
		inputInts = append(inputInts, toInt)
	}
	
	fmt.Println(inputInts)
	
	m := make(map[string]int)
	iterations := 0
	index := maxIndex(inputInts)
	
	for m[fmt.Sprintln(inputInts[:])] == 0 {
		fmt.Println("index: ", index)
		m[fmt.Sprintln(inputInts[:])] = iterations + 1
		iterations++
		
		blocks := inputInts[index]
		inputInts[index] = 0
		
		for blocks > 0 {
			
			
			if index == len(inputInts) - 1 {
				index = 0
			} else {
				index++
			}
			
			inputInts[index]++
			
			blocks--
		}
		
		fmt.Println(fmt.Sprintln(inputInts[:]))
		index = maxIndex(inputInts)
	}
	
	fmt.Println("P1 iterations: ", iterations)
	fmt.Println("P2 size: ", iterations - ( m[fmt.Sprintln(inputInts[:])] - 1))
	
	
}
