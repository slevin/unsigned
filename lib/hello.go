package main

import "fmt"

var x uint64
var y uint32
var res float64

func main() {
	x := 1

	res := 60 * 60 * 24 * x * -1
	fmt.Println(res)
}
