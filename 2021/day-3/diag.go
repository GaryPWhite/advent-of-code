package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strconv"
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
	var mostCommon, leastCommon string

	// construct gamma and epsilon values
	for i := 0; i < bytelen; i++ {
		toadd := int(math.Exp2(float64(bytelen - 1 - i)))
		// gamma gets the most common, epsilon the least common
		if count0[i] > count1[i] {
			epsilon += toadd
			mostCommon = fmt.Sprintf("%s%s", mostCommon, "0")
			leastCommon = fmt.Sprintf("%s%s", leastCommon, "1")
		}
		if count0[i] == count1[i] {
			epsilon += toadd
			gamma += toadd
			mostCommon = fmt.Sprintf("%s%s", mostCommon, "1")
			leastCommon = fmt.Sprintf("%s%s", leastCommon, "0")
		}
		if count0[i] < count1[i] {
			gamma += toadd
			mostCommon = fmt.Sprintf("%s%s", mostCommon, "1")
			leastCommon = fmt.Sprintf("%s%s", leastCommon, "0")
		}
	}

	var o2fittest, co2fittest string
	var o2fit, co2fit int

	for _, line := range inputLines {
		var lineo2fit, lineco2fit int
		lineo2 := true
		lineco2 := true
		for idx, bit := range strings.Split(line, "") {
			if !lineo2 && !lineco2 {
				break
			}
			if bit == string(mostCommon[idx]) && lineo2 {
				lineo2fit++
			} else {
				lineo2 = false
			}
			if bit == string(leastCommon[idx]) && lineco2 {
				lineco2fit++
			} else {
				lineco2 = false
			}
		}
		if lineo2fit > o2fit {
			o2fit = lineo2fit
			o2fittest = line
		}
		if lineco2fit > co2fit {
			co2fit = lineco2fit
			co2fittest = line
		}
	}

	co2scrubber := 0
	o2gen := 0
	// get the values of the binary o2 and co2 fits
	for i := 0; i < bytelen; i++ {
		multby := int(math.Exp2(float64(bytelen - 1 - i)))
		co2bit, err := strconv.Atoi(string(co2fittest[i]))
		if err != nil {
			fmt.Printf("%s", err)
		}
		o2bit, err := strconv.Atoi(string(o2fittest[i]))
		if err != nil {
			fmt.Printf("%s", err)
		}
		co2scrubber += (multby * co2bit)
		o2gen += (multby * o2bit)
	}

	fmt.Printf("gamma is %d, epsilon is %d, power consumption is %d\n", gamma, epsilon, gamma*epsilon)
	fmt.Printf("o2 fitness was %d,\n%s - most common\n%s - actual\n", o2fit, mostCommon, o2fittest)
	fmt.Printf("co2 fitness was %d,\n%s - least common\n%s - actual\n", co2fit, leastCommon, co2fittest)
	fmt.Printf("o2 generator rating: %d, co2 generator rating: %d, life support rating: %d\n", o2gen, co2scrubber, o2gen*co2scrubber)
}
