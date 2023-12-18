package main

import (
	"fmt"
	"os"
	"os/exec"
	"strings"

	"github.com/go-git/go-git/v5"
	"github.com/go-git/go-git/v5/plumbing"
	"github.com/go-git/go-git/v5/plumbing/object"
)

func fetchFromFork(repo *git.Repository, branch string) (*object.Commit, error) {
	// Get the username of the person who initiated the workflow run
	username := os.Getenv("GITHUB_ACTOR")

	// Get the repository name (owner/repo)
	repository := os.Getenv("GITHUB_REPOSITORY")
	parts := strings.Split(repository, "/")
	if len(parts) < 2 {
		return nil, fmt.Errorf("invalid repository format: %s", repository)
	}

	// Get the server URL: "https://github.com/" in general,
	// but can be different for GitHub Enterprise
	serverURL := os.Getenv("GITHUB_SERVER_URL")

	fmt.Printf("serverURL: %s\n", serverURL)

	forkURL := fmt.Sprintf("%s/%s/%s", serverURL, username, parts[1])

	cmd := exec.Command("git", "remote", "add", "fork", forkURL)
	err := cmd.Run()
	if err != nil {
		return nil, fmt.Errorf("error adding fork as remote: %w", err)
	}

	cmd = exec.Command("git", "fetch", "--depth", "1", "fork", branch)
	err = cmd.Run()
	if err != nil {
		return nil, fmt.Errorf("error fetching branch from fork: %w", err)
	}

	// Get the reference of the fetched branch
	refName := plumbing.ReferenceName(fmt.Sprintf("refs/remotes/fork/%s", branch))
	ref, err := repo.Reference(refName, true)
	if err != nil {
		return nil, fmt.Errorf("error getting reference: %w", err)
	}

	// Get the commit object of the fetched branch
	branchCommit, err := repo.CommitObject(ref.Hash())
	if err != nil {
		return nil, fmt.Errorf("error getting commit: %w", err)
	}

	return branchCommit, nil
}

func main() {
	branch := os.Getenv("GITHUB_HEAD_REF")

	workdir, err := os.Getwd()
	if err != nil {
		fmt.Printf("Error getting working directory: %s", err.Error())
		return
	}

	repo, err := git.PlainOpen(workdir)
	if err != nil {
		fmt.Printf("Error opening repo: %s", err.Error())
		return
	}

	commit, err := fetchFromFork(repo, branch)
	if err != nil {
		fmt.Printf("Error fetching from fork: %s", err.Error())
		return
	}

	fmt.Printf("Commit: %v\n", commit)
}
