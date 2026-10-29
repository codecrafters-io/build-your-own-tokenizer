package main

import (
	"fmt"
	"os"
)

// Usage: your_program.sh <command> <arg1> <arg2> ...
func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "usage: your_program.sh <command> [<args>...]\n")
		os.Exit(1)
	}

	switch command := os.Args[1]; command {
	case "bytes":
		data, err := os.ReadFile(os.Args[2])
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading file: %s\n", err)
			os.Exit(1)
		}
		fmt.Print("[")
		for i, b := range data {
			if i > 0 {
				fmt.Print(", ")
			}
			fmt.Print(b)
		}
		fmt.Println("]")

	default:
		fmt.Fprintf(os.Stderr, "Unknown command %s\n", command)
		os.Exit(1)
	}
}
