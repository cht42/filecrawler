package main

import "context"

type ExecutionFn func(ctx context.Context, filename string) (string, string, error)

type Job struct {
	ExecFn   ExecutionFn
	Filename string
}

type Result struct {
	Filename string
	Sha256   string
	Err      error
}

func (j Job) execute(ctx context.Context) Result {
	filename, sha256, err := j.ExecFn(ctx, j.Filename)
	if err != nil {
		return Result{
			Err:      err,
			Filename: filename,
		}
	}
	return Result{
		Filename: filename,
		Sha256:   sha256,
	}
}
