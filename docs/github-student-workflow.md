# GitHub And Student Workflow

## Goal

Students should be able to clone one repository, check out the exact episode state, run the project locally, and code along without guessing which version matches the video.

## Recommended workflow for publishing the series

1. Keep one repository for the whole series.
2. Keep `main` as the latest stable teaching state.
3. Create a tag for the start or end of every episode.
4. Put the matching tag link in the YouTube description and pinned comment.
5. Add a short note under `docs/episode_notes/` for each published episode.

## Suggested tag strategy

- `episode-00-v0-weak-system`
- `episode-01-timeouts`
- `episode-02-duplicate-execution`
- `episode-03-idempotency`

If you want students to start from the exact same point you start from on camera, tag the repo before recording the episode and again after finishing the code.

Example:

- `episode-01-start`
- `episode-01-finish`

## What students should do

1. Clone the repository.
2. Check out the tag that matches the lesson.
3. Create their own branch for notes or experiments.
4. Run the app locally.
5. Compare their solution to the next tag or the episode finish tag.

## Step-by-step student path

This is the simplest flow for a student arriving at the repo for the first time:

1. Open the repository on GitHub.
2. Read `README.md` first.
3. Clone the repo to the local machine.
4. Check out the tag that matches the episode.
5. Read `docs/version-0-weak-system.md`.
6. Read `docs/repo-tour.md` to understand the existing folders and files before changing anything.
7. Start the app locally with `cargo run`.
8. Confirm the service is up with `http://127.0.0.1:3000/health`.
9. Run the happy-path demo for the current platform.
10. Follow the episode and make changes on a personal branch.
11. Compare the result with the next episode tag or finish tag.

## Suggested GitHub README promises

Make these expectations obvious in the main README:

- This repo stays intentionally weak at the start.
- Each episode introduces one failure mode and one architectural improvement.
- `main` may be ahead of the current video.
- Episode tags are the safest way to follow along.

## Minimal release checklist

- Update `README.md` if the running instructions changed.
- Add an episode note under `docs/episode_notes/`.
- Tag the repo for that episode.
- Push tags to GitHub.
- Paste the exact tag link in the video description.

## Suggested commands when you are ready to publish

```bash
git add .
git commit -m "Bootstrap Rust Reliability Series v0 weak system"
git remote add origin <your-github-repo-url>
git push -u origin main
git tag episode-00-v0-weak-system
git push origin episode-00-v0-weak-system
```

If you start from a fresh folder later, run `git init -b main` before the commands above.

## Student experience rule

Do not make students reverse-engineer the repo state from the date of the video.

Always give them an exact Git tag, because reliability content already has enough complexity without version confusion.
