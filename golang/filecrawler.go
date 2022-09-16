package main

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"time"
)

func Sha256File(ctx context.Context, filename string) (string, string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return filename, "", err
	}
	defer file.Close()

	sha := sha256.New()
	if _, err := io.Copy(sha, file); err != nil {
		return filename, "", err
	}
	return filename, hex.EncodeToString(sha.Sum(nil)), nil
}

func Visit(path string, wp *WorkerPool) {
	defer close(wp.jobs)
	var visit func(path string)
	visit = func(path string) {
		files, err := os.ReadDir(path)
		if err != nil {
			fmt.Println(err.Error())
			return
		}

		for _, file := range files {
			filePath := filepath.Join(path, file.Name())
			if file.IsDir() {
				visit(filePath)
			} else {
				wp.jobs <- Job{
					ExecFn:   Sha256File,
					Filename: filePath,
				}
			}
		}
	}
	visit(path)
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Wrong number of arguments")
		os.Exit(1)
	}

	start := time.Now()

	rootDir := os.Args[1]

	totalWorker := 16
	wp := New(totalWorker)

	ctx, cancel := context.WithCancel(context.TODO())
	defer cancel()

	go Visit(rootDir, &wp)

	go wp.Run(ctx)

outer:
	for {
		select {
		case res, ok := <-wp.Results():
			if !ok {
				continue
			}
			filename := res.Filename
			err := res.Err
			if err != nil {
				fmt.Printf("%v: error: %v\n", filename, err.Error())
			} else {
				sha256 := res.Sha256
				fmt.Printf("%v: %v\n", filename, sha256)
			}
		case <-wp.Done:
			break outer
		default:
		}
	}

	elapsed := time.Since(start)
	fmt.Printf("Took %s seconds to crawl %s", elapsed, rootDir)
}
