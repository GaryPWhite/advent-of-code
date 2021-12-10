package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strings"
)

func main() {
	content, err := ioutil.ReadFile("input.txt")
	if err != nil {
		fmt.Println(fmt.Errorf("err! %s", err))
	}
	inputLines := strings.Split(string(content), "\n")
	bytelen := len(inputLines[0])

	// for each position, count number of occurances.
	count0 := make([]int, bytelen)
	count1 := make([]int, bytelen)

	for _, line := range inputLines {
		for idx, bit := range strings.Split(line, "") {
			switch bit {
			case "0":
				count0[idx] += 1
			case "1":
				count1[idx] += 1
			}
		}
	}

	var gamma, epsilon int

	// construct gamma and epsilon values
	for i := 0; i < bytelen; i++ {
		toadd := int(math.Exp2(float64(bytelen - 1 - i)))
		// gamma gets the most common, epsilon the least common
		if count0[i] > count1[i] {
			epsilon += toadd
		}
		if count0[i] == count1[i] {
			epsilon += toadd
			gamma += toadd
		}
		if count0[i] < count1[i] {
			gamma += toadd
		}
	}

	fmt.Printf("gamma is %d, epsilon is %d, power consumption is %d\n", gamma, epsilon, gamma*epsilon)
}
