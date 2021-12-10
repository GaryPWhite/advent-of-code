package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	content, err := ioutil.ReadFile("path.txt")
	if err != nil {
		fmt.Println(fmt.Errorf("Err! %s", err))
	}
	inputLines := strings.Split(string(content), "\n")

	var horizontal, depth int

	for _, line := range inputLines {
		direction1value2 := strings.Split(line, " ")
		dir := direction1value2[0]
		value, err := strconv.Atoi(direction1value2[1])
		if err != nil {
			fmt.Println(fmt.Errorf("Err converting to integer: %s", err))
		}
		switch dir {
		case "forward":
			horizontal += value
			break
		case "up":
			depth -= value
			break
		case "down":
			depth += value
			break
		}
	}

	fmt.Printf("Moved forward %d, depth is %d, solution is %d\n", horizontal, depth, horizontal*depth)
}
